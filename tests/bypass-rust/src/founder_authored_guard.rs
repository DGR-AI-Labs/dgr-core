//! {FOUNDER-AUTHORS}: T0 — FOUNDER AUTHORS. AGENTS MUST NOT IMPLEMENT.
//!
//! Founder-owned decision surface for CORE-002. Capability-token verification
//! and fail-closed handling have separate founder-owned units beside this one.
//! Absence, invalidity, or internal error must deny. This placeholder contains
//! no enforcement logic and deliberately cannot return allow or deny.

use crate::before_tool_call::{
    BeforeToolCallRequest, GuardDecision, GuardDecisionPort, GuardFault,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct FounderAuthoredGuard;

impl GuardDecisionPort for FounderAuthoredGuard {
    fn decide(&self, _request: &BeforeToolCallRequest<'_>) -> Result<GuardDecision, GuardFault> {
        // {FOUNDER-AUTHORS}: replace only through the mandatory T0 process.
        unimplemented!("FounderImplementationRequired")
    }
}
