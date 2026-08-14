//! Deterministic VAL-002 fixture authoring.
//!
//! This module constructs test data only. It deliberately contains no token
//! verification, authorization decision, replay consumption, or trust-store
//! lookup logic. Expected outcomes are conformance labels, not computed
//! decisions.

use base64ct::{Base64UrlUnpadded, Encoding};
use ed25519_dalek::{Signer, SigningKey};
use sha2::{Digest, Sha256};

pub const FIXED_NOW_UNIX_SECONDS: u64 = 1_800_000_000;
pub const MAXIMUM_LIFETIME_SECONDS: u64 = 300;
pub const EXPIRY_SKEW_SECONDS: u64 = 30;
pub const TOKEN_WIRE_LENGTH: usize = 145;
pub const SIGNATURE_PREIMAGE_LENGTH: usize = 90;
pub const REGISTERED_KEY_ID: [u8; 16] = *b"DGR-TEST-KEY-001";
pub const UNKNOWN_KEY_ID: [u8; 16] = *b"DGR-TEST-KEY-999";

const FORMAT_VERSION: u8 = 1;
const DOMAIN_TAG: &[u8; 9] = b"DGR-CAP1\x00";

// TEST KEY ONLY — not a trust root; do not use outside deterministic fixtures.
const REGISTERED_TEST_SIGNING_SEED: [u8; 32] = [0x11; 32];
// TEST KEY ONLY — deliberately absent from the fixture K2 set.
const UNKNOWN_TEST_SIGNING_SEED: [u8; 32] = [0x22; 32];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExpectedFixtureOutcome {
    Allow,
    Deny,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedFixtureClock {
    pub now_unix_seconds: u64,
}

/// Clock seam supplied as fixture plumbing for future founder-authored
/// conformance tests. Reading a timestamp is not an expiry decision.
pub trait FixtureClock {
    fn now_unix_seconds(&self) -> u64;
}

impl FixtureClock for FixedFixtureClock {
    fn now_unix_seconds(&self) -> u64 {
        self.now_unix_seconds
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PayInvoiceFixtureRequest {
    pub action: &'static str,
    pub amount: &'static str,
    pub currency: &'static str,
    pub destination: &'static str,
    pub invoice_id: &'static str,
    pub source_account: &'static str,
    pub idempotency_key: &'static str,
    pub memo: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureToken {
    pub format_version: u8,
    pub key_id: [u8; 16],
    pub issued_at: u64,
    pub signed_expires_at: u64,
    pub transmitted_expires_at: u64,
    pub nonce: [u8; 16],
    pub action_commitment: [u8; 32],
    pub signature: [u8; 64],
    pub signature_preimage: Vec<u8>,
    pub wire_bytes: Vec<u8>,
    pub base64url: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixtureK2PublicKey {
    pub key_id: [u8; 16],
    pub public_key: [u8; 32],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Val002Fixture {
    pub id: &'static str,
    pub attack_ids: &'static [&'static str],
    pub description: &'static str,
    pub expected_sequence: Vec<ExpectedFixtureOutcome>,
    pub clock: FixedFixtureClock,
    pub request: Option<PayInvoiceFixtureRequest>,
    pub token: Option<FixtureToken>,
    pub token_canonical_action_bytes: Option<Vec<u8>>,
    pub presented_action_canonical_bytes: Option<Vec<u8>>,
    pub malformed_reason: Option<&'static str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Val002FixtureCatalog {
    /// Fixture data representing the pinned K2 public-key set. This is not a
    /// verifier or lookup service.
    pub fixture_k2_keys: Vec<FixtureK2PublicKey>,
    pub fixtures: Vec<Val002Fixture>,
}

impl Val002FixtureCatalog {
    pub fn by_id(&self, id: &str) -> Option<&Val002Fixture> {
        self.fixtures.iter().find(|fixture| fixture.id == id)
    }
}

pub fn fixture_catalog() -> Val002FixtureCatalog {
    let clock = FixedFixtureClock {
        now_unix_seconds: FIXED_NOW_UNIX_SECONDS,
    };
    let registered_signing_key = SigningKey::from_bytes(&REGISTERED_TEST_SIGNING_SEED);
    let unknown_signing_key = SigningKey::from_bytes(&UNKNOWN_TEST_SIGNING_SEED);
    let baseline_request = baseline_request();
    let baseline_canonical = fixture_canonical_action_bytes(&baseline_request);
    let baseline_commitment = fixture_action_commitment(&baseline_canonical);
    let valid_token = author_fixture_token(
        &registered_signing_key,
        REGISTERED_KEY_ID,
        FIXED_NOW_UNIX_SECONDS - 60,
        FIXED_NOW_UNIX_SECONDS + 240,
        [0x01; 16],
        baseline_commitment,
    );

    let mut fixtures = vec![fixture(
        "valid",
        &["ATK-11"],
        "valid signed pay_invoice token and matching request (ATK-11 control)",
        vec![ExpectedFixtureOutcome::Allow],
        baseline_request,
        valid_token.clone(),
        baseline_canonical.clone(),
        baseline_canonical.clone(),
    )];

    fixtures.push(fixture(
        "expired-beyond-skew",
        &["ATK-02"],
        "signed token expired 120 seconds before the fixed clock",
        vec![ExpectedFixtureOutcome::Deny],
        baseline_request,
        author_fixture_token(
            &registered_signing_key,
            REGISTERED_KEY_ID,
            FIXED_NOW_UNIX_SECONDS - 420,
            FIXED_NOW_UNIX_SECONDS - 120,
            [0x02; 16],
            baseline_commitment,
        ),
        baseline_canonical.clone(),
        baseline_canonical.clone(),
    ));
    fixtures.push(fixture(
        "expired-within-skew",
        &["ATK-02"],
        "signed token expires exactly 30 seconds before the fixed clock",
        vec![ExpectedFixtureOutcome::Allow],
        baseline_request,
        author_fixture_token(
            &registered_signing_key,
            REGISTERED_KEY_ID,
            FIXED_NOW_UNIX_SECONDS - 330,
            FIXED_NOW_UNIX_SECONDS - 30,
            [0x03; 16],
            baseline_commitment,
        ),
        baseline_canonical.clone(),
        baseline_canonical.clone(),
    ));
    fixtures.push(fixture(
        "expired-just-outside-skew",
        &["ATK-02"],
        "signed token expires 31 seconds before the fixed clock",
        vec![ExpectedFixtureOutcome::Deny],
        baseline_request,
        author_fixture_token(
            &registered_signing_key,
            REGISTERED_KEY_ID,
            FIXED_NOW_UNIX_SECONDS - 331,
            FIXED_NOW_UNIX_SECONDS - 31,
            [0x04; 16],
            baseline_commitment,
        ),
        baseline_canonical.clone(),
        baseline_canonical.clone(),
    ));

    fixtures.push(fixture(
        "lifetime-over-maximum",
        &["ATK-02"],
        "correctly signed token has a 301-second lifetime",
        vec![ExpectedFixtureOutcome::Deny],
        baseline_request,
        author_fixture_token(
            &registered_signing_key,
            REGISTERED_KEY_ID,
            FIXED_NOW_UNIX_SECONDS - 60,
            FIXED_NOW_UNIX_SECONDS + 241,
            [0x06; 16],
            baseline_commitment,
        ),
        baseline_canonical.clone(),
        baseline_canonical.clone(),
    ));

    fixtures.push(fixture(
        "lifetime-reversed",
        &["ATK-02"],
        "correctly signed token expires before it is issued",
        vec![ExpectedFixtureOutcome::Deny],
        baseline_request,
        author_fixture_token(
            &registered_signing_key,
            REGISTERED_KEY_ID,
            FIXED_NOW_UNIX_SECONDS + 60,
            FIXED_NOW_UNIX_SECONDS + 30,
            [0x07; 16],
            baseline_commitment,
        ),
        baseline_canonical.clone(),
        baseline_canonical.clone(),
    ));

    for (id, attack_ids, description, changed_request) in [
        (
            "swap-amount",
            &["ATK-08"][..],
            "amount changed after the baseline token was signed",
            PayInvoiceFixtureRequest {
                amount: "100001",
                ..baseline_request
            },
        ),
        (
            "swap-destination",
            &["ATK-08"][..],
            "destination changed after the baseline token was signed",
            PayInvoiceFixtureRequest {
                destination: "acct_payee_32",
                ..baseline_request
            },
        ),
        (
            "swap-invoice-id",
            &["ATK-09"][..],
            "invoice_id changed after the baseline token was signed",
            PayInvoiceFixtureRequest {
                invoice_id: "INV-8843",
                ..baseline_request
            },
        ),
        (
            "wrong-action",
            &["ATK-11"][..],
            "action literal changed after the baseline token was signed",
            PayInvoiceFixtureRequest {
                action: "refund_invoice",
                ..baseline_request
            },
        ),
        (
            "swap-source-account",
            &["ATK-08"][..],
            "source_account changed after the baseline token was signed",
            PayInvoiceFixtureRequest {
                source_account: "acct_ops_08",
                ..baseline_request
            },
        ),
    ] {
        fixtures.push(fixture(
            id,
            attack_ids,
            description,
            vec![ExpectedFixtureOutcome::Deny],
            changed_request,
            valid_token.clone(),
            baseline_canonical.clone(),
            fixture_canonical_action_bytes(&changed_request),
        ));
    }

    for (id, description, changed_request) in [
        (
            "change-idempotency-key",
            "non-binding idempotency_key changed after signing",
            PayInvoiceFixtureRequest {
                idempotency_key: "idem-0002",
                ..baseline_request
            },
        ),
        (
            "change-memo",
            "non-binding memo changed after signing",
            PayInvoiceFixtureRequest {
                memo: "updated fixture memo",
                ..baseline_request
            },
        ),
    ] {
        fixtures.push(fixture(
            id,
            &["ATK-08", "ATK-09", "ATK-11"],
            description,
            vec![ExpectedFixtureOutcome::Allow],
            changed_request,
            valid_token.clone(),
            baseline_canonical.clone(),
            fixture_canonical_action_bytes(&changed_request),
        ));
    }

    fixtures.push(fixture(
        "replay",
        &["ATK-03"],
        "the exact same valid token is presented twice",
        vec![ExpectedFixtureOutcome::Allow, ExpectedFixtureOutcome::Deny],
        baseline_request,
        valid_token.clone(),
        baseline_canonical.clone(),
        baseline_canonical.clone(),
    ));
    fixtures.push(fixture(
        "unknown-key-id",
        &["ATK-10"],
        "well-formed token signed by a key absent from the fixture K2 set",
        vec![ExpectedFixtureOutcome::Deny],
        baseline_request,
        author_fixture_token(
            &unknown_signing_key,
            UNKNOWN_KEY_ID,
            FIXED_NOW_UNIX_SECONDS - 60,
            FIXED_NOW_UNIX_SECONDS + 240,
            [0x05; 16],
            baseline_commitment,
        ),
        baseline_canonical.clone(),
        baseline_canonical.clone(),
    ));
    fixtures.push(Val002Fixture {
        id: "absent-token",
        attack_ids: &["ATK-01"],
        description: "effectful request has no capability token",
        expected_sequence: vec![ExpectedFixtureOutcome::Deny],
        clock,
        request: Some(baseline_request),
        token: None,
        token_canonical_action_bytes: None,
        presented_action_canonical_bytes: Some(baseline_canonical.clone()),
        malformed_reason: None,
    });

    let mut tampered_token = valid_token.clone();
    tampered_token.transmitted_expires_at = FIXED_NOW_UNIX_SECONDS + 600;
    tampered_token.wire_bytes[25..33]
        .copy_from_slice(&tampered_token.transmitted_expires_at.to_be_bytes());
    tampered_token.base64url = Base64UrlUnpadded::encode_string(&tampered_token.wire_bytes);
    fixtures.push(fixture(
        "tampered-expires-at",
        &["ATK-10"],
        "expires_at wire bytes changed without regenerating the signature",
        vec![ExpectedFixtureOutcome::Deny],
        baseline_request,
        tampered_token,
        baseline_canonical.clone(),
        baseline_canonical.clone(),
    ));

    for (id, description, amount, reason) in [
        (
            "malformed-amount-decimal",
            "amount uses a decimal representation forbidden by ARCH-005",
            "1000.00",
            "amount must be canonical unsigned base-10 minor units",
        ),
        (
            "malformed-amount-leading-zero",
            "amount has a forbidden leading zero",
            "0100000",
            "non-zero amount must not contain leading zeroes",
        ),
    ] {
        fixtures.push(Val002Fixture {
            id,
            attack_ids: &["ATK-08"],
            description,
            expected_sequence: vec![ExpectedFixtureOutcome::Deny],
            clock,
            request: Some(PayInvoiceFixtureRequest {
                amount,
                ..baseline_request
            }),
            token: Some(valid_token.clone()),
            token_canonical_action_bytes: Some(baseline_canonical.clone()),
            presented_action_canonical_bytes: None,
            malformed_reason: Some(reason),
        });
    }

    Val002FixtureCatalog {
        fixture_k2_keys: vec![FixtureK2PublicKey {
            key_id: REGISTERED_KEY_ID,
            public_key: registered_signing_key.verifying_key().to_bytes(),
        }],
        fixtures,
    }
}

pub fn baseline_request() -> PayInvoiceFixtureRequest {
    PayInvoiceFixtureRequest {
        action: "pay_invoice",
        amount: "100000",
        currency: "USD",
        destination: "acct_payee_31",
        invoice_id: "INV-8842",
        source_account: "acct_ops_07",
        idempotency_key: "idem-0001",
        memo: "deterministic fixture memo",
    }
}

/// Encodes already-canonical fixture literals using ARCH-005's frozen field
/// tags and lengths. This helper does not validate or normalize input.
pub fn fixture_canonical_action_bytes(request: &PayInvoiceFixtureRequest) -> Vec<u8> {
    let fields = [
        (0x01, request.action),
        (0x02, request.amount),
        (0x03, request.currency),
        (0x04, request.destination),
        (0x05, request.invoice_id),
        (0x06, request.source_account),
    ];
    let mut encoded = Vec::new();
    for (tag, value) in fields {
        encoded.push(tag);
        encoded.extend_from_slice(&(value.len() as u32).to_be_bytes());
        encoded.extend_from_slice(value.as_bytes());
    }
    encoded
}

fn fixture_action_commitment(canonical_action_bytes: &[u8]) -> [u8; 32] {
    Sha256::digest(canonical_action_bytes).into()
}

fn fixture(
    id: &'static str,
    attack_ids: &'static [&'static str],
    description: &'static str,
    expected_sequence: Vec<ExpectedFixtureOutcome>,
    request: PayInvoiceFixtureRequest,
    token: FixtureToken,
    token_canonical_action_bytes: Vec<u8>,
    presented_action_canonical_bytes: Vec<u8>,
) -> Val002Fixture {
    Val002Fixture {
        id,
        attack_ids,
        description,
        expected_sequence,
        clock: FixedFixtureClock {
            now_unix_seconds: FIXED_NOW_UNIX_SECONDS,
        },
        request: Some(request),
        token: Some(token),
        token_canonical_action_bytes: Some(token_canonical_action_bytes),
        presented_action_canonical_bytes: Some(presented_action_canonical_bytes),
        malformed_reason: None,
    }
}

fn author_fixture_token(
    signing_key: &SigningKey,
    key_id: [u8; 16],
    issued_at: u64,
    expires_at: u64,
    nonce: [u8; 16],
    action_commitment: [u8; 32],
) -> FixtureToken {
    let mut signature_preimage = Vec::with_capacity(SIGNATURE_PREIMAGE_LENGTH);
    signature_preimage.extend_from_slice(DOMAIN_TAG);
    signature_preimage.push(FORMAT_VERSION);
    signature_preimage.extend_from_slice(&key_id);
    signature_preimage.extend_from_slice(&issued_at.to_be_bytes());
    signature_preimage.extend_from_slice(&expires_at.to_be_bytes());
    signature_preimage.extend_from_slice(&nonce);
    signature_preimage.extend_from_slice(&action_commitment);

    let signature = signing_key.sign(&signature_preimage).to_bytes();
    let mut wire_bytes = Vec::with_capacity(TOKEN_WIRE_LENGTH);
    wire_bytes.push(FORMAT_VERSION);
    wire_bytes.extend_from_slice(&key_id);
    wire_bytes.extend_from_slice(&issued_at.to_be_bytes());
    wire_bytes.extend_from_slice(&expires_at.to_be_bytes());
    wire_bytes.extend_from_slice(&nonce);
    wire_bytes.extend_from_slice(&action_commitment);
    wire_bytes.extend_from_slice(&signature);
    let base64url = Base64UrlUnpadded::encode_string(&wire_bytes);

    FixtureToken {
        format_version: FORMAT_VERSION,
        key_id,
        issued_at,
        signed_expires_at: expires_at,
        transmitted_expires_at: expires_at,
        nonce,
        action_commitment,
        signature,
        signature_preimage,
        wire_bytes,
        base64url,
    }
}
