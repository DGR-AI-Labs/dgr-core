//! RED CORE-004 conformance scaffolding.
//!
//! These tests freeze the T3-observable ATK-06 contract without creating the
//! founder-owned approval port, durable store, observation variant, escalation
//! decision, or R-3 timeout evaluator. Required CI compiles this file while the
//! dedicated CORE-004 cases remain ignored. Run the ignored cases explicitly to
//! observe RED until the founder-authored T0 surfaces exist.

use dgr_core_bypass_harness::RequiredOutcome;
use dgr_core_bypass_harness::attack_by_id;
use dgr_core_bypass_harness::before_tool_call::{
    BeforeToolCallAdapter, BeforeToolCallObservation, EffectfulToolProbe,
};
use dgr_core_bypass_harness::fixtures::RecordingToolProbe;
use dgr_core_bypass_harness::founder_authored_guard::FounderAuthoredGuard;
use dgr_core_bypass_harness::founder_s2_consumption_store::S2ConsumptionStore;
use dgr_core_bypass_harness::val_002_fixtures::FixtureClock;
use dgr_core_bypass_harness::val_004_fixtures::{
    ATTACK_ID, ExpectedVal004Observation, FIXED_DEADLINE, FIXED_REVIEW_REQUEST_ID,
    NonceExpectation, PendingApprovalFacts, PendingRecordExpectation, Val004Fixture,
    before_tool_call_request, fixture_catalog,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FakeApprovalPrecondition {
    NoPendingRecord,
    Existing(PendingApprovalFacts),
}

/// Test-only scenario state. It records inputs and does not implement approval
/// persistence, lookup, transition, policy, or an observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct FakeApprovalState {
    precondition: FakeApprovalPrecondition,
    approval_received: bool,
}

impl FakeApprovalState {
    const fn no_pending_record() -> Self {
        Self {
            precondition: FakeApprovalPrecondition::NoPendingRecord,
            approval_received: false,
        }
    }

    const fn unanswered(pending: PendingApprovalFacts) -> Self {
        Self {
            precondition: FakeApprovalPrecondition::Existing(pending),
            approval_received: false,
        }
    }
}

fn fixture(id: &str) -> Val004Fixture {
    fixture_catalog()
        .by_id(id)
        .unwrap_or_else(|| panic!("missing VAL-004 fixture {id}"))
        .clone()
}

fn pending(fixture: &Val004Fixture) -> PendingApprovalFacts {
    fixture
        .pending
        .unwrap_or_else(|| panic!("{} must carry pending facts", fixture.id))
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

fn assert_future_escalated_observation(
    observed: BeforeToolCallObservation,
    tool: &RecordingToolProbe,
) {
    assert_eq!(
        tool.invocation_count(),
        0,
        "ATK-06 RED: escalation reached the effectful probe"
    );

    // `Escalated` is a founder-owned T0 observation variant and deliberately
    // does not exist yet. The wildcard becomes reachable only after that
    // variant is authored; every currently available terminal observation is
    // explicitly rejected.
    #[allow(unreachable_patterns)]
    match observed {
        BeforeToolCallObservation::Blocked { .. } => {
            panic!("ATK-06 RED: the escalation moment was collapsed into Blocked")
        }
        BeforeToolCallObservation::Proceeded { .. } => {
            panic!("ATK-06 RED: an unanswered approval request proceeded")
        }
        BeforeToolCallObservation::GuardFault { .. } => {
            panic!("ATK-06 RED: the escalation moment exposed a raw guard fault")
        }
        _ => {}
    }
}

fn founder_timeout_surface_pending(
    state: FakeApprovalState,
    now_unix_seconds: u64,
    expected: ExpectedVal004Observation,
) -> ! {
    panic!(
        "CORE-004 RED: founder R-3 timeout evaluator is absent; state={state:?}, now={now_unix_seconds}, expected={expected:?}"
    )
}

#[test]
fn below_threshold_control_retains_the_existing_consume_and_allow_path() {
    let fixture = fixture("below-threshold-control");
    let case = attack_by_id(ATTACK_ID).expect("ATK-06 is registered");
    let request = before_tool_call_request(case, &fixture).expect("token-bearing control");
    let adapter = BeforeToolCallAdapter::new(FounderAuthoredGuard);
    let mut store = S2ConsumptionStore::open_in_memory().expect("store");
    let mut tool = RecordingToolProbe::default();

    let observed = adapter.before_tool_call(
        &request,
        fixture.clock.now_unix_seconds(),
        &mut store,
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
#[ignore = "CORE-004-T0-boundary pending founder escalation authoring"]
fn atk_06_above_threshold_before_tool_call_requires_escalation() {
    let fixture = fixture("escalation-required");
    let case = attack_by_id(ATTACK_ID).expect("ATK-06 is registered");
    let request = before_tool_call_request(case, &fixture).expect("token-bearing escalation");
    let state = FakeApprovalState::no_pending_record();
    let adapter = BeforeToolCallAdapter::new(FounderAuthoredGuard);
    let mut store = S2ConsumptionStore::open_in_memory().expect("consumption store");
    let mut tool = RecordingToolProbe::default();

    assert_eq!(
        state.precondition,
        FakeApprovalPrecondition::NoPendingRecord
    );
    assert!(!state.approval_received);
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
        &mut tool,
    );
    assert_future_escalated_observation(observed, &tool);
}

#[test]
#[ignore = "CORE-004-T0-boundary pending founder R-3 timeout evaluator"]
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

    founder_timeout_surface_pending(
        FakeApprovalState::unanswered(pending(&timeout)),
        timeout.clock.now_unix_seconds(),
        timeout.expected,
    );
}

#[test]
#[ignore = "CORE-004-T0-boundary pending founder R-3 timeout evaluator"]
fn atk_06_timeout_boundary_preserves_id_and_deadline_until_strictly_after() {
    for id in ["timeout-deadline-minus-one", "timeout-at-deadline"] {
        let fixture = fixture(id);
        let pending = pending(&fixture);

        assert_eq!(pending.review_request_id, FIXED_REVIEW_REQUEST_ID);
        assert_eq!(pending.deadline, FIXED_DEADLINE);
        assert!(fixture.clock.now_unix_seconds() <= pending.deadline);
        assert_escalated_expectation(&fixture);
    }

    let timed_out = fixture("timeout-deadline-plus-one");
    let pending = pending(&timed_out);
    assert_eq!(pending.review_request_id, FIXED_REVIEW_REQUEST_ID);
    assert_eq!(pending.deadline, FIXED_DEADLINE);
    assert_eq!(timed_out.clock.now_unix_seconds(), pending.deadline + 1);

    founder_timeout_surface_pending(
        FakeApprovalState::unanswered(pending),
        timed_out.clock.now_unix_seconds(),
        timed_out.expected,
    );
}

#[test]
#[ignore = "CORE-004-T0-boundary pending founder escalation authoring"]
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
        &mut first_tool,
    );
    assert_future_escalated_observation(first, &first_tool);

    let mut second_tool = RecordingToolProbe::default();
    let second = adapter.before_tool_call(
        &repeated_request,
        repeated.clock.now_unix_seconds(),
        &mut store,
        &mut second_tool,
    );
    assert_future_escalated_observation(second, &second_tool);
}
