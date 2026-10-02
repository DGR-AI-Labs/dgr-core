use std::os::unix::fs::DirBuilderExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

use dgr_core_bypass_harness::founder_consumption_store::{ConsumeOutcome, ConsumptionStore};
use dgr_core_bypass_harness::founder_s2_consumption_store::S2ConsumptionStore;

struct TemporaryDatabase {
    directory: PathBuf,
    path: PathBuf,
}

impl TemporaryDatabase {
    fn new() -> Self {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock must be after the Unix epoch")
            .as_nanos();
        static NEXT: AtomicU64 = AtomicU64::new(0);
        for _ in 0..128 {
            let sequence = NEXT.fetch_add(1, Ordering::Relaxed);
            let directory = std::env::temp_dir().join(format!(
                "dgr-core-s2-restart-{}-{unique}-{sequence}",
                std::process::id()
            ));
            match Self::create_at(directory) {
                Ok(database) => return database,
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => panic!("create private database directory: {error}"),
            }
        }
        panic!("temporary database directory collisions exhausted");
    }

    fn create_at(directory: PathBuf) -> std::io::Result<Self> {
        // Atomic, non-recursive creation refuses existing directories and symlinks.
        // The name need not be secret: only a successful creator uses this 0700 directory.
        std::fs::DirBuilder::new().mode(0o700).create(&directory)?;
        let path = directory.join("database.sqlite3");
        Ok(Self { directory, path })
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TemporaryDatabase {
    fn drop(&mut self) {
        if let Err(error) = std::fs::remove_dir_all(&self.directory)
            && !std::thread::panicking()
        {
            panic!("remove private database directory: {error}");
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

#[test]
fn temporary_database_refuses_preexisting_directory_and_symlink() {
    use std::os::unix::fs::{PermissionsExt, symlink};
    let owner = TemporaryDatabase::new();
    assert_eq!(
        std::fs::metadata(&owner.directory)
            .unwrap()
            .permissions()
            .mode()
            & 0o077,
        0
    );
    let sentinel = owner.directory.join("sentinel");
    std::fs::write(&sentinel, b"unchanged").unwrap();
    assert_eq!(
        TemporaryDatabase::create_at(owner.directory.clone())
            .err()
            .unwrap()
            .kind(),
        std::io::ErrorKind::AlreadyExists
    );
    let link = owner.directory.join("precreated-link");
    symlink(&owner.directory, &link).unwrap();
    assert_eq!(
        TemporaryDatabase::create_at(link).err().unwrap().kind(),
        std::io::ErrorKind::AlreadyExists
    );
    assert_eq!(std::fs::read(sentinel).unwrap(), b"unchanged");
    let directory = owner.directory.clone();
    drop(owner);
    assert!(!directory.exists());
}
