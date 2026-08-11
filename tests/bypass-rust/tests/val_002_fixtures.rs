use std::collections::BTreeSet;

use dgr_core_bypass_harness::attack_by_id;
use dgr_core_bypass_harness::fixtures::request_for_val_002_fixture;
use dgr_core_bypass_harness::val_002_fixtures::{
    EXPIRY_SKEW_SECONDS, ExpectedFixtureOutcome, FIXED_NOW_UNIX_SECONDS, FixtureClock,
    MAXIMUM_LIFETIME_SECONDS, REGISTERED_KEY_ID, SIGNATURE_PREIMAGE_LENGTH, TOKEN_WIRE_LENGTH,
    UNKNOWN_KEY_ID, fixture_canonical_action_bytes, fixture_catalog,
};

fn lowercase_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[test]
fn registered_test_public_key_and_valid_artifact_are_golden() {
    let catalog = fixture_catalog();
    let valid = catalog.by_id("valid").expect("valid fixture");
    let token = valid.token.as_ref().expect("valid token");
    assert_eq!(
        lowercase_hex(&catalog.fixture_k2_keys[0].public_key),
        "d04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737"
    );
    assert_eq!(
        lowercase_hex(&token.action_commitment),
        "eae6c0a0fea8c2a0902855a367108b90d359c491aab607cc33bd32ec18c76ef3"
    );
    assert_eq!(
        token.base64url,
        "AURHUi1URVNULUtFWS0wMDEAAAAAa0nRxAAAAABrSdLwAQEBAQEBAQEBAQEBAQEBAermwKD-qMKgkChVo2cQi5DTWcSRqrYHzDO9MuwYx27zJ1jcKrddW9PZg_AH0nK7pxAAf7li9ZIzyLJ4BAEWSyhV6jv5JS3WlXkZPWRKwuV7Sq7ro_GjpEmLMtPbkarpDw"
    );
}

#[test]
fn catalog_is_complete_unique_and_deterministic() {
    let first = fixture_catalog();
    let second = fixture_catalog();
    assert_eq!(
        first, second,
        "fixed inputs must reproduce identical fixtures"
    );

    let expected_ids = [
        "valid",
        "expired-beyond-skew",
        "expired-within-skew",
        "expired-just-outside-skew",
        "swap-amount",
        "swap-destination",
        "swap-invoice-id",
        "wrong-action",
        "swap-source-account",
        "change-idempotency-key",
        "change-memo",
        "replay",
        "unknown-key-id",
        "absent-token",
        "tampered-expires-at",
        "malformed-amount-decimal",
        "malformed-amount-leading-zero",
    ];
    let actual: BTreeSet<_> = first.fixtures.iter().map(|fixture| fixture.id).collect();
    let expected: BTreeSet<_> = expected_ids.into_iter().collect();
    assert_eq!(actual, expected);
    assert_eq!(first.fixtures.len(), actual.len(), "duplicate fixture id");
    assert!(
        first
            .fixtures
            .iter()
            .all(|fixture| !fixture.attack_ids.is_empty()),
        "every fixture must name at least one conformance attack/control"
    );
}

#[test]
fn arch_005_canonical_bytes_bind_exactly_six_fields_in_frozen_order() {
    let catalog = fixture_catalog();
    let valid = catalog.by_id("valid").expect("valid fixture");
    let request = valid.request.expect("presented action");
    let expected = concat!(
        "\x01\x00\x00\x00\x0bpay_invoice",
        "\x02\x00\x00\x00\x06100000",
        "\x03\x00\x00\x00\x03USD",
        "\x04\x00\x00\x00\x0dacct_payee_31",
        "\x05\x00\x00\x00\x08INV-8842",
        "\x06\x00\x00\x00\x0bacct_ops_07",
    )
    .as_bytes();

    assert_eq!(fixture_canonical_action_bytes(&request), expected);
    assert_eq!(
        valid.token_canonical_action_bytes.as_deref(),
        Some(expected)
    );
    assert_eq!(
        valid.presented_action_canonical_bytes.as_deref(),
        Some(expected)
    );
}

#[test]
fn arch_006_artifacts_have_the_frozen_layout_and_transport() {
    let catalog = fixture_catalog();
    for fixture in &catalog.fixtures {
        let Some(token) = &fixture.token else {
            continue;
        };
        assert_eq!(token.format_version, 1, "{} version", fixture.id);
        assert_eq!(
            token.signature_preimage.len(),
            SIGNATURE_PREIMAGE_LENGTH,
            "{} preimage length",
            fixture.id
        );
        assert_eq!(
            token.wire_bytes.len(),
            TOKEN_WIRE_LENGTH,
            "{} wire",
            fixture.id
        );
        assert_eq!(&token.signature_preimage[..9], b"DGR-CAP1\x00");
        assert_eq!(token.signature_preimage[9], 1);
        assert_eq!(&token.signature_preimage[10..26], &token.key_id);
        assert_eq!(
            &token.signature_preimage[26..34],
            &token.issued_at.to_be_bytes()
        );
        assert_eq!(
            &token.signature_preimage[34..42],
            &token.signed_expires_at.to_be_bytes()
        );
        assert_eq!(&token.signature_preimage[42..58], &token.nonce);
        assert_eq!(&token.signature_preimage[58..90], &token.action_commitment);

        assert_eq!(token.wire_bytes[0], 1);
        assert_eq!(&token.wire_bytes[1..17], &token.key_id);
        assert_eq!(&token.wire_bytes[17..25], &token.issued_at.to_be_bytes());
        assert_eq!(
            &token.wire_bytes[25..33],
            &token.transmitted_expires_at.to_be_bytes()
        );
        assert_eq!(&token.wire_bytes[33..49], &token.nonce);
        assert_eq!(&token.wire_bytes[49..81], &token.action_commitment);
        assert_eq!(&token.wire_bytes[81..145], &token.signature);
        assert!(!token.base64url.contains('='), "{} has padding", fixture.id);
    }
}

#[test]
fn valid_and_skew_boundary_outcomes_are_explicit_fixture_data() {
    let catalog = fixture_catalog();
    let valid = catalog.by_id("valid").expect("valid fixture");
    let valid_token = valid.token.as_ref().expect("valid token");
    assert_eq!(valid.clock.now_unix_seconds(), FIXED_NOW_UNIX_SECONDS);
    assert_eq!(
        valid_token.signed_expires_at - valid_token.issued_at,
        MAXIMUM_LIFETIME_SECONDS
    );
    assert_eq!(valid.expected_sequence, [ExpectedFixtureOutcome::Allow]);

    let within = catalog
        .by_id("expired-within-skew")
        .expect("within-skew fixture");
    let within_token = within.token.as_ref().expect("within-skew token");
    assert_eq!(
        FIXED_NOW_UNIX_SECONDS - within_token.signed_expires_at,
        EXPIRY_SKEW_SECONDS
    );
    assert_eq!(within.expected_sequence, [ExpectedFixtureOutcome::Allow]);

    let expired = catalog
        .by_id("expired-beyond-skew")
        .expect("expired fixture");
    let expired_token = expired.token.as_ref().expect("expired token");
    assert_eq!(
        FIXED_NOW_UNIX_SECONDS - expired_token.signed_expires_at,
        120
    );
    assert_eq!(expired.expected_sequence, [ExpectedFixtureOutcome::Deny]);

    let just_outside = catalog
        .by_id("expired-just-outside-skew")
        .expect("just-outside fixture");
    let just_outside_token = just_outside.token.as_ref().expect("just-outside token");
    assert_eq!(
        FIXED_NOW_UNIX_SECONDS - just_outside_token.signed_expires_at,
        EXPIRY_SKEW_SECONDS + 1
    );
    assert_eq!(
        just_outside.expected_sequence,
        [ExpectedFixtureOutcome::Deny]
    );
}

#[test]
fn binding_and_nonbinding_changes_have_the_required_expected_outcomes() {
    let catalog = fixture_catalog();
    let valid = catalog.by_id("valid").expect("valid fixture");
    let signed_bytes = valid
        .token_canonical_action_bytes
        .as_ref()
        .expect("signed bytes");

    for id in [
        "swap-amount",
        "swap-destination",
        "swap-invoice-id",
        "wrong-action",
        "swap-source-account",
    ] {
        let fixture = catalog.by_id(id).expect("bound swap fixture");
        assert_ne!(
            fixture.presented_action_canonical_bytes.as_ref(),
            Some(signed_bytes),
            "{id} did not change the canonical bytes"
        );
        assert_eq!(fixture.expected_sequence, [ExpectedFixtureOutcome::Deny]);
    }

    for id in ["change-idempotency-key", "change-memo"] {
        let fixture = catalog.by_id(id).expect("non-binding fixture");
        assert_eq!(
            fixture.presented_action_canonical_bytes.as_ref(),
            Some(signed_bytes)
        );
        assert_eq!(fixture.expected_sequence, [ExpectedFixtureOutcome::Allow]);
    }
}

#[test]
fn replay_absence_unknown_key_and_tampering_are_wired_without_deciding() {
    let catalog = fixture_catalog();
    let replay = catalog.by_id("replay").expect("replay fixture");
    let valid = catalog.by_id("valid").expect("valid fixture");
    assert_eq!(
        replay.token, valid.token,
        "replay must reuse the exact token"
    );
    assert_eq!(
        replay.expected_sequence,
        [ExpectedFixtureOutcome::Allow, ExpectedFixtureOutcome::Deny]
    );

    let absent = catalog.by_id("absent-token").expect("absent fixture");
    assert!(absent.token.is_none());
    assert_eq!(absent.expected_sequence, [ExpectedFixtureOutcome::Deny]);

    assert_eq!(catalog.fixture_k2_keys.len(), 1);
    assert_eq!(catalog.fixture_k2_keys[0].key_id, REGISTERED_KEY_ID);
    let unknown = catalog
        .by_id("unknown-key-id")
        .expect("unknown key fixture");
    assert_eq!(
        unknown.token.as_ref().expect("unknown token").key_id,
        UNKNOWN_KEY_ID
    );
    assert_ne!(UNKNOWN_KEY_ID, REGISTERED_KEY_ID);
    assert_eq!(unknown.expected_sequence, [ExpectedFixtureOutcome::Deny]);

    let tampered = catalog
        .by_id("tampered-expires-at")
        .expect("tampered fixture");
    let tampered_token = tampered.token.as_ref().expect("tampered token");
    let valid_token = valid.token.as_ref().expect("valid token");
    assert_ne!(
        tampered_token.transmitted_expires_at,
        tampered_token.signed_expires_at
    );
    assert_eq!(
        tampered_token.signature_preimage,
        valid_token.signature_preimage
    );
    assert_eq!(tampered_token.signature, valid_token.signature);
    assert_ne!(tampered_token.wire_bytes, valid_token.wire_bytes);
    assert_eq!(tampered.expected_sequence, [ExpectedFixtureOutcome::Deny]);
}

#[test]
fn malformed_amounts_are_data_not_silently_canonicalized() {
    let catalog = fixture_catalog();
    for id in ["malformed-amount-decimal", "malformed-amount-leading-zero"] {
        let fixture = catalog.by_id(id).expect("malformed fixture");
        assert!(fixture.malformed_reason.is_some());
        assert!(fixture.presented_action_canonical_bytes.is_none());
        assert_eq!(fixture.expected_sequence, [ExpectedFixtureOutcome::Deny]);
    }
}

#[test]
fn attack_tags_resolve_and_fixture_token_bytes_reach_the_conformance_adapter() {
    let catalog = fixture_catalog();
    for fixture in &catalog.fixtures {
        for attack_id in fixture.attack_ids {
            let case = attack_by_id(attack_id).expect("fixture attack id must be registered");
            let request = request_for_val_002_fixture(case, fixture);
            assert_eq!(
                request.capability_token.map(|token| token.bytes),
                fixture
                    .token
                    .as_ref()
                    .map(|token| token.wire_bytes.as_slice()),
                "{} token was not wired to {}",
                fixture.id,
                attack_id
            );
        }
    }
}
