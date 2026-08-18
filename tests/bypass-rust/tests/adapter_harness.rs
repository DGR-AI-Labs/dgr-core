use dgr_core_bypass_harness::RequiredOutcome;

use dgr_core_bypass_harness::before_tool_call::{
    BeforeToolCallAdapter, BeforeToolCallObservation, BeforeToolCallRequest, EffectfulToolProbe,
    GuardDecision, GuardDecisionPort, GuardFault,
};

use dgr_core_bypass_harness::founder_consumption_store::ConsumptionStore;
use dgr_core_bypass_harness::founder_s2_consumption_store::S2ConsumptionStore;

use dgr_core_bypass_harness::fixtures::{
    RecordingToolProbe, no_token_request, valid_token_request,
};
use dgr_core_bypass_harness::val_002_fixtures::FIXED_NOW_UNIX_SECONDS;

#[derive(Clone, Copy)]
struct ScriptedDecision(Result<GuardDecision, GuardFault>);

impl GuardDecisionPort for ScriptedDecision {
    fn decide(
        &self,
        _request: &BeforeToolCallRequest<'_>,
        _now_unix_seconds: u64,
        _store: &mut dyn ConsumptionStore,
    ) -> Result<GuardDecision, GuardFault> {
        self.0
    }
}

#[test]
fn adapter_does_not_invoke_tool_for_a_returned_deny() {
    let adapter = BeforeToolCallAdapter::new(ScriptedDecision(Ok(GuardDecision::Deny {
        outcome: RequiredOutcome::Block,
        denial_signal: "scripted test denial",
    })));

    let mut store = S2ConsumptionStore::open_in_memory().expect("store");
    let mut tool = RecordingToolProbe::default();

    let observed = adapter.before_tool_call(
        &no_token_request(),
        FIXED_NOW_UNIX_SECONDS,
        &mut store,
        &mut tool,
    );

    assert_eq!(tool.invocation_count(), 0);
    assert_eq!(
        observed,
        BeforeToolCallObservation::Blocked {
            outcome: RequiredOutcome::Block,
            denial_signal: "scripted test denial",
            authorization_issued: false,
            effectful_invocations: 0,
        }
    );
}

#[test]
fn adapter_invokes_probe_only_for_a_returned_allow() {
    let adapter = BeforeToolCallAdapter::new(ScriptedDecision(Ok(GuardDecision::Allow {
        authorization_reference: "scripted-test-authorization",
    })));
    let mut store = S2ConsumptionStore::open_in_memory().expect("store");
    let mut tool = RecordingToolProbe::default();

    let observed = adapter.before_tool_call(
        &valid_token_request(),
        FIXED_NOW_UNIX_SECONDS,
        &mut store,
        &mut tool,
    );

    assert_eq!(tool.invocation_count(), 1);
    assert_eq!(
        observed,
        BeforeToolCallObservation::Proceeded {
            authorization_reference: "scripted-test-authorization",
            authorization_issued: true,
            effectful_invocations: 1,
        }
    );
}

#[test]
fn adapter_fail_closes_for_a_guard_fault() {
    let adapter = BeforeToolCallAdapter::new(ScriptedDecision(Err(GuardFault::InternalError)));
    let mut store = S2ConsumptionStore::open_in_memory().expect("store");
    let mut tool = RecordingToolProbe::default();

    let observed = adapter.before_tool_call(
        &no_token_request(),
        FIXED_NOW_UNIX_SECONDS,
        &mut store,
        &mut tool,
    );

    assert_eq!(tool.invocation_count(), 0);

    assert_eq!(
        observed,
        BeforeToolCallObservation::Blocked {
            outcome: RequiredOutcome::FailClosed,
            denial_signal: "CORE-003 boundary fail-closed",
            authorization_issued: false,
            effectful_invocations: 0,
        }
    );
}
