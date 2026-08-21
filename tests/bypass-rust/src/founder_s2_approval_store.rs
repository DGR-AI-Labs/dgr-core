//! {FOUNDER-AUTHORS}: T0 S2 durable-local approval-store unit.
//!
//! One persistent SQLite connection is created per store instance and reused
//! for pending-request recording and atomic timeout evaluation.

use crate::before_tool_call::GuardFault;
use crate::founder_approval_store::{
    ApprovalStore, EvaluatePendingOutcome, PendingApproval, RecordPendingOutcome, ReviewRequestId,
};
use rusqlite::{
    Connection, Error as SqliteError, ErrorCode, OptionalExtension, Transaction,
    TransactionBehavior, params,
};
use std::path::Path;

#[derive(Debug)]
pub struct S2ApprovalStore {
    conn: Connection,
}

impl S2ApprovalStore {
    pub fn open_in_memory() -> Result<Self, GuardFault> {
        let conn = Connection::open_in_memory().map_err(|_| GuardFault::Unavailable)?;
        Self::initialize(conn)
    }

    pub fn open_at(path: impl AsRef<Path>) -> Result<Self, GuardFault> {
        let conn = Connection::open(path).map_err(|_| GuardFault::Unavailable)?;
        Self::initialize(conn)
    }

    fn initialize(conn: Connection) -> Result<Self, GuardFault> {
        conn.pragma_update(None, "synchronous", "FULL")
            .map_err(|_| GuardFault::Unavailable)?;

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS pending_approvals (
                review_request_id BLOB NOT NULL
                    PRIMARY KEY
                    CHECK(length(review_request_id) = 32),
                key_id BLOB NOT NULL
                    CHECK(length(key_id) = 16),
                nonce BLOB NOT NULL
                    CHECK(length(nonce) = 16),
                action_commitment BLOB NOT NULL
                    CHECK(length(action_commitment) = 32),
                requested_at INTEGER NOT NULL
                    CHECK(requested_at >= 0),
                deadline INTEGER NOT NULL
                    CHECK(deadline >= requested_at),
                status TEXT NOT NULL
                    CHECK(status IN ('requested', 'denied_on_timeout')),
                UNIQUE(key_id, nonce)
            ) STRICT;",
        )
        .map_err(|_| GuardFault::Unavailable)?;

        Ok(Self { conn })
    }
}

struct ExistingPendingRow {
    review_request_id: Vec<u8>,
    action_commitment: Vec<u8>,
    requested_at: i64,
    deadline: i64,
    status: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ApprovalStatus {
    Requested,
    DeniedOnTimeout,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct StoredApproval {
    pending: PendingApproval,
    status: ApprovalStatus,
}

fn sqlite_fault(error: SqliteError) -> GuardFault {
    match error {
        SqliteError::SqliteFailure(error, _)
            if matches!(
                error.code,
                ErrorCode::DatabaseBusy
                    | ErrorCode::DatabaseLocked
                    | ErrorCode::OutOfMemory
                    | ErrorCode::PermissionDenied
                    | ErrorCode::ReadOnly
                    | ErrorCode::SystemIoFailure
                    | ErrorCode::DiskFull
                    | ErrorCode::CannotOpen
                    | ErrorCode::FileLockingProtocolFailed
            ) =>
        {
            GuardFault::Unavailable
        }
        _ => GuardFault::InternalError,
    }
}

fn fixed_bytes<const N: usize>(bytes: Vec<u8>) -> Result<[u8; N], GuardFault> {
    bytes.try_into().map_err(|_| GuardFault::InternalError)
}

fn decode_existing(
    row: ExistingPendingRow,
    key_id: [u8; 16],
    nonce: [u8; 16],
) -> Result<StoredApproval, GuardFault> {
    let requested_at = u64::try_from(row.requested_at).map_err(|_| GuardFault::InternalError)?;
    let deadline = u64::try_from(row.deadline).map_err(|_| GuardFault::InternalError)?;

    if deadline < requested_at {
        return Err(GuardFault::InternalError);
    }

    let status = match row.status.as_str() {
        "requested" => ApprovalStatus::Requested,
        "denied_on_timeout" => ApprovalStatus::DeniedOnTimeout,
        _ => return Err(GuardFault::InternalError),
    };

    Ok(StoredApproval {
        pending: PendingApproval {
            review_request_id: ReviewRequestId::from_bytes(fixed_bytes(row.review_request_id)?),
            key_id,
            nonce,
            action_commitment: fixed_bytes(row.action_commitment)?,
            requested_at,
            deadline,
        },
        status,
    })
}

fn load_by_identity(
    conn: &Connection,
    key_id: &[u8; 16],
    nonce: &[u8; 16],
) -> Result<Option<StoredApproval>, GuardFault> {
    let row = conn
        .query_row(
            "SELECT review_request_id,
                    action_commitment,
                    requested_at,
                    deadline,
                    status
             FROM pending_approvals
             WHERE key_id = ?1 AND nonce = ?2",
            params![&key_id[..], &nonce[..]],
            |row| {
                Ok(ExistingPendingRow {
                    review_request_id: row.get(0)?,
                    action_commitment: row.get(1)?,
                    requested_at: row.get(2)?,
                    deadline: row.get(3)?,
                    status: row.get(4)?,
                })
            },
        )
        .optional()
        .map_err(sqlite_fault)?;

    row.map(|row| decode_existing(row, *key_id, *nonce))
        .transpose()
}

fn load_by_review_request_id(
    conn: &Connection,
    review_request_id: &ReviewRequestId,
) -> Result<Option<StoredApproval>, GuardFault> {
    let row = conn
        .query_row(
            "SELECT key_id,
                    nonce,
                    review_request_id,
                    action_commitment,
                    requested_at,
                    deadline,
                    status
             FROM pending_approvals
             WHERE review_request_id = ?1",
            params![&review_request_id.as_bytes()[..]],
            |row| {
                Ok((
                    row.get::<_, Vec<u8>>(0)?,
                    row.get::<_, Vec<u8>>(1)?,
                    ExistingPendingRow {
                        review_request_id: row.get(2)?,
                        action_commitment: row.get(3)?,
                        requested_at: row.get(4)?,
                        deadline: row.get(5)?,
                        status: row.get(6)?,
                    },
                ))
            },
        )
        .optional()
        .map_err(sqlite_fault)?;

    row.map(|(key_id, nonce, row)| decode_existing(row, fixed_bytes(key_id)?, fixed_bytes(nonce)?))
        .transpose()
}

fn checked_sql_times(pending: &PendingApproval) -> Result<(i64, i64), GuardFault> {
    if pending.deadline < pending.requested_at {
        return Err(GuardFault::InternalError);
    }

    let requested_at =
        i64::try_from(pending.requested_at).map_err(|_| GuardFault::InternalError)?;
    let deadline = i64::try_from(pending.deadline).map_err(|_| GuardFault::InternalError)?;

    Ok((requested_at, deadline))
}

fn same_committed_request(existing: &PendingApproval, candidate: &PendingApproval) -> bool {
    existing.review_request_id == candidate.review_request_id
        && existing.key_id == candidate.key_id
        && existing.nonce == candidate.nonce
        && existing.action_commitment == candidate.action_commitment
}

fn commit_evaluation(
    transaction: Transaction<'_>,
    outcome: EvaluatePendingOutcome,
) -> EvaluatePendingOutcome {
    match transaction.commit() {
        Ok(()) => outcome,
        Err(error) => EvaluatePendingOutcome::Faulted(sqlite_fault(error)),
    }
}

impl ApprovalStore for S2ApprovalStore {
    fn record_pending(&mut self, pending: PendingApproval) -> RecordPendingOutcome {
        let (requested_at, deadline) = match checked_sql_times(&pending) {
            Ok(times) => times,
            Err(fault) => return RecordPendingOutcome::Faulted(fault),
        };

        let transaction = match self
            .conn
            .transaction_with_behavior(TransactionBehavior::Immediate)
        {
            Ok(transaction) => transaction,
            Err(error) => return RecordPendingOutcome::Faulted(sqlite_fault(error)),
        };

        let existing = match load_by_identity(&transaction, &pending.key_id, &pending.nonce) {
            Ok(existing) => existing,
            Err(fault) => return RecordPendingOutcome::Faulted(fault),
        };

        if let Some(existing) = existing {
            if !same_committed_request(&existing.pending, &pending) {
                return RecordPendingOutcome::Faulted(GuardFault::InternalError);
            }

            return match transaction.commit() {
                Ok(()) => RecordPendingOutcome::AlreadyPending(existing.pending),
                Err(error) => RecordPendingOutcome::Faulted(sqlite_fault(error)),
            };
        }

        match load_by_review_request_id(&transaction, &pending.review_request_id) {
            Ok(None) => {}
            Ok(Some(_)) => {
                return RecordPendingOutcome::Faulted(GuardFault::InternalError);
            }
            Err(fault) => return RecordPendingOutcome::Faulted(fault),
        }

        let inserted = transaction.execute(
            "INSERT INTO pending_approvals (
                review_request_id,
                key_id,
                nonce,
                action_commitment,
                requested_at,
                deadline,
                status
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'requested')",
            params![
                &pending.review_request_id.as_bytes()[..],
                &pending.key_id[..],
                &pending.nonce[..],
                &pending.action_commitment[..],
                requested_at,
                deadline,
            ],
        );

        match inserted {
            Ok(1) => {}
            Ok(_) => return RecordPendingOutcome::Faulted(GuardFault::InternalError),
            Err(error) => return RecordPendingOutcome::Faulted(sqlite_fault(error)),
        }

        match transaction.commit() {
            Ok(()) => RecordPendingOutcome::Recorded(pending),
            Err(error) => RecordPendingOutcome::Faulted(sqlite_fault(error)),
        }
    }
    fn evaluate_pending(
        &mut self,
        review_request_id: &ReviewRequestId,
        now_unix_seconds: u64,
    ) -> EvaluatePendingOutcome {
        let now = match i64::try_from(now_unix_seconds) {
            Ok(now) => now,
            Err(_) => {
                return EvaluatePendingOutcome::Faulted(GuardFault::InternalError);
            }
        };

        let transaction = match self
            .conn
            .transaction_with_behavior(TransactionBehavior::Immediate)
        {
            Ok(transaction) => transaction,
            Err(error) => return EvaluatePendingOutcome::Faulted(sqlite_fault(error)),
        };

        let stored = match load_by_review_request_id(&transaction, review_request_id) {
            Ok(Some(stored)) => stored,
            Ok(None) => {
                return commit_evaluation(transaction, EvaluatePendingOutcome::NotFound);
            }
            Err(fault) => return EvaluatePendingOutcome::Faulted(fault),
        };

        match stored.status {
            ApprovalStatus::DeniedOnTimeout => commit_evaluation(
                transaction,
                EvaluatePendingOutcome::TimedOut(stored.pending),
            ),
            ApprovalStatus::Requested if now_unix_seconds <= stored.pending.deadline => {
                commit_evaluation(transaction, EvaluatePendingOutcome::Pending(stored.pending))
            }
            ApprovalStatus::Requested => {
                let updated = transaction.execute(
                    "UPDATE pending_approvals
                         SET status = 'denied_on_timeout'
                         WHERE review_request_id = ?1
                           AND status = 'requested'
                           AND deadline < ?2",
                    params![&review_request_id.as_bytes()[..], now],
                );

                match updated {
                    Ok(1) => commit_evaluation(
                        transaction,
                        EvaluatePendingOutcome::TimedOut(stored.pending),
                    ),
                    Ok(_) => EvaluatePendingOutcome::Faulted(GuardFault::InternalError),
                    Err(error) => EvaluatePendingOutcome::Faulted(sqlite_fault(error)),
                }
            }
        }
    }
}
