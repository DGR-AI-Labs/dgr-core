//! {FOUNDER-AUTHORS}: T0 — FOUNDER AUTHORS.
//!
//! Founder-owned decision surface for CORE-002. Capability-token verification
//! and fail-closed handling have separate founder-owned units beside this one.
//! Absence, invalidity, or internal error must deny.

use crate::before_tool_call::{
    BeforeToolCallRequest, GuardDecision, GuardDecisionPort, GuardFault,
};

use crate::founder_consumption_store::{ConsumeOutcome, ConsumptionStore};

use crate::{ProposedAction, RequiredOutcome};
use sha2::{Digest, Sha256};

use crate::founder_token_verification::{
    TokenRejection, VerifiedToken,
    VerifyOutcome::{Faulted, Rejected, Verified},
    verify_capability_token,
};

use crate::founder_fail_closed::fail_closed_decision;

use crate::founder_approval_store::{
    ApprovalStore, PendingApproval, RecordPendingOutcome, ReviewRequestId,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct FounderAuthoredGuard;

const MAXIMUM_LIFETIME_SECONDS: u64 = 300;
const EXPIRY_SKEW_SECONDS: u64 = 30;

#[doc(hidden)]
pub const CONFORMANCE_MAXIMUM_LIFETIME_SECONDS: u64 = MAXIMUM_LIFETIME_SECONDS;

#[doc(hidden)]
pub const CONFORMANCE_EXPIRY_SKEW_SECONDS: u64 = EXPIRY_SKEW_SECONDS;

const APPROVAL_REQUIRED_ABOVE_MINOR_UNITS: u64 = 1_000_000;
const APPROVAL_WINDOW_SECONDS: u64 = 86_400;

#[doc(hidden)]
pub const CONFORMANCE_APPROVAL_REQUIRED_ABOVE_MINOR_UNITS: u64 =
    APPROVAL_REQUIRED_ABOVE_MINOR_UNITS;

#[doc(hidden)]
pub const CONFORMANCE_APPROVAL_WINDOW_SECONDS: u64 = APPROVAL_WINDOW_SECONDS;

const REVIEW_REQUEST_ID_DOMAIN: &[u8] = b"DGR-CORE004-REVIEW-V1\0";

#[doc(hidden)]
pub const CONFORMANCE_REVIEW_REQUEST_ID_DOMAIN: &[u8] = REVIEW_REQUEST_ID_DOMAIN;

impl GuardDecisionPort for FounderAuthoredGuard {
    fn decide(
        &self,
        request: &BeforeToolCallRequest<'_>,
        now_unix_seconds: u64,
        consumption_store: &mut dyn ConsumptionStore,
        approval_store: &mut dyn ApprovalStore,
    ) -> Result<GuardDecision, GuardFault> {
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
                Faulted(fault) => fail_closed_decision(fault),
                Verified(token) => {
                    let lifetime_is_valid = token
                        .expires_at
                        .checked_sub(token.issued_at)
                        .is_some_and(|lifetime| lifetime <= MAXIMUM_LIFETIME_SECONDS);

                    if !lifetime_is_valid {
                        return Ok(GuardDecision::Deny {
                            outcome: RequiredOutcome::Deny,
                            denial_signal: "ATK-02 invalid capability token lifetime",
                        });
                    }

                    let latest_valid_time = match token.expires_at.checked_add(EXPIRY_SKEW_SECONDS)
                    {
                        Some(time) => time,
                        None => return fail_closed_decision(GuardFault::InternalError),
                    };

                    if now_unix_seconds > latest_valid_time {
                        return Ok(GuardDecision::Deny {
                            outcome: RequiredOutcome::Deny,
                            denial_signal: "ATK-02 expired capability token",
                        });
                    }

                    let recomputed_commitment = canonical_action_bytes(&request.proposed_action)
                        .map(|bytes| {
                            let digest: [u8; 32] = Sha256::digest(bytes).into();
                            digest
                        });

                    if recomputed_commitment != Some(token.action_commitment) {
                        return Ok(GuardDecision::Deny {
                            outcome: RequiredOutcome::Deny,
                            denial_signal: "ATK-08/09/11 action commitment mismatch",
                        });
                    }

                    let requires_approval =
                        match canonical_amount_requires_approval(request.proposed_action.amount) {
                            Some(requires_approval) => requires_approval,
                            None => {
                                return Ok(GuardDecision::Deny {
                                    outcome: RequiredOutcome::Deny,
                                    denial_signal: "CORE-004 non-canonical amount",
                                });
                            }
                        };

                    if requires_approval {
                        let candidate = match pending_approval(&token, now_unix_seconds) {
                            Ok(candidate) => candidate,
                            Err(fault) => return fail_closed_decision(fault),
                        };

                        let stored = match approval_store.record_pending(candidate) {
                            RecordPendingOutcome::Recorded(stored) => {
                                if stored != candidate {
                                    return fail_closed_decision(GuardFault::InternalError);
                                }

                                stored
                            }
                            RecordPendingOutcome::AlreadyPending(stored) => {
                                if !same_pending_identity(&stored, &candidate)
                                    || stored.deadline < stored.requested_at
                                {
                                    return fail_closed_decision(GuardFault::InternalError);
                                }

                                stored
                            }
                            RecordPendingOutcome::Faulted(fault) => {
                                return fail_closed_decision(fault);
                            }
                        };

                        return Ok(GuardDecision::Escalate {
                            review_request_id: stored.review_request_id,
                            deadline: stored.deadline,
                        });
                    }

                    match consumption_store.consume(&token.nonce) {
                        ConsumeOutcome::Consumed => Ok(GuardDecision::Allow {
                            authorization_reference: "CORE-002 authorized",
                        }),
                        ConsumeOutcome::AlreadyConsumed => Ok(GuardDecision::Deny {
                            outcome: RequiredOutcome::Deny,
                            denial_signal: "ATK-03 replayed capability token",
                        }),
                        ConsumeOutcome::Faulted(fault) => fail_closed_decision(fault),
                    }
                }
            },
        }
    }
}

fn canonical_amount_requires_approval(amount: &str) -> Option<bool> {
    let amount = amount.as_bytes();

    if amount.is_empty()
        || !amount.iter().all(u8::is_ascii_digit)
        || (amount.len() > 1 && amount[0] == b'0')
    {
        return None;
    }

    let threshold = APPROVAL_REQUIRED_ABOVE_MINOR_UNITS.to_string();
    let threshold = threshold.as_bytes();

    Some(if amount.len() != threshold.len() {
        amount.len() > threshold.len()
    } else {
        amount > threshold
    })
}

fn derive_review_request_id(token: &VerifiedToken) -> ReviewRequestId {
    let mut digest = Sha256::new();
    digest.update(REVIEW_REQUEST_ID_DOMAIN);
    digest.update(token.key_id);
    digest.update(token.nonce);
    digest.update(token.action_commitment);

    ReviewRequestId::from_bytes(digest.finalize().into())
}

fn pending_approval(
    token: &VerifiedToken,
    requested_at: u64,
) -> Result<PendingApproval, GuardFault> {
    let deadline = requested_at
        .checked_add(APPROVAL_WINDOW_SECONDS)
        .ok_or(GuardFault::InternalError)?;

    Ok(PendingApproval {
        review_request_id: derive_review_request_id(token),
        key_id: token.key_id,
        nonce: token.nonce,
        action_commitment: token.action_commitment,
        requested_at,
        deadline,
    })
}

fn same_pending_identity(existing: &PendingApproval, candidate: &PendingApproval) -> bool {
    existing.review_request_id == candidate.review_request_id
        && existing.key_id == candidate.key_id
        && existing.nonce == candidate.nonce
        && existing.action_commitment == candidate.action_commitment
}

#[doc(hidden)]
pub fn conformance_pending_approval(
    token: &VerifiedToken,
    requested_at: u64,
) -> Result<PendingApproval, GuardFault> {
    pending_approval(token, requested_at)
}

#[doc(hidden)]
pub fn conformance_amount_requires_approval(amount: &str) -> Option<bool> {
    canonical_amount_requires_approval(amount)
}

fn canonical_action_bytes(action: &ProposedAction) -> Option<Vec<u8>> {
    let fields = [
        (0x01, action.action),
        (0x02, action.amount),
        (0x03, action.currency),
        (0x04, action.destination),
        (0x05, action.invoice_id),
        (0x06, action.source_account),
    ];

    let mut encoded = Vec::new();

    for (tag, value) in fields {
        let value = value.as_bytes();
        let length = u32::try_from(value.len()).ok()?;

        encoded.push(tag);
        encoded.extend_from_slice(&length.to_be_bytes());
        encoded.extend_from_slice(value);
    }

    Some(encoded)
}

#[doc(hidden)]
pub fn conformance_canonical_action_bytes(action: &ProposedAction) -> Option<Vec<u8>> {
    canonical_action_bytes(action)
}
