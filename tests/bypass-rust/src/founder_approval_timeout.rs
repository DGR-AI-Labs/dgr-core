//! {FOUNDER-AUTHORS}: T0 token-free approval-timeout evaluation.
//!
//! This surface evaluates an already-persisted approval request. It accepts no
//! capability token and cannot issue authorization or invoke a tool.

use crate::RequiredOutcome;
use crate::attack_by_id;
use crate::before_tool_call::{GuardDecision, GuardFault};
use crate::founder_approval_store::{
    ApprovalStore, EvaluatePendingOutcome, PendingApproval, ReviewRequestId,
};
use crate::founder_fail_closed::fail_closed_decision;

fn matching_pending(
    review_request_id: &ReviewRequestId,
    pending: PendingApproval,
) -> Result<PendingApproval, GuardFault> {
    if pending.review_request_id != *review_request_id {
        return Err(GuardFault::InternalError);
    }

    Ok(pending)
}

pub fn evaluate_approval_timeout(
    approval_store: &mut dyn ApprovalStore,
    review_request_id: &ReviewRequestId,
    now_unix_seconds: u64,
) -> Result<GuardDecision, GuardFault> {
    match approval_store.evaluate_pending(review_request_id, now_unix_seconds) {
        EvaluatePendingOutcome::Pending(pending) => {
            let pending = match matching_pending(review_request_id, pending) {
                Ok(pending) => pending,
                Err(fault) => return fail_closed_decision(fault),
            };

            Ok(GuardDecision::Escalate {
                review_request_id: pending.review_request_id,
                deadline: pending.deadline,
            })
        }
        EvaluatePendingOutcome::TimedOut(pending) => {
            if let Err(fault) = matching_pending(review_request_id, pending) {
                return fail_closed_decision(fault);
            }

            let outcome = match attack_by_id("ATK-06") {
                Some(case) => case.expected,
                None => return fail_closed_decision(GuardFault::InternalError),
            };

            Ok(GuardDecision::Deny {
                outcome,
                denial_signal: "ATK-06 approval timed out",
            })
        }
        EvaluatePendingOutcome::NotFound => Ok(GuardDecision::Deny {
            outcome: RequiredOutcome::FailClosed,
            denial_signal: "CORE-004 pending approval not found",
        }),
        EvaluatePendingOutcome::Faulted(fault) => fail_closed_decision(fault),
    }
}
