//! T3 conformance adapter for the OpenClaw `before_tool_call` boundary.
//!
//! This module re-exports the T0 request/decision surface for compatibility and
//! owns only observation and test-probe plumbing. The effectful target remains
//! a test probe, never a real tool.

use crate::founder_approval_store::{ApprovalStore, ReviewRequestId};
use crate::founder_consumption_store::ConsumptionStore;
use crate::{ProposedAction, RequiredOutcome};

use crate::founder_before_tool_call_floor::before_tool_call_floor;
pub use crate::founder_before_tool_call_floor::{
    BeforeToolCallOutcome, BeforeToolCallRequest, GuardDecision, GuardDecisionPort, GuardFault,
    OpaqueCapabilityToken,
};

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
    /// Negative-conformance sentinel: ATK-07 rejects this raw-fault shape if it is ever observed.
    GuardFault {
        fault: GuardFault,
        authorization_issued: bool,
        effectful_invocations: u32,
    },
}

/// OpenClaw-shaped test boundary. Returned `Ok(Deny | Escalate | Allow)` decisions retain
/// their established relay behavior. A typed guard fault or an unwinding panic
/// is contained by the Amendment-B T0 floor and becomes an explicit
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
        match before_tool_call_floor(
            &self.guard,
            request,
            now_unix_seconds,
            consumption_store,
            approval_store,
        ) {
            BeforeToolCallOutcome::Blocked {
                outcome,
                denial_signal,
            } => BeforeToolCallObservation::Blocked {
                outcome,
                denial_signal,
                authorization_issued: false,
                effectful_invocations: tool.invocation_count(),
            },
            BeforeToolCallOutcome::Escalated {
                review_request_id,
                deadline,
            } => BeforeToolCallObservation::Escalated {
                review_request_id,
                deadline,
                authorization_issued: false,
                effectful_invocations: tool.invocation_count(),
            },
            BeforeToolCallOutcome::Authorized {
                authorization_reference,
            } => {
                tool.invoke(&request.proposed_action);
                BeforeToolCallObservation::Proceeded {
                    authorization_reference,
                    authorization_issued: true,
                    effectful_invocations: tool.invocation_count(),
                }
            }
        }
    }
}
