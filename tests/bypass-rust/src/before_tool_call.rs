//! Mixed-tier conformance adapter for the OpenClaw `before_tool_call` boundary.
//!
//! The request, observation, and probe types are test-only plumbing. The body of
//! `BeforeToolCallAdapter::before_tool_call` contains the founder-authored T0
//! fail-closed floor for a reached boundary whose guard faults or unwinds.
//! The effectful target remains a test probe, never a real tool.

use crate::founder_approval_store::{ApprovalStore, ReviewRequestId};

use crate::founder_consumption_store::ConsumptionStore;
use crate::{DecisionContext, ProposedAction, RequiredOutcome};

/// Opaque bytes passed to the founder-authored guard without interpretation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OpaqueCapabilityToken<'a> {
    pub bytes: &'a [u8],
}

/// The intercepted call presented at OpenClaw's `before_tool_call` seam.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BeforeToolCallRequest<'a> {
    pub proposed_action: ProposedAction,
    pub context: &'a DecisionContext,
    pub capability_token: Option<OpaqueCapabilityToken<'a>>,
}

/// A decision supplied by the founder-authored guard.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GuardDecision {
    Allow {
        authorization_reference: &'static str,
    },
    Escalate {
        review_request_id: ReviewRequestId,
        deadline: u64,
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
        consumption_store: &mut dyn ConsumptionStore,
        approval_store: &mut dyn ApprovalStore,
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
    Escalated {
        review_request_id: ReviewRequestId,
        deadline: u64,
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

/// OpenClaw-shaped test boundary. Returned `Ok(Deny | Escalate | Allow)` decisions retain
/// their established relay behavior. A typed guard fault or an unwinding panic
/// is contained by the founder-authored T0 floor and becomes an explicit
/// fail-closed block before the effectful probe can run.
///
/// This containment covers Rust unwinding panics after the boundary is reached.
/// It does not cover `panic=abort`, process termination, OOM abort, or a hook
/// that is never invoked.
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
        consumption_store: &mut dyn ConsumptionStore,
        approval_store: &mut dyn ApprovalStore,
        tool: &mut T,
    ) -> BeforeToolCallObservation
    where
        T: EffectfulToolProbe,
    {
        // `AssertUnwindSafe` is deliberately bounded to this invocation. If `decide`
        // unwinds, this method does not inspect or reuse either store and returns
        // fail-closed immediately. This does not certify either store's invariants
        // for reuse by a later invocation.
        let decision = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.guard
                .decide(request, now_unix_seconds, consumption_store, approval_store)
        }));

        match decision {
            Ok(Ok(GuardDecision::Deny {
                outcome,
                denial_signal,
            })) => BeforeToolCallObservation::Blocked {
                outcome,
                denial_signal,
                authorization_issued: false,
                effectful_invocations: tool.invocation_count(),
            },
            Ok(Ok(GuardDecision::Escalate {
                review_request_id,
                deadline,
            })) => BeforeToolCallObservation::Escalated {
                review_request_id,
                deadline,
                authorization_issued: false,
                effectful_invocations: tool.invocation_count(),
            },
            Ok(Ok(GuardDecision::Allow {
                authorization_reference,
            })) => {
                tool.invoke(&request.proposed_action);
                BeforeToolCallObservation::Proceeded {
                    authorization_reference,
                    authorization_issued: true,
                    effectful_invocations: tool.invocation_count(),
                }
            }
            Ok(Err(_)) | Err(_) => BeforeToolCallObservation::Blocked {
                outcome: RequiredOutcome::FailClosed,
                denial_signal: "CORE-003 boundary fail-closed",
                authorization_issued: false,
                effectful_invocations: tool.invocation_count(),
            },
        }
    }
}
