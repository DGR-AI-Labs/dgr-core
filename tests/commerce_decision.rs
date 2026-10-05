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

    // Exercise each currency site separately so one guard cannot mask another.
    let mut allow = decision.clone();
    allow.outcome = CommerceOutcomeV1::Allow;
    allow.primary_reason = ReasonV1::OK_POLICY_PERMIT;
    allow.constraints = Some(CommerceConstraintsV1 {
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
        review_ids: vec![],
        evaluation_time_ms: 0,
    });
    for currency in [None, Some("USD"), Some("EUR"), Some("ZZZ")] {
        let mut r = request.clone();
        r.currency = currency.map(str::to_owned);
        assert!(encode_decision_v1(&r, &context, &[0; 32], &decision).is_ok());
        let mut c = context.clone();
        c.budget.currency = currency.map(str::to_owned);
        assert!(encode_decision_v1(&request, &c, &[0; 32], &decision).is_ok());
        let mut a = allow.clone();
        a.constraints.as_mut().unwrap().currency = currency.map(str::to_owned);
        assert!(encode_decision_v1(&request, &context, &[0; 32], &a).is_ok());
    }
    for invalid in ["usd", "US", "USDD", "US1", "€", "dollars", ""] {
        let mut r = request.clone();
        r.currency = Some(invalid.into());
        assert!(
            encode_decision_v1(&r, &context, &[0; 32], &decision).is_err(),
            "request {invalid}"
        );
        let mut c = context.clone();
        c.budget.currency = Some(invalid.into());
        assert!(
            encode_decision_v1(&request, &c, &[0; 32], &decision).is_err(),
            "budget {invalid}"
        );
        let mut a = allow.clone();
        a.constraints.as_mut().unwrap().currency = Some(invalid.into());
        assert!(
            encode_decision_v1(&request, &context, &[0; 32], &a).is_err(),
            "constraints {invalid}"
        );
    }
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

fn preparation_settings() -> CommerceActionSettingsV1 {
    CommerceActionSettingsV1 {
        enabled: true,
        currencies: vec!["USD".into()],
        required_evidence: vec![(EvidenceFieldV1::SourceRevision, 1000)],
        amount_ceiling_minor: Some(5000),
        count_ceiling: Some(3),
        value_ceiling_minor: Some(5000),
        budget_window_ms: Some(60000),
        order_age_limit_seconds: Some(1000),
        require_provenance: true,
        attestation_enabled: true,
        require_monetary_review: false,
        review_request_timeout_ms: Some(1000),
        grant_max_lifetime_ms: Some(1000),
        attestation_max_lifetime_ms: Some(1000),
        reviewer_role_policy_id: Some("reviewer".into()),
        review_routes: vec![],
    }
}
fn preparation_source() -> CommerceBundleSourceV1 {
    let actions = [
        CommerceActionV1::RefundCreate,
        CommerceActionV1::OrderAddressUpdate,
        CommerceActionV1::OrderCancel,
        CommerceActionV1::DiscountCreate,
        CommerceActionV1::CustomerEmailSend,
    ];
    let mut schema_actions = serde_json::Map::new();
    for a in actions {
        schema_actions.insert(
            format!("{a:?}"),
            serde_json::json!({"appliesTo": {
                "principalTypes": ["Principal"], "resourceTypes": ["Resource"],
                "context": {"type": "Record", "attributes": {}}
            }}),
        );
    }
    let mut source = CommerceBundleSourceV1 {
        schema_text: serde_json::json!({"HermesCommerce": {
            "entityTypes": {"Principal": {}, "Resource": {}}, "actions": schema_actions
        }})
        .to_string(),
        permissions_text: "permit(principal, action, resource);".into(),
        action_settings: actions
            .into_iter()
            .map(|a| (a, preparation_settings()))
            .collect(),
        registry_digest: [7; 32],
        declared_digest: [0; 32],
    };
    recommit(&mut source);
    source
}
fn recommit(source: &mut CommerceBundleSourceV1) {
    source.declared_digest = bundle_digest_v1(source).unwrap();
}
fn prep_error(source: &CommerceBundleSourceV1) -> BundleErrorV1 {
    prepare_bundle_v1(source, &[7; 32]).unwrap_err()
}

#[test]
fn bundle_error_categories() {
    let base = preparation_source();
    let prepared = prepare_bundle_v1(&base, &[7; 32]).unwrap();
    assert_eq!(prepared.digest(), &base.declared_digest);
    assert_eq!(prepared.registry_digest(), &[7; 32]);
    assert_eq!(prepared.policies().policies().count(), 1);
    assert_eq!(prepared.action_settings().len(), 5);
    let _ = prepared.schema();
    let mut bad = base.clone();
    bad.schema_text = "{".into();
    assert_eq!(prep_error(&bad), BundleErrorV1::MalformedSource);
    bad = base.clone();
    bad.permissions_text = " ".repeat(131073);
    assert_eq!(prep_error(&bad), BundleErrorV1::SourceTooLarge);
    bad = base.clone();
    bad.permissions_text = "permit(principal, action, resource);\n".repeat(65);
    assert_eq!(prep_error(&bad), BundleErrorV1::StructureLimitExceeded);
    bad = base.clone();
    bad.schema_text = r#"{"HermesCommerce":{"entityTypes":{"Principal":{"shape":{"type":"NonexistentType"}}},"actions":{}}}"#.into();
    assert_eq!(prep_error(&bad), BundleErrorV1::SchemaInvalid);
    bad = base.clone();
    bad.permissions_text =
        "permit(principal, action, resource) when { context.unknownAttribute == true };".into();
    assert_eq!(prep_error(&bad), BundleErrorV1::PolicyInvalid);
    bad = base.clone();
    bad.action_settings[0].1.budget_window_ms = Some(0);
    assert_eq!(prep_error(&bad), BundleErrorV1::SettingsInvalid);
    bad = base.clone();
    bad.declared_digest = [0; 32];
    assert_eq!(prep_error(&bad), BundleErrorV1::DigestMismatch);
    bad = base.clone();
    bad.registry_digest = [0; 32];
    assert_eq!(prep_error(&bad), BundleErrorV1::RegistryMismatch);
    bad = base.clone();
    bad.permissions_text =
        "permit(principal, action, resource) when { decimal(\"1.0\") == decimal(\"1.0\") };".into();
    assert_eq!(prep_error(&bad), BundleErrorV1::UnsupportedFeature);
    // Trust must be independent of the declaration; digest agreement alone is
    // not trust. Supplying an unrelated expected identity rejects valid source.
    assert_eq!(
        prepare_bundle_v1(&base, &[8; 32]).unwrap_err(),
        BundleErrorV1::RegistryMismatch
    );
}

#[test]
fn preparation_error_precedence() {
    let mut source = preparation_source();
    source.schema_text = r#"{"HermesCommerce":{"entityTypes":{"Principal":{"shape":{"type":"NonexistentType"}}},"actions":{}}}"#.into();
    source.permissions_text = "permit(principal, action, resource) when { decimal(\"1.0\") == decimal(\"1.0\") && context.unknownAttribute };".into();
    assert_eq!(prep_error(&source), BundleErrorV1::UnsupportedFeature);
    source.permissions_text =
        "permit(principal, action, resource) when { context.unknownAttribute };".into();
    assert_eq!(prep_error(&source), BundleErrorV1::SchemaInvalid);
    source.schema_text = preparation_source().schema_text;
    assert_eq!(prep_error(&source), BundleErrorV1::PolicyInvalid);
    source
        .permissions_text
        .push_str(&"permit(principal, action, resource);".repeat(64));
    assert_eq!(prep_error(&source), BundleErrorV1::StructureLimitExceeded);
    source.permissions_text =
        "permit(principal, action, resource) when { decimal(\"1\") == decimal(\"1\") };".repeat(65);
    assert_eq!(prep_error(&source), BundleErrorV1::StructureLimitExceeded);
    source.schema_text = "{".into();
    assert_eq!(prep_error(&source), BundleErrorV1::MalformedSource);
    source.permissions_text = " ".repeat(131073);
    assert_eq!(prep_error(&source), BundleErrorV1::SourceTooLarge);
}

#[test]
fn extension_rejection_before_authorizer() {
    // This slice has no Authorizer or Context construction path. The unit tests
    // separately instrument the actual semantic-schema conversion boundary.
    // These are preparation assertions, not evaluator reachability proof.
    let cases = [
        "decimal(\"1.0\") == decimal(\"1.0\")",
        "ip(\"127.0.0.1\") == ip(\"127.0.0.1\")",
        "datetime(\"2026-01-01T00:00:00Z\") == datetime(\"2026-01-01T00:00:00Z\")",
        "duration(\"1h\") == duration(\"1h\")",
        "decimal(\"1.0\").lessThan(decimal(\"2.0\"))",
        "{amount: decimal(\"1.0\")}.amount == decimal(\"1.0\")",
        "[decimal(\"1.0\")].contains(decimal(\"1.0\"))",
        "if true then true else decimal(\"1.0\") == decimal(\"1.0\")",
        "if false then decimal(\"1.0\") == decimal(\"1.0\") else true",
        "false && decimal(\"1.0\") == decimal(\"1.0\")",
        "true || decimal(\"1.0\") == decimal(\"1.0\")",
    ];
    for expr in cases {
        let mut source = preparation_source();
        source.permissions_text = format!("permit(principal, action, resource) when {{ {expr} }};");
        recommit(&mut source);
        assert_eq!(
            prep_error(&source),
            BundleErrorV1::UnsupportedFeature,
            "{expr}"
        );
    }
    let mut source = preparation_source();
    source.permissions_text.push_str("permit(principal == HermesCommerce::Principal::\"never\", action, resource) when { decimal(\"1.0\") == decimal(\"1.0\") };");
    recommit(&mut source);
    assert_eq!(prep_error(&source), BundleErrorV1::UnsupportedFeature);
    source.permissions_text = "permit(principal == ?principal, action, resource);".into();
    recommit(&mut source);
    assert_eq!(prep_error(&source), BundleErrorV1::UnsupportedFeature);
    for ty in ["decimal", "ipaddr", "datetime", "duration"] {
        source = preparation_source();
        let mut schema: serde_json::Value = serde_json::from_str(&source.schema_text).unwrap();
        schema["HermesCommerce"]["commonTypes"] = serde_json::json!({
            "Unused": {"type":"Alias"}, "Alias": {"type":"Set", "element":{
                "type":"Record", "attributes":{"nested":{"type":"Extension","name":ty}}
            }}
        });
        source.schema_text = schema.to_string();
        recommit(&mut source);
        assert_eq!(
            prep_error(&source),
            BundleErrorV1::UnsupportedFeature,
            "{ty}"
        );
    }
    for literal in [
        serde_json::json!({"__extn":{"fn":"decimal","arg":"1.0"}}),
        serde_json::json!({"nested":[{"__extn":{"fn":"decimal","arg":"1.0"}}]}),
        serde_json::json!({"__expr":"decimal(\"1.0\")"}),
    ] {
        source = preparation_source();
        let mut schema: serde_json::Value = serde_json::from_str(&source.schema_text).unwrap();
        schema["HermesCommerce"]["actions"]["RefundCreate"]["attributes"] =
            serde_json::json!({"x":literal});
        source.schema_text = schema.to_string();
        recommit(&mut source);
        assert_eq!(prep_error(&source), BundleErrorV1::UnsupportedFeature);
    }
}

#[test]
fn native_policy_positive_controls() {
    for expr in [
        "1 + 2 == 3",
        "!false",
        "-1 < 0",
        "1 <= 2 && 2 >= 1 && 2 > 1 && 1 != 2",
        "[1, 2].contains(1)",
        "[1, 2].containsAll([1])",
        "[1].containsAny([1])",
        "[1].isEmpty()",
        "{decimal: \"decimal(1)\", ip: \"ip\"}.decimal == \"decimal(1)\"",
        "{x: 1} has x",
        "\"duration\" like \"dur*\"",
        "if true then true else false",
        "principal is HermesCommerce::Principal",
        "principal == HermesCommerce::Principal::\"decimal(1)\"",
    ] {
        let mut source = preparation_source();
        source.permissions_text = format!(
            "@description(\"datetime duration decimal ip\") permit(principal, action, resource) when {{ {expr} }};"
        );
        recommit(&mut source);
        assert!(
            prepare_bundle_v1(&source, &[7; 32]).is_ok(),
            "{expr}: {:?}",
            prepare_bundle_v1(&source, &[7; 32])
        );
    }
}

#[test]
fn bounded_inputs_preparation() {
    let base = preparation_source();
    for size in [131071, 131072, 131073] {
        let mut s = base.clone();
        s.permissions_text
            .push_str(&" ".repeat(size - s.schema_text.len() - s.permissions_text.len()));
        assert_eq!(s.schema_text.len() + s.permissions_text.len(), size);
        if size <= 131072 {
            recommit(&mut s);
            assert!(prepare_bundle_v1(&s, &[7; 32]).is_ok());
        } else {
            assert_eq!(prep_error(&s), BundleErrorV1::SourceTooLarge);
        }
    }
    for count in [64, 65] {
        let mut s = base.clone();
        s.permissions_text = "permit(principal, action, resource);".repeat(count);
        recommit(&mut s);
        if count == 64 {
            assert!(prepare_bundle_v1(&s, &[7; 32]).is_ok());
        } else {
            assert_eq!(prep_error(&s), BundleErrorV1::StructureLimitExceeded);
        }
    }
    for text in [
        "{\"x\":{},\"x\":{}}",
        "{\"x\":{\"actions\":{},\"actions\":{}}}",
        "{} {}",
    ] {
        let mut s = base.clone();
        s.schema_text = text.into();
        assert_eq!(prep_error(&s), BundleErrorV1::MalformedSource);
    }
    let mut s = base;
    s.permissions_text = format!(
        "permit(principal, action, resource) when {{ \"{}\" == \"x\" }};",
        "x".repeat(1025)
    );
    assert_eq!(prep_error(&s), BundleErrorV1::StructureLimitExceeded);
}

#[test]
fn preparation_settings_and_identity() {
    let base = preparation_source();
    for action in [0usize, 3] {
        for missing in 0..5 {
            let mut s = base.clone();
            let settings = &mut s.action_settings[action].1;
            match missing {
                0 => settings.currencies.clear(),
                1 => settings.amount_ceiling_minor = None,
                2 => settings.count_ceiling = None,
                3 => settings.value_ceiling_minor = None,
                _ => settings.budget_window_ms = None,
            }
            assert_eq!(
                prep_error(&s),
                BundleErrorV1::SettingsInvalid,
                "action={action}, missing={missing}"
            );
        }
    }
    for field in 0..7 {
        let mut s = base.clone();
        let settings = &mut s.action_settings[0].1;
        match field {
            0 => settings.currencies.push("USD".into()),
            1 => settings
                .required_evidence
                .push((EvidenceFieldV1::SourceRevision, 2)),
            2 => settings.amount_ceiling_minor = Some(-1),
            3 => settings.attestation_max_lifetime_ms = None,
            4 => settings.review_request_timeout_ms = Some(0),
            5 => settings.reviewer_role_policy_id = None,
            _ => settings.currencies = vec!["usd".into()],
        }
        assert_eq!(
            prep_error(&s),
            BundleErrorV1::SettingsInvalid,
            "field={field}"
        );
    }
    let mut s = base.clone();
    s.action_settings[1].0 = CommerceActionV1::RefundCreate;
    assert_eq!(prep_error(&s), BundleErrorV1::SettingsInvalid);
    s = base.clone();
    s.action_settings.reverse();
    recommit(&mut s);
    assert_eq!(s.declared_digest, base.declared_digest);
    assert_eq!(
        prepare_bundle_v1(&s, &[7; 32]).unwrap().action_settings()[0].0,
        CommerceActionV1::RefundCreate
    );
    s = base.clone();
    s.permissions_text.push(' ');
    recommit(&mut s);
    assert_ne!(s.declared_digest, base.declared_digest);
    assert!(prepare_bundle_v1(&s, &[7; 32]).is_ok());
    s = base.clone();
    s.action_settings[0].1.enabled = false;
    s.action_settings[0].1.amount_ceiling_minor = None;
    recommit(&mut s);
    assert!(
        !prepare_bundle_v1(&s, &[7; 32]).unwrap().action_settings()[0]
            .1
            .enabled
    );
    assert_ne!(s.declared_digest, base.declared_digest);
}

#[test]
fn schema_depth_collections_and_alias_controls() {
    for depth in [32, 33] {
        let mut source = preparation_source();
        let mut schema: serde_json::Value = serde_json::from_str(&source.schema_text).unwrap();
        let mut ty = serde_json::json!({"type":"Long"});
        for _ in 0..depth {
            ty = serde_json::json!({"type":"Set", "element":ty});
        }
        schema["HermesCommerce"]["commonTypes"] = serde_json::json!({"UnusedNative":ty});
        source.schema_text = schema.to_string();
        recommit(&mut source);
        if depth == 32 {
            assert!(prepare_bundle_v1(&source, &[7; 32]).is_ok());
        } else {
            assert_eq!(prep_error(&source), BundleErrorV1::StructureLimitExceeded);
        }
    }
    for n in [32, 33] {
        let mut source = preparation_source();
        let fields = (0..n)
            .map(|i| format!("a{i}: {i}"))
            .collect::<Vec<_>>()
            .join(",");
        source.permissions_text =
            format!("permit(principal, action, resource) when {{ {{{fields}}}.a0 == 0 }};");
        recommit(&mut source);
        if n == 32 {
            assert!(prepare_bundle_v1(&source, &[7; 32]).is_ok());
        } else {
            assert_eq!(prep_error(&source), BundleErrorV1::StructureLimitExceeded);
        }
    }
    let mut source = preparation_source();
    let mut schema: serde_json::Value = serde_json::from_str(&source.schema_text).unwrap();
    schema["HermesCommerce"]["commonTypes"] =
        serde_json::json!({"A":{"type":"B"},"B":{"type":"A"}});
    source.schema_text = schema.to_string();
    assert_eq!(prep_error(&source), BundleErrorV1::SchemaInvalid);
    // Native tag methods must not be confused with extension methods.
    let mut source = preparation_source();
    let mut schema: serde_json::Value = serde_json::from_str(&source.schema_text).unwrap();
    schema["HermesCommerce"]["entityTypes"]["Principal"]["tags"] =
        serde_json::json!({"type":"String"});
    source.schema_text = schema.to_string();
    source.permissions_text = "permit(principal, action, resource) when { principal.hasTag(\"decimal\") && principal.getTag(\"decimal\") == \"ip\" };".into();
    recommit(&mut source);
    assert!(prepare_bundle_v1(&source, &[7; 32]).is_ok());
}

#[test]
fn preparation_vocabulary_tags() {
    assert_eq!(
        (EvidenceFieldV1::SourceRevision as u16).to_be_bytes(),
        [0, 0],
        "EvidenceFieldV1::SourceRevision"
    );
    assert_eq!(
        (EvidenceFieldV1::FetchedAtMs as u16).to_be_bytes(),
        [0, 1],
        "EvidenceFieldV1::FetchedAtMs"
    );
    assert_eq!(
        (EvidenceFieldV1::EvidenceDigest as u16).to_be_bytes(),
        [0, 2],
        "EvidenceFieldV1::EvidenceDigest"
    );
    assert_eq!(
        (EvidenceFieldV1::CapturedMinor as u16).to_be_bytes(),
        [0, 3],
        "EvidenceFieldV1::CapturedMinor"
    );
    assert_eq!(
        (EvidenceFieldV1::PriorRefundsMinor as u16).to_be_bytes(),
        [0, 4],
        "EvidenceFieldV1::PriorRefundsMinor"
    );
    assert_eq!(
        (EvidenceFieldV1::OrderAgeSeconds as u16).to_be_bytes(),
        [0, 5],
        "EvidenceFieldV1::OrderAgeSeconds"
    );
    assert_eq!(
        (EvidenceFieldV1::LineItemsEligible as u16).to_be_bytes(),
        [0, 6],
        "EvidenceFieldV1::LineItemsEligible"
    );
    assert_eq!(
        (EvidenceFieldV1::AnyFulfillment as u16).to_be_bytes(),
        [0, 7],
        "EvidenceFieldV1::AnyFulfillment"
    );
    assert_eq!(
        (EvidenceFieldV1::CancellationEligible as u16).to_be_bytes(),
        [0, 8],
        "EvidenceFieldV1::CancellationEligible"
    );
    assert_eq!(
        (EvidenceFieldV1::DiscountConflict as u16).to_be_bytes(),
        [0, 9],
        "EvidenceFieldV1::DiscountConflict"
    );
    assert_eq!(
        (EvidenceFieldV1::DerivedRecipientDigest as u16).to_be_bytes(),
        [0, 10],
        "EvidenceFieldV1::DerivedRecipientDigest"
    );
    assert_eq!(
        (EvidenceFieldV1::ApprovedTemplateDigest as u16).to_be_bytes(),
        [0, 11],
        "EvidenceFieldV1::ApprovedTemplateDigest"
    );
    assert_eq!(
        (EvidenceFieldV1::RecipientCount as u16).to_be_bytes(),
        [0, 12],
        "EvidenceFieldV1::RecipientCount"
    );
    assert_eq!(
        (EvidenceFieldV1::Provenance as u16).to_be_bytes(),
        [0, 13],
        "EvidenceFieldV1::Provenance"
    );
    assert_eq!(
        (EvidenceFieldV1::ProvenanceBindingDigest as u16).to_be_bytes(),
        [0, 14],
        "EvidenceFieldV1::ProvenanceBindingDigest"
    );
    assert_eq!(
        (ReviewRouteV1::OrderWindow as u16).to_be_bytes(),
        [0, 0],
        "ReviewRouteV1::OrderWindow"
    );
    assert_eq!(
        (ReviewRouteV1::DiscountConflict as u16).to_be_bytes(),
        [0, 1],
        "ReviewRouteV1::DiscountConflict"
    );
    assert_eq!(
        (BundleErrorV1::MalformedSource as u16).to_be_bytes(),
        [0, 0],
        "BundleErrorV1::MalformedSource"
    );
    assert_eq!(
        (BundleErrorV1::SourceTooLarge as u16).to_be_bytes(),
        [0, 1],
        "BundleErrorV1::SourceTooLarge"
    );
    assert_eq!(
        (BundleErrorV1::StructureLimitExceeded as u16).to_be_bytes(),
        [0, 2],
        "BundleErrorV1::StructureLimitExceeded"
    );
    assert_eq!(
        (BundleErrorV1::SchemaInvalid as u16).to_be_bytes(),
        [0, 3],
        "BundleErrorV1::SchemaInvalid"
    );
    assert_eq!(
        (BundleErrorV1::PolicyInvalid as u16).to_be_bytes(),
        [0, 4],
        "BundleErrorV1::PolicyInvalid"
    );
    assert_eq!(
        (BundleErrorV1::SettingsInvalid as u16).to_be_bytes(),
        [0, 5],
        "BundleErrorV1::SettingsInvalid"
    );
    assert_eq!(
        (BundleErrorV1::DigestMismatch as u16).to_be_bytes(),
        [0, 6],
        "BundleErrorV1::DigestMismatch"
    );
    assert_eq!(
        (BundleErrorV1::RegistryMismatch as u16).to_be_bytes(),
        [0, 7],
        "BundleErrorV1::RegistryMismatch"
    );
    assert_eq!(
        (BundleErrorV1::UnsupportedFeature as u16).to_be_bytes(),
        [0, 8],
        "BundleErrorV1::UnsupportedFeature"
    );
}

#[test]
fn aggregate_policy_bound_precedes_template_rejection() {
    let mut source = preparation_source();
    source.permissions_text = "permit(principal, action, resource);".repeat(64);
    source
        .permissions_text
        .push_str("permit(principal == ?principal, action, resource);");
    assert_eq!(prep_error(&source), BundleErrorV1::StructureLimitExceeded);
    source.permissions_text = "permit(principal, action, resource);".repeat(63);
    source
        .permissions_text
        .push_str("permit(principal == ?principal, action, resource);");
    assert_eq!(prep_error(&source), BundleErrorV1::UnsupportedFeature);
}

const NATIVE_SCHEMA: &str = r###"{
  "HermesCommerce": {
    "entityTypes": {
      "Principal": {
        "memberOfTypes": [],
        "shape": {
          "type": "Record",
          "attributes": {
            "profileId": {
              "type": "String"
            },
            "shopId": {
              "type": "String"
            }
          }
        }
      },
      "Resource": {
        "memberOfTypes": [],
        "shape": {
          "type": "Record",
          "attributes": {
            "profileId": {
              "type": "String"
            },
            "shopId": {
              "type": "String"
            }
          }
        }
      }
    },
    "actions": {
      "commerce.refund.create": {
        "appliesTo": {
          "principalTypes": [
            "Principal"
          ],
          "resourceTypes": [
            "Resource"
          ],
          "context": {
            "type": "Record",
            "attributes": {
              "profileId": {
                "type": "String"
              },
              "shopId": {
                "type": "String"
              },
              "actionEnabled": {
                "type": "Boolean"
              }
            }
          }
        }
      },
      "commerce.order.address_update": {
        "appliesTo": {
          "principalTypes": [
            "Principal"
          ],
          "resourceTypes": [
            "Resource"
          ],
          "context": {
            "type": "Record",
            "attributes": {
              "profileId": {
                "type": "String"
              },
              "shopId": {
                "type": "String"
              },
              "actionEnabled": {
                "type": "Boolean"
              }
            }
          }
        }
      },
      "commerce.order.cancel": {
        "appliesTo": {
          "principalTypes": [
            "Principal"
          ],
          "resourceTypes": [
            "Resource"
          ],
          "context": {
            "type": "Record",
            "attributes": {
              "profileId": {
                "type": "String"
              },
              "shopId": {
                "type": "String"
              },
              "actionEnabled": {
                "type": "Boolean"
              }
            }
          }
        }
      },
      "commerce.discount.create": {
        "appliesTo": {
          "principalTypes": [
            "Principal"
          ],
          "resourceTypes": [
            "Resource"
          ],
          "context": {
            "type": "Record",
            "attributes": {
              "profileId": {
                "type": "String"
              },
              "shopId": {
                "type": "String"
              },
              "actionEnabled": {
                "type": "Boolean"
              }
            }
          }
        }
      },
      "comms.customer_email.send": {
        "appliesTo": {
          "principalTypes": [
            "Principal"
          ],
          "resourceTypes": [
            "Resource"
          ],
          "context": {
            "type": "Record",
            "attributes": {
              "profileId": {
                "type": "String"
              },
              "shopId": {
                "type": "String"
              },
              "actionEnabled": {
                "type": "Boolean"
              }
            }
          }
        }
      }
    }
  }
}
"###;

fn native_bundle(enabled: bool) -> PreparedCommerceBundleV1 {
    let mut source = preparation_source();
    source.schema_text = NATIVE_SCHEMA.into();
    for (_, setting) in &mut source.action_settings {
        setting.enabled = enabled;
    }
    recommit(&mut source);
    prepare_bundle_v1(&source, &[7; 32]).unwrap()
}
fn native_frame(bundle: &PreparedCommerceBundleV1) -> (CommerceRequestV1, DecisionContextV1) {
    let (request, mut context, _) = frame();
    context.policy_digest = *bundle.digest();
    context.registry_digest = *bundle.registry_digest();
    (request, context)
}

#[test]
fn native_cedar_mapping_all_actions() {
    let bundle = native_bundle(true);
    let (mut request, context) = native_frame(&bundle);
    for (action, id) in [
        (CommerceActionV1::RefundCreate, "commerce.refund.create"),
        (
            CommerceActionV1::OrderAddressUpdate,
            "commerce.order.address_update",
        ),
        (CommerceActionV1::OrderCancel, "commerce.order.cancel"),
        (CommerceActionV1::DiscountCreate, "commerce.discount.create"),
        (
            CommerceActionV1::CustomerEmailSend,
            "comms.customer_email.send",
        ),
    ] {
        request.action = action;
        let inputs = build_cedar_request_v1(&request, &context, &bundle).unwrap();
        assert_eq!(
            inputs.request().action().unwrap().to_string(),
            format!("HermesCommerce::Action::\"{id}\"")
        );
        let native = inputs.request().context().unwrap();
        assert_eq!(
            native.get("profileId"),
            Some(cedar_policy::EvalResult::String("a".into()))
        );
        assert_eq!(
            native.get("shopId"),
            Some(cedar_policy::EvalResult::String("a".into()))
        );
        assert_eq!(
            native.get("actionEnabled"),
            Some(cedar_policy::EvalResult::Bool(true))
        );
        assert!(
            inputs
                .entities()
                .get(inputs.request().principal().unwrap())
                .is_some()
        );
        assert!(
            inputs
                .entities()
                .get(inputs.request().resource().unwrap())
                .is_some()
        );
        assert_ne!(inputs.request().principal(), inputs.request().resource());
        assert_eq!(
            inputs.request().principal().unwrap().to_string(),
            r#"HermesCommerce::Principal::"a1079fb65eb44b610a35023ac9b01632cc1f9cdbcf3f4412fe396faed00384fb""#
        );
        assert_eq!(
            inputs.request().resource().unwrap().to_string(),
            r#"HermesCommerce::Resource::"312a5f15bb15781c745d8f20b71b5b49c98b6522961a631ce8dda17ebffdba13""#
        );
    }
}

#[test]
fn native_cedar_scoping_and_literal_strings() {
    let bundle = native_bundle(true);
    let (mut request, mut context) = native_frame(&bundle);
    let original = build_cedar_request_v1(&request, &context, &bundle).unwrap();
    // Cedar-looking syntax is passed as string data, never parsed as an expression.
    request.profile_id = "decimal(\"1.0\")".into();
    context.authenticated_profile = request.profile_id.clone();
    let changed = build_cedar_request_v1(&request, &context, &bundle).unwrap();
    assert_ne!(
        original.request().principal(),
        changed.request().principal()
    );
    assert_ne!(original.request().resource(), changed.request().resource());
    assert_eq!(
        changed.request().context().unwrap().get("profileId"),
        Some(cedar_policy::EvalResult::String(request.profile_id.clone()))
    );
    assert_eq!(
        changed.request().context().unwrap().get("shopId"),
        Some(cedar_policy::EvalResult::String(request.shop_id.clone()))
    );
    for uid in [
        changed.request().principal().unwrap(),
        changed.request().resource().unwrap(),
    ] {
        let entity = changed.entities().get(uid).unwrap();
        assert_eq!(
            entity.attr("profileId").unwrap().unwrap(),
            cedar_policy::EvalResult::String(request.profile_id.clone())
        );
        assert_eq!(
            entity.attr("shopId").unwrap().unwrap(),
            cedar_policy::EvalResult::String(request.shop_id.clone())
        );
    }
    let disabled = native_bundle(false);
    let (request, context) = native_frame(&disabled);
    let inputs = build_cedar_request_v1(&request, &context, &disabled).unwrap();
    assert_eq!(
        inputs.request().context().unwrap().get("actionEnabled"),
        Some(cedar_policy::EvalResult::Bool(false))
    );
}

#[test]
fn native_cedar_rejects_bad_bindings_and_representations() {
    let bundle = native_bundle(true);
    let (request, context) = native_frame(&bundle);
    for field in 0..5 {
        let mut bad = context.clone();
        match field {
            0 => bad.authenticated_profile.push('x'),
            1 => bad.authenticated_shop.push('x'),
            2 => bad.authenticated_subject.push('x'),
            3 => bad.policy_digest[0] ^= 1,
            _ => bad.registry_digest[0] ^= 1,
        }
        assert_eq!(
            build_cedar_request_v1(&request, &bad, &bundle).unwrap_err(),
            ReasonV1::E_BINDING_MISMATCH
        );
    }
    let mut bad = request.clone();
    bad.currency = Some("usd".into());
    assert_eq!(
        build_cedar_request_v1(&bad, &context, &bundle).unwrap_err(),
        ReasonV1::E_MALFORMED_REQUEST
    );
    let mut bad = context.clone();
    bad.now_ms = i64::MAX as u64 + 1;
    assert_eq!(
        build_cedar_request_v1(&request, &bad, &bundle).unwrap_err(),
        ReasonV1::E_MALFORMED_REQUEST
    );
    // A prepared but incompatible native schema must not fall back to unvalidated construction.
    let source = preparation_source();
    let incompatible = prepare_bundle_v1(&source, &[7; 32]).unwrap();
    let (request, context) = native_frame(&incompatible);
    assert_eq!(
        build_cedar_request_v1(&request, &context, &incompatible).unwrap_err(),
        ReasonV1::E_INTERNAL_EVALUATION
    );
}

#[test]
fn native_cedar_uid_components_are_bound_without_concatenation_ambiguity() {
    let bundle = native_bundle(true);
    let (request, context) = native_frame(&bundle);
    let original = build_cedar_request_v1(&request, &context, &bundle).unwrap();
    for part in 0..4 {
        let mut r = request.clone();
        let mut c = context.clone();
        match part {
            0 => {
                r.profile_id.push('b');
                c.authenticated_profile = r.profile_id.clone();
            }
            1 => {
                r.shop_id.push('b');
                c.authenticated_shop = r.shop_id.clone();
            }
            2 => {
                r.subject_id.push('b');
                c.authenticated_subject = r.subject_id.clone();
            }
            _ => r.resource_id.push('b'),
        }
        let changed = build_cedar_request_v1(&r, &c, &bundle).unwrap();
        assert_eq!(
            original.request().principal() != changed.request().principal(),
            part != 3
        );
        assert_eq!(
            original.request().resource() != changed.request().resource(),
            part != 2
        );
    }
    let mut r = request.clone();
    let mut c = context.clone();
    r.profile_id = "ab".into();
    r.shop_id = "c".into();
    c.authenticated_profile = r.profile_id.clone();
    c.authenticated_shop = r.shop_id.clone();
    let first = build_cedar_request_v1(&r, &c, &bundle).unwrap();
    r.profile_id = "a".into();
    r.shop_id = "bc".into();
    c.authenticated_profile = r.profile_id.clone();
    c.authenticated_shop = r.shop_id.clone();
    let second = build_cedar_request_v1(&r, &c, &bundle).unwrap();
    assert_ne!(first.request().principal(), second.request().principal());
    assert_ne!(first.request().resource(), second.request().resource());
}

#[test]
fn native_cedar_request_and_entity_schema_checks_are_independent() {
    for entity_fault in [false, true] {
        let mut source = preparation_source();
        let mut schema: serde_json::Value = serde_json::from_str(NATIVE_SCHEMA).unwrap();
        if entity_fault {
            schema["HermesCommerce"]["entityTypes"]["Principal"]["shape"]["attributes"]["profileId"]
                ["type"] = "Boolean".into();
        } else {
            schema["HermesCommerce"]["actions"]["commerce.refund.create"]["appliesTo"]["context"]
                ["attributes"]["profileId"]["type"] = "Boolean".into();
        }
        source.schema_text = schema.to_string();
        recommit(&mut source);
        let bundle = prepare_bundle_v1(&source, &[7; 32]).unwrap();
        let (request, context) = native_frame(&bundle);
        assert_eq!(
            build_cedar_request_v1(&request, &context, &bundle).unwrap_err(),
            ReasonV1::E_INTERNAL_EVALUATION,
            "entity fault: {entity_fault}"
        );
    }
}

fn bound_frame(bundle: &PreparedCommerceBundleV1) -> (CommerceRequestV1, DecisionContextV1) {
    let (request, mut context) = native_frame(bundle);
    let digest = request_binding_digest_v1(&request).unwrap();
    context.evidence.provenance_binding_digest = Some(digest);
    for (slot, kind) in [
        (&mut context.review.grant, ReviewKindV1::Grant),
        (&mut context.review.attestation, ReviewKindV1::Attestation),
    ] {
        *slot = Some(ReviewFactV1 {
            id: "review".into(),
            reviewer_id: "reviewer".into(),
            authorized: true,
            issued_at_ms: 0,
            expires_at_ms: 100,
            binding_digest: digest,
            policy_digest: *bundle.digest(),
            consumed: false,
            kind,
        });
    }
    context.review_request = Some(ReviewRequestV1 {
        id: "pending".into(),
        created_at_ms: 0,
        expires_at_ms: 100,
        binding_digest: digest,
        policy_digest: *bundle.digest(),
    });
    (request, context)
}

#[test]
fn request_binding_golden_and_every_field() {
    let (request, _, _) = frame();
    // Literal derived from the declared frame, not from the Rust encoder output:
    // domain, five length-prefixed a strings, action 0, two None tags, digest, u64 length.
    assert_eq!(
        hex(&request_binding_digest_v1(&request).unwrap()),
        "e0e844423b1d9c0b6a4aa75a3de4b75dcf67a489ad4e8a618ab1464d1c72d0ed"
    );
    let original = request_binding_digest_v1(&request).unwrap();
    for field in 0..10 {
        let mut changed = request.clone();
        match field {
            0 => changed.profile_id.push('b'),
            1 => changed.shop_id.push('b'),
            2 => changed.subject_id.push('b'),
            3 => changed.operation_id.push('b'),
            4 => changed.resource_id.push('b'),
            5 => changed.action = CommerceActionV1::OrderCancel,
            6 => changed.amount_minor = Some(0),
            7 => changed.currency = Some("USD".into()),
            8 => changed.payload_digest[0] = 1,
            9 => changed.payload_length = 1,
            _ => unreachable!(),
        }
        assert_ne!(
            request_binding_digest_v1(&changed).unwrap(),
            original,
            "field {field}"
        );
    }
    let mut invalid = request.clone();
    invalid.currency = Some("usd".into());
    assert_eq!(
        request_binding_digest_v1(&invalid),
        Err(ReasonV1::E_MALFORMED_REQUEST)
    );
}

#[test]
fn request_binding_each_artifact_and_policy_is_independent() {
    let bundle = native_bundle(true);
    let (request, context) = bound_frame(&bundle);
    assert_eq!(
        validate_request_bindings_v1(&request, &context, &bundle),
        Ok(())
    );
    for field in 0..12 {
        let mut c = context.clone();
        match field {
            0 => c.evidence.provenance_binding_digest.as_mut().unwrap()[0] ^= 1,
            1 => c.review.grant.as_mut().unwrap().binding_digest[0] ^= 1,
            2 => c.review.attestation.as_mut().unwrap().binding_digest[0] ^= 1,
            3 => c.review_request.as_mut().unwrap().binding_digest[0] ^= 1,
            4 => c.review.grant.as_mut().unwrap().policy_digest[0] ^= 1,
            5 => c.review.attestation.as_mut().unwrap().policy_digest[0] ^= 1,
            6 => c.review_request.as_mut().unwrap().policy_digest[0] ^= 1,
            7 => c.authenticated_profile.push('b'),
            8 => c.authenticated_shop.push('b'),
            9 => c.authenticated_subject.push('b'),
            10 => c.policy_digest[0] ^= 1,
            11 => c.registry_digest[0] ^= 1,
            _ => unreachable!(),
        }
        assert_eq!(
            validate_request_bindings_v1(&request, &c, &bundle),
            Err(ReasonV1::E_BINDING_MISMATCH),
            "field {field}"
        );
    }
}

#[test]
fn request_binding_substitution_and_consistent_rebinding() {
    let bundle = native_bundle(true);
    let (request, context) = bound_frame(&bundle);
    for resource in [true, false] {
        let mut changed = request.clone();
        if resource {
            changed.resource_id.push('b');
        } else {
            changed.payload_digest[0] ^= 1;
        }
        // Valid UID construction alone cannot detect retained-artifact substitution.
        assert!(build_cedar_request_v1(&changed, &context, &bundle).is_ok());
        assert_eq!(
            validate_request_bindings_v1(&changed, &context, &bundle),
            Err(ReasonV1::E_BINDING_MISMATCH)
        );
        let mut rebound = context.clone();
        let digest = request_binding_digest_v1(&changed).unwrap();
        rebound.evidence.provenance_binding_digest = Some(digest);
        rebound.review.grant.as_mut().unwrap().binding_digest = digest;
        rebound.review.attestation.as_mut().unwrap().binding_digest = digest;
        rebound.review_request.as_mut().unwrap().binding_digest = digest;
        assert_eq!(
            validate_request_bindings_v1(&changed, &rebound, &bundle),
            Ok(())
        );
    }
}

#[test]
fn request_binding_success_is_not_artifact_acceptance() {
    let bundle = native_bundle(false);
    let (request, mut context) = bound_frame(&bundle);
    context.evidence.provenance = ProvenanceStateV1::Invalid;
    context.evidence.approved_template_digest = Some([42; 32]);
    context.evidence.derived_recipient_digest = Some([43; 32]);
    context.review.grant.as_mut().unwrap().authorized = false;
    context.review.attestation.as_mut().unwrap().consumed = true;
    // These semantic defects are deliberately NOT validated by this binding primitive.
    assert_eq!(
        validate_request_bindings_v1(&request, &context, &bundle),
        Ok(())
    );
    context.evidence.provenance_binding_digest = None;
    context.review.grant = None;
    context.review.attestation = None;
    context.review_request = None;
    assert_eq!(
        validate_request_bindings_v1(&request, &context, &bundle),
        Ok(())
    );
    context.now_ms = u64::MAX;
    assert_eq!(
        validate_request_bindings_v1(&request, &context, &bundle),
        Err(ReasonV1::E_MALFORMED_REQUEST)
    );
}

fn review_fixture(
    edit: impl Fn(&mut CommerceActionSettingsV1),
) -> (
    CommerceRequestV1,
    DecisionContextV1,
    PreparedCommerceBundleV1,
) {
    let mut source = preparation_source();
    source.schema_text = NATIVE_SCHEMA.into();
    for (_, settings) in &mut source.action_settings {
        edit(settings);
    }
    recommit(&mut source);
    let bundle = prepare_bundle_v1(&source, &[7; 32]).unwrap();
    let (request, mut context) = bound_frame(&bundle);
    context.now_ms = 50;
    context.review.grant.as_mut().unwrap().expires_at_ms = 1000;
    let attestation = context.review.attestation.as_mut().unwrap();
    attestation.id = "attestation".into();
    attestation.reviewer_id = "other-reviewer".into();
    attestation.expires_at_ms = 1000;
    context.review_request.as_mut().unwrap().expires_at_ms = 1000;
    (request, context, bundle)
}

fn review_interval(context: &mut DecisionContextV1, slot: usize, start: u64, end: u64) {
    match slot {
        0 | 1 => {
            let fact = if slot == 0 {
                &mut context.review.grant
            } else {
                &mut context.review.attestation
            };
            let fact = fact.as_mut().unwrap();
            fact.issued_at_ms = start;
            fact.expires_at_ms = end;
        }
        2 => {
            let pending = context.review_request.as_mut().unwrap();
            pending.created_at_ms = start;
            pending.expires_at_ms = end;
        }
        _ => unreachable!(),
    }
}

fn review_time_cases(slot: usize) {
    let (request, context, bundle) = review_fixture(|_| {});
    assert_eq!(
        validate_review_artifacts_v1(&request, &context, &bundle),
        Ok(())
    );
    for (now, result) in [
        (99, Ok(())),
        (100, Err(vec![ReasonV1::E_APPROVAL_EXPIRED])),
        (101, Err(vec![ReasonV1::E_APPROVAL_EXPIRED])),
    ] {
        let mut c = context.clone();
        c.now_ms = now;
        review_interval(&mut c, slot, 0, 100);
        assert_eq!(
            validate_review_artifacts_v1(&request, &c, &bundle),
            result,
            "slot {slot}, now {now}"
        );
    }
    for (start, end) in [(51, 100), (10, 10), (10, 9)] {
        let mut c = context.clone();
        review_interval(&mut c, slot, start, end);
        assert_eq!(
            validate_review_artifacts_v1(&request, &c, &bundle),
            Err(vec![ReasonV1::E_MALFORMED_REQUEST]),
            "slot {slot}, interval {start}..{end}"
        );
    }
    let mut c = context.clone();
    review_interval(&mut c, slot, 0, 1001);
    assert_eq!(
        validate_review_artifacts_v1(&request, &c, &bundle),
        Err(vec![ReasonV1::E_APPROVAL_INVALID])
    );
}

#[test]
fn review_validity_grant_time_boundaries() {
    review_time_cases(0);
}
#[test]
fn review_validity_attestation_time_boundaries() {
    review_time_cases(1);
}
#[test]
fn review_validity_request_time_boundaries() {
    review_time_cases(2);
}

fn review_fact_cases(grant: bool) {
    let (request, context, bundle) = review_fixture(|s| {
        s.require_monetary_review = false;
        s.require_provenance = false;
        s.attestation_enabled = false;
    });
    for (fault, expected) in [
        (0, ReasonV1::E_APPROVAL_INVALID),
        (1, ReasonV1::E_REPLAY),
        (2, ReasonV1::E_MALFORMED_REQUEST),
    ] {
        let mut c = context.clone();
        let f = if grant {
            &mut c.review.grant
        } else {
            &mut c.review.attestation
        }
        .as_mut()
        .unwrap();
        match fault {
            0 => f.authorized = false,
            1 => f.consumed = true,
            2 => {
                f.kind = if grant {
                    ReviewKindV1::Attestation
                } else {
                    ReviewKindV1::Grant
                }
            }
            _ => unreachable!(),
        }
        assert_eq!(
            validate_review_artifacts_v1(&request, &c, &bundle),
            Err(vec![expected]),
            "grant {grant}, fault {fault}"
        );
    }
}
#[test]
fn review_validity_optional_grant_still_validated() {
    review_fact_cases(true);
}
#[test]
fn review_validity_optional_attestation_still_validated() {
    review_fact_cases(false);
}

fn review_limit_cases(slot: usize) {
    for limit in [None, Some(0)] {
        let (request, mut context, bundle) = review_fixture(|s| {
            s.require_monetary_review = false;
            s.require_provenance = false;
            s.attestation_enabled = false;
            match slot {
                0 => s.grant_max_lifetime_ms = limit,
                1 => s.attestation_max_lifetime_ms = limit,
                2 => s.review_request_timeout_ms = limit,
                _ => unreachable!(),
            }
        });
        assert_eq!(
            validate_review_artifacts_v1(&request, &context, &bundle),
            Err(vec![ReasonV1::E_POLICY_UNCONFIGURED])
        );
        match slot {
            0 => context.review.grant = None,
            1 => context.review.attestation = None,
            2 => context.review_request = None,
            _ => unreachable!(),
        }
        assert_eq!(
            validate_review_artifacts_v1(&request, &context, &bundle),
            Ok(())
        );
    }
}
#[test]
fn review_validity_grant_requires_positive_limit() {
    review_limit_cases(0);
}
#[test]
fn review_validity_attestation_requires_positive_limit() {
    review_limit_cases(1);
}
#[test]
fn review_validity_request_requires_positive_limit() {
    review_limit_cases(2);
}

#[test]
fn review_validity_reviewer_separation_is_conditional() {
    for required in [false, true] {
        let (request, mut context, bundle) =
            review_fixture(|s| s.require_monetary_review = required);
        assert_eq!(
            validate_review_artifacts_v1(&request, &context, &bundle),
            Ok(())
        );
        context.review.attestation.as_mut().unwrap().reviewer_id = "reviewer".into();
        assert_eq!(
            validate_review_artifacts_v1(&request, &context, &bundle),
            if required {
                Err(vec![ReasonV1::E_REVIEWER_SEPARATION])
            } else {
                Ok(())
            }
        );
        context.evidence.provenance = ProvenanceStateV1::TrustedBoundUnused;
        assert_eq!(
            validate_review_artifacts_v1(&request, &context, &bundle),
            Ok(())
        );
    }
}

#[test]
fn review_validity_retains_independent_causes_in_registry_order() {
    let (mut request, mut context, bundle) = review_fixture(|s| s.require_monetary_review = true);
    request.resource_id.push('x');
    let grant = context.review.grant.as_mut().unwrap();
    grant.kind = ReviewKindV1::Attestation;
    grant.authorized = false;
    grant.consumed = true;
    grant.expires_at_ms = 50;
    let att = context.review.attestation.as_mut().unwrap();
    att.reviewer_id = "reviewer".into();
    att.authorized = false;
    att.consumed = true;
    assert_eq!(
        validate_review_artifacts_v1(&request, &context, &bundle),
        Err(vec![
            ReasonV1::E_MALFORMED_REQUEST,
            ReasonV1::E_BINDING_MISMATCH,
            ReasonV1::E_REPLAY,
            ReasonV1::E_APPROVAL_EXPIRED,
            ReasonV1::E_REVIEWER_SEPARATION,
            ReasonV1::E_APPROVAL_INVALID
        ])
    );
}

#[test]
fn review_validity_binding_and_representation_are_prerequisites() {
    let (mut request, context, bundle) = review_fixture(|_| {});
    request.payload_digest[0] ^= 1;
    assert_eq!(
        validate_review_artifacts_v1(&request, &context, &bundle),
        Err(vec![ReasonV1::E_BINDING_MISMATCH])
    );
    request.currency = Some("usd".into());
    assert_eq!(
        validate_review_artifacts_v1(&request, &context, &bundle),
        Err(vec![ReasonV1::E_MALFORMED_REQUEST])
    );
}

#[test]
fn review_validity_success_is_not_permission_or_artifact_authentication() {
    let (request, mut context, bundle) = review_fixture(|s| s.enabled = false);
    context.review.grant = None;
    context.review.attestation = None;
    context.review_request = None;
    context.evidence.provenance = ProvenanceStateV1::Invalid;
    assert_eq!(
        validate_review_artifacts_v1(&request, &context, &bundle),
        Ok(())
    );
}

#[test]
fn review_validity_time_high_bits_are_not_truncated() {
    let (request, mut context, bundle) = review_fixture(|s| {
        s.grant_max_lifetime_ms = Some(100_000);
        s.attestation_max_lifetime_ms = Some(100_000);
        s.review_request_timeout_ms = Some(100_000);
    });
    for slot in 0..3 {
        review_interval(&mut context, slot, 0, 100_000);
    }
    context.now_ms = 65_535;
    review_interval(&mut context, 0, 0, 65_536);
    assert_eq!(
        validate_review_artifacts_v1(&request, &context, &bundle),
        Ok(())
    );
    context.now_ms = 65_536;
    assert_eq!(
        validate_review_artifacts_v1(&request, &context, &bundle),
        Err(vec![ReasonV1::E_APPROVAL_EXPIRED])
    );
}

fn provenance_state_cases(state: ProvenanceStateV1, expected: [Vec<ReasonV1>; 3]) {
    for required in [false, true] {
        for attestation_enabled in [false, true] {
            let (request, context, bundle) = review_fixture(|s| {
                s.require_provenance = required;
                s.attestation_enabled = attestation_enabled;
            });
            for (binding_case, reasons) in expected.iter().enumerate() {
                let mut c = context.clone();
                c.evidence.provenance = state;
                match binding_case {
                    0 => {}
                    1 => c.evidence.provenance_binding_digest = None,
                    2 => c.evidence.provenance_binding_digest.as_mut().unwrap()[0] ^= 1,
                    _ => unreachable!(),
                }
                assert_eq!(
                    validate_provenance_and_reviews_v1(&request, &c, &bundle)
                        .map_err(|e| e.causes().to_vec()),
                    if reasons.is_empty() {
                        Ok(state)
                    } else {
                        Err(reasons.clone())
                    },
                    "state {state:?}, binding {binding_case}, required {required}, attestation {attestation_enabled}"
                );
            }
        }
    }
}

#[test]
fn provenance_trusted_requires_present_matching_binding() {
    provenance_state_cases(
        ProvenanceStateV1::TrustedBoundUnused,
        [
            vec![],
            vec![ReasonV1::E_MISSING_EVIDENCE],
            vec![ReasonV1::E_BINDING_MISMATCH],
        ],
    );
}
#[test]
fn provenance_missing_is_preserved_not_upgraded_by_a_digest_or_review() {
    provenance_state_cases(
        ProvenanceStateV1::Missing,
        [vec![], vec![], vec![ReasonV1::E_BINDING_MISMATCH]],
    );
}
#[test]
fn provenance_invalid_is_terminal_even_if_optional_or_attested() {
    provenance_state_cases(
        ProvenanceStateV1::Invalid,
        [
            vec![ReasonV1::E_PROVENANCE_UNVERIFIABLE],
            vec![
                ReasonV1::E_MISSING_EVIDENCE,
                ReasonV1::E_PROVENANCE_UNVERIFIABLE,
            ],
            vec![
                ReasonV1::E_BINDING_MISMATCH,
                ReasonV1::E_PROVENANCE_UNVERIFIABLE,
            ],
        ],
    );
}
#[test]
fn provenance_consumed_is_terminal_even_if_optional_or_attested() {
    provenance_state_cases(
        ProvenanceStateV1::Consumed,
        [
            vec![ReasonV1::E_REPLAY],
            vec![ReasonV1::E_REPLAY, ReasonV1::E_MISSING_EVIDENCE],
            vec![ReasonV1::E_BINDING_MISMATCH, ReasonV1::E_REPLAY],
        ],
    );
}

#[test]
fn provenance_and_reviewer_separation_form_one_combined_check() {
    let (request, mut context, bundle) = review_fixture(|s| s.require_monetary_review = true);
    context.review.attestation.as_mut().unwrap().reviewer_id = "reviewer".into();
    for (state, reason) in [
        (ProvenanceStateV1::Missing, ReasonV1::E_REVIEWER_SEPARATION),
        (
            ProvenanceStateV1::Invalid,
            ReasonV1::E_PROVENANCE_UNVERIFIABLE,
        ),
        (ProvenanceStateV1::Consumed, ReasonV1::E_REPLAY),
    ] {
        context.evidence.provenance = state;
        assert_eq!(
            validate_provenance_and_reviews_v1(&request, &context, &bundle)
                .map_err(|e| e.causes().to_vec()),
            Err(vec![reason])
        );
        if state != ProvenanceStateV1::Missing {
            // The narrower review-only API remains insufficient on its own.
            assert_eq!(
                validate_review_artifacts_v1(&request, &context, &bundle),
                Ok(())
            );
        }
    }
    context.evidence.provenance = ProvenanceStateV1::TrustedBoundUnused;
    assert_eq!(
        validate_provenance_and_reviews_v1(&request, &context, &bundle)
            .map_err(|e| e.causes().to_vec()),
        Ok(ProvenanceStateV1::TrustedBoundUnused)
    );
}

#[test]
fn provenance_keeps_independent_review_defects_sorted_and_deduplicated() {
    let (mut request, mut context, bundle) = review_fixture(|_| {});
    context.evidence.provenance = ProvenanceStateV1::Invalid;
    context.evidence.provenance_binding_digest = None;
    request.resource_id.push('x');
    let grant = context.review.grant.as_mut().unwrap();
    grant.kind = ReviewKindV1::Attestation;
    grant.authorized = false;
    grant.consumed = true;
    grant.expires_at_ms = context.now_ms;
    context.review.attestation.as_mut().unwrap().consumed = true;
    assert_eq!(
        validate_provenance_and_reviews_v1(&request, &context, &bundle)
            .map_err(|e| e.causes().to_vec()),
        Err(vec![
            ReasonV1::E_MALFORMED_REQUEST,
            ReasonV1::E_BINDING_MISMATCH,
            ReasonV1::E_REPLAY,
            ReasonV1::E_MISSING_EVIDENCE,
            ReasonV1::E_PROVENANCE_UNVERIFIABLE,
            ReasonV1::E_APPROVAL_EXPIRED,
            ReasonV1::E_APPROVAL_INVALID,
        ])
    );
    context.evidence.provenance = ProvenanceStateV1::Consumed;
    assert_eq!(
        validate_provenance_and_reviews_v1(&request, &context, &bundle)
            .map_err(|e| e.causes().to_vec()),
        Err(vec![
            ReasonV1::E_MALFORMED_REQUEST,
            ReasonV1::E_BINDING_MISMATCH,
            ReasonV1::E_REPLAY,
            ReasonV1::E_MISSING_EVIDENCE,
            ReasonV1::E_APPROVAL_EXPIRED,
            ReasonV1::E_APPROVAL_INVALID,
        ])
    );
}

#[test]
fn provenance_unrepresentable_input_stops_semantic_collection() {
    let (mut request, mut context, bundle) = review_fixture(|_| {});
    context.evidence.provenance = ProvenanceStateV1::Invalid;
    request.currency = Some("usd".into());
    assert_eq!(
        validate_provenance_and_reviews_v1(&request, &context, &bundle)
            .map_err(|e| e.causes().to_vec()),
        Err(vec![ReasonV1::E_MALFORMED_REQUEST])
    );
    request.currency = None;
    context.now_ms = u64::MAX;
    assert_eq!(
        validate_provenance_and_reviews_v1(&request, &context, &bundle)
            .map_err(|e| e.causes().to_vec()),
        Err(vec![ReasonV1::E_MALFORMED_REQUEST])
    );
}

#[test]
fn provenance_success_is_not_policy_permission_or_proof_of_authentication() {
    let (request, mut context, bundle) = review_fixture(|s| s.enabled = false);
    context.evidence.provenance = ProvenanceStateV1::Missing;
    context.evidence.provenance_binding_digest = None;
    context.review.grant = None;
    context.review.attestation = None;
    context.review_request = None;
    assert_eq!(
        validate_provenance_and_reviews_v1(&request, &context, &bundle)
            .map_err(|e| e.causes().to_vec()),
        Ok(ProvenanceStateV1::Missing)
    );
}

#[test]
fn terminal_error_invalid_retains_state_despite_escalatable_reason() {
    for required in [false, true] {
        for attestation_enabled in [false, true] {
            let (request, mut context, bundle) = review_fixture(|s| {
                s.require_provenance = required;
                s.attestation_enabled = attestation_enabled;
            });
            context.evidence.provenance = ProvenanceStateV1::Invalid;
            let error =
                validate_provenance_and_reviews_v1(&request, &context, &bundle).unwrap_err();
            assert_eq!(error.provenance(), ProvenanceStateV1::Invalid);
            assert_eq!(error.causes(), vec![ReasonV1::E_PROVENANCE_UNVERIFIABLE]);
            // Registry permission is not outcome selection. A future router must
            // honor this state-derived veto even when the sole reason allows review.
            assert!(error.requires_terminal_deny());
        }
    }
}

#[test]
fn terminal_error_consumed_retains_state_and_veto() {
    let (request, mut context, bundle) = review_fixture(|_| {});
    context.evidence.provenance = ProvenanceStateV1::Consumed;
    let error = validate_provenance_and_reviews_v1(&request, &context, &bundle).unwrap_err();
    assert_eq!(error.provenance(), ProvenanceStateV1::Consumed);
    assert_eq!(error.causes(), vec![ReasonV1::E_REPLAY]);
    assert!(error.requires_terminal_deny());
}

#[test]
fn terminal_error_nonterminal_state_is_not_reclassified_by_other_defects() {
    let (request, mut context, bundle) = review_fixture(|_| {});
    context.review.grant.as_mut().unwrap().consumed = true;
    for state in [
        ProvenanceStateV1::Missing,
        ProvenanceStateV1::TrustedBoundUnused,
    ] {
        context.evidence.provenance = state;
        let error = validate_provenance_and_reviews_v1(&request, &context, &bundle).unwrap_err();
        assert_eq!(error.provenance(), state);
        assert_eq!(error.causes(), vec![ReasonV1::E_REPLAY]);
        // The replay cause still demands Deny; false only means that the
        // provenance state itself is not Invalid or Consumed.
        assert!(!error.requires_terminal_deny());
    }
}

#[test]
fn terminal_error_representation_boundary_matches_binding_validator() {
    for state in [
        ProvenanceStateV1::TrustedBoundUnused,
        ProvenanceStateV1::Missing,
        ProvenanceStateV1::Invalid,
        ProvenanceStateV1::Consumed,
    ] {
        for malformed in 0..6 {
            let (mut request, mut context, bundle) = review_fixture(|_| {});
            context.evidence.provenance = state;
            match malformed {
                0 => request.profile_id.clear(),
                1 => request.currency = Some("usd".into()),
                2 => request.resource_id = "x".repeat(129),
                3 => context.now_ms = u64::MAX,
                4 => context.authenticated_subject.clear(),
                5 => context.review.grant.as_mut().unwrap().reviewer_id.clear(),
                _ => unreachable!(),
            }
            assert_eq!(
                validate_request_bindings_v1(&request, &context, &bundle),
                Err(ReasonV1::E_MALFORMED_REQUEST),
                "case {malformed}"
            );
            let error =
                validate_provenance_and_reviews_v1(&request, &context, &bundle).unwrap_err();
            assert_eq!(error.provenance(), state, "case {malformed}");
            assert_eq!(
                error.causes(),
                vec![ReasonV1::E_MALFORMED_REQUEST],
                "case {malformed}"
            );
            assert_eq!(
                error.requires_terminal_deny(),
                matches!(
                    state,
                    ProvenanceStateV1::Invalid | ProvenanceStateV1::Consumed
                )
            );
        }
    }
}

#[test]
fn terminal_error_representable_fact_keeps_state_and_multiple_causes() {
    let (request, mut context, bundle) = review_fixture(|_| {});
    context.evidence.provenance = ProvenanceStateV1::Invalid;
    context.review.grant.as_mut().unwrap().kind = ReviewKindV1::Attestation;
    assert_eq!(
        validate_request_bindings_v1(&request, &context, &bundle),
        Ok(())
    );
    let error = validate_provenance_and_reviews_v1(&request, &context, &bundle).unwrap_err();
    assert_eq!(error.provenance(), ProvenanceStateV1::Invalid);
    assert_eq!(
        error.causes(),
        vec![
            ReasonV1::E_MALFORMED_REQUEST,
            ReasonV1::E_PROVENANCE_UNVERIFIABLE
        ]
    );
    assert!(error.requires_terminal_deny());
}

#[test]
fn terminal_error_public_accessors_preserve_validated_value() {
    let (request, mut context, bundle) = review_fixture(|_| {});
    context.evidence.provenance = ProvenanceStateV1::Invalid;
    let error = validate_provenance_and_reviews_v1(&request, &context, &bundle).unwrap_err();
    let clone = error.clone();
    let mut copied_state = error.provenance();
    assert_eq!(copied_state, ProvenanceStateV1::Invalid);
    copied_state = ProvenanceStateV1::Missing;
    let mut copied_causes = error.causes().to_vec();
    copied_causes.clear();
    assert_eq!(copied_state, ProvenanceStateV1::Missing);
    assert!(copied_causes.is_empty());
    assert_eq!(error, clone);
    assert_eq!(error.provenance(), ProvenanceStateV1::Invalid);
    assert_eq!(error.causes(), &[ReasonV1::E_PROVENANCE_UNVERIFIABLE]);
    assert!(error.requires_terminal_deny());
    assert!(clone.requires_terminal_deny());
}
