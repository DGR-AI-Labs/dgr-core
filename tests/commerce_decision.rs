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
        assert_eq!(
            prep_error(&source),
            BundleErrorV1::UnsupportedFeature,
            "{expr}"
        );
    }
    let mut source = preparation_source();
    source.permissions_text.push_str("permit(principal == HermesCommerce::Principal::\"never\", action, resource) when { decimal(\"1.0\") == decimal(\"1.0\") };");
    assert_eq!(prep_error(&source), BundleErrorV1::UnsupportedFeature);
    source.permissions_text = "permit(principal == ?principal, action, resource);".into();
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
