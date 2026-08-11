use dgr_core_bypass_harness::before_tool_call::{
    BeforeToolCallAdapter, BeforeToolCallObservation, EffectfulToolProbe,
};
use dgr_core_bypass_harness::fixtures::{RecordingToolProbe, no_token_request, request_for_attack};
use dgr_core_bypass_harness::founder_authored_guard::FounderAuthoredGuard;
use dgr_core_bypass_harness::{ATTACK_SET, HarnessTarget, RequiredOutcome, attack_by_id};
use std::panic::{AssertUnwindSafe, catch_unwind};

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
    let mut tool = RecordingToolProbe::default();
    let observed = adapter.before_tool_call(&request_for_attack(case), &mut tool);

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
fn unimplemented_guard_cannot_reach_the_effectful_probe() {
    let request = no_token_request();
    let adapter = BeforeToolCallAdapter::new(FounderAuthoredGuard);
    let mut tool = RecordingToolProbe::default();

    let result = catch_unwind(AssertUnwindSafe(|| {
        adapter.before_tool_call(&request, &mut tool)
    }));

    assert_eq!(
        tool.invocation_count(),
        0,
        "the founder stub reached the effectful probe"
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
    let mut tool = RecordingToolProbe::default();

    let observed = adapter.before_tool_call(&request, &mut tool);

    assert_eq!(
        tool.invocation_count(),
        0,
        "the effectful tool ran before the guard returned a decision"
    );
    assert_eq!(
        observed,
        BeforeToolCallObservation::Blocked {
            outcome: RequiredOutcome::Block,
            denial_signal: "ATK-01 no valid capability token",
            authorization_issued: false,
            effectful_invocations: 0,
        },
        "ATK-01 RED: an absent or erroring guard must emit a denial signal, never allow"
    );
}

ignored_gate_attack!(atk_02_expired_token, "ATK-02");
ignored_gate_attack!(atk_03_replayed_token, "ATK-03");
ignored_gate_attack!(atk_04_missing_justification, "ATK-04");
ignored_gate_attack!(atk_05_ambiguous_evidence, "ATK-05");
ignored_gate_attack!(atk_06_approval_timeout, "ATK-06");
ignored_gate_attack!(atk_07_hook_error, "ATK-07");
ignored_gate_attack!(atk_08_scope_escalation, "ATK-08");
ignored_gate_attack!(atk_09_token_substitution, "ATK-09");
ignored_gate_attack!(atk_10_forged_authorization, "ATK-10");
ignored_gate_attack!(atk_11_parameter_swap, "ATK-11");
ignored_gate_attack!(atk_12_revoked_credential, "ATK-12");
ignored_gate_attack!(atk_13_audit_append_failure, "ATK-13");
ignored_gate_attack!(atk_14_cross_tenant_use, "ATK-14");

#[test]
#[ignore = "waiting for hosted IAM assertion harness; this is not a gate test"]
fn atk_15_deploy_role_data_access() {
    assert_external_iam_case_is_not_a_gate_test("ATK-15");
}
