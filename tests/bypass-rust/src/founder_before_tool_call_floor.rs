//! {AGENT-AUTHORS}: T0 reached-boundary fail-closed floor.
//!
//! Existing request and decision shapes are relocated here for PROD-000. The
//! T0 type surface and transformed floor are agent-authored under ADR-13
//! Amendment B; the floor retains identified founder-source provenance and
//! remains subject to founder review and disposition.

use crate::founder_approval_store::{ApprovalStore, ReviewRequestId};
use crate::founder_consumption_store::ConsumptionStore;
use crate::{DecisionContext, ProposedAction, RequiredOutcome};

/// Opaque bytes passed to the guard without interpretation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OpaqueCapabilityToken<'a> {
    pub bytes: &'a [u8],
}

/// The intercepted call presented at the `before_tool_call` seam.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BeforeToolCallRequest<'a> {
    pub proposed_action: ProposedAction,
    pub context: &'a DecisionContext,
    pub capability_token: Option<OpaqueCapabilityToken<'a>>,
}

/// A decision supplied by the guard.
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

/// Interface implemented by the guard unit.
pub trait GuardDecisionPort {
    fn decide(
        &self,
        request: &BeforeToolCallRequest<'_>,
        now_unix_seconds: u64,
        consumption_store: &mut dyn ConsumptionStore,
        approval_store: &mut dyn ApprovalStore,
    ) -> Result<GuardDecision, GuardFault>;
}

/// Product-level result of a reached `before_tool_call` boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BeforeToolCallOutcome {
    Blocked {
        outcome: RequiredOutcome,
        denial_signal: &'static str,
    },
    Escalated {
        review_request_id: ReviewRequestId,
        deadline: u64,
    },
    Authorized {
        authorization_reference: &'static str,
    },
}

/// Invoke the guard after the boundary is reached and contain typed faults or
/// unwinding panics as a product-level fail-closed block.
///
/// This containment covers Rust unwinding panics after the boundary is reached.
/// It does not cover `panic=abort`, process termination, OOM abort, or a hook
/// that is never invoked.
pub fn before_tool_call_floor<G>(
    guard: &G,
    request: &BeforeToolCallRequest<'_>,
    now_unix_seconds: u64,
    consumption_store: &mut dyn ConsumptionStore,
    approval_store: &mut dyn ApprovalStore,
) -> BeforeToolCallOutcome
where
    G: GuardDecisionPort,
{
    // `AssertUnwindSafe` is deliberately bounded to this invocation. If `decide`
    // unwinds, this function does not inspect or reuse either store and returns
    // fail-closed immediately. This does not certify either store's invariants
    // for reuse by a later invocation.
    let decision = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        guard.decide(request, now_unix_seconds, consumption_store, approval_store)
    }));

    match decision {
        Ok(Ok(GuardDecision::Deny {
            outcome,
            denial_signal,
        })) => BeforeToolCallOutcome::Blocked {
            outcome,
            denial_signal,
        },
        Ok(Ok(GuardDecision::Escalate {
            review_request_id,
            deadline,
        })) => BeforeToolCallOutcome::Escalated {
            review_request_id,
            deadline,
        },
        Ok(Ok(GuardDecision::Allow {
            authorization_reference,
        })) => BeforeToolCallOutcome::Authorized {
            authorization_reference,
        },
        Ok(Err(_)) | Err(_) => BeforeToolCallOutcome::Blocked {
            outcome: RequiredOutcome::FailClosed,
            denial_signal: "CORE-003 boundary fail-closed",
        },
    }
}
