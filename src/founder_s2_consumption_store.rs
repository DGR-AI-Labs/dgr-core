//! {FOUNDER-AUTHORS}: T0 S2 durable-local consumption-store unit.
//!
//! One persistent SQLite connection is created per store instance and reused
//! for every consumption attempt.

use crate::founder_before_tool_call_floor::GuardFault;
use crate::founder_consumption_store::{ConsumeOutcome, ConsumptionStore};
use rusqlite::{Connection, params};
use std::path::Path;

#[derive(Debug)]
pub struct S2ConsumptionStore {
    conn: Connection,
}

impl S2ConsumptionStore {
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
            "CREATE TABLE IF NOT EXISTS consumed_authorizations (
                reference BLOB PRIMARY KEY
            ) STRICT;",
        )
        .map_err(|_| GuardFault::Unavailable)?;

        Ok(Self { conn })
    }
}

impl ConsumptionStore for S2ConsumptionStore {
    fn consume(&mut self, authorization_reference: &[u8]) -> ConsumeOutcome {
        match self.conn.execute(
            "INSERT INTO consumed_authorizations (reference) VALUES (?1)",
            params![authorization_reference],
        ) {
            Ok(_) => ConsumeOutcome::Consumed,
            Err(rusqlite::Error::SqliteFailure(error, _))
                if matches!(
                    error.extended_code,
                    rusqlite::ffi::SQLITE_CONSTRAINT_PRIMARYKEY
                        | rusqlite::ffi::SQLITE_CONSTRAINT_UNIQUE
                ) =>
            {
                ConsumeOutcome::AlreadyConsumed
            }
            Err(_) => ConsumeOutcome::Faulted(GuardFault::Unavailable),
        }
    }
}
