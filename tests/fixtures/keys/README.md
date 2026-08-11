# VAL-002 deterministic test keys

> **TEST KEY — not a trust root; do not use outside fixtures.**

VAL-002 derives two Ed25519 keypairs from deliberately public, fixed 32-byte
seeds in `tests/bypass-rust/src/val_002_fixtures.rs`. This material exists only
to make fixture output reproducible; it provides no secrecy or production
authority.

| Role | Seed | 16-byte `key_id` | Public key (hex) | Fixture K2 set |
|---|---|---|---|---|
| Registered fixture signer | `0x11` repeated 32 times | `DGR-TEST-KEY-001` | `d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737` | Present |
| Unknown-key fixture signer | `0x22` repeated 32 times | `DGR-TEST-KEY-999` | Derived only while authoring the negative fixture | Absent |

The fixture catalog carries only the registered public key as K2 test data.
It does not implement key lookup, signature verification, trust decisions, or
any other enforcement behavior.
