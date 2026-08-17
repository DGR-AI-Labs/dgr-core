//! {FOUNDER-AUTHORS}: T0 fail-closed decision unit.

use crate::RequiredOutcome;
use crate::before_tool_call::{GuardDecision, GuardFault};

pub fn fail_closed_decision(fault: GuardFault) -> Result<GuardDecision, GuardFault> {
    let denial_signal = match fault {
        GuardFault::FounderImplementationRequired => "CORE-002 founder implementation required",
        GuardFault::Unavailable => "CORE-002 guard unavailable",
        GuardFault::InternalError => "CORE-002 guard internal error",
    };

    Ok(GuardDecision::Deny {
        outcome: RequiredOutcome::FailClosed,
        denial_signal,
    })
}
