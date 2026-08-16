use std::path::{Path, PathBuf};
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

use dgr_core_bypass_harness::founder_consumption_store::{ConsumeOutcome, ConsumptionStore};
use dgr_core_bypass_harness::founder_s2_consumption_store::S2ConsumptionStore;

struct TemporaryDatabase {
    path: PathBuf,
}

impl TemporaryDatabase {
    fn new() -> Self {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock must be after the Unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "dgr-core-s2-restart-{}-{unique}.sqlite3",
            std::process::id()
        ));
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TemporaryDatabase {
    fn drop(&mut self) {
        for suffix in ["", "-journal", "-shm", "-wal"] {
            let candidate = PathBuf::from(format!("{}{suffix}", self.path.display()));
            if let Err(error) = std::fs::remove_file(&candidate) {
                assert_eq!(
                    error.kind(),
                    std::io::ErrorKind::NotFound,
                    "failed to remove temporary SQLite file {}",
                    candidate.display()
                );
            }
        }
    }
}

#[test]
fn file_backed_consumption_survives_connection_restart() {
    let database = TemporaryDatabase::new();
    let authorization_reference = [0xA5; 16];

    {
        let mut first_process_store =
            S2ConsumptionStore::open_at(database.path()).expect("open first file-backed store");
        assert_eq!(
            first_process_store.consume(&authorization_reference),
            ConsumeOutcome::Consumed
        );
    }

    {
        let mut restarted_process_store =
            S2ConsumptionStore::open_at(database.path()).expect("reopen file-backed store");
        assert_eq!(
            restarted_process_store.consume(&authorization_reference),
            ConsumeOutcome::AlreadyConsumed
        );
    }
}

#[test]
fn concurrent_presentations_cannot_both_consume() {
    let database = TemporaryDatabase::new();
    let authorization_reference = [0x5A; 16];
    let first_store =
        S2ConsumptionStore::open_at(database.path()).expect("open first concurrent store");
    let second_store =
        S2ConsumptionStore::open_at(database.path()).expect("open second concurrent store");
    let barrier = Arc::new(Barrier::new(3));

    let workers = [first_store, second_store].map(|mut store| {
        let barrier = Arc::clone(&barrier);
        thread::spawn(move || {
            barrier.wait();
            store.consume(&authorization_reference)
        })
    });

    barrier.wait();
    let outcomes = workers.map(|worker| worker.join().expect("consumption worker"));
    let consumed = outcomes
        .iter()
        .filter(|outcome| **outcome == ConsumeOutcome::Consumed)
        .count();

    assert_eq!(
        consumed, 1,
        "concurrent presentation permitted more than once"
    );
    assert!(
        outcomes.iter().all(|outcome| matches!(
            outcome,
            ConsumeOutcome::Consumed | ConsumeOutcome::AlreadyConsumed | ConsumeOutcome::Faulted(_)
        )),
        "concurrent presentation produced an unknown outcome"
    );
}
