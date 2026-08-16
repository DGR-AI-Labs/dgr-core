use dgr_core_bypass_harness::RequiredOutcome;
use dgr_core_bypass_harness::before_tool_call::{GuardDecision, GuardFault};
use dgr_core_bypass_harness::founder_fail_closed::fail_closed_decision;

#[test]
fn every_guard_fault_maps_to_an_explicit_fail_closed_denial() {
    for (fault, denial_signal) in [
        (
            GuardFault::FounderImplementationRequired,
            "CORE-002 founder implementation required",
        ),
        (GuardFault::Unavailable, "CORE-002 guard unavailable"),
        (GuardFault::InternalError, "CORE-002 guard internal error"),
    ] {
        assert_eq!(
            fail_closed_decision(fault),
            Ok(GuardDecision::Deny {
                outcome: RequiredOutcome::FailClosed,
                denial_signal,
            })
        );
    }
}
