use std::collections::BTreeSet;

use dgr_core_bypass_harness::RequiredOutcome;
use dgr_core_bypass_harness::attack_by_id;
use dgr_core_bypass_harness::founder_token_verification::{VerifyOutcome, verify_capability_token};
use dgr_core_bypass_harness::val_002_fixtures::{
    ExpectedFixtureOutcome, FixtureClock, fixture_catalog as val_002_catalog,
};
use dgr_core_bypass_harness::val_004_fixtures::{
    ABOVE_THRESHOLD_AMOUNT, APPROVAL_REQUIRED_ABOVE_MINOR_UNITS, APPROVAL_WINDOW_SECONDS,
    ATTACK_ID, BELOW_THRESHOLD_CONTROL_AMOUNT, ExpectedVal004Observation, FIXED_DEADLINE,
    FIXED_REQUESTED_AT, FIXED_REVIEW_REQUEST_ID, NonceExpectation,
    PENDING_FOUNDER_CONSTANT_MIRROR_ASSERTION, PendingRecordExpectation, Val004Surface,
    before_tool_call_request, fixture_catalog,
};
use sha2::{Digest, Sha256};

#[test]
fn catalog_is_complete_unique_and_deterministic() {
    let first = fixture_catalog();
    let second = fixture_catalog();
    assert_eq!(
        first, second,
        "fixed inputs must reproduce identical fixtures"
    );

    let expected: BTreeSet<_> = [
        "escalation-required",
        "timeout-deadline-minus-one",
        "timeout-at-deadline",
        "timeout-deadline-plus-one",
        "re-presentation-does-not-extend",
        "below-threshold-control",
    ]
    .into_iter()
    .collect();
    let actual: BTreeSet<_> = first.fixtures.iter().map(|fixture| fixture.id).collect();

    assert_eq!(actual, expected);
    assert_eq!(first.fixtures.len(), actual.len(), "duplicate fixture id");
    assert!(
        first
            .fixtures
            .iter()
            .all(|fixture| fixture.attack_id == ATTACK_ID)
    );
}

#[test]
fn above_threshold_artifact_is_valid_and_bound_to_its_presented_action() {
    let catalog = fixture_catalog();
    let fixture = catalog
        .by_id("escalation-required")
        .expect("escalation fixture");
    let token = fixture.token.as_ref().expect("valid fixture token");
    let request = fixture.request.expect("presented action");
    let canonical = fixture
        .token_canonical_action_bytes
        .as_ref()
        .expect("canonical action bytes");

    assert_eq!(request.amount, ABOVE_THRESHOLD_AMOUNT);
    assert_eq!(request.amount, "1000001");
    assert!(
        request
            .amount
            .parse::<u64>()
            .expect("canonical fixture amount")
            > APPROVAL_REQUIRED_ABOVE_MINOR_UNITS
    );
    assert!(matches!(
        verify_capability_token(&token.wire_bytes),
        VerifyOutcome::Verified(_)
    ));
    assert_eq!(
        <[u8; 32]>::from(Sha256::digest(canonical)),
        token.action_commitment
    );
    assert_eq!(token.signed_expires_at - token.issued_at, 300);
    assert!(token.issued_at <= fixture.clock.now_unix_seconds());
    assert!(fixture.clock.now_unix_seconds() <= token.signed_expires_at);
    assert_eq!(
        fixture.nonce_expectation,
        Some(NonceExpectation::RemainsUnconsumed)
    );
    assert_eq!(
        fixture.expected,
        ExpectedVal004Observation::Escalated {
            review_request_id: FIXED_REVIEW_REQUEST_ID,
            deadline: FIXED_DEADLINE,
            authorization_issued: false,
            effectful_invocations: 0,
        }
    );
}

#[test]
fn timeout_boundary_is_three_explicit_pending_record_cases() {
    let catalog = fixture_catalog();
    for (id, clock) in [
        ("timeout-deadline-minus-one", FIXED_DEADLINE - 1),
        ("timeout-at-deadline", FIXED_DEADLINE),
    ] {
        let fixture = catalog.by_id(id).expect("pre-deadline fixture");
        assert_eq!(fixture.surface, Val004Surface::PendingTimeoutEvaluation);
        assert_eq!(fixture.clock.now_unix_seconds(), clock);
        assert_eq!(
            fixture.expected,
            ExpectedVal004Observation::Escalated {
                review_request_id: FIXED_REVIEW_REQUEST_ID,
                deadline: FIXED_DEADLINE,
                authorization_issued: false,
                effectful_invocations: 0,
            }
        );
    }

    let timed_out = catalog
        .by_id("timeout-deadline-plus-one")
        .expect("post-deadline fixture");
    assert_eq!(timed_out.clock.now_unix_seconds(), FIXED_DEADLINE + 1);
    assert_eq!(
        timed_out.expected,
        ExpectedVal004Observation::BlockedFromRegistry {
            attack_id: ATTACK_ID,
            authorization_issued: false,
            effectful_invocations: 0,
        }
    );
    assert_eq!(
        attack_by_id(ATTACK_ID).expect("ATK-06").expected,
        RequiredOutcome::EscalateThenDenyOnTimeout
    );
}

#[test]
fn two_surface_shape_never_re_presents_a_token_at_timeout() {
    let catalog = fixture_catalog();
    let case = attack_by_id(ATTACK_ID).expect("ATK-06");
    let escalation = catalog
        .by_id("escalation-required")
        .expect("escalation fixture");
    let timeout = catalog
        .by_id("timeout-deadline-plus-one")
        .expect("timeout fixture");

    let request = before_tool_call_request(case, escalation).expect("token surface request");
    assert_eq!(request.proposed_action.amount, ABOVE_THRESHOLD_AMOUNT);
    assert!(request.capability_token.is_some());
    assert!(before_tool_call_request(case, timeout).is_none());
    assert!(timeout.request.is_none());
    assert!(timeout.token.is_none());
}

#[test]
fn re_presentation_preserves_token_review_id_deadline_and_nonce_expectation() {
    let catalog = fixture_catalog();
    let original = catalog
        .by_id("escalation-required")
        .expect("original escalation");
    let repeated = catalog
        .by_id("re-presentation-does-not-extend")
        .expect("re-presentation");

    assert_eq!(repeated.token, original.token);
    assert_eq!(repeated.request, original.request);
    assert_eq!(repeated.pending, original.pending);
    assert!(repeated.clock.now_unix_seconds() > FIXED_REQUESTED_AT);
    assert!(repeated.clock.now_unix_seconds() <= FIXED_DEADLINE);
    assert_eq!(
        repeated.nonce_expectation,
        Some(NonceExpectation::RemainsUnconsumed)
    );
    assert_eq!(
        repeated.pending_record_expectation,
        Some(PendingRecordExpectation::AlreadyPending)
    );
    assert_eq!(repeated.expected, original.expected);
}

#[test]
fn below_threshold_control_is_the_unchanged_val_002_authorize_artifact() {
    let val_004 = fixture_catalog();
    let control = val_004
        .by_id("below-threshold-control")
        .expect("below-threshold control");
    let val_002 = val_002_catalog();
    let baseline = val_002.by_id("valid").expect("VAL-002 valid fixture");

    assert_eq!(BELOW_THRESHOLD_CONTROL_AMOUNT, "100000");
    assert!(
        BELOW_THRESHOLD_CONTROL_AMOUNT
            .parse::<u64>()
            .expect("canonical fixture amount")
            <= APPROVAL_REQUIRED_ABOVE_MINOR_UNITS
    );
    assert_eq!(control.request, baseline.request);
    assert_eq!(control.token, baseline.token);
    assert_eq!(
        control.expected,
        ExpectedVal004Observation::ProceedNormally {
            existing_outcome: ExpectedFixtureOutcome::Allow,
        }
    );
    assert_eq!(
        control.nonce_expectation,
        Some(NonceExpectation::ExistingConsumePath)
    );
}

#[test]
fn fixture_arithmetic_and_pending_founder_mirror_dependency_are_explicit() {
    assert_eq!(FIXED_DEADLINE, FIXED_REQUESTED_AT + 86_400);
    assert_eq!(APPROVAL_WINDOW_SECONDS, 86_400);
    assert_eq!(APPROVAL_REQUIRED_ABOVE_MINOR_UNITS, 1_000_000);
    assert_eq!(
        PENDING_FOUNDER_CONSTANT_MIRROR_ASSERTION.window_symbol,
        "CONFORMANCE_APPROVAL_WINDOW_SECONDS"
    );
    assert_eq!(
        PENDING_FOUNDER_CONSTANT_MIRROR_ASSERTION.fixture_window,
        APPROVAL_WINDOW_SECONDS
    );
    assert_eq!(
        PENDING_FOUNDER_CONSTANT_MIRROR_ASSERTION.threshold_symbol,
        "CONFORMANCE_APPROVAL_REQUIRED_ABOVE_MINOR_UNITS"
    );
    assert_eq!(
        PENDING_FOUNDER_CONSTANT_MIRROR_ASSERTION.fixture_threshold,
        APPROVAL_REQUIRED_ABOVE_MINOR_UNITS
    );
}
