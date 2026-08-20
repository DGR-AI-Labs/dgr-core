//! Deterministic VAL-004 approval-timeout fixture data.
//!
//! This module represents frozen scenario facts and expected labels only. It
//! does not implement an approval port/store, escalation decision, timeout
//! evaluation, observation variant, nonce consumption, or authorization path.

use crate::before_tool_call::{BeforeToolCallRequest, OpaqueCapabilityToken};
use crate::val_002_fixtures::{
    ExpectedFixtureOutcome, FixedFixtureClock, FixtureToken, PayInvoiceFixtureRequest,
    author_registered_fixture_token, baseline_request,
};
use crate::{AttackCase, ProposedAction};

pub const ATTACK_ID: &str = "ATK-06";
pub const FIXED_REQUESTED_AT: u64 = 1_800_000_000;
pub const APPROVAL_WINDOW_SECONDS: u64 = 86_400;
pub const APPROVAL_REQUIRED_ABOVE_MINOR_UNITS: u64 = 1_000_000;
pub const FIXED_DEADLINE: u64 = FIXED_REQUESTED_AT + APPROVAL_WINDOW_SECONDS;
pub const ABOVE_THRESHOLD_AMOUNT: &str = "1000001";
pub const BELOW_THRESHOLD_CONTROL_AMOUNT: &str = "100000";

/// Opaque deterministic fixture label. It does not freeze the production ID
/// representation or construction algorithm, both of which remain founder T0.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixtureReviewRequestId(pub &'static str);

pub const FIXED_REVIEW_REQUEST_ID: FixtureReviewRequestId =
    FixtureReviewRequestId("val-004-review-0001");

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Val004Surface {
    BeforeToolCall,
    PendingTimeoutEvaluation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NonceExpectation {
    RemainsUnconsumed,
    ExistingConsumePath,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PendingRecordExpectation {
    Recorded,
    AlreadyPending,
    Existing,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PendingApprovalFacts {
    pub review_request_id: FixtureReviewRequestId,
    pub requested_at: u64,
    pub deadline: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExpectedVal004Observation {
    Escalated {
        review_request_id: FixtureReviewRequestId,
        deadline: u64,
        authorization_issued: bool,
        effectful_invocations: u32,
    },
    /// The terminal outcome is intentionally resolved through `attack_id` by
    /// conformance tests rather than duplicated in fixture data.
    BlockedFromRegistry {
        attack_id: &'static str,
        authorization_issued: bool,
        effectful_invocations: u32,
    },
    ProceedNormally {
        existing_outcome: ExpectedFixtureOutcome,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Val004Fixture {
    pub id: &'static str,
    pub attack_id: &'static str,
    pub description: &'static str,
    pub surface: Val004Surface,
    pub clock: FixedFixtureClock,
    pub request: Option<PayInvoiceFixtureRequest>,
    pub token: Option<FixtureToken>,
    pub token_canonical_action_bytes: Option<Vec<u8>>,
    pub pending: Option<PendingApprovalFacts>,
    pub pending_record_expectation: Option<PendingRecordExpectation>,
    pub nonce_expectation: Option<NonceExpectation>,
    pub expected: ExpectedVal004Observation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Val004FixtureCatalog {
    pub fixtures: Vec<Val004Fixture>,
}

impl Val004FixtureCatalog {
    pub fn by_id(&self, id: &str) -> Option<&Val004Fixture> {
        self.fixtures.iter().find(|fixture| fixture.id == id)
    }
}

/// Records the founder-mirror assertion that becomes direct only after T0
/// creates the two `CONFORMANCE_*` constants. This marker prevents T3 from
/// inventing founder constants while keeping the dependency explicit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PendingFounderConstantMirrorAssertion {
    pub window_symbol: &'static str,
    pub fixture_window: u64,
    pub threshold_symbol: &'static str,
    pub fixture_threshold: u64,
}

pub const PENDING_FOUNDER_CONSTANT_MIRROR_ASSERTION: PendingFounderConstantMirrorAssertion =
    PendingFounderConstantMirrorAssertion {
        window_symbol: "CONFORMANCE_APPROVAL_WINDOW_SECONDS",
        fixture_window: APPROVAL_WINDOW_SECONDS,
        threshold_symbol: "CONFORMANCE_APPROVAL_REQUIRED_ABOVE_MINOR_UNITS",
        fixture_threshold: APPROVAL_REQUIRED_ABOVE_MINOR_UNITS,
    };

pub fn fixture_catalog() -> Val004FixtureCatalog {
    let pending = PendingApprovalFacts {
        review_request_id: FIXED_REVIEW_REQUEST_ID,
        requested_at: FIXED_REQUESTED_AT,
        deadline: FIXED_DEADLINE,
    };
    let above_threshold_request = PayInvoiceFixtureRequest {
        amount: ABOVE_THRESHOLD_AMOUNT,
        ..baseline_request()
    };
    let (above_threshold_token, above_threshold_canonical) = author_registered_fixture_token(
        &above_threshold_request,
        FIXED_REQUESTED_AT - 60,
        FIXED_REQUESTED_AT + 240,
        [0x41; 16],
    );
    let below_threshold_request = baseline_request();
    let (below_threshold_token, below_threshold_canonical) = author_registered_fixture_token(
        &below_threshold_request,
        FIXED_REQUESTED_AT - 60,
        FIXED_REQUESTED_AT + 240,
        [0x01; 16],
    );

    Val004FixtureCatalog {
        fixtures: vec![
            Val004Fixture {
                id: "escalation-required",
                attack_id: ATTACK_ID,
                description: "valid above-threshold token presented at before_tool_call",
                surface: Val004Surface::BeforeToolCall,
                clock: fixed_clock(FIXED_REQUESTED_AT),
                request: Some(above_threshold_request),
                token: Some(above_threshold_token.clone()),
                token_canonical_action_bytes: Some(above_threshold_canonical.clone()),
                pending: Some(pending),
                pending_record_expectation: Some(PendingRecordExpectation::Recorded),
                nonce_expectation: Some(NonceExpectation::RemainsUnconsumed),
                expected: escalated(pending),
            },
            timeout_fixture(
                "timeout-deadline-minus-one",
                "pending record evaluated one second before its deadline",
                FIXED_DEADLINE - 1,
                pending,
                escalated(pending),
            ),
            timeout_fixture(
                "timeout-at-deadline",
                "pending record evaluated exactly at its deadline",
                FIXED_DEADLINE,
                pending,
                escalated(pending),
            ),
            timeout_fixture(
                "timeout-deadline-plus-one",
                "pending record evaluated one second after its deadline",
                FIXED_DEADLINE + 1,
                pending,
                ExpectedVal004Observation::BlockedFromRegistry {
                    attack_id: ATTACK_ID,
                    authorization_issued: false,
                    effectful_invocations: 0,
                },
            ),
            Val004Fixture {
                id: "re-presentation-does-not-extend",
                attack_id: ATTACK_ID,
                description: "same token re-presented before deadline with original pending facts",
                surface: Val004Surface::BeforeToolCall,
                clock: fixed_clock(FIXED_REQUESTED_AT + 60),
                request: Some(above_threshold_request),
                token: Some(above_threshold_token),
                token_canonical_action_bytes: Some(above_threshold_canonical),
                pending: Some(pending),
                pending_record_expectation: Some(PendingRecordExpectation::AlreadyPending),
                nonce_expectation: Some(NonceExpectation::RemainsUnconsumed),
                expected: escalated(pending),
            },
            Val004Fixture {
                id: "below-threshold-control",
                attack_id: ATTACK_ID,
                description: "VAL-002 baseline amount follows the existing authorize path",
                surface: Val004Surface::BeforeToolCall,
                clock: fixed_clock(FIXED_REQUESTED_AT),
                request: Some(below_threshold_request),
                token: Some(below_threshold_token),
                token_canonical_action_bytes: Some(below_threshold_canonical),
                pending: None,
                pending_record_expectation: None,
                nonce_expectation: Some(NonceExpectation::ExistingConsumePath),
                expected: ExpectedVal004Observation::ProceedNormally {
                    existing_outcome: ExpectedFixtureOutcome::Allow,
                },
            },
        ],
    }
}

/// Wires only token-bearing fixture data to the existing adapter request type.
/// Timeout fixtures deliberately return `None`: R-3 does not re-present a token.
pub fn before_tool_call_request<'a>(
    case: &'static AttackCase,
    fixture: &'a Val004Fixture,
) -> Option<BeforeToolCallRequest<'a>> {
    if fixture.surface != Val004Surface::BeforeToolCall {
        return None;
    }

    let presented = fixture.request?;
    let token = fixture.token.as_ref()?;

    Some(BeforeToolCallRequest {
        proposed_action: ProposedAction {
            tool: case.proposed_action.tool,
            action: presented.action,
            amount: presented.amount,
            currency: presented.currency,
            destination: presented.destination,
            invoice_id: presented.invoice_id,
            source_account: presented.source_account,
        },
        context: &case.context,
        capability_token: Some(OpaqueCapabilityToken {
            bytes: &token.wire_bytes,
        }),
    })
}

const fn fixed_clock(now_unix_seconds: u64) -> FixedFixtureClock {
    FixedFixtureClock { now_unix_seconds }
}

const fn escalated(pending: PendingApprovalFacts) -> ExpectedVal004Observation {
    ExpectedVal004Observation::Escalated {
        review_request_id: pending.review_request_id,
        deadline: pending.deadline,
        authorization_issued: false,
        effectful_invocations: 0,
    }
}

fn timeout_fixture(
    id: &'static str,
    description: &'static str,
    now_unix_seconds: u64,
    pending: PendingApprovalFacts,
    expected: ExpectedVal004Observation,
) -> Val004Fixture {
    Val004Fixture {
        id,
        attack_id: ATTACK_ID,
        description,
        surface: Val004Surface::PendingTimeoutEvaluation,
        clock: fixed_clock(now_unix_seconds),
        request: None,
        token: None,
        token_canonical_action_bytes: None,
        pending: Some(pending),
        pending_record_expectation: Some(PendingRecordExpectation::Existing),
        nonce_expectation: None,
        expected,
    }
}
