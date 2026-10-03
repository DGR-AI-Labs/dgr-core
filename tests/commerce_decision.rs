use dgr_core::commerce::*;

fn frame() -> (CommerceRequestV1, DecisionContextV1, CommerceDecisionV1) {
    let request = CommerceRequestV1 {
        profile_id: "a".into(),
        shop_id: "a".into(),
        subject_id: "a".into(),
        operation_id: "a".into(),
        resource_id: "a".into(),
        action: CommerceActionV1::RefundCreate,
        amount_minor: None,
        currency: None,
        payload_digest: [0; 32],
        payload_length: 0,
    };
    let context = DecisionContextV1 {
        authenticated_subject: "a".into(),
        authenticated_profile: "a".into(),
        authenticated_shop: "a".into(),
        now_ms: 0,
        evidence: CommerceEvidenceV1 {
            source_revision: "a".into(),
            fetched_at_ms: 0,
            evidence_digest: [0; 32],
            captured_minor: None,
            prior_refunds_minor: None,
            order_age_seconds: None,
            line_items_eligible: None,
            any_fulfillment: None,
            cancellation_eligible: None,
            discount_conflict: None,
            derived_recipient_digest: None,
            approved_template_digest: None,
            recipient_count: None,
            provenance: ProvenanceStateV1::Missing,
            provenance_binding_digest: None,
        },
        budget: CommerceBudgetV1 {
            profile_id: "a".into(),
            shop_id: "a".into(),
            subject_id: "a".into(),
            action: CommerceActionV1::RefundCreate,
            currency: None,
            window_start_ms: 0,
            window_length_ms: 1,
            count_used: 0,
            value_used_minor: 0,
            revision: "a".into(),
        },
        review: CommerceReviewV1 {
            grant: None,
            attestation: None,
        },
        review_request: None,
        approved_evidence_revision: None,
        policy_digest: [0; 32],
        registry_digest: [0; 32],
    };
    let decision = CommerceDecisionV1 {
        outcome: CommerceOutcomeV1::Deny,
        primary_reason: ReasonV1::E_MALFORMED_REQUEST,
        diagnostics: vec![],
        constraints: None,
        escalation: None,
        decision_digest: [0; 32],
        policy_digest: [0; 32],
        registry_digest: [0; 32],
        evidence_digest: [0; 32],
    };
    (request, context, decision)
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|v| format!("{v:02x}")).collect()
}

#[test]
fn deterministic_digest() {
    let (request, context, mut decision) = frame();
    // Fixed structural frame; this is not an expected evaluator verdict.
    assert_eq!(
        hex(&encode_decision_v1(&request, &context, &[0; 32], &decision).unwrap()),
        GOLDEN_DENY
    );
    let digest = decision_digest_v1(&request, &context, &[0; 32], &decision).unwrap();
    assert_eq!(hex(&digest), GOLDEN_DENY_HASH);
    // Historical Allow and Escalate frames exercise encoding only. In particular,
    // the Allow frame's a,b review IDs are not evidence of supplied/used reviews.
    let (mut ar, mut ac, mut ad) = frame();
    ar.amount_minor = Some(1000);
    ar.currency = Some("USD".into());
    ac.now_ms = 1000;
    ac.evidence.captured_minor = Some(10000);
    ac.evidence.prior_refunds_minor = Some(0);
    ad.outcome = CommerceOutcomeV1::Allow;
    ad.primary_reason = ReasonV1::OK_POLICY_PERMIT;
    ad.constraints = Some(CommerceConstraintsV1 {
        profile_id: "a".into(),
        shop_id: "a".into(),
        subject_id: "a".into(),
        operation_id: "a".into(),
        resource_id: "a".into(),
        action: CommerceActionV1::RefundCreate,
        payload_digest: [0; 32],
        amount_ceiling_minor: Some(1000),
        currency: Some("USD".into()),
        evidence_revision: "a".into(),
        policy_digest: [0; 32],
        registry_digest: [0; 32],
        review_ids: vec!["a".into(), "b".into()],
        evaluation_time_ms: 1000,
    });
    assert_eq!(
        hex(&encode_decision_v1(&ar, &ac, &[0; 32], &ad).unwrap()),
        GOLDEN_ALLOW
    );
    assert_eq!(
        hex(&decision_digest_v1(&ar, &ac, &[0; 32], &ad).unwrap()),
        GOLDEN_ALLOW_HASH
    );
    let (er, mut ec, mut ed) = frame();
    ec.now_ms = 1500;
    ec.review_request = Some(ReviewRequestV1 {
        id: "r".into(),
        created_at_ms: 1000,
        expires_at_ms: 2000,
        binding_digest: [0; 32],
        policy_digest: [0; 32],
    });
    ed.outcome = CommerceOutcomeV1::Escalate;
    ed.primary_reason = ReasonV1::E_PROVENANCE_UNVERIFIABLE;
    ed.diagnostics = vec![ReasonV1::E_APPROVAL_REQUIRED];
    ed.escalation = Some(EscalationRequirementV1 {
        requested_at_ms: 1000,
        expires_at_ms: 2000,
        review_request_id: Some("r".into()),
        required_kinds: vec![ReviewKindV1::Grant, ReviewKindV1::Attestation],
    });
    assert_eq!(
        hex(&encode_decision_v1(&er, &ec, &[0; 32], &ed).unwrap()),
        GOLDEN_ESCALATE
    );
    assert_eq!(
        hex(&decision_digest_v1(&er, &ec, &[0; 32], &ed).unwrap()),
        GOLDEN_ESCALATE_HASH
    );
    decision.decision_digest = [7; 32];
    assert_eq!(
        decision_digest_v1(&request, &context, &[0; 32], &decision).unwrap(),
        digest
    );
    assert_ne!(
        decision_digest_v1(&request, &context, &[1; 32], &decision).unwrap(),
        digest
    );

    decision.primary_reason = ReasonV1::E_INTERNAL_EVALUATION;
    decision.diagnostics = vec![ReasonV1::E_POLICY_FORBID, ReasonV1::E_MALFORMED_REQUEST];
    let encoded = encode_decision_v1(&request, &context, &[0; 32], &decision).unwrap();
    assert_eq!(hex(&encoded), GOLDEN_MULTIPLE);
    assert_eq!(
        hex(&decision_digest_v1(&request, &context, &[0; 32], &decision).unwrap()),
        GOLDEN_MULTIPLE_HASH
    );
    decision.diagnostics.reverse();
    assert_eq!(
        encode_decision_v1(&request, &context, &[0; 32], &decision).unwrap(),
        encoded
    );
    decision.diagnostics.push(ReasonV1::E_POLICY_FORBID);
    assert!(encode_decision_v1(&request, &context, &[0; 32], &decision).is_err());
    decision.diagnostics = vec![ReasonV1::E_INTERNAL_EVALUATION];
    assert!(encode_decision_v1(&request, &context, &[0; 32], &decision).is_err());
}

#[test]
fn bounded_inputs() {
    let (request, context, decision) = frame();
    for invalid in [
        String::new(),
        "x".repeat(129),
        "bad\0id".into(),
        "bad\u{85}id".into(),
    ] {
        let mut r = request.clone();
        r.profile_id = invalid;
        assert!(encode_decision_v1(&r, &context, &[0; 32], &decision).is_err());
    }
    let mut r = request.clone();
    r.profile_id = "é".repeat(64);
    assert!(encode_decision_v1(&r, &context, &[0; 32], &decision).is_ok());
    r.profile_id.push('é');
    assert!(encode_decision_v1(&r, &context, &[0; 32], &decision).is_err());
    r = request.clone();
    r.amount_minor = Some(-1);
    assert!(encode_decision_v1(&r, &context, &[0; 32], &decision).is_err());
    r.amount_minor = Some(i64::MAX);
    assert!(encode_decision_v1(&r, &context, &[0; 32], &decision).is_ok());
    let mut c = context.clone();
    c.now_ms = i64::MAX as u64;
    assert!(encode_decision_v1(&request, &c, &[0; 32], &decision).is_ok());
    c.now_ms += 1;
    assert!(encode_decision_v1(&request, &c, &[0; 32], &decision).is_err());
    c = context.clone();
    c.budget.count_used = -1;
    assert!(encode_decision_v1(&request, &c, &[0; 32], &decision).is_err());
    c = context.clone();
    c.evidence.prior_refunds_minor = Some(-1);
    assert!(encode_decision_v1(&request, &c, &[0; 32], &decision).is_err());
}

#[test]
fn constraint_binding() {
    let (r, mut c, mut d) = frame();
    let absent = decision_digest_v1(&r, &c, &[0; 32], &d).unwrap();
    c.evidence.provenance_binding_digest = Some([0; 32]);
    assert_ne!(decision_digest_v1(&r, &c, &[0; 32], &d).unwrap(), absent);
    c.evidence.provenance_binding_digest = None;
    c.evidence.line_items_eligible = Some(false);
    assert_ne!(decision_digest_v1(&r, &c, &[0; 32], &d).unwrap(), absent);
    c.evidence.line_items_eligible = None;
    c.evidence.recipient_count = Some(0);
    assert_ne!(decision_digest_v1(&r, &c, &[0; 32], &d).unwrap(), absent);
    c.evidence.recipient_count = None;
    c.approved_evidence_revision = Some("prior".into());
    assert_ne!(decision_digest_v1(&r, &c, &[0; 32], &d).unwrap(), absent);
    // Mismatched trusted/request identities remain committed in a Deny frame.
    c = frame().1;
    c.authenticated_subject = "other".into();
    assert_ne!(decision_digest_v1(&r, &c, &[0; 32], &d).unwrap(), absent);
    d.outcome = CommerceOutcomeV1::Allow;
    d.primary_reason = ReasonV1::OK_POLICY_PERMIT;
    assert!(encode_decision_v1(&r, &c, &[0; 32], &d).is_err());
    d.constraints = Some(CommerceConstraintsV1 {
        profile_id: "a".into(),
        shop_id: "a".into(),
        subject_id: "a".into(),
        operation_id: "a".into(),
        resource_id: "a".into(),
        action: CommerceActionV1::RefundCreate,
        payload_digest: [0; 32],
        amount_ceiling_minor: Some(0),
        currency: Some("USD".into()),
        evidence_revision: "a".into(),
        policy_digest: [0; 32],
        registry_digest: [0; 32],
        review_ids: vec!["b".into(), "a".into()],
        evaluation_time_ms: 0,
    });
    // Encoding this synthetic body asserts no semantic Allow or review use.
    let unordered = encode_decision_v1(&r, &c, &[0; 32], &d).unwrap();
    d.constraints.as_mut().unwrap().review_ids.reverse();
    assert_eq!(encode_decision_v1(&r, &c, &[0; 32], &d).unwrap(), unordered);
    d.constraints.as_mut().unwrap().review_ids.push("a".into());
    assert!(encode_decision_v1(&r, &c, &[0; 32], &d).is_err());
    d.constraints = None;
    d.outcome = CommerceOutcomeV1::Escalate;
    d.primary_reason = ReasonV1::E_APPROVAL_REQUIRED;
    d.escalation = Some(EscalationRequirementV1 {
        requested_at_ms: 0,
        expires_at_ms: 1,
        review_request_id: None,
        required_kinds: vec![],
    });
    assert!(encode_decision_v1(&r, &c, &[0; 32], &d).is_err());
    d.escalation.as_mut().unwrap().required_kinds =
        vec![ReviewKindV1::Attestation, ReviewKindV1::Grant];
    let two = encode_decision_v1(&r, &c, &[0; 32], &d).unwrap();
    d.escalation.as_mut().unwrap().required_kinds.reverse();
    assert_eq!(encode_decision_v1(&r, &c, &[0; 32], &d).unwrap(), two);
    d.escalation.as_mut().unwrap().required_kinds = vec![ReviewKindV1::Grant, ReviewKindV1::Grant];
    assert!(encode_decision_v1(&r, &c, &[0; 32], &d).is_err());
}

#[test]
fn no_implicit_v1_conversion() {
    // Positive type/import controls accompany the isolated compile_fail doctests.
    let _: CommerceOutcomeV1 = CommerceOutcomeV1::Deny;
    let _: CommerceDecisionV1 = frame().2;
    let _: dgr_core::RequiredOutcome = dgr_core::RequiredOutcome::Deny;
}

const GOLDEN_DENY: &str = "4447522d4845524d45532d4445434953494f4e2d563100000000016100000001610000000161000000016100000001610000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000016100000001610000000161000000000000000000000001610000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000016100000001610000000161000000000000000000000000000000000000010000000000000000000000000000000000000001610000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010003000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
const GOLDEN_DENY_HASH: &str = "bc7f6b6fd7c25f2e951306dc0db186694867ab56ba2e26d2a754484c6054cb6e";

const GOLDEN_MULTIPLE: &str = "4447522d4845524d45532d4445434953494f4e2d56310000000001610000000161000000016100000001610000000161000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001610000000161000000016100000000000000000000000161000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000001610000000161000000016100000000000000000000000000000000000001000000000000000000000000000000000000000161000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000000002000300140000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
const GOLDEN_MULTIPLE_HASH: &str =
    "40d7bf243c28e1e3c661961632287843ddeb9370f870b9fefacef0c82b0a4d4f";

const GOLDEN_ALLOW: &str = "4447522d4845524d45532d4445434953494f4e2d5631000000000161000000016100000001610000000161000000016100000100000000000003e801000000035553440000000000000000000000000000000000000000000000000000000000000000000000000000000000000001610000000161000000016100000000000003e8000000016100000000000000000000000000000000000000000000000000000000000000000000000000000000010000000000002710010000000000000000000000000000000000010000000001610000000161000000016100000000000000000000000000000000000001000000000000000000000000000000000000000161000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001f000000000100000001610000000161000000016100000001610000000161000000000000000000000000000000000000000000000000000000000000000000000100000000000003e80100000003555344000000016100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000020000000161000000016200000000000003e800000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
const GOLDEN_ALLOW_HASH: &str = "6004d9638710a844c7c0ecd1e56a0cb27353af6e62417a06b033d28747711fda";

const GOLDEN_ESCALATE: &str = "4447522d4845524d45532d4445434953494f4e2d56310000000001610000000161000000016100000001610000000161000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001610000000161000000016100000000000005dc0000000161000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000001610000000161000000016100000000000000000000000000000000000001000000000000000000000000000000000000000161000001000000017200000000000003e800000000000007d000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002001b00000001001c000100000000000003e800000000000007d00100000001720000000200000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
const GOLDEN_ESCALATE_HASH: &str =
    "06d05529e303341a4d920ce92a1a93a0064bb6f695aa7f6df03348d3b057604f";
