//! {FOUNDER-AUTHORS}: T0 fail-closed decision unit.
//!
//! Agents must not implement, complete, or replace this fail-closed stub.

use crate::before_tool_call::{GuardDecision, GuardFault};

pub fn fail_closed_decision(_fault: GuardFault) -> Result<GuardDecision, GuardFault> {
    // {FOUNDER-AUTHORS}: signature and non-functional default only.
    unimplemented!("FounderImplementationRequired")
}
