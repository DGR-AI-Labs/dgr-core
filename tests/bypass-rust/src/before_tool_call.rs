//! T3 conformance adapter for the OpenClaw `before_tool_call` boundary.
//!
//! This module contains plumbing only. It does not classify tokens, choose a
//! decision, convert a gate fault into an authorization result, or write an
//! audit record. The effectful target is a test probe, never a real tool.

use crate::{DecisionContext, ProposedAction, RequiredOutcome};

/// Opaque bytes passed to the founder-authored guard without interpretation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OpaqueCapabilityToken<'a> {
    pub bytes: &'a [u8],
}

/// The intercepted call presented at OpenClaw's `before_tool_call` seam.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BeforeToolCallRequest<'a> {
    pub proposed_action: &'a ProposedAction,
    pub context: &'a DecisionContext,
    pub capability_token: Option<OpaqueCapabilityToken<'a>>,
}

/// A decision supplied by the founder-authored guard.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GuardDecision {
    Allow {
        authorization_reference: &'static str,
    },
    Deny {
        outcome: RequiredOutcome,
        denial_signal: &'static str,
    },
}

/// A guard fault is not a decision and must never be treated as an allow.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GuardFault {
    FounderImplementationRequired,
    Unavailable,
    InternalError,
}

/// Interface implemented only by the founder-authored guard unit.
pub trait GuardDecisionPort {
    fn decide(
        &self,
        request: &BeforeToolCallRequest<'_>,
        now_unix_seconds: u64,
    ) -> Result<GuardDecision, GuardFault>;
}

/// A fake effectful boundary used only to observe whether execution occurred.
pub trait EffectfulToolProbe {
    fn invoke(&mut self, action: &ProposedAction);
    fn invocation_count(&self) -> u32;
}

/// What the conformance harness observed at `before_tool_call`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BeforeToolCallObservation {
    Blocked {
        outcome: RequiredOutcome,
        denial_signal: &'static str,
        authorization_issued: bool,
        effectful_invocations: u32,
    },
    Proceeded {
        authorization_reference: &'static str,
        authorization_issued: bool,
        effectful_invocations: u32,
    },
    GuardFault {
        fault: GuardFault,
        authorization_issued: bool,
        effectful_invocations: u32,
    },
}

/// OpenClaw-shaped test adapter. It relays the guard's returned decision; it
/// does not make, repair, or default that decision.
pub struct BeforeToolCallAdapter<G> {
    guard: G,
}

impl<G> BeforeToolCallAdapter<G>
where
    G: GuardDecisionPort,
{
    pub const fn new(guard: G) -> Self {
        Self { guard }
    }

    pub fn before_tool_call<T>(
        &self,
        request: &BeforeToolCallRequest<'_>,
        now_unix_seconds: u64,
        tool: &mut T,
    ) -> BeforeToolCallObservation
    where
        T: EffectfulToolProbe,
    {
        match self.guard.decide(request, now_unix_seconds) {
            Ok(GuardDecision::Deny {
                outcome,
                denial_signal,
            }) => BeforeToolCallObservation::Blocked {
                outcome,
                denial_signal,
                authorization_issued: false,
                effectful_invocations: tool.invocation_count(),
            },
            Ok(GuardDecision::Allow {
                authorization_reference,
            }) => {
                tool.invoke(request.proposed_action);
                BeforeToolCallObservation::Proceeded {
                    authorization_reference,
                    authorization_issued: true,
                    effectful_invocations: tool.invocation_count(),
                }
            }
            Err(fault) => BeforeToolCallObservation::GuardFault {
                fault,
                authorization_issued: false,
                effectful_invocations: tool.invocation_count(),
            },
        }
    }
}
