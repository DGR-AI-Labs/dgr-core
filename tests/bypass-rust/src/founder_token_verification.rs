//! {FOUNDER-AUTHORS}: T0 capability-token verification unit.
//!
//! Agents must not implement, complete, or replace this fail-closed stub.

use crate::before_tool_call::{BeforeToolCallRequest, GuardFault};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VerifiedCapability {
    pub authorization_reference: &'static str,
}

pub fn verify_capability_token(
    _request: &BeforeToolCallRequest<'_>,
) -> Result<VerifiedCapability, GuardFault> {
    // {FOUNDER-AUTHORS}: signature and non-functional default only.
    unimplemented!("FounderImplementationRequired")
}
