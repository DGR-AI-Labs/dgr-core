//! {FOUNDER-AUTHORS}: T0 — FOUNDER AUTHORS.
//!
//! Founder-owned decision surface for CORE-002. Capability-token verification
//! and fail-closed handling have separate founder-owned units beside this one.
//! Absence, invalidity, or internal error must deny.

use crate::RequiredOutcome;
use crate::before_tool_call::{
    BeforeToolCallRequest, GuardDecision, GuardDecisionPort, GuardFault,
};

use crate::founder_token_verification::{
    TokenRejection,
    VerifyOutcome::{Faulted, Rejected, Verified},
    verify_capability_token,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct FounderAuthoredGuard;

impl GuardDecisionPort for FounderAuthoredGuard {
    fn decide(&self, request: &BeforeToolCallRequest<'_>) -> Result<GuardDecision, GuardFault> {
        match request.capability_token {
            None => Ok(GuardDecision::Deny {
                outcome: RequiredOutcome::Block,
                denial_signal: "ATK-01 no valid capability token",
            }),
            Some(token) => match verify_capability_token(token.bytes) {
                Rejected(r) => {
                    let denial_signal = match r {
                        TokenRejection::MalformedLength => "ATK-10 malformed token length",
                        TokenRejection::UnsupportedVersion => "ATK-10 unsupported token version",
                        TokenRejection::UnknownKeyId => "ATK-10 unknown key id",
                        TokenRejection::InvalidSignature => "ATK-10 invalid signature",
                    };
                    Ok(GuardDecision::Deny {
                        outcome: RequiredOutcome::Deny,
                        denial_signal,
                    })
                }
                Faulted(f) => Err(f),
                Verified(_) => Err(GuardFault::FounderImplementationRequired), // still no Allow
            },
        }
    }
}
