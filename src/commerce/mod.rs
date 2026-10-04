//! Pure commerce data, canonical encoding and extension-free bundle preparation.
//!
//! These values do not authenticate a caller, evaluate policy or authorize a
//! provider operation. A digest commits bytes; it is not an execution capability.
//! The commerce API is intentionally separate from the legacy enforcement API.

mod decision;
mod reasons;
pub use decision::{
    PreparedCedarInputsV1, PreparedCommerceBundleV1, ProvenanceReviewErrorV1,
    build_cedar_request_v1, bundle_digest_v1, decision_digest_v1, encode_decision_v1,
    prepare_bundle_v1, request_binding_digest_v1, validate_provenance_and_reviews_v1,
    validate_request_bindings_v1, validate_review_artifacts_v1,
};
pub use reasons::ReasonV1;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u16)]
pub enum CommerceActionV1 {
    RefundCreate = 0,
    OrderAddressUpdate = 1,
    OrderCancel = 2,
    DiscountCreate = 3,
    CustomerEmailSend = 4,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u16)]
pub enum ProvenanceStateV1 {
    TrustedBoundUnused = 0,
    Missing = 1,
    Invalid = 2,
    Consumed = 3,
}

/// A pure policy outcome, never an executable capability.
///
/// No implicit conversion to or from the legacy enforcement outcome is defined.
/// ```compile_fail
/// use dgr_core::{commerce::CommerceOutcomeV1, RequiredOutcome};
/// let _: RequiredOutcome = CommerceOutcomeV1::Deny.into();
/// ```
/// ```compile_fail
/// use dgr_core::{commerce::CommerceOutcomeV1, RequiredOutcome};
/// let _: CommerceOutcomeV1 = RequiredOutcome::Deny.into();
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u16)]
pub enum CommerceOutcomeV1 {
    Allow = 0,
    Deny = 1,
    Escalate = 2,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u16)]
pub enum ReviewKindV1 {
    Grant = 0,
    Attestation = 1,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommerceRequestV1 {
    pub profile_id: String,
    pub shop_id: String,
    pub subject_id: String,
    pub operation_id: String,
    pub resource_id: String,
    pub action: CommerceActionV1,
    pub amount_minor: Option<i64>,
    pub currency: Option<String>,
    pub payload_digest: [u8; 32],
    pub payload_length: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecisionContextV1 {
    pub authenticated_subject: String,
    pub authenticated_profile: String,
    pub authenticated_shop: String,
    pub now_ms: u64,
    pub evidence: CommerceEvidenceV1,
    pub budget: CommerceBudgetV1,
    pub review: CommerceReviewV1,
    pub review_request: Option<ReviewRequestV1>,
    pub approved_evidence_revision: Option<String>,
    pub policy_digest: [u8; 32],
    pub registry_digest: [u8; 32],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommerceEvidenceV1 {
    pub source_revision: String,
    pub fetched_at_ms: u64,
    pub evidence_digest: [u8; 32],
    pub captured_minor: Option<i64>,
    pub prior_refunds_minor: Option<i64>,
    pub order_age_seconds: Option<u64>,
    pub line_items_eligible: Option<bool>,
    pub any_fulfillment: Option<bool>,
    pub cancellation_eligible: Option<bool>,
    pub discount_conflict: Option<bool>,
    pub derived_recipient_digest: Option<[u8; 32]>,
    pub approved_template_digest: Option<[u8; 32]>,
    pub recipient_count: Option<u32>,
    pub provenance: ProvenanceStateV1,
    pub provenance_binding_digest: Option<[u8; 32]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommerceBudgetV1 {
    pub profile_id: String,
    pub shop_id: String,
    pub subject_id: String,
    pub action: CommerceActionV1,
    pub currency: Option<String>,
    pub window_start_ms: u64,
    pub window_length_ms: u64,
    pub count_used: i64,
    pub value_used_minor: i64,
    pub revision: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommerceReviewV1 {
    pub grant: Option<ReviewFactV1>,
    pub attestation: Option<ReviewFactV1>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReviewFactV1 {
    pub id: String,
    pub reviewer_id: String,
    pub authorized: bool,
    pub issued_at_ms: u64,
    pub expires_at_ms: u64,
    pub binding_digest: [u8; 32],
    pub policy_digest: [u8; 32],
    pub consumed: bool,
    pub kind: ReviewKindV1,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReviewRequestV1 {
    pub id: String,
    pub created_at_ms: u64,
    pub expires_at_ms: u64,
    pub binding_digest: [u8; 32],
    pub policy_digest: [u8; 32],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EscalationRequirementV1 {
    pub requested_at_ms: u64,
    pub expires_at_ms: u64,
    pub review_request_id: Option<String>,
    pub required_kinds: Vec<ReviewKindV1>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommerceConstraintsV1 {
    pub profile_id: String,
    pub shop_id: String,
    pub subject_id: String,
    pub operation_id: String,
    pub resource_id: String,
    pub action: CommerceActionV1,
    pub payload_digest: [u8; 32],
    pub amount_ceiling_minor: Option<i64>,
    pub currency: Option<String>,
    pub evidence_revision: String,
    pub policy_digest: [u8; 32],
    pub registry_digest: [u8; 32],
    pub review_ids: Vec<String>,
    pub evaluation_time_ms: u64,
}

/// A complete pure decision record, not a legacy enforcement outcome.
/// ```compile_fail
/// use dgr_core::{commerce::CommerceDecisionV1, RequiredOutcome};
/// fn convert(value: CommerceDecisionV1) -> RequiredOutcome { value.into() }
/// ```
/// ```compile_fail
/// use dgr_core::{commerce::CommerceDecisionV1, RequiredOutcome};
/// fn convert(value: RequiredOutcome) -> CommerceDecisionV1 { value.into() }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommerceDecisionV1 {
    pub outcome: CommerceOutcomeV1,
    pub primary_reason: ReasonV1,
    pub diagnostics: Vec<ReasonV1>,
    pub constraints: Option<CommerceConstraintsV1>,
    pub escalation: Option<EscalationRequirementV1>,
    pub decision_digest: [u8; 32],
    pub policy_digest: [u8; 32],
    pub registry_digest: [u8; 32],
    pub evidence_digest: [u8; 32],
}

/// Errors are preparation failures, not commerce decisions. Declaration tags
/// are wire vocabulary; preparation precedence is the order of checks, not tags.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u16)]
pub enum BundleErrorV1 {
    MalformedSource = 0,
    SourceTooLarge = 1,
    StructureLimitExceeded = 2,
    SchemaInvalid = 3,
    PolicyInvalid = 4,
    SettingsInvalid = 5,
    DigestMismatch = 6,
    RegistryMismatch = 7,
    UnsupportedFeature = 8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u16)]
pub enum EvidenceFieldV1 {
    SourceRevision = 0,
    FetchedAtMs = 1,
    EvidenceDigest = 2,
    CapturedMinor = 3,
    PriorRefundsMinor = 4,
    OrderAgeSeconds = 5,
    LineItemsEligible = 6,
    AnyFulfillment = 7,
    CancellationEligible = 8,
    DiscountConflict = 9,
    DerivedRecipientDigest = 10,
    ApprovedTemplateDigest = 11,
    RecipientCount = 12,
    Provenance = 13,
    ProvenanceBindingDigest = 14,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u16)]
pub enum ReviewRouteV1 {
    OrderWindow = 0,
    DiscountConflict = 1,
}

/// Operator configuration. There are deliberately no live defaults.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommerceActionSettingsV1 {
    pub enabled: bool,
    pub currencies: Vec<String>,
    pub required_evidence: Vec<(EvidenceFieldV1, u64)>,
    pub amount_ceiling_minor: Option<i64>,
    pub count_ceiling: Option<i64>,
    pub value_ceiling_minor: Option<i64>,
    pub budget_window_ms: Option<u64>,
    pub order_age_limit_seconds: Option<u64>,
    pub require_provenance: bool,
    pub attestation_enabled: bool,
    pub require_monetary_review: bool,
    pub review_request_timeout_ms: Option<u64>,
    pub grant_max_lifetime_ms: Option<u64>,
    pub attestation_max_lifetime_ms: Option<u64>,
    pub reviewer_role_policy_id: Option<String>,
    pub review_routes: Vec<ReviewRouteV1>,
}

/// Untrusted bundle declarations. The expected registry identity is supplied
/// separately by trusted integration code; a bundle cannot select its trust root.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommerceBundleSourceV1 {
    pub schema_text: String,
    pub permissions_text: String,
    pub action_settings: Vec<(CommerceActionV1, CommerceActionSettingsV1)>,
    pub registry_digest: [u8; 32],
    pub declared_digest: [u8; 32],
}
