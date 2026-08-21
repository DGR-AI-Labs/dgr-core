//! Live CORE-004 two-surface conformance coverage.
//!
//! Token presentation exercises durable escalation without consuming the nonce.
//! Token-free timeout evaluation then re-observes the original pending request
//! through its deadline and commits the registry-derived denial after it.

use dgr_core_bypass_harness::RequiredOutcome;
use dgr_core_bypass_harness::attack_by_id;
use dgr_core_bypass_harness::before_tool_call::{
    BeforeToolCallAdapter, BeforeToolCallObservation, EffectfulToolProbe, GuardDecision,
};
use dgr_core_bypass_harness::fixtures::{FailClosedApprovalStore, RecordingToolProbe};
use dgr_core_bypass_harness::founder_approval_store::ReviewRequestId;
use dgr_core_bypass_harness::founder_approval_timeout::evaluate_approval_timeout;
use dgr_core_bypass_harness::founder_authored_guard::FounderAuthoredGuard;
use dgr_core_bypass_harness::founder_consumption_store::{ConsumeOutcome, ConsumptionStore};
use dgr_core_bypass_harness::founder_s2_approval_store::S2ApprovalStore;
use dgr_core_bypass_harness::founder_s2_consumption_store::S2ConsumptionStore;
use dgr_core_bypass_harness::val_002_fixtures::FixtureClock;
use dgr_core_bypass_harness::val_004_fixtures::{
    ATTACK_ID, ExpectedVal004Observation, FIXED_DEADLINE, FIXED_REVIEW_REQUEST_ID,
    NonceExpectation, PendingRecordExpectation, Val004Fixture, before_tool_call_request,
    fixture_catalog,
};

fn fixture(id: &str) -> Val004Fixture {
    fixture_catalog()
        .by_id(id)
        .unwrap_or_else(|| panic!("missing VAL-004 fixture {id}"))
        .clone()
}

fn assert_escalated_expectation(fixture: &Val004Fixture) {
    assert_eq!(
        fixture.expected,
        ExpectedVal004Observation::Escalated {
            review_request_id: FIXED_REVIEW_REQUEST_ID,
            deadline: FIXED_DEADLINE,
            authorization_issued: false,
            effectful_invocations: 0,
        }
    );
}

fn assert_escalated_observation(
    observed: BeforeToolCallObservation,
    fixture: &Val004Fixture,
    tool: &RecordingToolProbe,
) -> ReviewRequestId {
    assert_eq!(
        tool.invocation_count(),
        0,
        "ATK-06 escalation reached the effectful probe"
    );

    let expected_deadline = fixture
        .pending
        .unwrap_or_else(|| panic!("{} must carry pending facts", fixture.id))
        .deadline;

    match observed {
        BeforeToolCallObservation::Escalated {
            review_request_id,
            deadline,
            authorization_issued: false,
            effectful_invocations: 0,
        } => {
            assert_eq!(deadline, expected_deadline);
            review_request_id
        }
        other => {
            panic!("ATK-06 expected Escalated without authorization or effects, got {other:?}")
        }
    }
}

fn pending_approval_store() -> (S2ApprovalStore, ReviewRequestId, u64) {
    let escalation = fixture("escalation-required");
    let case = attack_by_id(ATTACK_ID).expect("ATK-06 is registered");
    let request = before_tool_call_request(case, &escalation).expect("token-bearing escalation");
    let adapter = BeforeToolCallAdapter::new(FounderAuthoredGuard);
    let mut consumption_store = S2ConsumptionStore::open_in_memory().expect("consumption store");
    let mut approval_store = S2ApprovalStore::open_in_memory().expect("approval store");
    let mut tool = RecordingToolProbe::default();

    let observed = adapter.before_tool_call(
        &request,
        escalation.clock.now_unix_seconds(),
        &mut consumption_store,
        &mut approval_store,
        &mut tool,
    );
    let review_request_id = assert_escalated_observation(observed, &escalation, &tool);

    (approval_store, review_request_id, FIXED_DEADLINE)
}

#[test]
fn below_threshold_control_retains_the_existing_consume_and_allow_path() {
    let fixture = fixture("below-threshold-control");
    let case = attack_by_id(ATTACK_ID).expect("ATK-06 is registered");
    let request = before_tool_call_request(case, &fixture).expect("token-bearing control");
    let adapter = BeforeToolCallAdapter::new(FounderAuthoredGuard);
    let mut store = S2ConsumptionStore::open_in_memory().expect("store");
    let mut approval_store = FailClosedApprovalStore;
    let mut tool = RecordingToolProbe::default();

    let observed = adapter.before_tool_call(
        &request,
        fixture.clock.now_unix_seconds(),
        &mut store,
        &mut approval_store,
        &mut tool,
    );

    assert_eq!(
        observed,
        BeforeToolCallObservation::Proceeded {
            authorization_reference: "CORE-002 authorized",
            authorization_issued: true,
            effectful_invocations: 1,
        }
    );
    assert_eq!(tool.invocation_count(), 1);
}

#[test]
fn atk_06_above_threshold_before_tool_call_requires_escalation() {
    let fixture = fixture("escalation-required");
    let case = attack_by_id(ATTACK_ID).expect("ATK-06 is registered");
    let request = before_tool_call_request(case, &fixture).expect("token-bearing escalation");
    let adapter = BeforeToolCallAdapter::new(FounderAuthoredGuard);
    let mut store = S2ConsumptionStore::open_in_memory().expect("consumption store");
    let mut approval_store = S2ApprovalStore::open_in_memory().expect("approval store");
    let mut tool = RecordingToolProbe::default();

    assert_eq!(
        fixture.pending_record_expectation,
        Some(PendingRecordExpectation::Recorded)
    );
    assert_eq!(
        fixture.nonce_expectation,
        Some(NonceExpectation::RemainsUnconsumed)
    );
    assert_escalated_expectation(&fixture);

    let observed = adapter.before_tool_call(
        &request,
        fixture.clock.now_unix_seconds(),
        &mut store,
        &mut approval_store,
        &mut tool,
    );
    assert_escalated_observation(observed, &fixture, &tool);
    let nonce = fixture.token.as_ref().expect("escalation token").nonce;
    assert_eq!(store.consume(&nonce), ConsumeOutcome::Consumed);
}

#[test]
fn atk_06_sequence_is_escalated_then_registry_derived_timeout_block() {
    let escalation = fixture("escalation-required");
    let timeout = fixture("timeout-deadline-plus-one");
    let case = attack_by_id(ATTACK_ID).expect("ATK-06 is registered");
    let expected_terminal = case.expected;

    assert_eq!(
        expected_terminal,
        RequiredOutcome::EscalateThenDenyOnTimeout
    );
    assert_escalated_expectation(&escalation);
    assert_eq!(
        timeout.expected,
        ExpectedVal004Observation::BlockedFromRegistry {
            attack_id: case.id,
            authorization_issued: false,
            effectful_invocations: 0,
        }
    );
    assert_eq!(
        [escalation.expected, timeout.expected],
        [
            ExpectedVal004Observation::Escalated {
                review_request_id: FIXED_REVIEW_REQUEST_ID,
                deadline: FIXED_DEADLINE,
                authorization_issued: false,
                effectful_invocations: 0,
            },
            ExpectedVal004Observation::BlockedFromRegistry {
                attack_id: case.id,
                authorization_issued: false,
                effectful_invocations: 0,
            },
        ]
    );

    let (mut approval_store, review_request_id, _) = pending_approval_store();
    let observed = evaluate_approval_timeout(
        &mut approval_store,
        &review_request_id,
        timeout.clock.now_unix_seconds(),
    );

    assert_eq!(
        observed,
        Ok(GuardDecision::Deny {
            outcome: expected_terminal,
            denial_signal: "ATK-06 approval timed out",
        })
    );
}

#[test]
fn atk_06_timeout_boundary_preserves_id_and_deadline_until_strictly_after() {
    for id in ["timeout-deadline-minus-one", "timeout-at-deadline"] {
        let fixture = fixture(id);
        let (mut approval_store, review_request_id, deadline) = pending_approval_store();

        assert_eq!(deadline, FIXED_DEADLINE);
        assert!(fixture.clock.now_unix_seconds() <= deadline);
        assert_escalated_expectation(&fixture);

        assert_eq!(
            evaluate_approval_timeout(
                &mut approval_store,
                &review_request_id,
                fixture.clock.now_unix_seconds(),
            ),
            Ok(GuardDecision::Escalate {
                review_request_id,
                deadline,
            })
        );
    }

    let timed_out = fixture("timeout-deadline-plus-one");
    let (mut approval_store, review_request_id, deadline) = pending_approval_store();
    assert_eq!(deadline, FIXED_DEADLINE);
    assert_eq!(timed_out.clock.now_unix_seconds(), deadline + 1);

    assert_eq!(
        evaluate_approval_timeout(
            &mut approval_store,
            &review_request_id,
            timed_out.clock.now_unix_seconds(),
        ),
        Ok(GuardDecision::Deny {
            outcome: RequiredOutcome::EscalateThenDenyOnTimeout,
            denial_signal: "ATK-06 approval timed out",
        })
    );
}

#[test]
fn atk_06_re_presentation_keeps_original_pending_facts_and_unconsumed_nonce() {
    let catalog = fixture_catalog();
    let original = catalog
        .by_id("escalation-required")
        .expect("original escalation");
    let repeated = catalog
        .by_id("re-presentation-does-not-extend")
        .expect("re-presentation");
    let case = attack_by_id(ATTACK_ID).expect("ATK-06 is registered");
    let original_request =
        before_tool_call_request(case, original).expect("original token presentation");
    let repeated_request =
        before_tool_call_request(case, repeated).expect("re-presented token presentation");
    let adapter = BeforeToolCallAdapter::new(FounderAuthoredGuard);
    let mut store = S2ConsumptionStore::open_in_memory().expect("consumption store");
    let mut approval_store = S2ApprovalStore::open_in_memory().expect("approval store");

    assert_eq!(repeated.token, original.token);
    assert_eq!(repeated.pending, original.pending);
    assert_eq!(
        repeated.pending_record_expectation,
        Some(PendingRecordExpectation::AlreadyPending)
    );
    assert_eq!(
        repeated.nonce_expectation,
        Some(NonceExpectation::RemainsUnconsumed)
    );
    assert_escalated_expectation(original);
    assert_escalated_expectation(repeated);

    let mut first_tool = RecordingToolProbe::default();
    let first = adapter.before_tool_call(
        &original_request,
        original.clock.now_unix_seconds(),
        &mut store,
        &mut approval_store,
        &mut first_tool,
    );
    let first_review_request_id = assert_escalated_observation(first, original, &first_tool);

    let mut second_tool = RecordingToolProbe::default();
    let second = adapter.before_tool_call(
        &repeated_request,
        repeated.clock.now_unix_seconds(),
        &mut store,
        &mut approval_store,
        &mut second_tool,
    );
    let second_review_request_id = assert_escalated_observation(second, repeated, &second_tool);
    assert_eq!(second_review_request_id, first_review_request_id);
    let nonce = original.token.as_ref().expect("original token").nonce;
    assert_eq!(store.consume(&nonce), ConsumeOutcome::Consumed);
}
