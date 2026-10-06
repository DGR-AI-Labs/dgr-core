use super::*;
use sha2::{Digest, Sha256};

type EncodingResult<T = ()> = Result<T, ReasonV1>;
const INVALID: ReasonV1 = ReasonV1::E_MALFORMED_REQUEST;

/// Encode a bounded typed frame. This checks representation, not policy truth.
///
/// Invalid representations return an error and no partial frame. Semantically
/// mismatched identities are retained so a future evaluator can commit a Deny
/// without rewriting its inputs. Sets are sorted on a copy; duplicates reject.
/// `decision.decision_digest` is deliberately excluded from its own preimage.
pub fn encode_decision_v1(
    request: &CommerceRequestV1,
    context: &DecisionContextV1,
    prepared_bundle_digest: &[u8; 32],
    decision: &CommerceDecisionV1,
) -> EncodingResult<Vec<u8>> {
    let mut e = Encoder(Vec::new());
    e.0.extend_from_slice(b"DGR-HERMES-DECISION-V1\0");
    let start = e.0.len();
    e.request(request)?;
    // Currently slack under the fixed fields and identifier bounds; retain the
    // request-section ceiling so later field growth cannot bypass it.
    if e.0.len() - start > 64 * 1024 {
        return Err(INVALID);
    }
    e.context(context)?;
    e.digest(prepared_bundle_digest)?;
    e.body(decision)?;
    Ok(e.0)
}

/// SHA-256 of the canonical frame; hashing does not authorize execution.
pub fn decision_digest_v1(
    request: &CommerceRequestV1,
    context: &DecisionContextV1,
    prepared_bundle_digest: &[u8; 32],
    decision: &CommerceDecisionV1,
) -> EncodingResult<[u8; 32]> {
    Ok(Sha256::digest(encode_decision_v1(
        request,
        context,
        prepared_bundle_digest,
        decision,
    )?)
    .into())
}

/// Canonical request identity used by provenance and review artifacts.
/// This is not the payload digest, a signature, or execution authority.
pub fn request_binding_digest_v1(request: &CommerceRequestV1) -> EncodingResult<[u8; 32]> {
    let mut e = Encoder(b"DGR-HERMES-REQUEST-V1\0".to_vec());
    let start = e.0.len();
    e.request(request)?;
    if e.0.len() - start > 64 * 1024 {
        return Err(INVALID);
    }
    Ok(Sha256::digest(e.0).into())
}

// One representation boundary for both the binding and combined validators.
// Return the checked request digest so the binding validator need not re-encode it.
fn validated_request_digest_v1(
    request: &CommerceRequestV1,
    context: &DecisionContextV1,
) -> EncodingResult<[u8; 32]> {
    let digest = request_binding_digest_v1(request)?;
    Encoder(Vec::new()).context(context)?;
    Ok(digest)
}

/// Check identity and every supplied request-bound artifact, including optional
/// unused reviews. Missing artifacts are left to the applicable policy checks.
///
/// Success means only that these bindings match. It does not validate provenance
/// state, reviewer authorization, kind, expiry, consumption, budget or policy.
/// Template and recipient digests have distinct domains and are not compared to
/// the request binding. No Cedar conversion or Authorizer call occurs here.
/// This primitive is not a decision evaluator and grants no permission.
pub fn validate_request_bindings_v1(
    request: &CommerceRequestV1,
    context: &DecisionContextV1,
    bundle: &PreparedCommerceBundleV1,
) -> EncodingResult {
    let expected = validated_request_digest_v1(request, context)?;
    if request.profile_id != context.authenticated_profile
        || request.shop_id != context.authenticated_shop
        || request.subject_id != context.authenticated_subject
        || context.policy_digest != bundle.digest
        || context.registry_digest != bundle.registry_digest
    {
        return Err(ReasonV1::E_BINDING_MISMATCH);
    }
    if context
        .evidence
        .provenance_binding_digest
        .is_some_and(|digest| digest != expected)
    {
        return Err(ReasonV1::E_BINDING_MISMATCH);
    }
    for fact in [&context.review.grant, &context.review.attestation]
        .into_iter()
        .flatten()
    {
        if fact.binding_digest != expected || fact.policy_digest != bundle.digest {
            return Err(ReasonV1::E_BINDING_MISMATCH);
        }
    }
    if context.review_request.as_ref().is_some_and(|pending| {
        pending.binding_digest != expected || pending.policy_digest != bundle.digest
    }) {
        return Err(ReasonV1::E_BINDING_MISMATCH);
    }
    Ok(())
}

/// Check the four hard action-specific eligibility predicates.
///
/// RefundCreate requires line_items_eligible=true; OrderAddressUpdate requires
/// any_fulfillment=false; OrderCancel requires cancellation_eligible=true; and
/// CustomerEmailSend requires recipient_count=1. Missing applicable facts return
/// E_MISSING_EVIDENCE; present disallowed values return E_CONSTRAINT_VIOLATION.
/// Each rule applies only to its named action. Review facts and policy routes
/// cannot override it. Representation validation precedes semantic checks.
///
/// These applicable facts are mandatory independently of required_evidence:
/// configured entries add requirements; omitting an entry cannot disable a hard
/// predicate. This primitive does not consult settings.enabled and classifies
/// eligibility even for a disabled action. The final evaluator must separately
/// enforce action enablement and compose the other applicable checks.
///
/// DiscountCreate has no predicate in this primitive; success there does not
/// establish discount eligibility. Success for any action is not permission:
/// bindings, freshness, provenance, reviews, monetary limits, order windows,
/// discount conflicts, recipient/template domains and Cedar remain separate.
/// No evidence acquisition, Cedar evaluation or provider effect occurs here.
pub fn validate_action_eligibility_v1(
    request: &CommerceRequestV1,
    context: &DecisionContextV1,
) -> EncodingResult {
    validated_request_digest_v1(request, context)?;
    let evidence = &context.evidence;
    let eligible = match request.action {
        CommerceActionV1::RefundCreate => evidence.line_items_eligible,
        CommerceActionV1::OrderAddressUpdate => evidence.any_fulfillment.map(|value| !value),
        CommerceActionV1::OrderCancel => evidence.cancellation_eligible,
        CommerceActionV1::CustomerEmailSend => evidence.recipient_count.map(|count| count == 1),
        CommerceActionV1::DiscountCreate => return Ok(()),
    };
    match eligible {
        None => Err(ReasonV1::E_MISSING_EVIDENCE),
        Some(false) => Err(ReasonV1::E_CONSTRAINT_VIOLATION),
        Some(true) => Ok(()),
    }
}

/// Check the two monetary classes, policy amount ceilings and refund balances.
///
/// RefundCreate and DiscountCreate require amount/currency independently of
/// review flags or required_evidence. Other actions have no monetary semantic
/// requirements here. Representation validation always runs first.
/// Well-formed nonmember currencies contribute E_CONSTRAINT_VIOLATION; missing
/// inputs contribute E_UNDERSPECIFIED_ACTION. An absent amount ceiling or empty
/// permitted set contributes E_POLICY_UNCONFIGURED, never a live default.
///
/// RefundCreate also requires captured/prior-refund facts. Their difference is
/// checked before comparison; prior refunds above capture contribute
/// E_EVIDENCE_CONFLICT, not a usable zero balance. Policy and valid remaining
/// balances are inclusive ceilings; exceeding either contributes E_AMOUNT_LIMIT.
/// Independently established causes are deduplicated in registry order.
///
/// Preparation guarantees a present amount ceiling and non-empty currency set
/// for enabled monetary entries. The E_POLICY_UNCONFIGURED branches here are
/// defence-in-depth classifications reachable through prepared disabled entries,
/// not live unconfigured-policy paths. Their absence tests use disabled settings.
///
/// Configuration and predicates have separate roles: required_evidence adds
/// snapshot presence/age checks but does not define every hard requirement.
/// A structural Provenance field can still be Missing; require_provenance and
/// final provenance resolution remain separate. Applicable hard eligibility and
/// monetary inputs are mandatory even when omitted from required_evidence.
/// These classifiers do not enforce enabled; the final evaluator must do so.
///
/// This classifies supplied facts even for disabled settings; action enablement
/// remains an evaluator obligation. Success is not permission and does not check
/// bindings, freshness, provenance, reviews or budget scope/window/count/value.
/// It does not invoke Cedar, authenticate evidence, acquire facts or cause effects.
pub fn validate_monetary_constraints_v1(
    request: &CommerceRequestV1,
    context: &DecisionContextV1,
    bundle: &PreparedCommerceBundleV1,
) -> Result<(), Vec<ReasonV1>> {
    if let Err(reason) = validated_request_digest_v1(request, context) {
        return Err(vec![reason]);
    }
    if !matches!(
        request.action,
        CommerceActionV1::RefundCreate | CommerceActionV1::DiscountCreate
    ) {
        return Ok(());
    }
    let Some((_, settings)) = bundle
        .settings
        .iter()
        .find(|(action, _)| *action == request.action)
    else {
        return Err(vec![ReasonV1::E_INTERNAL_EVALUATION]);
    };
    let amount = request.amount_minor;
    let currency = request.currency.as_deref();
    let mut causes = Vec::new();
    if amount.is_none() || currency.is_none() {
        causes.push(ReasonV1::E_UNDERSPECIFIED_ACTION);
    }
    match settings.amount_ceiling_minor {
        None => causes.push(ReasonV1::E_POLICY_UNCONFIGURED),
        Some(ceiling) => {
            if amount.is_some_and(|amount| amount > ceiling) {
                causes.push(ReasonV1::E_AMOUNT_LIMIT);
            }
        }
    }
    if settings.currencies.is_empty() {
        causes.push(ReasonV1::E_POLICY_UNCONFIGURED);
    } else if currency.is_some_and(|currency| !settings.currencies.iter().any(|c| c == currency)) {
        causes.push(ReasonV1::E_CONSTRAINT_VIOLATION);
    }
    if request.action == CommerceActionV1::RefundCreate {
        match (
            context.evidence.captured_minor,
            context.evidence.prior_refunds_minor,
        ) {
            (Some(captured), Some(prior)) => {
                match captured.checked_sub(prior).filter(|net| *net >= 0) {
                    None => causes.push(ReasonV1::E_EVIDENCE_CONFLICT),
                    Some(net) => {
                        if amount.is_some_and(|amount| amount > net) {
                            causes.push(ReasonV1::E_AMOUNT_LIMIT);
                        }
                    }
                }
            }
            _ => causes.push(ReasonV1::E_MISSING_EVIDENCE),
        }
    }
    causes.sort_by_key(|reason| reason.rank());
    causes.dedup();
    if causes.is_empty() {
        Ok(())
    } else {
        Err(causes)
    }
}

/// Classify prospective count and monetary value against the supplied budget.
///
/// This pure classifier does not reserve or reset budget, consume an artifact,
/// invoke Cedar or authorize an effect. Count increments by one for every action;
/// value increments by the requested amount for RefundCreate/DiscountCreate only.
/// Exact ceilings pass, while checked-add overflow contributes the corresponding
/// E_COUNT_LIMIT or E_VALUE_LIMIT. Required missing limits fail closed.
///
/// Profile/shop/subject/action, monetary currency and configured window length
/// must match. Nonmonetary budget currency must be absent. Missing monetary request
/// currency is underspecified; membership remains a separate monetary check.
/// Scope/length mismatches contribute E_BINDING_MISMATCH. Windows are [start,end):
/// a future start or unrepresentable end is E_MALFORMED_REQUEST; a well-formed
/// expired window is E_STALE_STATE. End must fit the signed-64-bit time domain.
/// A supplied zero length cannot match a configured positive window. No window
/// rollover or reset occurs here. Established causes are registry-sorted/deduped.
///
/// Preparation guarantees count/window limits for enabled actions and value
/// limits for enabled monetary actions. Unconfigured controls therefore use
/// disabled entries; this classifier does not enforce enabled. Authentication,
/// bindings of other facts, budget revision freshness, durable reservations,
/// evidence/review validity, hard predicates and Cedar remain separate. Budget
/// revision is representation-checked, not reconciled with a durable store here.
/// Success classifies the supplied snapshot only and is never permission.
pub fn validate_budget_constraints_v1(
    request: &CommerceRequestV1,
    context: &DecisionContextV1,
    bundle: &PreparedCommerceBundleV1,
) -> Result<(), Vec<ReasonV1>> {
    if let Err(reason) = validated_request_digest_v1(request, context) {
        return Err(vec![reason]);
    }
    let Some((_, settings)) = bundle
        .settings
        .iter()
        .find(|(action, _)| *action == request.action)
    else {
        return Err(vec![ReasonV1::E_INTERNAL_EVALUATION]);
    };
    let budget = &context.budget;
    let monetary = matches!(
        request.action,
        CommerceActionV1::RefundCreate | CommerceActionV1::DiscountCreate
    );
    let mut causes = Vec::new();
    if budget.profile_id != request.profile_id
        || budget.shop_id != request.shop_id
        || budget.subject_id != request.subject_id
        || budget.action != request.action
        || if monetary {
            request
                .currency
                .as_ref()
                .is_some_and(|currency| budget.currency.as_ref() != Some(currency))
        } else {
            budget.currency.is_some()
        }
    {
        causes.push(ReasonV1::E_BINDING_MISMATCH);
    }
    match settings.budget_window_ms.filter(|window| *window > 0) {
        None => causes.push(ReasonV1::E_POLICY_UNCONFIGURED),
        Some(window) => {
            if budget.window_length_ms != window {
                causes.push(ReasonV1::E_BINDING_MISMATCH);
            }
        }
    }
    // The computed end must fit the same signed-64-bit time domain as the inputs.
    // Compute once, before comparing; never reset a stale or mismatched snapshot.
    let end = budget
        .window_start_ms
        .checked_add(budget.window_length_ms)
        .filter(|end| *end <= 9_223_372_036_854_775_807);
    if budget.window_start_ms > context.now_ms || end.is_none() {
        causes.push(INVALID);
    } else if budget.window_length_ms > 0 && end.is_some_and(|end| context.now_ms >= end) {
        causes.push(ReasonV1::E_STALE_STATE);
    }
    match settings.count_ceiling {
        None => causes.push(ReasonV1::E_POLICY_UNCONFIGURED),
        Some(ceiling) => {
            if budget
                .count_used
                .checked_add(1)
                .is_none_or(|next| next > ceiling)
            {
                causes.push(ReasonV1::E_COUNT_LIMIT);
            }
        }
    }
    if monetary {
        if request.amount_minor.is_none() || request.currency.is_none() {
            causes.push(ReasonV1::E_UNDERSPECIFIED_ACTION);
        }
        match settings.value_ceiling_minor {
            None => causes.push(ReasonV1::E_POLICY_UNCONFIGURED),
            Some(ceiling) => {
                if let Some(amount) = request.amount_minor
                    && budget
                        .value_used_minor
                        .checked_add(amount)
                        .is_none_or(|next| next > ceiling)
                {
                    causes.push(ReasonV1::E_VALUE_LIMIT);
                }
            }
        }
    }
    causes.sort_by_key(|reason| reason.rank());
    causes.dedup();
    if causes.is_empty() {
        Ok(())
    } else {
        Err(causes)
    }
}

/// Classify configured order-age windows and action-specific discount conflicts.
///
/// A present age limit applies to its selected action, including non-refund
/// actions. Equality passes; a greater age contributes E_ORDER_WINDOW. A
/// configured limit requires the age fact independently of required_evidence.
/// RefundCreate additionally requires an explicit limit; its absence contributes
/// E_POLICY_UNCONFIGURED. A zero limit is valid and accepts only age zero.
/// Missing applicable age contributes E_MISSING_EVIDENCE, never an age default.
///
/// DiscountCreate requires discount_conflict independently of the configured
/// evidence list: absent contributes E_MISSING_EVIDENCE, true contributes
/// E_DISCOUNT_CONFLICT, false passes. Other actions ignore this fact semantically.
/// Representation validation precedes all semantic checks. Established causes
/// are deduplicated in registry order, including independent simultaneous faults.
///
/// Enabled refunds already have an age limit guaranteed by bundle preparation;
/// missing-limit controls use prepared disabled entries as defence in depth.
/// This classifier does not enforce enabled. Configured review routes and
/// supplied reviews never erase a cause or turn this result into permission.
/// The final evaluator must default to Deny and may route only explicitly enabled
/// review predicates after all hard checks and Cedar permissions pass. Neither
/// an Escalate outcome nor review acceptance is decided by this primitive.
///
/// Success is not permission. Bindings, freshness, provenance, review validity,
/// monetary/budget and other hard predicates remain separate. No trusted clock,
/// evidence acquisition, Cedar evaluation, artifact consumption or effects occur.
pub fn validate_order_window_and_discount_v1(
    request: &CommerceRequestV1,
    context: &DecisionContextV1,
    bundle: &PreparedCommerceBundleV1,
) -> Result<(), Vec<ReasonV1>> {
    if let Err(reason) = validated_request_digest_v1(request, context) {
        return Err(vec![reason]);
    }
    let Some((_, settings)) = bundle
        .settings
        .iter()
        .find(|(action, _)| *action == request.action)
    else {
        return Err(vec![ReasonV1::E_INTERNAL_EVALUATION]);
    };
    let mut causes = Vec::new();
    let age = context.evidence.order_age_seconds;
    let limit = settings.order_age_limit_seconds;
    let refund = request.action == CommerceActionV1::RefundCreate;
    if (limit.is_some() || refund) && age.is_none() {
        causes.push(ReasonV1::E_MISSING_EVIDENCE);
    }
    match limit {
        None if refund => causes.push(ReasonV1::E_POLICY_UNCONFIGURED),
        Some(limit) => {
            if age.is_some_and(|age| age > limit) {
                causes.push(ReasonV1::E_ORDER_WINDOW);
            }
        }
        None => (),
    }
    if request.action == CommerceActionV1::DiscountCreate {
        match context.evidence.discount_conflict {
            None => causes.push(ReasonV1::E_MISSING_EVIDENCE),
            Some(true) => causes.push(ReasonV1::E_DISCOUNT_CONFLICT),
            Some(false) => (),
        }
    }
    causes.sort_by_key(|reason| reason.rank());
    causes.dedup();
    if causes.is_empty() {
        Ok(())
    } else {
        Err(causes)
    }
}

/// Check configured evidence presence/age and the supplied approved revision.
///
/// Future acquisition timestamps are malformed, even with no configured fields.
/// Age is inclusive at each configured maximum. False and zero are present facts.
/// All fields share one acquisition timestamp; this does not support mixed snapshots.
/// Representation errors short-circuit before time arithmetic. Other causes are
/// deduplicated in registry order. No Cedar evaluation or I/O occurs here.
///
/// For the non-optional SourceRevision, FetchedAtMs, EvidenceDigest and Provenance
/// fields, a required_evidence entry constrains snapshot age only. In particular,
/// requiring Provenance here accepts a structurally present Missing state; it does
/// not require usable provenance. The separate require_provenance setting expresses
/// that policy requirement. Resolving required-but-missing provenance belongs to
/// the future requirement/evaluator layer; validate_provenance_and_reviews_v1 does
/// not complete that resolution either.
///
/// Success does not authenticate evidence, recompute its digest, check request
/// bindings, validate provenance/reviews, enforce action-specific mandatory facts,
/// or grant permission. Those remain separate evaluator obligations.
pub fn validate_evidence_snapshot_v1(
    request: &CommerceRequestV1,
    context: &DecisionContextV1,
    bundle: &PreparedCommerceBundleV1,
) -> Result<(), Vec<ReasonV1>> {
    if let Err(reason) = validated_request_digest_v1(request, context) {
        return Err(vec![reason]);
    }
    let Some((_, settings)) = bundle
        .settings
        .iter()
        .find(|(action, _)| *action == request.action)
    else {
        return Err(vec![ReasonV1::E_INTERNAL_EVALUATION]);
    };
    let evidence = &context.evidence;
    let mut causes = Vec::new();
    // Check the future boundary before subtracting unsigned timestamps.
    let age = context.now_ms.checked_sub(evidence.fetched_at_ms);
    if age.is_none() {
        causes.push(INVALID);
    }
    for (field, maximum_age) in &settings.required_evidence {
        let present = match field {
            EvidenceFieldV1::SourceRevision
            | EvidenceFieldV1::FetchedAtMs
            | EvidenceFieldV1::EvidenceDigest
            | EvidenceFieldV1::Provenance => true,
            EvidenceFieldV1::CapturedMinor => evidence.captured_minor.is_some(),
            EvidenceFieldV1::PriorRefundsMinor => evidence.prior_refunds_minor.is_some(),
            EvidenceFieldV1::OrderAgeSeconds => evidence.order_age_seconds.is_some(),
            EvidenceFieldV1::LineItemsEligible => evidence.line_items_eligible.is_some(),
            EvidenceFieldV1::AnyFulfillment => evidence.any_fulfillment.is_some(),
            EvidenceFieldV1::CancellationEligible => evidence.cancellation_eligible.is_some(),
            EvidenceFieldV1::DiscountConflict => evidence.discount_conflict.is_some(),
            EvidenceFieldV1::DerivedRecipientDigest => evidence.derived_recipient_digest.is_some(),
            EvidenceFieldV1::ApprovedTemplateDigest => evidence.approved_template_digest.is_some(),
            EvidenceFieldV1::RecipientCount => evidence.recipient_count.is_some(),
            EvidenceFieldV1::ProvenanceBindingDigest => {
                evidence.provenance_binding_digest.is_some()
            }
        };
        if !present || age.is_some_and(|age| age > *maximum_age) {
            causes.push(ReasonV1::E_MISSING_EVIDENCE);
        }
    }
    if context
        .approved_evidence_revision
        .as_ref()
        .is_some_and(|revision| revision != &evidence.source_revision)
    {
        causes.push(ReasonV1::E_STALE_STATE);
    }
    causes.sort_by_key(|reason| reason.rank());
    causes.dedup();
    if causes.is_empty() {
        Ok(())
    } else {
        Err(causes)
    }
}

/// Validate supplied review-state defects after checking request/artifact bindings.
///
/// Returns distinct reasons in registry order, not a decision or permission.
/// Every supplied fact is checked even when unused by the configured policy.
/// Missing facts, provenance state, review-ID uniqueness/use, commerce predicates
/// and provider evidence remain separate obligations. This does not invoke Cedar.
/// Unrepresentable inputs short-circuit; otherwise independently established
/// defects are retained. An empty result never means the action is authorized.
pub fn validate_review_artifacts_v1(
    request: &CommerceRequestV1,
    context: &DecisionContextV1,
    bundle: &PreparedCommerceBundleV1,
) -> Result<(), Vec<ReasonV1>> {
    let mut causes = Vec::new();
    if let Err(reason) = validate_request_bindings_v1(request, context, bundle) {
        // Representation bounds must hold before further semantic inspection.
        if reason == INVALID {
            return Err(vec![reason]);
        }
        causes.push(reason);
    }
    let Some((_, settings)) = bundle
        .settings
        .iter()
        .find(|(action, _)| *action == request.action)
    else {
        return Err(vec![ReasonV1::E_INTERNAL_EVALUATION]);
    };
    for (fact, kind, limit) in [
        (
            &context.review.grant,
            ReviewKindV1::Grant,
            settings.grant_max_lifetime_ms,
        ),
        (
            &context.review.attestation,
            ReviewKindV1::Attestation,
            settings.attestation_max_lifetime_ms,
        ),
    ] {
        if let Some(fact) = fact {
            if fact.kind != kind {
                causes.push(INVALID);
            }
            if !fact.authorized {
                causes.push(ReasonV1::E_APPROVAL_INVALID);
            }
            if fact.consumed {
                causes.push(ReasonV1::E_REPLAY);
            }
            review_interval_causes(
                fact.issued_at_ms,
                fact.expires_at_ms,
                context.now_ms,
                limit,
                &mut causes,
            );
        }
    }
    if let Some(pending) = &context.review_request {
        review_interval_causes(
            pending.created_at_ms,
            pending.expires_at_ms,
            context.now_ms,
            settings.review_request_timeout_ms,
            &mut causes,
        );
    }
    // Separation applies when both review requirements apply, not merely when
    // two optional artifacts happen to be supplied. Their other defects persist.
    let requires_attestation = settings.require_provenance
        && settings.attestation_enabled
        && context.evidence.provenance == ProvenanceStateV1::Missing;
    if settings.require_monetary_review
        && requires_attestation
        && let (Some(grant), Some(attestation)) =
            (&context.review.grant, &context.review.attestation)
        && grant.reviewer_id == attestation.reviewer_id
    {
        causes.push(ReasonV1::E_REVIEWER_SEPARATION);
    }
    causes.sort_unstable_by_key(|reason| reason.rank());
    causes.dedup();
    if causes.is_empty() {
        Ok(())
    } else {
        Err(causes)
    }
}

fn review_interval_causes(
    created: u64,
    expires: u64,
    now: u64,
    max_lifetime: Option<u64>,
    causes: &mut Vec<ReasonV1>,
) {
    let configured = max_lifetime.filter(|limit| *limit > 0);
    if configured.is_none() {
        causes.push(ReasonV1::E_POLICY_UNCONFIGURED);
    }
    if created > now || expires <= created {
        causes.push(INVALID);
        return; // Do not call malformed intervals expired or subtract unsigned times.
    }
    if now >= expires {
        causes.push(ReasonV1::E_APPROVAL_EXPIRED);
    }
    if configured.is_some_and(|limit| expires - created > limit) {
        causes.push(ReasonV1::E_APPROVAL_INVALID);
    }
}

/// Failed provenance/review validation, retaining the supplied provenance state.
///
/// This is an intermediate validation error, not a serialized decision. The state
/// is retained even for malformed representations; it is not authenticated here.
/// Consumers must honor `requires_terminal_deny()` before considering any review
/// route permitted by an individual reason. A false result grants no permission:
/// other causes and the remaining policy checks may still require Deny.
///
/// Safe external callers cannot construct or mutate this value directly:
///
/// ```compile_fail
/// use dgr_core::commerce::{ProvenanceReviewErrorV1, ProvenanceStateV1};
/// let _ = ProvenanceReviewErrorV1 {
///     provenance: ProvenanceStateV1::Missing,
///     causes: vec![],
/// };
/// ```
///
/// ```compile_fail
/// use dgr_core::commerce::{ProvenanceReviewErrorV1, ProvenanceStateV1};
/// fn replace_state(mut error: ProvenanceReviewErrorV1) {
///     error.provenance = ProvenanceStateV1::Missing;
/// }
/// ```
///
/// ```compile_fail
/// use dgr_core::commerce::ProvenanceReviewErrorV1;
/// fn replace_causes(mut error: ProvenanceReviewErrorV1) {
///     error.causes.clear();
/// }
/// ```
///
/// Access to causes is read-only, including through a mutable error reference:
///
/// ```compile_fail
/// use dgr_core::commerce::{ProvenanceReviewErrorV1, ReasonV1};
/// fn edit_causes(error: &mut ProvenanceReviewErrorV1) {
///     error.causes()[0] = ReasonV1::E_INTERNAL_EVALUATION;
/// }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvenanceReviewErrorV1 {
    provenance: ProvenanceStateV1,
    causes: Vec<ReasonV1>,
}

impl ProvenanceReviewErrorV1 {
    /// The supplied state retained by validation; this is not authentication.
    pub fn provenance(&self) -> ProvenanceStateV1 {
        self.provenance
    }

    /// Read-only access to the distinct causes in registry order.
    pub fn causes(&self) -> &[ReasonV1] {
        &self.causes
    }

    /// Invalid or consumed provenance cannot be routed to review or repaired.
    /// This requirement is independent of reason order and registry allowances.
    pub fn requires_terminal_deny(&self) -> bool {
        matches!(
            self.provenance,
            ProvenanceStateV1::Invalid | ProvenanceStateV1::Consumed
        )
    }
}

/// Combine terminal provenance defects with supplied-review validity checks.
///
/// Success classifies the trusted input as `TrustedBoundUnused` or `Missing`;
/// it never authenticates that input, accepts an attestation, grants permission,
/// or decides whether missing provenance can be remedied. Requirement resolution
/// and the final evaluator remain separate. Invalid and consumed provenance are
/// terminal even when provenance is optional; an attestation cannot repair them.
/// Non-missing provenance requires a matching immutable request binding.
/// Independent review defects are retained in registry order. This is the
/// combined entry point for provenance/review diagnostics, not an Authorizer.
pub fn validate_provenance_and_reviews_v1(
    request: &CommerceRequestV1,
    context: &DecisionContextV1,
    bundle: &PreparedCommerceBundleV1,
) -> Result<ProvenanceStateV1, ProvenanceReviewErrorV1> {
    // Distinguish unrepresentable inputs from a representable kind/time defect:
    // only the former prevents further semantic inspection.
    if let Err(reason) = validated_request_digest_v1(request, context) {
        return Err(ProvenanceReviewErrorV1 {
            provenance: context.evidence.provenance,
            causes: vec![reason],
        });
    }
    let mut causes = validate_review_artifacts_v1(request, context, bundle)
        .err()
        .unwrap_or_default();
    let state = context.evidence.provenance;
    if state != ProvenanceStateV1::Missing && context.evidence.provenance_binding_digest.is_none() {
        causes.push(ReasonV1::E_MISSING_EVIDENCE);
    }
    match state {
        ProvenanceStateV1::TrustedBoundUnused | ProvenanceStateV1::Missing => {}
        ProvenanceStateV1::Invalid => causes.push(ReasonV1::E_PROVENANCE_UNVERIFIABLE),
        ProvenanceStateV1::Consumed => causes.push(ReasonV1::E_REPLAY),
    }
    causes.sort_unstable_by_key(|reason| reason.rank());
    causes.dedup();
    if causes.is_empty() {
        Ok(state)
    } else {
        Err(ProvenanceReviewErrorV1 {
            provenance: state,
            causes,
        })
    }
}

/// Validated review usage and still-missing kinds; this is never permission.
///
/// IDs include only facts used for configured monetary approval or required
/// missing-provenance attestation. Valid optional facts are omitted, not ignored:
/// every supplied artifact is validated before this value can be constructed.
/// Missing kinds are potential remediable requirements, not an Escalate verdict.
/// Other hard predicates, enabled configuration, Cedar permission, review-route
/// policy and immutable deadline construction must still pass in the evaluator.
/// No provenance state is upgraded, no artifact is consumed and no audit is stored.
///
/// Safe external callers cannot manufacture usage or erase missing requirements:
/// ```compile_fail
/// use dgr_core::commerce::ResolvedCommerceReviewsV1;
/// let _ = ResolvedCommerceReviewsV1 { review_ids: vec![], required_kinds: vec![] };
/// ```
/// ```compile_fail
/// use dgr_core::commerce::ResolvedCommerceReviewsV1;
/// fn erase(value: &mut ResolvedCommerceReviewsV1) { value.required_kinds.clear(); }
/// ```
/// ```compile_fail
/// use dgr_core::commerce::ResolvedCommerceReviewsV1;
/// fn fabricate(value: &mut ResolvedCommerceReviewsV1) { value.review_ids.push("fake".into()); }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedCommerceReviewsV1 {
    review_ids: Vec<String>,
    required_kinds: Vec<ReviewKindV1>,
}

impl ResolvedCommerceReviewsV1 {
    /// Raw-UTF-8-sorted IDs actually used, excluding valid optional facts.
    pub fn review_ids(&self) -> &[String] {
        &self.review_ids
    }

    /// Sorted missing kinds; an empty slice does not mean authorization.
    pub fn required_kinds(&self) -> &[ReviewKindV1] {
        &self.required_kinds
    }
}

/// Resolve configured review requirements after validating all supplied artifacts.
///
/// The explicit require_monetary_review flag requires a Grant on any selected
/// action, not only on monetary classes. Required Missing provenance needs an
/// Attestation only when attestation_enabled; otherwise it contributes
/// E_PROVENANCE_UNVERIFIABLE. Trusted provenance needs no attestation. Invalid or
/// Consumed provenance remains terminal even if optional or a valid fact exists.
/// An attestation does not replace a separately required grant or vice versa.
/// Conflicting supplied grant/attestation IDs are E_MALFORMED_REQUEST, even when
/// optional. They are never silently repaired by sorting or deduplication.
///
/// All supplied facts, including unused ones and any pending request, must pass
/// bindings, kind, authorization, consumption, lifetime, expiry and applicable
/// reviewer-separation checks. A defective fact returns actual causes, not a
/// manufactured missing-kind requirement. Representation failure short-circuits;
/// other independently established causes are sorted and deduplicated.
///
/// Ok reports used IDs and missing kinds only. Err is not review-remediable through
/// this result, even when requires_terminal_deny() is false: that method identifies
/// terminal provenance specifically, not all terminal defects or route eligibility.
/// Review-route resolution, timeout configuration for new requests, enabled refusal,
/// hard predicates and Cedar remain separate. Success never constructs an Allow,
/// an Escalate, or execution authority. No trusted ingress, I/O or effects occur.
pub fn resolve_review_requirements_v1(
    request: &CommerceRequestV1,
    context: &DecisionContextV1,
    bundle: &PreparedCommerceBundleV1,
) -> Result<ResolvedCommerceReviewsV1, ProvenanceReviewErrorV1> {
    if let Err(reason) = validated_request_digest_v1(request, context) {
        return Err(ProvenanceReviewErrorV1 {
            provenance: context.evidence.provenance,
            causes: vec![reason],
        });
    }
    let mut causes = validate_provenance_and_reviews_v1(request, context, bundle)
        .err()
        .map(|error| error.causes)
        .unwrap_or_default();
    if let (Some(grant), Some(attestation)) = (&context.review.grant, &context.review.attestation)
        && grant.id == attestation.id
    {
        causes.push(INVALID);
    }
    let Some((_, settings)) = bundle
        .settings
        .iter()
        .find(|(action, _)| *action == request.action)
    else {
        return Err(ProvenanceReviewErrorV1 {
            provenance: context.evidence.provenance,
            causes: vec![ReasonV1::E_INTERNAL_EVALUATION],
        });
    };
    let missing_provenance =
        settings.require_provenance && context.evidence.provenance == ProvenanceStateV1::Missing;
    if missing_provenance && !settings.attestation_enabled {
        causes.push(ReasonV1::E_PROVENANCE_UNVERIFIABLE);
    }
    causes.sort_unstable_by_key(|reason| reason.rank());
    causes.dedup();
    if !causes.is_empty() {
        return Err(ProvenanceReviewErrorV1 {
            provenance: context.evidence.provenance,
            causes,
        });
    }
    let mut review_ids = Vec::new();
    let mut required_kinds = Vec::new();
    for (required, kind, fact) in [
        (
            settings.require_monetary_review,
            ReviewKindV1::Grant,
            &context.review.grant,
        ),
        (
            missing_provenance && settings.attestation_enabled,
            ReviewKindV1::Attestation,
            &context.review.attestation,
        ),
    ] {
        if required {
            match fact {
                Some(fact) => review_ids.push(fact.id.clone()),
                None => required_kinds.push(kind),
            }
        }
    }
    review_ids.sort();
    required_kinds.sort();
    Ok(ResolvedCommerceReviewsV1 {
        review_ids,
        required_kinds,
    })
}

struct Encoder(Vec<u8>);
impl Encoder {
    fn string(&mut self, s: &str) -> EncodingResult {
        if s.is_empty() || s.len() > 128 || s.chars().any(char::is_control) {
            return Err(INVALID);
        }
        self.0.extend_from_slice(&(s.len() as u32).to_be_bytes());
        self.0.extend_from_slice(s.as_bytes());
        Ok(())
    }
    fn currency(&mut self, value: &str) -> EncodingResult {
        if value.len() != 3 || !value.bytes().all(|byte| byte.is_ascii_uppercase()) {
            return Err(INVALID);
        }
        // Membership in the configured currency set belongs to policy evaluation.
        self.string(value)
    }
    fn number(&mut self, value: u64) -> EncodingResult {
        self.0.extend_from_slice(&value.to_be_bytes());
        Ok(())
    }
    fn time(&mut self, value: u64) -> EncodingResult {
        if value > i64::MAX as u64 {
            return Err(INVALID);
        }
        self.number(value)
    }
    fn money(&mut self, value: &i64) -> EncodingResult {
        self.number(u64::try_from(*value).map_err(|_| INVALID)?)
    }
    fn tag(&mut self, value: u16) {
        self.0.extend_from_slice(&value.to_be_bytes());
    }
    fn digest(&mut self, value: &[u8; 32]) -> EncodingResult {
        self.0.extend_from_slice(value);
        Ok(())
    }
    fn boolean(&mut self, value: &bool) -> EncodingResult {
        self.0.push(u8::from(*value));
        Ok(())
    }
    fn option<T>(
        &mut self,
        value: &Option<T>,
        put: impl FnOnce(&mut Self, &T) -> EncodingResult,
    ) -> EncodingResult {
        match value {
            None => self.0.push(0),
            Some(value) => {
                self.0.push(1);
                put(self, value)?;
            }
        }
        Ok(())
    }
    fn count(&mut self, value: usize) -> EncodingResult {
        if value > 64 {
            return Err(INVALID);
        }
        self.0.extend_from_slice(&(value as u32).to_be_bytes());
        Ok(())
    }
    fn request(&mut self, r: &CommerceRequestV1) -> EncodingResult {
        for s in [
            &r.profile_id,
            &r.shop_id,
            &r.subject_id,
            &r.operation_id,
            &r.resource_id,
        ] {
            self.string(s)?;
        }
        self.tag(r.action as u16);
        self.option(&r.amount_minor, Self::money)?;
        self.option(&r.currency, |e, s| e.currency(s))?;
        self.digest(&r.payload_digest)?;
        self.number(u64::from(r.payload_length))
    }
    fn evidence(&mut self, v: &CommerceEvidenceV1) -> EncodingResult {
        self.string(&v.source_revision)?;
        self.time(v.fetched_at_ms)?;
        self.digest(&v.evidence_digest)?;
        self.option(&v.captured_minor, Self::money)?;
        self.option(&v.prior_refunds_minor, Self::money)?;
        self.option(&v.order_age_seconds, |e, n| e.number(*n))?;
        self.option(&v.line_items_eligible, Self::boolean)?;
        self.option(&v.any_fulfillment, Self::boolean)?;
        self.option(&v.cancellation_eligible, Self::boolean)?;
        self.option(&v.discount_conflict, Self::boolean)?;
        self.option(&v.derived_recipient_digest, Self::digest)?;
        self.option(&v.approved_template_digest, Self::digest)?;
        self.option(&v.recipient_count, |e, n| e.number(u64::from(*n)))?;
        self.tag(v.provenance as u16);
        self.option(&v.provenance_binding_digest, Self::digest)
    }
    fn budget(&mut self, v: &CommerceBudgetV1) -> EncodingResult {
        for s in [&v.profile_id, &v.shop_id, &v.subject_id] {
            self.string(s)?;
        }
        self.tag(v.action as u16);
        self.option(&v.currency, |e, s| e.currency(s))?;
        self.time(v.window_start_ms)?;
        self.time(v.window_length_ms)?;
        self.money(&v.count_used)?;
        self.money(&v.value_used_minor)?;
        self.string(&v.revision)
    }
    fn fact(&mut self, v: &ReviewFactV1) -> EncodingResult {
        self.string(&v.id)?;
        self.string(&v.reviewer_id)?;
        self.boolean(&v.authorized)?;
        self.time(v.issued_at_ms)?;
        self.time(v.expires_at_ms)?;
        self.digest(&v.binding_digest)?;
        self.digest(&v.policy_digest)?;
        self.boolean(&v.consumed)?;
        self.tag(v.kind as u16);
        Ok(())
    }
    fn review_request(&mut self, v: &ReviewRequestV1) -> EncodingResult {
        self.string(&v.id)?;
        self.time(v.created_at_ms)?;
        self.time(v.expires_at_ms)?;
        self.digest(&v.binding_digest)?;
        self.digest(&v.policy_digest)
    }
    fn context(&mut self, v: &DecisionContextV1) -> EncodingResult {
        for s in [
            &v.authenticated_subject,
            &v.authenticated_profile,
            &v.authenticated_shop,
        ] {
            self.string(s)?;
        }
        self.time(v.now_ms)?;
        self.evidence(&v.evidence)?;
        self.budget(&v.budget)?;
        self.option(&v.review.grant, Self::fact)?;
        self.option(&v.review.attestation, Self::fact)?;
        self.option(&v.review_request, Self::review_request)?;
        self.option(&v.approved_evidence_revision, |e, s| e.string(s))?;
        self.digest(&v.policy_digest)?;
        self.digest(&v.registry_digest)
    }
    fn constraints(&mut self, v: &CommerceConstraintsV1) -> EncodingResult {
        for s in [
            &v.profile_id,
            &v.shop_id,
            &v.subject_id,
            &v.operation_id,
            &v.resource_id,
        ] {
            self.string(s)?;
        }
        self.tag(v.action as u16);
        self.digest(&v.payload_digest)?;
        self.option(&v.amount_ceiling_minor, Self::money)?;
        self.option(&v.currency, |e, s| e.currency(s))?;
        self.string(&v.evidence_revision)?;
        self.digest(&v.policy_digest)?;
        self.digest(&v.registry_digest)?;
        self.count(v.review_ids.len())?;
        let mut ids: Vec<_> = v.review_ids.iter().collect();
        ids.sort_unstable_by(|a, b| a.as_bytes().cmp(b.as_bytes()));
        if ids.windows(2).any(|w| w[0] == w[1]) {
            return Err(INVALID);
        }
        for id in ids {
            self.string(id)?;
        }
        self.time(v.evaluation_time_ms)
    }
    fn escalation(&mut self, v: &EscalationRequirementV1) -> EncodingResult {
        self.time(v.requested_at_ms)?;
        self.time(v.expires_at_ms)?;
        self.option(&v.review_request_id, |e, s| e.string(s))?;
        if v.required_kinds.is_empty() || v.required_kinds.len() > 2 {
            return Err(INVALID);
        }
        let mut kinds = v.required_kinds.clone();
        kinds.sort_unstable();
        if kinds.windows(2).any(|w| w[0] == w[1]) {
            return Err(INVALID);
        }
        self.count(kinds.len())?;
        for kind in kinds {
            self.tag(kind as u16);
        }
        Ok(())
    }
    fn body(&mut self, v: &CommerceDecisionV1) -> EncodingResult {
        let shape_ok = match v.outcome {
            CommerceOutcomeV1::Allow => v.constraints.is_some() && v.escalation.is_none(),
            CommerceOutcomeV1::Deny => v.constraints.is_none() && v.escalation.is_none(),
            CommerceOutcomeV1::Escalate => v.constraints.is_none() && v.escalation.is_some(),
        };
        if !shape_ok || !v.primary_reason.permits_outcome(v.outcome) || v.diagnostics.len() > 32 {
            return Err(INVALID);
        }
        let mut reasons = v.diagnostics.clone();
        reasons.sort_unstable_by_key(|reason| reason.rank());
        if reasons.windows(2).any(|w| w[0] == w[1])
            || reasons.iter().any(|r| {
                *r == v.primary_reason
                    || *r == ReasonV1::OK_POLICY_PERMIT
                    || r.rank() < v.primary_reason.rank()
            })
        {
            return Err(INVALID);
        }
        self.tag(v.outcome as u16);
        self.tag(v.primary_reason as u16);
        self.count(reasons.len())?;
        for reason in reasons {
            self.tag(reason as u16);
        }
        self.option(&v.constraints, Self::constraints)?;
        self.option(&v.escalation, Self::escalation)?;
        self.digest(&v.policy_digest)?;
        self.digest(&v.registry_digest)?;
        self.digest(&v.evidence_digest)
    }
}

#[cfg(test)]
mod encoding_tests {
    use super::*;

    #[test]
    fn registry_conformance() {
        // Literal expected bytes: 46 preserved enum-tag vectors plus the
        // additive E_APPROVAL_INVALID (tag 32) vector. No observed-output oracle.
        let cases: &[(u16, [u8; 2])] = &[
            (CommerceActionV1::RefundCreate as u16, [0x00, 0x00]),
            (CommerceActionV1::OrderAddressUpdate as u16, [0x00, 0x01]),
            (CommerceActionV1::OrderCancel as u16, [0x00, 0x02]),
            (CommerceActionV1::DiscountCreate as u16, [0x00, 0x03]),
            (CommerceActionV1::CustomerEmailSend as u16, [0x00, 0x04]),
            (ProvenanceStateV1::TrustedBoundUnused as u16, [0x00, 0x00]),
            (ProvenanceStateV1::Missing as u16, [0x00, 0x01]),
            (ProvenanceStateV1::Invalid as u16, [0x00, 0x02]),
            (ProvenanceStateV1::Consumed as u16, [0x00, 0x03]),
            (CommerceOutcomeV1::Allow as u16, [0x00, 0x00]),
            (CommerceOutcomeV1::Deny as u16, [0x00, 0x01]),
            (CommerceOutcomeV1::Escalate as u16, [0x00, 0x02]),
            (ReviewKindV1::Grant as u16, [0x00, 0x00]),
            (ReviewKindV1::Attestation as u16, [0x00, 0x01]),
            (ReasonV1::E_INTERNAL_EVALUATION as u16, [0x00, 0x00]),
            (ReasonV1::E_LEDGER_UNAVAILABLE as u16, [0x00, 0x01]),
            (ReasonV1::E_STATE_UNAVAILABLE as u16, [0x00, 0x02]),
            (ReasonV1::E_MALFORMED_REQUEST as u16, [0x00, 0x03]),
            (ReasonV1::E_UNAUTHENTICATED_CALLER as u16, [0x00, 0x04]),
            (ReasonV1::E_CALLER_IDENTITY_ASSERTED as u16, [0x00, 0x05]),
            (ReasonV1::E_UNKNOWN_ACTION as u16, [0x00, 0x06]),
            (ReasonV1::E_UNDERSPECIFIED_ACTION as u16, [0x00, 0x07]),
            (ReasonV1::E_FORBIDDEN_CALLER_FIELD as u16, [0x00, 0x08]),
            (ReasonV1::E_UNTRUSTED_KEY as u16, [0x00, 0x09]),
            (ReasonV1::E_REVOKED_KEY as u16, [0x00, 0x0a]),
            (ReasonV1::E_INVALID_SIGNATURE as u16, [0x00, 0x0b]),
            (ReasonV1::E_CAPABILITY_EXPIRED as u16, [0x00, 0x0c]),
            (ReasonV1::E_BINDING_MISMATCH as u16, [0x00, 0x0d]),
            (ReasonV1::E_REPLAY as u16, [0x00, 0x0e]),
            (ReasonV1::E_MISSING_EVIDENCE as u16, [0x00, 0x0f]),
            (ReasonV1::E_EVIDENCE_CONFLICT as u16, [0x00, 0x10]),
            (ReasonV1::E_STALE_STATE as u16, [0x00, 0x11]),
            (ReasonV1::E_OPERATION_UNKNOWN as u16, [0x00, 0x12]),
            (ReasonV1::E_POLICY_UNCONFIGURED as u16, [0x00, 0x13]),
            (ReasonV1::E_POLICY_FORBID as u16, [0x00, 0x14]),
            (ReasonV1::E_AMOUNT_LIMIT as u16, [0x00, 0x15]),
            (ReasonV1::E_COUNT_LIMIT as u16, [0x00, 0x16]),
            (ReasonV1::E_VALUE_LIMIT as u16, [0x00, 0x17]),
            (ReasonV1::E_ORDER_WINDOW as u16, [0x00, 0x18]),
            (ReasonV1::E_DISCOUNT_CONFLICT as u16, [0x00, 0x19]),
            (ReasonV1::E_CONSTRAINT_VIOLATION as u16, [0x00, 0x1a]),
            (ReasonV1::E_PROVENANCE_UNVERIFIABLE as u16, [0x00, 0x1b]),
            (ReasonV1::E_APPROVAL_REQUIRED as u16, [0x00, 0x1c]),
            (ReasonV1::E_APPROVAL_EXPIRED as u16, [0x00, 0x1d]),
            (ReasonV1::E_REVIEWER_SEPARATION as u16, [0x00, 0x1e]),
            (ReasonV1::OK_POLICY_PERMIT as u16, [0x00, 0x1f]),
            (ReasonV1::E_APPROVAL_INVALID as u16, [0x00, 0x20]),
        ];
        assert_eq!(cases.len(), 47);
        for &(tag, expected) in cases {
            let mut e = Encoder(Vec::new());
            e.tag(tag);
            assert_eq!(e.0, expected, "wire tag {tag}");
        }
    }

    fn expected(hex: &str) -> Vec<u8> {
        assert_eq!(hex.len() % 2, 0);
        hex.as_bytes()
            .chunks_exact(2)
            .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
            .collect()
    }

    #[test]
    fn deterministic_digest() {
        // All nine primitive/collection vectors; use the production writer.
        // Literal preimages are copied from the preserved vector input.
        let mut e = Encoder(Vec::new());
        e.option::<[u8; 32]>(&None, Encoder::digest).unwrap();
        assert_eq!(e.0, expected("00"));
        e.0.clear();
        e.option(&Some([0; 32]), Encoder::digest).unwrap();

        assert_eq!(
            e.0,
            expected("010000000000000000000000000000000000000000000000000000000000000000")
        );
        e.0.clear();
        e.option(&Some(0), Encoder::money).unwrap();
        assert_eq!(e.0, expected("010000000000000000"));
        e.0.clear();
        e.money(&i64::MAX).unwrap();
        assert_eq!(e.0, expected("7fffffffffffffff"));
        e.0.clear();
        e.number(u64::from(u32::MAX)).unwrap();
        assert_eq!(e.0, expected("00000000ffffffff"));
        e.0.clear();
        e.string("é").unwrap();
        assert_eq!(e.0, expected("00000002c3a9"));
        e.0.clear();
        e.string(&"a".repeat(128)).unwrap();
        assert_eq!(
            e.0,
            expected(
                "000000806161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161616161"
            )
        );
        e.0.clear();
        let constraints = CommerceConstraintsV1 {
            profile_id: "a".into(),
            shop_id: "a".into(),
            subject_id: "a".into(),
            operation_id: "a".into(),
            resource_id: "a".into(),
            action: CommerceActionV1::RefundCreate,
            payload_digest: [0; 32],
            amount_ceiling_minor: None,
            currency: None,
            evidence_revision: "a".into(),
            policy_digest: [0; 32],
            registry_digest: [0; 32],
            review_ids: vec!["b".into(), "a".into()],
            evaluation_time_ms: 0,
        };
        e.constraints(&constraints).unwrap();
        // The final field is an eight-byte time, immediately after the ID set.

        let ids = expected("0000000200000001610000000162");
        assert_eq!(&e.0[e.0.len() - 8 - ids.len()..e.0.len() - 8], ids);
        e.0.clear();
        let body = CommerceDecisionV1 {
            outcome: CommerceOutcomeV1::Deny,
            primary_reason: ReasonV1::E_INTERNAL_EVALUATION,
            diagnostics: vec![ReasonV1::E_POLICY_FORBID, ReasonV1::E_MALFORMED_REQUEST],
            constraints: None,
            escalation: None,
            decision_digest: [0; 32],
            policy_digest: [0; 32],
            registry_digest: [0; 32],
            evidence_digest: [0; 32],
        };
        e.body(&body).unwrap();

        let diagnostics = expected("0000000200030014");
        // Outcome and primary are the two two-byte tags before diagnostics.
        assert_eq!(&e.0[4..4 + diagnostics.len()], diagnostics);
    }
}

// Preparation uses only syntax-preserving parsing before the positive grammar
// walk. In particular, Schema conversion can evaluate action attribute values;
// it must remain below both guards. No Context, Entities or Authorizer is built.
use cedar_policy::{PolicySet, Schema, ValidationMode, Validator};
use serde::de::{Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use serde_json::{Map, Value};
use std::collections::BTreeSet;
use std::str::FromStr;

type BundleResult<T = ()> = Result<T, BundleErrorV1>;
const UNSUPPORTED: BundleErrorV1 = BundleErrorV1::UnsupportedFeature;
const STRUCTURE: BundleErrorV1 = BundleErrorV1::StructureLimitExceeded;
const SETTINGS: BundleErrorV1 = BundleErrorV1::SettingsInvalid;
const SOURCE_LIMIT: usize = 128 * 1024;

/// Immutable validated preparation result. No mutable Cedar accessor is exposed,
/// and callers cannot construct this type themselves. It grants no authority.
///
/// ```compile_fail
/// use dgr_core::commerce::PreparedCommerceBundleV1;
/// let bundle = PreparedCommerceBundleV1 { };
/// ```
#[derive(Debug)]
pub struct PreparedCommerceBundleV1 {
    schema: Schema,
    policies: PolicySet,
    settings: Vec<(CommerceActionV1, CommerceActionSettingsV1)>,
    digest: [u8; 32],
    registry_digest: [u8; 32],
}

impl PreparedCommerceBundleV1 {
    pub fn schema(&self) -> &Schema {
        &self.schema
    }
    pub fn policies(&self) -> &PolicySet {
        &self.policies
    }
    pub fn action_settings(&self) -> &[(CommerceActionV1, CommerceActionSettingsV1)] {
        &self.settings
    }
    pub fn digest(&self) -> &[u8; 32] {
        &self.digest
    }
    pub fn registry_digest(&self) -> &[u8; 32] {
        &self.registry_digest
    }
}

/// Prepare an operator bundle without evaluation. `trusted_registry_digest`
/// must come from separately trusted configuration, never from the bundle or
/// an agent request. No generated registry identity is implied by this API.
///
/// Limits bound accepted inputs, not worst-case parser time or memory. No hard
/// deadline is promised. Cedar's extension code remains compiled in the graph;
/// this boundary rejects extension syntax before semantic schema conversion.
pub fn prepare_bundle_v1(
    source: &CommerceBundleSourceV1,
    trusted_registry_digest: &[u8; 32],
) -> BundleResult<PreparedCommerceBundleV1> {
    validate_source_bounds(source)?;
    let schema = parse_schema_json(&source.schema_text)?;
    let policies = PolicySet::from_str(&source.permissions_text)
        .map_err(|_| BundleErrorV1::MalformedSource)?;
    let est = policies
        .clone()
        .to_json()
        .map_err(|_| BundleErrorV1::MalformedSource)?;
    validate_bundle_limits_v1(&schema, &est)?;
    validate_no_extensions_v1(&schema, &est)?;
    #[cfg(test)]
    PREPARATION_TRACE.with(|trace| trace.borrow_mut().push("schema-conversion"));
    let schema = Schema::from_json_value(schema).map_err(|_| BundleErrorV1::SchemaInvalid)?;
    if !Validator::new(schema.clone())
        .validate(&policies, ValidationMode::Strict)
        .validation_passed()
    {
        return Err(BundleErrorV1::PolicyInvalid);
    }
    validate_settings(&source.action_settings)?;
    if source.registry_digest != *trusted_registry_digest {
        return Err(BundleErrorV1::RegistryMismatch);
    }
    let digest = bundle_digest_v1(source)?;
    if digest != source.declared_digest {
        return Err(BundleErrorV1::DigestMismatch);
    }
    let mut settings = source.action_settings.clone();
    settings.sort_by_key(|(action, _)| *action);
    Ok(PreparedCommerceBundleV1 {
        schema,
        policies,
        settings,
        digest,
        registry_digest: source.registry_digest,
    })
}

fn validate_source_bounds(source: &CommerceBundleSourceV1) -> BundleResult {
    if source
        .schema_text
        .len()
        .checked_add(source.permissions_text.len())
        .is_none_or(|n| n > SOURCE_LIMIT)
    {
        return Err(BundleErrorV1::SourceTooLarge);
    }
    Ok(())
}

// Value's ordinary deserializer accepts duplicate map keys. This syntax-only
// visitor rejects them at every nesting level before any semantic conversion.
struct UniqueJson(Value);
impl<'de> Deserialize<'de> for UniqueJson {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct JsonVisitor;
        impl<'de> Visitor<'de> for JsonVisitor {
            type Value = UniqueJson;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("JSON without duplicate keys")
            }
            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> {
                Ok(UniqueJson(v.into()))
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
                Ok(UniqueJson(v.into()))
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
                Ok(UniqueJson(v.into()))
            }
            fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<Self::Value, E> {
                serde_json::Number::from_f64(v)
                    .map(|n| UniqueJson(Value::Number(n)))
                    .ok_or_else(|| E::custom("nonfinite number"))
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(UniqueJson(v.into()))
            }
            fn visit_unit<E>(self) -> Result<Self::Value, E> {
                Ok(UniqueJson(Value::Null))
            }
            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut values = Vec::new();
                while let Some(UniqueJson(v)) = seq.next_element()? {
                    values.push(v);
                }
                Ok(UniqueJson(Value::Array(values)))
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut values = Map::new();
                while let Some((k, UniqueJson(v))) = map.next_entry::<String, UniqueJson>()? {
                    if values.insert(k, v).is_some() {
                        return Err(serde::de::Error::custom("duplicate key"));
                    }
                }
                Ok(UniqueJson(Value::Object(values)))
            }
        }
        deserializer.deserialize_any(JsonVisitor)
    }
}
fn parse_schema_json(text: &str) -> BundleResult<Value> {
    serde_json::from_str::<UniqueJson>(text)
        .map(|v| v.0)
        .map_err(|_| BundleErrorV1::MalformedSource)
}

// A separate complete structural pass gives size/depth errors precedence even
// when unsupported syntax occurs earlier in a different branch.
fn validate_bundle_limits_v1(schema: &Value, est: &Value) -> BundleResult {
    // Bound the whole parsed set, including templates that will subsequently
    // be refused. Splitting entries across the two maps cannot evade precedence.
    let policy_count = ["staticPolicies", "templates"]
        .iter()
        .filter_map(|key| est.get(*key).and_then(Value::as_object))
        .map(Map::len)
        .sum::<usize>();
    if policy_count > 64 {
        return Err(STRUCTURE);
    }
    fn literal_limits(v: &Value, depth: usize) -> BundleResult {
        if depth > 32 {
            return Err(STRUCTURE);
        }
        match v {
            Value::Object(o) => {
                if o.len() > 32 {
                    return Err(STRUCTURE);
                }
                for child in o.values() {
                    literal_limits(child, depth + 1)?;
                }
            }
            Value::Array(a) => {
                if a.len() > 64 {
                    return Err(STRUCTURE);
                }
                for child in a {
                    literal_limits(child, depth + 1)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
    // Literal record bounds must also win over a forbidden expression elsewhere.
    for ns in schema.as_object().into_iter().flat_map(|o| o.values()) {
        for action in ns
            .get("actions")
            .and_then(Value::as_object)
            .into_iter()
            .flat_map(|o| o.values())
        {
            if let Some(attrs) = action.get("attributes") {
                literal_limits(attrs, 0)?;
            }
        }
    }
    let mut pending = vec![(schema, 0usize), (est, 0)];
    while let Some((node, depth)) = pending.pop() {
        if depth > 128 {
            return Err(STRUCTURE);
        }
        match node {
            Value::String(s) if s.len() > 1024 => return Err(STRUCTURE),
            Value::Array(a) => {
                if a.len() > 64 {
                    return Err(STRUCTURE);
                }
                pending.extend(a.iter().map(|v| (v, depth + 1)));
            }
            Value::Object(o) => {
                if let Some(literal) = o.get("Value") {
                    literal_limits(literal, 0)?;
                }
                if o.len() > 64 || o.keys().any(|s| s.len() > 1024) {
                    return Err(STRUCTURE);
                }
                for key in ["attributes", "Record"] {
                    if o.get(key)
                        .and_then(Value::as_object)
                        .is_some_and(|v| v.len() > 32)
                    {
                        return Err(STRUCTURE);
                    }
                }
                pending.extend(o.values().map(|v| (v, depth + 1)));
            }
            _ => {}
        }
    }
    // Type nesting and EST expression nesting use semantic levels, not JSON
    // wrapper levels. Every definition is visited; aliases need no expansion.
    fn nesting(v: &Value, types: usize, expressions: usize) -> BundleResult {
        let t = types + usize::from(v.get("type").is_some_and(Value::is_string));
        let e = expressions
            + usize::from(v.as_object().is_some_and(|o| {
                o.len() == 1
                    && o.keys().any(|k| {
                        matches!(
                            k.as_str(),
                            "Value"
                                | "Var"
                                | "!"
                                | "neg"
                                | "=="
                                | "!="
                                | "in"
                                | "<"
                                | "<="
                                | ">"
                                | ">="
                                | "&&"
                                | "||"
                                | "+"
                                | "-"
                                | "*"
                                | "."
                                | "has"
                                | "is"
                                | "like"
                                | "if-then-else"
                                | "Set"
                                | "Record"
                                | "contains"
                                | "containsAll"
                                | "containsAny"
                                | "isEmpty"
                                | "getTag"
                                | "hasTag"
                        )
                    })
            }));
        if t > 33 || e > 33 {
            return Err(STRUCTURE);
        }
        match v {
            Value::Object(o) => {
                for child in o.values() {
                    nesting(child, t, e)?;
                }
            }
            Value::Array(a) => {
                for child in a {
                    nesting(child, t, e)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
    nesting(schema, 0, 0)?;
    nesting(est, 0, 0)
}

fn object(v: &Value) -> BundleResult<&Map<String, Value>> {
    v.as_object().ok_or(UNSUPPORTED)
}
fn array(v: &Value) -> BundleResult<&[Value]> {
    v.as_array().map(Vec::as_slice).ok_or(UNSUPPORTED)
}
fn fields<'a>(
    v: &'a Value,
    required: &[&str],
    optional: &[&str],
) -> BundleResult<&'a Map<String, Value>> {
    let o = object(v)?;
    if required.iter().any(|k| !o.contains_key(*k))
        || o.keys()
            .any(|k| !required.contains(&k.as_str()) && !optional.contains(&k.as_str()))
    {
        return Err(UNSUPPORTED);
    }
    Ok(o)
}
fn text(v: &Value) -> BundleResult<&str> {
    v.as_str().ok_or(UNSUPPORTED)
}
fn strings(v: &Value) -> BundleResult {
    for item in array(v)? {
        text(item)?;
    }
    Ok(())
}
fn entity_literal(v: &Value) -> BundleResult {
    let o = fields(v, &["type", "id"], &[])?;
    if text(&o["type"])?.is_empty() || text(&o["id"])?.is_empty() {
        return Err(UNSUPPORTED);
    }
    Ok(())
}
fn validate_extension_free_literal_v1(v: &Value) -> BundleResult {
    match v {
        Value::Bool(_) | Value::String(_) => Ok(()),
        Value::Number(n) if n.as_i64().is_some() => Ok(()),
        Value::Array(a) => {
            for item in a {
                validate_extension_free_literal_v1(item)?;
            }
            Ok(())
        }
        Value::Object(o) => {
            if o.contains_key("__extn") || o.contains_key("__expr") {
                return Err(UNSUPPORTED);
            }
            if let Some(uid) = o.get("__entity") {
                if o.len() != 1 {
                    return Err(UNSUPPORTED);
                }
                return entity_literal(uid);
            }
            if o.len() > 32 {
                return Err(STRUCTURE);
            }
            for value in o.values() {
                validate_extension_free_literal_v1(value)?;
            }
            Ok(())
        }
        _ => Err(UNSUPPORTED),
    }
}
fn validate_expression(v: &Value) -> BundleResult {
    let o = object(v)?;
    if o.len() != 1 {
        return Err(UNSUPPORTED);
    }
    let (tag, data) = o.iter().next().ok_or(UNSUPPORTED)?;
    match tag.as_str() {
        "Value" => validate_extension_free_literal_v1(data),
        "Var" if matches!(text(data)?, "principal" | "action" | "resource" | "context") => Ok(()),
        "!" | "neg" | "isEmpty" => {
            fields(data, &["arg"], &[])?;
            validate_expression(&data["arg"])
        }
        "==" | "!=" | "in" | "<" | "<=" | ">" | ">=" | "&&" | "||" | "+" | "-" | "*"
        | "contains" | "containsAll" | "containsAny" | "getTag" | "hasTag" => {
            fields(data, &["left", "right"], &[])?;
            validate_expression(&data["left"])?;
            validate_expression(&data["right"])
        }
        "." | "has" => {
            fields(data, &["left", "attr"], &[])?;
            if data["attr"].is_string() {
                text(&data["attr"])?;
            } else if tag == "has" && !array(&data["attr"])?.is_empty() {
                strings(&data["attr"])?;
            } else {
                return Err(UNSUPPORTED);
            }
            validate_expression(&data["left"])
        }
        "is" => {
            fields(data, &["left", "entity_type"], &["in"])?;
            text(&data["entity_type"])?;
            validate_expression(&data["left"])?;
            if let Some(expr) = data.get("in") {
                validate_expression(expr)?;
            }
            Ok(())
        }
        "like" => {
            fields(data, &["left", "pattern"], &[])?;
            for p in array(&data["pattern"])? {
                if p.as_str() != Some("Wildcard") {
                    fields(p, &["Literal"], &[])?;
                    text(&p["Literal"])?;
                }
            }
            validate_expression(&data["left"])
        }
        "if-then-else" => {
            fields(data, &["if", "then", "else"], &[])?;
            for k in ["if", "then", "else"] {
                validate_expression(&data[k])?;
            }
            Ok(())
        }
        "Set" => {
            for e in array(data)? {
                validate_expression(e)?;
            }
            Ok(())
        }
        "Record" => {
            for e in object(data)?.values() {
                validate_expression(e)?;
            }
            Ok(())
        }
        // Every extension call/method, Slot and unknown future tag fails closed.
        _ => Err(UNSUPPORTED),
    }
}
fn validate_scope(v: &Value) -> BundleResult {
    let o = fields(v, &["op"], &["entity", "entities", "entity_type", "in"])?;
    match text(&o["op"])? {
        "All" => {
            fields(v, &["op"], &[])?;
        }
        "==" => {
            fields(v, &["op", "entity"], &[])?;
            entity_literal(&o["entity"])?;
        }
        "in" => {
            if o.contains_key("entity") == o.contains_key("entities") {
                return Err(UNSUPPORTED);
            }
            fields(v, &["op"], &["entity", "entities"])?;
            if let Some(uid) = o.get("entity") {
                entity_literal(uid)?;
            }
            if let Some(uids) = o.get("entities") {
                for uid in array(uids)? {
                    entity_literal(uid)?;
                }
            }
        }
        "is" => {
            fields(v, &["op", "entity_type"], &["in"])?;
            text(&o["entity_type"])?;
            if let Some(uid) = o.get("in") {
                entity_literal(uid)?;
            }
        }
        _ => return Err(UNSUPPORTED),
    }
    Ok(())
}
fn validate_policy_est_v1(est: &Value) -> BundleResult {
    let root = fields(est, &["templates", "staticPolicies", "templateLinks"], &[])?;
    if !object(&root["templates"])?.is_empty() || !array(&root["templateLinks"])?.is_empty() {
        return Err(UNSUPPORTED);
    }
    for p in object(&root["staticPolicies"])?.values() {
        let o = fields(
            p,
            &["effect", "principal", "action", "resource", "conditions"],
            &["annotations"],
        )?;
        if !matches!(text(&o["effect"])?, "permit" | "forbid") {
            return Err(UNSUPPORTED);
        }
        if let Some(a) = o.get("annotations") {
            for v in object(a)?.values() {
                text(v)?;
            }
        }
        for k in ["principal", "action", "resource"] {
            validate_scope(&o[k])?;
        }
        for c in array(&o["conditions"])? {
            fields(c, &["kind", "body"], &[])?;
            if !matches!(text(&c["kind"])?, "when" | "unless") {
                return Err(UNSUPPORTED);
            }
            validate_expression(&c["body"])?;
        }
    }
    Ok(())
}
fn validate_schema_type(v: &Value, attribute: bool) -> BundleResult {
    let mut ty = object(v)?.clone();
    if attribute
        && let Some(required) = ty.remove("required")
        && !required.is_boolean()
    {
        return Err(UNSUPPORTED);
    }
    let v = Value::Object(ty);
    let tag = text(&v["type"])?;
    match tag {
        "Extension" => return Err(UNSUPPORTED),
        "String" | "Long" | "Boolean" => {
            fields(&v, &["type"], &[])?;
        }
        "Set" => {
            fields(&v, &["type", "element"], &[])?;
            validate_schema_type(&v["element"], false)?;
        }
        "Record" => {
            fields(&v, &["type", "attributes"], &["additionalAttributes"])?;
            if v.get("additionalAttributes")
                .is_some_and(|a| a != &Value::Bool(false))
            {
                return Err(UNSUPPORTED);
            }
            for a in object(&v["attributes"])?.values() {
                validate_schema_type(a, true)?;
            }
        }
        "Entity" | "EntityOrCommon" if v.get("name").is_some() => {
            fields(&v, &["type", "name"], &[])?;
            if text(&v["name"])?.is_empty() {
                return Err(UNSUPPORTED);
            }
        }
        _ => {
            fields(&v, &["type"], &[])?;
            if tag.is_empty() {
                return Err(UNSUPPORTED);
            }
        }
    }
    Ok(())
}
fn validate_schema_types_v1(schema: &Value) -> BundleResult {
    for ns in object(schema)?.values() {
        fields(ns, &[], &["entityTypes", "actions", "commonTypes"])?;
        if let Some(types) = ns.get("commonTypes") {
            for t in object(types)?.values() {
                validate_schema_type(t, false)?;
            }
        }
        if let Some(entities) = ns.get("entityTypes") {
            for entity in object(entities)?.values() {
                fields(entity, &[], &["shape", "memberOfTypes", "tags"])?;
                if let Some(parents) = entity.get("memberOfTypes") {
                    strings(parents)?;
                }
                for k in ["shape", "tags"] {
                    if let Some(t) = entity.get(k) {
                        validate_schema_type(t, false)?;
                    }
                }
            }
        }
        if let Some(actions) = ns.get("actions") {
            for action in object(actions)?.values() {
                fields(action, &[], &["appliesTo", "memberOf", "attributes"])?;
                if let Some(attrs) = action.get("attributes") {
                    validate_extension_free_literal_v1(attrs)?;
                }
                if let Some(parents) = action.get("memberOf") {
                    for p in array(parents)? {
                        fields(p, &["id"], &["type"])?;
                        text(&p["id"])?;
                        if let Some(t) = p.get("type") {
                            text(t)?;
                        }
                    }
                }
                if let Some(applies) = action.get("appliesTo") {
                    fields(
                        applies,
                        &[],
                        &["principalTypes", "resourceTypes", "context"],
                    )?;
                    for k in ["principalTypes", "resourceTypes"] {
                        if let Some(a) = applies.get(k) {
                            strings(a)?;
                        }
                    }
                    if let Some(t) = applies.get("context") {
                        validate_schema_type(t, false)?;
                    }
                }
            }
        }
    }
    Ok(())
}
fn validate_no_extensions_v1(schema: &Value, est: &Value) -> BundleResult {
    validate_policy_est_v1(est)?;
    validate_schema_types_v1(schema)
}

fn validate_settings(entries: &[(CommerceActionV1, CommerceActionSettingsV1)]) -> BundleResult {
    if entries.len() != 5
        || entries
            .iter()
            .map(|(a, _)| a)
            .collect::<BTreeSet<_>>()
            .len()
            != 5
    {
        return Err(SETTINGS);
    }
    for (action, s) in entries {
        let monetary = matches!(
            action,
            CommerceActionV1::RefundCreate | CommerceActionV1::DiscountCreate
        );
        if s.enabled {
            if s.count_ceiling.is_none() || s.budget_window_ms.is_none_or(|n| n == 0) {
                return Err(SETTINGS);
            }
            if monetary
                && (s.currencies.is_empty()
                    || s.amount_ceiling_minor.is_none()
                    || s.value_ceiling_minor.is_none())
            {
                return Err(SETTINGS);
            }
            if *action == CommerceActionV1::RefundCreate && s.order_age_limit_seconds.is_none() {
                return Err(SETTINGS);
            }
            let grant = s.require_monetary_review || !s.review_routes.is_empty();
            let attest = s.attestation_enabled;
            if (grant || attest)
                && (s.review_request_timeout_ms.is_none_or(|n| n == 0)
                    || s.reviewer_role_policy_id.is_none())
            {
                return Err(SETTINGS);
            }
            if grant && s.grant_max_lifetime_ms.is_none_or(|n| n == 0) {
                return Err(SETTINGS);
            }
            if attest && s.attestation_max_lifetime_ms.is_none_or(|n| n == 0) {
                return Err(SETTINGS);
            }
        }
        // Encode on a throw-away buffer to enforce all representation and set
        // constraints, including supplied values for disabled actions.
        Encoder(Vec::new()).settings(s).map_err(|_| SETTINGS)?;
    }
    Ok(())
}

/// Commit source bytes and typed settings in the V1 bundle frame. This checks
/// source/setting bounds and configuration: callers still must prepare and
/// verify the declared identity. It does not parse policy or establish registry
/// trust, and it is not an encoder for intentionally unconfigured test inputs.
pub fn bundle_digest_v1(source: &CommerceBundleSourceV1) -> BundleResult<[u8; 32]> {
    Ok(Sha256::digest(encode_bundle_v1(source)?).into())
}
fn encode_bundle_v1(source: &CommerceBundleSourceV1) -> BundleResult<Vec<u8>> {
    validate_source_bounds(source)?;
    validate_settings(&source.action_settings)?;
    let mut e = Encoder(b"DGR-HERMES-BUNDLE-V1\0".to_vec());
    for s in [&source.schema_text, &source.permissions_text] {
        e.0.extend_from_slice(&(s.len() as u32).to_be_bytes());
        e.0.extend_from_slice(s.as_bytes());
    }
    let mut entries: Vec<_> = source.action_settings.iter().collect();
    entries.sort_by_key(|(action, _)| *action);
    e.count(entries.len()).map_err(|_| SETTINGS)?;
    for (action, settings) in entries {
        e.tag(*action as u16);
        e.settings(settings).map_err(|_| SETTINGS)?;
    }
    e.digest(&source.registry_digest).map_err(|_| SETTINGS)?;
    Ok(e.0)
}
impl Encoder {
    fn settings(&mut self, s: &CommerceActionSettingsV1) -> EncodingResult {
        self.boolean(&s.enabled)?;
        self.count(s.currencies.len())?;
        let mut currencies: Vec<_> = s.currencies.iter().collect();
        currencies.sort();
        if currencies.windows(2).any(|w| w[0] == w[1]) {
            return Err(INVALID);
        }
        for c in currencies {
            self.currency(c)?;
        }
        self.count(s.required_evidence.len())?;
        let mut evidence = s.required_evidence.clone();
        evidence.sort_by_key(|(field, _)| *field);
        if evidence.windows(2).any(|w| w[0].0 == w[1].0) {
            return Err(INVALID);
        }
        for (field, age) in evidence {
            self.tag(field as u16);
            self.time(age)?;
        }
        for n in [
            &s.amount_ceiling_minor,
            &s.count_ceiling,
            &s.value_ceiling_minor,
        ] {
            self.option(n, Self::money)?;
        }
        for n in [&s.budget_window_ms, &s.order_age_limit_seconds] {
            self.option(n, |e, v| e.time(*v))?;
        }
        for b in [
            &s.require_provenance,
            &s.attestation_enabled,
            &s.require_monetary_review,
        ] {
            self.boolean(b)?;
        }
        for n in [
            &s.review_request_timeout_ms,
            &s.grant_max_lifetime_ms,
            &s.attestation_max_lifetime_ms,
        ] {
            self.option(n, |e, v| e.time(*v))?;
        }
        self.option(&s.reviewer_role_policy_id, |e, s| e.string(s))?;
        self.count(s.review_routes.len())?;
        let mut routes = s.review_routes.clone();
        routes.sort();
        if routes.windows(2).any(|w| w[0] == w[1]) {
            return Err(INVALID);
        }
        for route in routes {
            self.tag(route as u16);
        }
        Ok(())
    }
}

#[cfg(test)]
thread_local! {
    static PREPARATION_TRACE: std::cell::RefCell<Vec<&'static str>> = const { std::cell::RefCell::new(Vec::new()) };
}

#[cfg(test)]
mod preparation_guard_tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn unknown_grammar_and_literals_fail_closed() {
        for expr in [
            json!({"newNativeOperator": {"Value": true}}),
            json!({"Slot":"?principal"}),
            json!({"Value":{"__extn":{"fn":"decimal","arg":"1.0"}}}),
            json!({"Var":"unknown"}),
            json!({"Value":true,"Var":"context"}),
        ] {
            assert_eq!(validate_expression(&expr), Err(UNSUPPORTED));
        }
        assert_eq!(
            validate_expression(&json!({"Record":{"__extn":{"Value":"decimal"}}})),
            Ok(())
        );
        assert_eq!(
            validate_expression(
                &json!({"Value":{"__entity":{"type":"Principal","id":"decimal(1)"}}})
            ),
            Ok(())
        );
        for value in [
            json!(null),
            json!(1.5),
            json!({"__entity":{"type":"P","id":"x"},"other":1}),
            json!({"__expr":"true"}),
        ] {
            assert_eq!(validate_extension_free_literal_v1(&value), Err(UNSUPPORTED));
        }
    }

    #[test]
    fn raw_schema_guard_precedes_semantic_conversion() {
        let mut source = CommerceBundleSourceV1 {
            schema_text: "{}".into(),
            permissions_text: "permit(principal, action, resource);".into(),
            action_settings: vec![],
            registry_digest: [0; 32],
            declared_digest: [0; 32],
        };
        PREPARATION_TRACE.with(|t| t.borrow_mut().clear());
        assert_eq!(
            prepare_bundle_v1(&source, &[0; 32]).unwrap_err(),
            BundleErrorV1::SettingsInvalid
        );
        PREPARATION_TRACE.with(|t| assert_eq!(*t.borrow(), ["schema-conversion"]));
        for schema in [
            json!({"":{"commonTypes":{"Unused":{"type":"Extension","name":"decimal"}}}}),
            json!({"":{"actions":{"a":{"attributes":{"x":{"__extn":{"fn":"decimal","arg":"1.0"}}}}}}}),
        ] {
            source.schema_text = schema.to_string();
            PREPARATION_TRACE.with(|t| t.borrow_mut().clear());
            assert_eq!(
                prepare_bundle_v1(&source, &[0; 32]).unwrap_err(),
                UNSUPPORTED
            );
            PREPARATION_TRACE.with(|t| assert!(t.borrow().is_empty()));
        }
        source.schema_text = "{}".into();
        source.permissions_text = "permit(principal, action, resource) when { false && decimal(\"1\") == decimal(\"1\") };".into();
        PREPARATION_TRACE.with(|t| t.borrow_mut().clear());
        assert_eq!(
            prepare_bundle_v1(&source, &[0; 32]).unwrap_err(),
            UNSUPPORTED
        );
        PREPARATION_TRACE.with(|t| assert!(t.borrow().is_empty()));
    }
}

#[cfg(test)]
mod bundle_encoding_tests {
    use super::*;
    #[test]
    fn historical_bundle_frame() {
        let settings = CommerceActionSettingsV1 {
            enabled: false,
            currencies: vec![],
            required_evidence: vec![],
            amount_ceiling_minor: None,
            count_ceiling: None,
            value_ceiling_minor: None,
            budget_window_ms: None,
            order_age_limit_seconds: None,
            require_provenance: false,
            attestation_enabled: false,
            require_monetary_review: false,
            review_request_timeout_ms: None,
            grant_max_lifetime_ms: None,
            attestation_max_lifetime_ms: None,
            reviewer_role_policy_id: None,
            review_routes: vec![],
        };
        let source = CommerceBundleSourceV1 {
            schema_text: "{}".into(),
            permissions_text: String::new(),
            action_settings: [
                CommerceActionV1::RefundCreate,
                CommerceActionV1::OrderAddressUpdate,
                CommerceActionV1::OrderCancel,
                CommerceActionV1::DiscountCreate,
                CommerceActionV1::CustomerEmailSend,
            ]
            .into_iter()
            .map(|a| (a, settings.clone()))
            .collect(),
            registry_digest: [0; 32],
            declared_digest: [0; 32],
        };
        // Historical structural frame: no commerce permission is asserted.
        let expected_hex = "4447522d4845524d45532d42554e444c452d563100000000027b7d00000000000000050000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000003000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
        let expected: Vec<u8> = (0..expected_hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&expected_hex[i..i + 2], 16).unwrap())
            .collect();
        assert_eq!(encode_bundle_v1(&source).unwrap(), expected);
        let digest = bundle_digest_v1(&source)
            .unwrap()
            .iter()
            .map(|v| format!("{v:02x}"))
            .collect::<String>();
        assert_eq!(
            digest,
            "e0c3fec499fb8d9e2cb243b3b77be8ed8cfedaaf2c9ad912a34637638e6b9d9f"
        );
    }
}

/// Native Cedar inputs for the pinned HermesCommerce schema. Construction is
/// not an authorization decision; no Authorizer is invoked here.
///
/// ```compile_fail
/// use dgr_core::commerce::PreparedCedarInputsV1;
/// let inputs = PreparedCedarInputsV1 {};
/// ```
#[derive(Debug)]
pub struct PreparedCedarInputsV1 {
    request: cedar_policy::Request,
    entities: cedar_policy::Entities,
}
impl PreparedCedarInputsV1 {
    pub fn request(&self) -> &cedar_policy::Request {
        &self.request
    }
    pub fn entities(&self) -> &cedar_policy::Entities {
        &self.entities
    }
}

/// Construct only native restricted literals, never parse caller JSON or Cedar
/// expressions. The complete typed request/context is representation-checked
/// before any extension-capable Cedar value constructor is reached.
///
/// This is the pinned three-field context projection, not an arbitrary schema
/// adapter. Other evidence, budget and review predicates belong to the decision
/// wrapper. Success neither checks those predicates nor grants permission.
pub fn build_cedar_request_v1(
    request: &CommerceRequestV1,
    context: &DecisionContextV1,
    bundle: &PreparedCommerceBundleV1,
) -> Result<PreparedCedarInputsV1, ReasonV1> {
    let mut bounds = Encoder(Vec::new());
    bounds.request(request)?;
    bounds.context(context)?;
    if request.profile_id != context.authenticated_profile
        || request.shop_id != context.authenticated_shop
        || request.subject_id != context.authenticated_subject
        || context.policy_digest != bundle.digest
        || context.registry_digest != bundle.registry_digest
    {
        return Err(ReasonV1::E_BINDING_MISMATCH);
    }
    let settings = bundle
        .settings
        .iter()
        .find(|(a, _)| *a == request.action)
        .map(|(_, s)| s)
        .ok_or(ReasonV1::E_INTERNAL_EVALUATION)?;
    let principal = scoped_cedar_uid(
        "Principal",
        &request.profile_id,
        &request.shop_id,
        &request.subject_id,
    )?;
    let resource = scoped_cedar_uid(
        "Resource",
        &request.profile_id,
        &request.shop_id,
        &request.resource_id,
    )?;
    let action_name = match request.action {
        CommerceActionV1::RefundCreate => "commerce.refund.create",
        CommerceActionV1::OrderAddressUpdate => "commerce.order.address_update",
        CommerceActionV1::OrderCancel => "commerce.order.cancel",
        CommerceActionV1::DiscountCreate => "commerce.discount.create",
        CommerceActionV1::CustomerEmailSend => "comms.customer_email.send",
    };
    let action = cedar_uid("Action", action_name.to_owned())?;
    // No caller-supplied RestrictedExpression can enter this closed literal set.
    let native_pairs = [
        (
            "profileId".to_owned(),
            cedar_policy::RestrictedExpression::new_string(request.profile_id.clone()),
        ),
        (
            "shopId".to_owned(),
            cedar_policy::RestrictedExpression::new_string(request.shop_id.clone()),
        ),
        (
            "actionEnabled".to_owned(),
            cedar_policy::RestrictedExpression::new_bool(settings.enabled),
        ),
    ];
    let native_context = cedar_policy::Context::from_pairs(native_pairs)
        .map_err(|_| ReasonV1::E_INTERNAL_EVALUATION)?;
    let entities = build_cedar_entities_v1(request, principal.clone(), resource.clone(), bundle)?;
    let request = cedar_policy::Request::new(
        principal,
        action,
        resource,
        native_context,
        Some(&bundle.schema),
    )
    .map_err(|_| ReasonV1::E_INTERNAL_EVALUATION)?;
    Ok(PreparedCedarInputsV1 { request, entities })
}

fn cedar_uid(kind: &str, id: String) -> Result<cedar_policy::EntityUid, ReasonV1> {
    let name = format!("HermesCommerce::{kind}")
        .parse()
        .map_err(|_| ReasonV1::E_INTERNAL_EVALUATION)?;
    Ok(cedar_policy::EntityUid::from_type_name_and_id(
        name,
        cedar_policy::EntityId::new(id),
    ))
}

fn scoped_cedar_uid(
    kind: &str,
    profile: &str,
    shop: &str,
    id: &str,
) -> Result<cedar_policy::EntityUid, ReasonV1> {
    let mut frame = Encoder(format!("HermesCommerce.{kind}\0").into_bytes());
    for value in [profile, shop, id] {
        frame.string(value)?;
    }
    let digest = Sha256::digest(frame.0);
    let hex = digest.iter().map(|byte| format!("{byte:02x}")).collect();
    cedar_uid(kind, hex)
}

fn build_cedar_entities_v1(
    request: &CommerceRequestV1,
    principal: cedar_policy::EntityUid,
    resource: cedar_policy::EntityUid,
    bundle: &PreparedCommerceBundleV1,
) -> Result<cedar_policy::Entities, ReasonV1> {
    let mut entities = Vec::new();
    for uid in [principal, resource] {
        let attrs = [
            (
                "profileId".to_owned(),
                cedar_policy::RestrictedExpression::new_string(request.profile_id.clone()),
            ),
            (
                "shopId".to_owned(),
                cedar_policy::RestrictedExpression::new_string(request.shop_id.clone()),
            ),
        ]
        .into_iter()
        .collect();
        entities.push(
            cedar_policy::Entity::new(uid, attrs, Default::default())
                .map_err(|_| ReasonV1::E_INTERNAL_EVALUATION)?,
        );
    }
    cedar_policy::Entities::from_entities(entities, Some(&bundle.schema))
        .map_err(|_| ReasonV1::E_INTERNAL_EVALUATION)
}

#[cfg(test)]
mod terminal_error_tests {
    use super::*;

    #[test]
    fn terminal_error_veto_does_not_follow_reason_allowances() {
        let invalid = ProvenanceReviewErrorV1 {
            provenance: ProvenanceStateV1::Invalid,
            causes: vec![ReasonV1::E_PROVENANCE_UNVERIFIABLE],
        };
        assert!(invalid.causes[0].permits_outcome(CommerceOutcomeV1::Escalate));
        assert!(invalid.requires_terminal_deny());
        let missing = ProvenanceReviewErrorV1 {
            provenance: ProvenanceStateV1::Missing,
            causes: vec![ReasonV1::E_REPLAY],
        };
        assert!(!missing.causes[0].permits_outcome(CommerceOutcomeV1::Escalate));
        assert!(!missing.requires_terminal_deny());
    }
}
