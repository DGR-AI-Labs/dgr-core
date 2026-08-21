use dgr_core_bypass_harness::before_tool_call::{
    BeforeToolCallAdapter, BeforeToolCallObservation, BeforeToolCallRequest, EffectfulToolProbe,
    GuardDecision, GuardDecisionPort, GuardFault, OpaqueCapabilityToken,
};
use dgr_core_bypass_harness::fixtures::{
    FailClosedApprovalStore, RecordingToolProbe, no_token_request, request_for_attack,
    request_for_val_002_fixture,
};

use dgr_core_bypass_harness::val_002_fixtures::{
    FIXED_NOW_UNIX_SECONDS, FixtureClock, fixture_catalog,
};

use dgr_core_bypass_harness::founder_approval_store::ApprovalStore;
use dgr_core_bypass_harness::founder_authored_guard::FounderAuthoredGuard;
use dgr_core_bypass_harness::founder_consumption_store::{ConsumeOutcome, ConsumptionStore};
use dgr_core_bypass_harness::founder_s2_consumption_store::S2ConsumptionStore;
use dgr_core_bypass_harness::{ATTACK_SET, HarnessTarget, RequiredOutcome, attack_by_id};
use std::panic::{AssertUnwindSafe, catch_unwind};

fn observe_val_002_fixture(fixture_id: &str, attack_id: &str) -> BeforeToolCallObservation {
    let catalog = fixture_catalog();
    let fixture = catalog.by_id(fixture_id).expect("VAL-002 fixture");
    let case = attack_by_id(attack_id).expect("registered attack");
    let request = request_for_val_002_fixture(case, fixture);
    let adapter = BeforeToolCallAdapter::new(FounderAuthoredGuard);
    let mut store = S2ConsumptionStore::open_in_memory().expect("store");
    let mut approval_store = FailClosedApprovalStore;
    let mut tool = RecordingToolProbe::default();

    adapter.before_tool_call(
        &request,
        fixture.clock.now_unix_seconds(),
        &mut store,
        &mut approval_store,
        &mut tool,
    )
}

fn required_outcome(attack_id: &str) -> RequiredOutcome {
    attack_by_id(attack_id)
        .unwrap_or_else(|| panic!("{attack_id} must be registered"))
        .expected
}

#[test]
fn atk_10_unknown_key_is_denied() {
    assert_eq!(
        observe_val_002_fixture("unknown-key-id", "ATK-10"),
        BeforeToolCallObservation::Blocked {
            outcome: required_outcome("ATK-10"),
            denial_signal: "ATK-10 unknown key id",
            authorization_issued: false,
            effectful_invocations: 0,
        }
    );
}

#[test]
fn atk_10_tampered_expiry_fails_signature_verification() {
    assert_eq!(
        observe_val_002_fixture("tampered-expires-at", "ATK-10"),
        BeforeToolCallObservation::Blocked {
            outcome: required_outcome("ATK-10"),
            denial_signal: "ATK-10 invalid signature",
            authorization_issued: false,
            effectful_invocations: 0,
        }
    );
}

#[test]
fn valid_token_is_authorized() {
    assert_eq!(
        observe_val_002_fixture("valid", "ATK-11"),
        BeforeToolCallObservation::Proceeded {
            authorization_reference: "CORE-002 authorized",
            authorization_issued: true,
            effectful_invocations: 1,
        }
    );
}

#[test]
fn atk_10_malformed_length_is_denied_before_parsing() {
    let catalog = fixture_catalog();
    let valid = catalog.by_id("valid").expect("valid fixture");
    let wire = &valid.token.as_ref().expect("valid token").wire_bytes;
    let malformed = &wire[..wire.len() - 1];
    let case = attack_by_id("ATK-10").expect("ATK-10");
    let request = BeforeToolCallRequest {
        proposed_action: case.proposed_action,
        context: &case.context,
        capability_token: Some(OpaqueCapabilityToken { bytes: malformed }),
    };
    let adapter = BeforeToolCallAdapter::new(FounderAuthoredGuard);
    let mut store = S2ConsumptionStore::open_in_memory().expect("store");
    let mut approval_store = FailClosedApprovalStore;
    let mut tool = RecordingToolProbe::default();

    let observed = adapter.before_tool_call(
        &request,
        FIXED_NOW_UNIX_SECONDS,
        &mut store,
        &mut approval_store,
        &mut tool,
    );
    assert_eq!(tool.invocation_count(), 0);
    assert_eq!(
        observed,
        BeforeToolCallObservation::Blocked {
            outcome: case.expected,
            denial_signal: "ATK-10 malformed token length",
            authorization_issued: false,
            effectful_invocations: 0,
        }
    );
}

#[test]
fn registry_contains_the_complete_core_001_id_set() {
    let actual: Vec<_> = ATTACK_SET.iter().map(|case| case.id).collect();
    let expected = vec![
        "ATK-01", "ATK-02", "ATK-03", "ATK-04", "ATK-05", "ATK-06", "ATK-07", "ATK-08", "ATK-09",
        "ATK-10", "ATK-11", "ATK-12", "ATK-13", "ATK-14", "ATK-15",
    ];

    assert_eq!(actual, expected);
}

#[test]
fn registry_has_unique_ids_and_closed_expected_outcomes() {
    for (index, case) in ATTACK_SET.iter().enumerate() {
        assert!(
            ATTACK_SET[..index].iter().all(|prior| prior.id != case.id),
            "duplicate attack id {}",
            case.id
        );
        assert!(
            matches!(
                case.expected,
                RequiredOutcome::Block
                    | RequiredOutcome::Deny
                    | RequiredOutcome::EscalateThenDenyOnTimeout
                    | RequiredOutcome::FailClosed
            ),
            "{} has a non-closed expected outcome",
            case.id
        );
        assert!(
            !case.srs07_trace.is_empty(),
            "{} has no SRS-07 trace",
            case.id
        );
    }
}

fn assert_gate_case(id: &str) {
    let case = attack_by_id(id).expect("registered attack");
    assert_ne!(
        case.target,
        HarnessTarget::ExternalIam,
        "external IAM assertions must not be simulated by the gate"
    );
    let adapter = BeforeToolCallAdapter::new(FounderAuthoredGuard);
    let mut store = S2ConsumptionStore::open_in_memory().expect("store");
    let mut approval_store = FailClosedApprovalStore;
    let mut tool = RecordingToolProbe::default();
    let observed = adapter.before_tool_call(
        &request_for_attack(case),
        FIXED_NOW_UNIX_SECONDS,
        &mut store,
        &mut approval_store,
        &mut tool,
    );

    match observed {
        BeforeToolCallObservation::Blocked {
            outcome,
            authorization_issued,
            effectful_invocations,
            ..
        } => {
            assert_eq!(outcome, case.expected, "{id} outcome");
            assert!(!authorization_issued, "{id} issued authorization");
            assert_eq!(effectful_invocations, 0, "{id} reached the tool");
        }
        other => panic!("{id} did not emit the required denial signal: {other:?}"),
    }
}

#[derive(Clone, Copy)]
struct Atk07FaultingGuard;

impl GuardDecisionPort for Atk07FaultingGuard {
    fn decide(
        &self,
        _request: &BeforeToolCallRequest<'_>,
        _now_unix_seconds: u64,
        _store: &mut dyn ConsumptionStore,
        _approval_store: &mut dyn ApprovalStore,
    ) -> Result<GuardDecision, GuardFault> {
        Err(GuardFault::InternalError)
    }
}

#[derive(Clone, Copy)]
struct Atk07PanickingGuard;

impl GuardDecisionPort for Atk07PanickingGuard {
    fn decide(
        &self,
        _request: &BeforeToolCallRequest<'_>,
        _now_unix_seconds: u64,
        _store: &mut dyn ConsumptionStore,
        _approval_store: &mut dyn ApprovalStore,
    ) -> Result<GuardDecision, GuardFault> {
        panic!("forced ATK-07 hook failure")
    }
}

fn assert_atk_07_fail_closed(observed: BeforeToolCallObservation, tool: &RecordingToolProbe) {
    assert_eq!(
        tool.invocation_count(),
        0,
        "ATK-07 reached the effectful probe"
    );

    match observed {
        BeforeToolCallObservation::Blocked {
            outcome,
            authorization_issued,
            effectful_invocations,
            ..
        } => {
            assert_eq!(outcome, required_outcome("ATK-07"));
            assert!(!authorization_issued, "ATK-07 issued authorization");
            assert_eq!(effectful_invocations, 0, "ATK-07 reached the tool");
        }
        BeforeToolCallObservation::Escalated { .. } => {
            panic!("ATK-07 RED: a faulting boundary emitted an escalation")
        }
        BeforeToolCallObservation::Proceeded { .. } => {
            panic!("ATK-07 RED: a faulting boundary proceeded to the tool")
        }
        BeforeToolCallObservation::GuardFault { .. } => {
            panic!("ATK-07 RED: the boundary exposed a raw guard fault instead of blocking")
        }
    }
}

// Active adversarial checks for the founder-authored CORE-003 T0 boundary.
#[test]
fn atk_07_typed_guard_fault_requires_fail_closed_floor() {
    let case = attack_by_id("ATK-07").expect("ATK-07 is registered");
    let request = request_for_attack(case);
    let adapter = BeforeToolCallAdapter::new(Atk07FaultingGuard);
    let mut store = S2ConsumptionStore::open_in_memory().expect("store");
    let mut approval_store = FailClosedApprovalStore;
    let mut tool = RecordingToolProbe::default();

    let observed = adapter.before_tool_call(
        &request,
        FIXED_NOW_UNIX_SECONDS,
        &mut store,
        &mut approval_store,
        &mut tool,
    );

    assert_atk_07_fail_closed(observed, &tool);
}

#[test]
fn atk_07_guard_panic_requires_fail_closed_floor() {
    let case = attack_by_id("ATK-07").expect("ATK-07 is registered");
    let request = request_for_attack(case);
    let adapter = BeforeToolCallAdapter::new(Atk07PanickingGuard);
    let mut store = S2ConsumptionStore::open_in_memory().expect("store");
    let mut approval_store = FailClosedApprovalStore;
    let mut tool = RecordingToolProbe::default();

    let result = catch_unwind(AssertUnwindSafe(|| {
        adapter.before_tool_call(
            &request,
            FIXED_NOW_UNIX_SECONDS,
            &mut store,
            &mut approval_store,
            &mut tool,
        )
    }));

    assert_eq!(
        tool.invocation_count(),
        0,
        "ATK-07 panic reached the effectful probe"
    );
    let observed = result
        .unwrap_or_else(|_| panic!("ATK-07 RED: the boundary allowed a guard panic to escape"));
    assert_atk_07_fail_closed(observed, &tool);
}

fn assert_external_iam_case_is_not_a_gate_test(id: &str) {
    let case = attack_by_id(id).expect("registered attack");
    assert_eq!(case.target, HarnessTarget::ExternalIam);
    panic!(
        "{id} requires the hosted IAM assertion suite; CORE-001 must not simulate it in the gate"
    );
}

macro_rules! ignored_gate_attack {
    ($test_name:ident, $id:literal) => {
        #[test]
        #[ignore = "waiting for reviewed CORE-002..005 enforcement/test adapter"]
        fn $test_name() {
            assert_gate_case($id);
        }
    };
}

#[test]
fn no_token_guard_cannot_reach_the_effectful_probe() {
    let request = no_token_request();
    let adapter = BeforeToolCallAdapter::new(FounderAuthoredGuard);
    let mut store = S2ConsumptionStore::open_in_memory().expect("store");
    let mut approval_store = FailClosedApprovalStore;
    let mut tool = RecordingToolProbe::default();

    let result = catch_unwind(AssertUnwindSafe(|| {
        adapter.before_tool_call(
            &request,
            FIXED_NOW_UNIX_SECONDS,
            &mut store,
            &mut approval_store,
            &mut tool,
        )
    }));

    assert_eq!(
        tool.invocation_count(),
        0,
        "the no-token guard reached the effectful probe"
    );
    if let Ok(observed) = result {
        assert!(
            matches!(observed, BeforeToolCallObservation::Blocked { .. }),
            "an implemented no-token guard must block before the probe"
        );
    }
}

#[test]
fn atk_01_no_authorization_is_blocked_before_tool_execution() {
    let case = attack_by_id("ATK-01").expect("ATK-01 is registered");
    assert_eq!(case.target, HarnessTarget::Gate);

    let request = no_token_request();
    assert!(request.capability_token.is_none());

    let adapter = BeforeToolCallAdapter::new(FounderAuthoredGuard);
    let mut store = S2ConsumptionStore::open_in_memory().expect("store");
    let mut approval_store = FailClosedApprovalStore;
    let mut tool = RecordingToolProbe::default();

    let observed = adapter.before_tool_call(
        &request,
        FIXED_NOW_UNIX_SECONDS,
        &mut store,
        &mut approval_store,
        &mut tool,
    );

    assert_eq!(
        tool.invocation_count(),
        0,
        "the effectful tool ran before the guard returned a decision"
    );
    assert_eq!(
        observed,
        BeforeToolCallObservation::Blocked {
            outcome: case.expected,
            denial_signal: "ATK-01 no valid capability token",
            authorization_issued: false,
            effectful_invocations: 0,
        },
        "ATK-01 RED: an absent or erroring guard must emit a denial signal, never allow"
    );
}

#[test]
fn atk_02_expired_beyond_skew_is_denied() {
    assert_eq!(
        observe_val_002_fixture("expired-beyond-skew", "ATK-02"),
        BeforeToolCallObservation::Blocked {
            outcome: required_outcome("ATK-02"),
            denial_signal: "ATK-02 expired capability token",
            authorization_issued: false,
            effectful_invocations: 0,
        }
    );
}

#[test]
fn atk_02_at_skew_boundary_is_authorized() {
    assert_eq!(
        observe_val_002_fixture("expired-within-skew", "ATK-02"),
        BeforeToolCallObservation::Proceeded {
            authorization_reference: "CORE-002 authorized",
            authorization_issued: true,
            effectful_invocations: 1,
        }
    );
}

#[test]
fn atk_02_just_outside_skew_is_denied() {
    assert_eq!(
        observe_val_002_fixture("expired-just-outside-skew", "ATK-02"),
        BeforeToolCallObservation::Blocked {
            outcome: required_outcome("ATK-02"),
            denial_signal: "ATK-02 expired capability token",
            authorization_issued: false,
            effectful_invocations: 0,
        }
    );
}

#[test]
fn atk_02_overlong_lifetime_is_denied() {
    assert_eq!(
        observe_val_002_fixture("lifetime-over-maximum", "ATK-02"),
        BeforeToolCallObservation::Blocked {
            outcome: required_outcome("ATK-02"),
            denial_signal: "ATK-02 invalid capability token lifetime",
            authorization_issued: false,
            effectful_invocations: 0,
        }
    );
}

#[test]
fn atk_02_reversed_lifetime_is_denied() {
    assert_eq!(
        observe_val_002_fixture("lifetime-reversed", "ATK-02"),
        BeforeToolCallObservation::Blocked {
            outcome: required_outcome("ATK-02"),
            denial_signal: "ATK-02 invalid capability token lifetime",
            authorization_issued: false,
            effectful_invocations: 0,
        }
    );
}

fn assert_binding_denied(fixture_id: &str, attack_id: &str) {
    assert_eq!(
        observe_val_002_fixture(fixture_id, attack_id),
        BeforeToolCallObservation::Blocked {
            outcome: required_outcome(attack_id),
            denial_signal: "ATK-08/09/11 action commitment mismatch",
            authorization_issued: false,
            effectful_invocations: 0,
        }
    );
}

#[test]
fn atk_08_scope_escalation_is_denied() {
    for fixture_id in [
        "swap-amount",
        "swap-destination",
        "swap-source-account",
        "malformed-amount-decimal",
        "malformed-amount-leading-zero",
    ] {
        assert_binding_denied(fixture_id, "ATK-08");
    }
}

#[test]
fn atk_09_token_substitution_is_denied() {
    assert_binding_denied("swap-invoice-id", "ATK-09");
}

#[test]
fn atk_11_parameter_swap_is_denied() {
    assert_binding_denied("wrong-action", "ATK-11");
}

#[test]
fn nonbinding_changes_are_authorized() {
    for fixture_id in ["change-idempotency-key", "change-memo"] {
        assert_eq!(
            observe_val_002_fixture(fixture_id, "ATK-11"),
            BeforeToolCallObservation::Proceeded {
                authorization_reference: "CORE-002 authorized",
                authorization_issued: true,
                effectful_invocations: 1,
            }
        );
    }
}

#[test]
fn atk_03_replayed_token() {
    let catalog = fixture_catalog();
    let fixture = catalog.by_id("replay").expect("replay fixture");
    let case = attack_by_id("ATK-03").expect("ATK-03 is registered");
    let request = request_for_val_002_fixture(case, fixture);
    let adapter = BeforeToolCallAdapter::new(FounderAuthoredGuard);
    let mut store = S2ConsumptionStore::open_in_memory().expect("store");
    let mut approval_store = FailClosedApprovalStore;

    let mut first_tool = RecordingToolProbe::default();
    let first = adapter.before_tool_call(
        &request,
        fixture.clock.now_unix_seconds(),
        &mut store,
        &mut approval_store,
        &mut first_tool,
    );

    let mut second_tool = RecordingToolProbe::default();
    let second = adapter.before_tool_call(
        &request,
        fixture.clock.now_unix_seconds(),
        &mut store,
        &mut approval_store,
        &mut second_tool,
    );

    assert_eq!(
        first,
        BeforeToolCallObservation::Proceeded {
            authorization_reference: "CORE-002 authorized",
            authorization_issued: true,
            effectful_invocations: 1,
        }
    );
    assert_eq!(first_tool.invocation_count(), 1);

    assert_eq!(
        second,
        BeforeToolCallObservation::Blocked {
            outcome: required_outcome("ATK-03"),
            denial_signal: "ATK-03 replayed capability token",
            authorization_issued: false,
            effectful_invocations: 0,
        }
    );
    assert_eq!(second_tool.invocation_count(), 0);
}

struct FaultingConsumptionStore;

impl ConsumptionStore for FaultingConsumptionStore {
    fn consume(&mut self, _authorization_reference: &[u8]) -> ConsumeOutcome {
        ConsumeOutcome::Faulted(GuardFault::Unavailable)
    }
}

#[test]
fn atk_13_audit_append_failure() {
    let catalog = fixture_catalog();
    let fixture = catalog.by_id("valid").expect("valid fixture");
    let case = attack_by_id("ATK-13").expect("ATK-13 is registered");
    let request = request_for_val_002_fixture(case, fixture);
    let adapter = BeforeToolCallAdapter::new(FounderAuthoredGuard);
    let mut store = FaultingConsumptionStore;
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
        BeforeToolCallObservation::Blocked {
            outcome: required_outcome("ATK-13"),
            denial_signal: "CORE-002 guard unavailable",
            authorization_issued: false,
            effectful_invocations: 0,
        }
    );
    assert_eq!(tool.invocation_count(), 0);
}

ignored_gate_attack!(atk_04_missing_justification, "ATK-04");
ignored_gate_attack!(atk_05_ambiguous_evidence, "ATK-05");
ignored_gate_attack!(atk_12_revoked_credential, "ATK-12");
ignored_gate_attack!(atk_14_cross_tenant_use, "ATK-14");

#[test]
#[ignore = "CORE-004 dedicated two-surface RED conformance pending founder T0"]
fn atk_06_approval_timeout() {
    let case = attack_by_id("ATK-06").expect("ATK-06 is registered");
    assert_eq!(case.target, HarnessTarget::Gate);
    assert_eq!(case.expected, RequiredOutcome::EscalateThenDenyOnTimeout);
    panic!(
        "ATK-06 RED: a generic terminal no-token block is not the required two-surface escalation/timeout proof; run core_004_conformance"
    );
}

#[test]
#[ignore = "waiting for hosted IAM assertion harness; this is not a gate test"]
fn atk_15_deploy_role_data_access() {
    assert_external_iam_case_is_not_a_gate_test("ATK-15");
}
