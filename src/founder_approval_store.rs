//! {FOUNDER-AUTHORS}: T0 approval-store interface and consequential outcome types.
//!
//! The default implementation fails closed until a concrete durable-local
//! approval store supplies pending-request and timeout-transition behavior.

use crate::founder_before_tool_call_floor::GuardFault;

/// Founder-derived, agent-independent identity for one approval request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReviewRequestId([u8; 32]);

impl ReviewRequestId {
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// Immutable facts written when an approval request first becomes pending.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PendingApproval {
    pub review_request_id: ReviewRequestId,
    pub key_id: [u8; 16],
    pub nonce: [u8; 16],
    pub action_commitment: [u8; 32],
    pub requested_at: u64,
    pub deadline: u64,
}

/// Result of recording or re-presenting a pending request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecordPendingOutcome {
    Recorded(PendingApproval),
    AlreadyPending(PendingApproval),
    Faulted(GuardFault),
}

/// Result of atomically evaluating an existing pending request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvaluatePendingOutcome {
    Pending(PendingApproval),
    TimedOut(PendingApproval),
    NotFound,
    Faulted(GuardFault),
}

/// Durable approval-state boundary.
///
/// Implementations must make timeout evaluation and the
/// `requested -> denied_on_timeout` transition one atomic store operation.
pub trait ApprovalStore {
    fn record_pending(&mut self, _pending: PendingApproval) -> RecordPendingOutcome {
        RecordPendingOutcome::Faulted(GuardFault::FounderImplementationRequired)
    }

    fn evaluate_pending(
        &mut self,
        _review_request_id: &ReviewRequestId,
        _now_unix_seconds: u64,
    ) -> EvaluatePendingOutcome {
        EvaluatePendingOutcome::Faulted(GuardFault::FounderImplementationRequired)
    }
}
