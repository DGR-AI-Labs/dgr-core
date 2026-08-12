//! {FOUNDER-AUTHORS}: T0 — FOUNDER AUTHORS.
//!
//! Founder-owned decision surface for CORE-002. Capability-token verification
//! and fail-closed handling have separate founder-owned units beside this one.
//! Absence, invalidity, or internal error must deny.

use crate::before_tool_call::{
    BeforeToolCallRequest, GuardDecision, GuardDecisionPort, GuardFault,
};
use crate::RequiredOutcome;


#[derive(Clone, Copy, Debug, Default)]
pub struct FounderAuthoredGuard;

impl GuardDecisionPort for FounderAuthoredGuard {
    fn decide(&self, request: &BeforeToolCallRequest<'_>) -> Result<GuardDecision, GuardFault> {
        match request.capability_token {
            None => {
                Ok(GuardDecision::Deny {
                     outcome: RequiredOutcome::Block,
                     denial_signal: "ATK-01 no valid capability token",
                 })
            }
            Some(_) => {
                Err(GuardFault::FounderImplementationRequired)
            }
        }
    }
}