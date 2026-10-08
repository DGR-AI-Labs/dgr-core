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
        let (bundle, context) = if action == CommerceActionV1::CustomerEmailSend {
            let b = email_migration_bundle(bundle.clone(), action);
            let mut c = context.clone();
            email_rebind_policy(&request, &mut c, &b);
            (b, c)
        } else {
            (bundle.clone(), context.clone())
        };
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

fn evidence_fixture(
    fields: Vec<(EvidenceFieldV1, u64)>,
) -> (
    CommerceRequestV1,
    DecisionContextV1,
    PreparedCommerceBundleV1,
) {
    let (r, mut c, b) = review_fixture(|s| s.required_evidence = fields.clone());
    c.now_ms = 100;
    c.evidence.fetched_at_ms = 90;
    c.approved_evidence_revision = None;
    c.evidence.captured_minor = Some(0);
    c.evidence.prior_refunds_minor = Some(0);
    c.evidence.order_age_seconds = Some(0);
    c.evidence.line_items_eligible = Some(false);
    c.evidence.any_fulfillment = Some(false);
    c.evidence.cancellation_eligible = Some(false);
    c.evidence.discount_conflict = Some(false);
    c.evidence.derived_recipient_digest = Some([0; 32]);
    c.evidence.approved_template_digest = Some([0; 32]);
    c.evidence.recipient_count = Some(0);
    (r, c, b)
}

#[test]
fn evidence_snapshot_every_field_age_boundary() {
    use EvidenceFieldV1::*;
    for field in [
        SourceRevision,
        FetchedAtMs,
        EvidenceDigest,
        CapturedMinor,
        PriorRefundsMinor,
        OrderAgeSeconds,
        LineItemsEligible,
        AnyFulfillment,
        CancellationEligible,
        DiscountConflict,
        DerivedRecipientDigest,
        ApprovedTemplateDigest,
        RecipientCount,
        Provenance,
        ProvenanceBindingDigest,
    ] {
        let (r, mut c, b) = evidence_fixture(vec![(field, 10)]);
        for (now, expected) in [
            (99, Ok(())),
            (100, Ok(())),
            (101, Err(vec![ReasonV1::E_MISSING_EVIDENCE])),
        ] {
            c.now_ms = now;
            assert_eq!(
                validate_evidence_snapshot_v1(&r, &c, &b),
                expected,
                "{field:?}: {now}"
            );
        }
    }
}

#[test]
fn evidence_snapshot_optional_presence_false_zero_are_present() {
    use EvidenceFieldV1::*;
    for field in [
        CapturedMinor,
        PriorRefundsMinor,
        OrderAgeSeconds,
        LineItemsEligible,
        AnyFulfillment,
        CancellationEligible,
        DiscountConflict,
        DerivedRecipientDigest,
        ApprovedTemplateDigest,
        RecipientCount,
        ProvenanceBindingDigest,
    ] {
        let (r, mut c, b) = evidence_fixture(vec![(field, 10)]);
        assert_eq!(
            validate_evidence_snapshot_v1(&r, &c, &b),
            Ok(()),
            "present {field:?}"
        );
        match field {
            CapturedMinor => c.evidence.captured_minor = None,
            PriorRefundsMinor => c.evidence.prior_refunds_minor = None,
            OrderAgeSeconds => c.evidence.order_age_seconds = None,
            LineItemsEligible => c.evidence.line_items_eligible = None,
            AnyFulfillment => c.evidence.any_fulfillment = None,
            CancellationEligible => c.evidence.cancellation_eligible = None,
            DiscountConflict => c.evidence.discount_conflict = None,
            DerivedRecipientDigest => c.evidence.derived_recipient_digest = None,
            ApprovedTemplateDigest => c.evidence.approved_template_digest = None,
            RecipientCount => c.evidence.recipient_count = None,
            ProvenanceBindingDigest => c.evidence.provenance_binding_digest = None,
            _ => unreachable!(),
        }
        assert_eq!(
            validate_evidence_snapshot_v1(&r, &c, &b),
            Err(vec![ReasonV1::E_MISSING_EVIDENCE]),
            "absent {field:?}"
        );
    }
}

#[test]
fn evidence_snapshot_future_and_unsigned_extremes() {
    for fields in [vec![], vec![(EvidenceFieldV1::FetchedAtMs, 10)]] {
        let (r, mut c, b) = evidence_fixture(fields);
        for now in [0, 100, i64::MAX as u64 - 1] {
            c.now_ms = now;
            c.evidence.fetched_at_ms = now;
            assert_eq!(validate_evidence_snapshot_v1(&r, &c, &b), Ok(()));
            c.evidence.fetched_at_ms = now + 1;
            assert_eq!(
                validate_evidence_snapshot_v1(&r, &c, &b),
                Err(vec![ReasonV1::E_MALFORMED_REQUEST])
            );
        }
        c.evidence.fetched_at_ms = u64::MAX;
        assert_eq!(
            validate_evidence_snapshot_v1(&r, &c, &b),
            Err(vec![ReasonV1::E_MALFORMED_REQUEST])
        );
    }
}

#[test]
fn evidence_snapshot_revision_is_exact_and_optional() {
    let (r, mut c, b) = evidence_fixture(vec![]);
    assert_eq!(validate_evidence_snapshot_v1(&r, &c, &b), Ok(()));
    c.approved_evidence_revision = Some(c.evidence.source_revision.clone());
    assert_eq!(validate_evidence_snapshot_v1(&r, &c, &b), Ok(()));
    c.approved_evidence_revision.as_mut().unwrap().push('x');
    assert_eq!(
        validate_evidence_snapshot_v1(&r, &c, &b),
        Err(vec![ReasonV1::E_STALE_STATE])
    );
}

#[test]
fn evidence_snapshot_causes_are_complete_ordered_unique() {
    let (r, mut c, b) = evidence_fixture(vec![
        (EvidenceFieldV1::CapturedMinor, 10),
        (EvidenceFieldV1::RecipientCount, 10),
    ]);
    c.evidence.captured_minor = None;
    c.evidence.recipient_count = None;
    c.evidence.fetched_at_ms = 101;
    c.approved_evidence_revision = Some("other".into());
    assert_eq!(
        validate_evidence_snapshot_v1(&r, &c, &b),
        Err(vec![
            ReasonV1::E_MALFORMED_REQUEST,
            ReasonV1::E_MISSING_EVIDENCE,
            ReasonV1::E_STALE_STATE
        ])
    );
    c.evidence.source_revision.clear();
    assert_eq!(
        validate_evidence_snapshot_v1(&r, &c, &b),
        Err(vec![ReasonV1::E_MALFORMED_REQUEST])
    );
}

#[test]
fn evidence_snapshot_uses_selected_action_and_individual_maximum() {
    let mut source = preparation_source();
    source.schema_text = NATIVE_SCHEMA.into();
    for (action, s) in &mut source.action_settings {
        s.required_evidence = if *action == CommerceActionV1::RefundCreate {
            vec![
                (EvidenceFieldV1::CapturedMinor, 20),
                (EvidenceFieldV1::RecipientCount, 5),
            ]
        } else {
            vec![]
        };
    }
    recommit(&mut source);
    let b = prepare_bundle_v1(&source, &[7; 32]).unwrap();
    let (mut r, mut c, _) = evidence_fixture(vec![]);
    assert_eq!(
        validate_evidence_snapshot_v1(&r, &c, &b),
        Err(vec![ReasonV1::E_MISSING_EVIDENCE])
    );
    c.evidence.fetched_at_ms = 95;
    assert_eq!(validate_evidence_snapshot_v1(&r, &c, &b), Ok(()));
    c.evidence.fetched_at_ms = 0;
    r.action = CommerceActionV1::OrderCancel;
    assert_eq!(validate_evidence_snapshot_v1(&r, &c, &b), Ok(()));
}

#[test]
fn evidence_snapshot_success_is_not_permission_or_digest_authentication() {
    let (r, mut c, b) = evidence_fixture(vec![]);
    c.evidence.provenance = ProvenanceStateV1::Invalid;
    c.evidence.evidence_digest = [42; 32];
    c.evidence.provenance_binding_digest = None;
    c.review.grant.as_mut().unwrap().authorized = false;
    c.authenticated_subject = "different".into();
    assert_eq!(validate_evidence_snapshot_v1(&r, &c, &b), Ok(()));
}

#[test]
fn evidence_snapshot_age_does_not_truncate_and_zero_limit_is_inclusive() {
    let (r, mut c, b) = evidence_fixture(vec![(EvidenceFieldV1::FetchedAtMs, 10)]);
    c.evidence.fetched_at_ms = 0;
    c.now_ms = 65546;
    assert_eq!(
        validate_evidence_snapshot_v1(&r, &c, &b),
        Err(vec![ReasonV1::E_MISSING_EVIDENCE])
    );
    let (r, mut c, b) = evidence_fixture(vec![(EvidenceFieldV1::FetchedAtMs, 0)]);
    c.evidence.fetched_at_ms = c.now_ms;
    assert_eq!(validate_evidence_snapshot_v1(&r, &c, &b), Ok(()));
    c.now_ms += 1;
    assert_eq!(
        validate_evidence_snapshot_v1(&r, &c, &b),
        Err(vec![ReasonV1::E_MISSING_EVIDENCE])
    );
}

#[test]
fn evidence_snapshot_unrequired_absence_and_request_bounds() {
    let (mut r, mut c, b) = evidence_fixture(vec![]);
    c.evidence.captured_minor = None;
    c.evidence.fetched_at_ms = 0;
    assert_eq!(validate_evidence_snapshot_v1(&r, &c, &b), Ok(()));
    r.operation_id.clear();
    assert_eq!(
        validate_evidence_snapshot_v1(&r, &c, &b),
        Err(vec![ReasonV1::E_MALFORMED_REQUEST])
    );
}

fn eligibility_frame(action: CommerceActionV1) -> (CommerceRequestV1, DecisionContextV1) {
    let (mut r, mut c, _) = frame();
    r.action = action;
    c.evidence.line_items_eligible = Some(true);
    c.evidence.any_fulfillment = Some(false);
    c.evidence.cancellation_eligible = Some(true);
    c.evidence.recipient_count = Some(1);
    (r, c)
}

macro_rules! eligibility_boolean_cases {
    ($name:ident,$action:ident,$field:ident,$good:expr) => {
        #[test]
        fn $name() {
            let (r, mut c) = eligibility_frame(CommerceActionV1::$action);
            for (value, expected) in [
                (Some($good), Ok(())),
                (Some(!$good), Err(ReasonV1::E_CONSTRAINT_VIOLATION)),
                (None, Err(ReasonV1::E_MISSING_EVIDENCE)),
            ] {
                c.evidence.$field = value;
                assert_eq!(
                    validate_action_eligibility_v1(&r, &c),
                    expected,
                    "{value:?}"
                );
            }
        }
    };
}
eligibility_boolean_cases!(
    action_eligibility_refund,
    RefundCreate,
    line_items_eligible,
    true
);
eligibility_boolean_cases!(
    action_eligibility_address,
    OrderAddressUpdate,
    any_fulfillment,
    false
);
eligibility_boolean_cases!(
    action_eligibility_cancel,
    OrderCancel,
    cancellation_eligible,
    true
);

#[test]
fn action_eligibility_recipient_count_exact() {
    let (r, mut c) = eligibility_frame(CommerceActionV1::CustomerEmailSend);
    for value in [0, 1, 2, 65537, u32::MAX] {
        c.evidence.recipient_count = Some(value);
        assert_eq!(
            validate_action_eligibility_v1(&r, &c),
            if value == 1 {
                Ok(())
            } else {
                Err(ReasonV1::E_CONSTRAINT_VIOLATION)
            },
            "{value}"
        );
    }
    c.evidence.recipient_count = None;
    assert_eq!(
        validate_action_eligibility_v1(&r, &c),
        Err(ReasonV1::E_MISSING_EVIDENCE)
    );
}

macro_rules! eligibility_off_target {
    ($name:ident,$target:ident,$field:ident,$bad:expr) => {
        #[test]
        fn $name() {
            for action in [
                CommerceActionV1::RefundCreate,
                CommerceActionV1::OrderAddressUpdate,
                CommerceActionV1::OrderCancel,
                CommerceActionV1::DiscountCreate,
                CommerceActionV1::CustomerEmailSend,
            ] {
                if action == CommerceActionV1::$target {
                    continue;
                }
                let (r, mut c) = eligibility_frame(action);
                assert_eq!(validate_action_eligibility_v1(&r, &c), Ok(()));
                c.evidence.$field = Some($bad);
                assert_eq!(
                    validate_action_eligibility_v1(&r, &c),
                    Ok(()),
                    "off-target {action:?}"
                );
                c.evidence.$field = None;
                assert_eq!(
                    validate_action_eligibility_v1(&r, &c),
                    Ok(()),
                    "unrequired absence {action:?}"
                );
            }
        }
    };
}
eligibility_off_target!(
    action_eligibility_refund_scope,
    RefundCreate,
    line_items_eligible,
    false
);
eligibility_off_target!(
    action_eligibility_address_scope,
    OrderAddressUpdate,
    any_fulfillment,
    true
);
eligibility_off_target!(
    action_eligibility_cancel_scope,
    OrderCancel,
    cancellation_eligible,
    false
);
eligibility_off_target!(
    action_eligibility_zero_recipient_scope,
    CustomerEmailSend,
    recipient_count,
    0
);
eligibility_off_target!(
    action_eligibility_two_recipient_scope,
    CustomerEmailSend,
    recipient_count,
    2
);

#[test]
fn action_eligibility_request_and_context_representation() {
    let (mut r, mut c) = eligibility_frame(CommerceActionV1::DiscountCreate);
    r.operation_id.clear();
    assert_eq!(
        validate_action_eligibility_v1(&r, &c),
        Err(ReasonV1::E_MALFORMED_REQUEST)
    );
    r.operation_id = "operation".into();
    c.now_ms = u64::MAX;
    assert_eq!(
        validate_action_eligibility_v1(&r, &c),
        Err(ReasonV1::E_MALFORMED_REQUEST)
    );
}

#[test]
fn action_eligibility_reviews_do_not_override_and_success_is_not_permission() {
    let b = native_bundle(true);
    let (mut r, mut c) = bound_frame(&b);
    r.action = CommerceActionV1::RefundCreate;
    c.evidence.line_items_eligible = Some(false);
    assert!(c.review.grant.is_some() && c.review.attestation.is_some());
    assert_eq!(
        validate_action_eligibility_v1(&r, &c),
        Err(ReasonV1::E_CONSTRAINT_VIOLATION)
    );
    c.evidence.line_items_eligible = Some(true);
    c.evidence.provenance = ProvenanceStateV1::Invalid;
    c.evidence.provenance_binding_digest = None;
    c.authenticated_subject = "other".into();
    assert_eq!(validate_action_eligibility_v1(&r, &c), Ok(()));
}

fn monetary_fixture(
    action: CommerceActionV1,
    ceiling: i64,
) -> (
    CommerceRequestV1,
    DecisionContextV1,
    PreparedCommerceBundleV1,
) {
    let mut source = preparation_source();
    for (_, settings) in &mut source.action_settings {
        settings.amount_ceiling_minor = Some(ceiling);
    }
    recommit(&mut source);
    let bundle = prepare_bundle_v1(&source, &[7; 32]).unwrap();
    let (mut request, mut context) = bound_frame(&bundle);
    request.action = action;
    request.amount_minor = Some(1000);
    request.currency = Some("USD".into());
    context.evidence.captured_minor = Some(10000);
    context.evidence.prior_refunds_minor = Some(2000);
    let bundle = email_migration_bundle(bundle, action);
    email_rebind_policy(&request, &mut context, &bundle);
    (request, context, bundle)
}

fn monetary_check(
    r: &CommerceRequestV1,
    c: &DecisionContextV1,
    b: &PreparedCommerceBundleV1,
    causes: &[ReasonV1],
) {
    assert_eq!(
        validate_monetary_constraints_v1(r, c, b),
        if causes.is_empty() {
            Ok(())
        } else {
            Err(causes.to_vec())
        }
    );
}

macro_rules! monetary_amount_cases {
    ($name:ident, $action:ident) => {
        #[test]
        fn $name() {
            let (mut r, c, b) = monetary_fixture(CommerceActionV1::$action, 1000);
            for value in [0, 999, 1000, 1001, 65537, i64::MAX] {
                r.amount_minor = Some(value);
                monetary_check(
                    &r,
                    &c,
                    &b,
                    if value > 1000 {
                        &[ReasonV1::E_AMOUNT_LIMIT]
                    } else {
                        &[]
                    },
                );
            }
        }
    };
}
monetary_amount_cases!(monetary_refund_amount_ceiling, RefundCreate);
monetary_amount_cases!(monetary_discount_amount_ceiling, DiscountCreate);

macro_rules! monetary_input_cases {
    ($name:ident, $action:ident, $field:ident) => {
        #[test]
        fn $name() {
            let (mut r, c, b) = monetary_fixture(CommerceActionV1::$action, 5000);
            monetary_check(&r, &c, &b, &[]);
            r.$field = None;
            monetary_check(&r, &c, &b, &[ReasonV1::E_UNDERSPECIFIED_ACTION]);
        }
    };
}
monetary_input_cases!(monetary_refund_amount_required, RefundCreate, amount_minor);
monetary_input_cases!(monetary_refund_currency_required, RefundCreate, currency);
monetary_input_cases!(
    monetary_discount_amount_required,
    DiscountCreate,
    amount_minor
);
monetary_input_cases!(
    monetary_discount_currency_required,
    DiscountCreate,
    currency
);

#[test]
fn monetary_refund_captured_required_without_configured_requirement() {
    let (r, mut c, b) = monetary_fixture(CommerceActionV1::RefundCreate, 5000);
    assert!(
        b.action_settings()[0]
            .1
            .required_evidence
            .iter()
            .all(|(f, _)| *f != EvidenceFieldV1::CapturedMinor)
    );
    c.evidence.captured_minor = None;
    monetary_check(&r, &c, &b, &[ReasonV1::E_MISSING_EVIDENCE]);
}

#[test]
fn monetary_refund_prior_refunds_required_without_configured_requirement() {
    let (r, mut c, b) = monetary_fixture(CommerceActionV1::RefundCreate, 5000);
    assert!(
        b.action_settings()[0]
            .1
            .required_evidence
            .iter()
            .all(|(f, _)| *f != EvidenceFieldV1::PriorRefundsMinor)
    );
    c.evidence.prior_refunds_minor = None;
    monetary_check(&r, &c, &b, &[ReasonV1::E_MISSING_EVIDENCE]);
}

#[test]
fn monetary_refund_net_balance_boundary() {
    let (mut r, c, b) = monetary_fixture(CommerceActionV1::RefundCreate, 10000);
    for amount in [7999, 8000, 8001] {
        r.amount_minor = Some(amount);
        monetary_check(
            &r,
            &c,
            &b,
            if amount > 8000 {
                &[ReasonV1::E_AMOUNT_LIMIT]
            } else {
                &[]
            },
        );
    }
}

#[test]
fn monetary_refund_captured_operand_is_used() {
    let (r, mut c, b) = monetary_fixture(CommerceActionV1::RefundCreate, 5000);
    c.evidence.prior_refunds_minor = Some(0);
    for captured in [1001, 1000, 999] {
        c.evidence.captured_minor = Some(captured);
        monetary_check(
            &r,
            &c,
            &b,
            if captured < 1000 {
                &[ReasonV1::E_AMOUNT_LIMIT]
            } else {
                &[]
            },
        );
    }
}

#[test]
fn monetary_refund_prior_refunds_operand_is_used() {
    let (r, mut c, b) = monetary_fixture(CommerceActionV1::RefundCreate, 5000);
    for prior in [8999, 9000, 9001] {
        c.evidence.prior_refunds_minor = Some(prior);
        monetary_check(
            &r,
            &c,
            &b,
            if prior > 9000 {
                &[ReasonV1::E_AMOUNT_LIMIT]
            } else {
                &[]
            },
        );
    }
}

#[test]
fn monetary_discount_does_not_require_refund_balance() {
    let (r, mut c, b) = monetary_fixture(CommerceActionV1::DiscountCreate, 5000);
    c.evidence.captured_minor = None;
    c.evidence.prior_refunds_minor = None;
    monetary_check(&r, &c, &b, &[]);
    c.evidence.captured_minor = Some(0);
    c.evidence.prior_refunds_minor = Some(i64::MAX);
    monetary_check(&r, &c, &b, &[]);
}

macro_rules! monetary_off_target {
    ($name:ident, $action:ident) => {
        #[test]
        fn $name() {
            let (mut r, mut c, b) = monetary_fixture(CommerceActionV1::$action, 0);
            r.amount_minor = Some(i64::MAX);
            r.currency = Some("ZZZ".into());
            c.evidence.captured_minor = Some(0);
            c.evidence.prior_refunds_minor = Some(i64::MAX);
            monetary_check(&r, &c, &b, &[]);
            r.amount_minor = None;
            r.currency = None;
            c.evidence.captured_minor = None;
            c.evidence.prior_refunds_minor = None;
            monetary_check(&r, &c, &b, &[]);
        }
    };
}
monetary_off_target!(monetary_address_scope, OrderAddressUpdate);
monetary_off_target!(monetary_cancel_scope, OrderCancel);
monetary_off_target!(monetary_email_scope, CustomerEmailSend);

#[test]
fn monetary_exact_i64_max_and_zero_balance() {
    for action in [
        CommerceActionV1::RefundCreate,
        CommerceActionV1::DiscountCreate,
    ] {
        let (mut r, mut c, b) = monetary_fixture(action, i64::MAX);
        r.amount_minor = Some(i64::MAX);
        c.evidence.captured_minor = Some(i64::MAX);
        c.evidence.prior_refunds_minor = Some(0);
        monetary_check(&r, &c, &b, &[]);
        if action == CommerceActionV1::RefundCreate {
            c.evidence.prior_refunds_minor = Some(1);
            monetary_check(&r, &c, &b, &[ReasonV1::E_AMOUNT_LIMIT]);
            r.amount_minor = Some(0);
            c.evidence.prior_refunds_minor = Some(i64::MAX);
            monetary_check(&r, &c, &b, &[]);
            r.amount_minor = Some(1);
            monetary_check(&r, &c, &b, &[ReasonV1::E_AMOUNT_LIMIT]);
        }
    }
}

#[test]
fn monetary_representation_precedes_semantics() {
    let (r, c, b) = monetary_fixture(CommerceActionV1::RefundCreate, 5000);
    for field in 0..6 {
        let (mut r, mut c) = (r.clone(), c.clone());
        match field {
            0 => r.amount_minor = Some(-1),
            1 => r.currency = Some("usd".into()),
            2 => c.evidence.captured_minor = Some(-1),
            3 => c.evidence.prior_refunds_minor = Some(-1),
            4 => r.operation_id.clear(),
            _ => c.now_ms = u64::MAX,
        }
        monetary_check(&r, &c, &b, &[ReasonV1::E_MALFORMED_REQUEST]);
    }
}

#[test]
fn monetary_missing_causes_are_complete_and_deduplicated() {
    let (mut r, mut c, b) = monetary_fixture(CommerceActionV1::RefundCreate, 5000);
    r.amount_minor = None;
    r.currency = None;
    c.evidence.captured_minor = None;
    c.evidence.prior_refunds_minor = None;
    monetary_check(
        &r,
        &c,
        &b,
        &[
            ReasonV1::E_UNDERSPECIFIED_ACTION,
            ReasonV1::E_MISSING_EVIDENCE,
        ],
    );
}

#[test]
fn monetary_reviews_do_not_override_amount_limit() {
    let (mut r, c, b) = monetary_fixture(CommerceActionV1::RefundCreate, 5000);
    assert!(c.review.grant.is_some() && c.review.attestation.is_some());
    r.amount_minor = Some(5001);
    monetary_check(&r, &c, &b, &[ReasonV1::E_AMOUNT_LIMIT]);
}

#[test]
fn monetary_success_is_not_permission_or_budget_validation() {
    let (r, mut c, b) = monetary_fixture(CommerceActionV1::RefundCreate, 5000);
    c.authenticated_subject = "different".into();
    c.evidence.provenance = ProvenanceStateV1::Invalid;
    c.evidence.provenance_binding_digest = None;
    c.evidence.fetched_at_ms = 0;
    c.budget.subject_id = "another".into();
    c.budget.currency = Some("EUR".into());
    c.budget.count_used = i64::MAX;
    c.budget.value_used_minor = i64::MAX;
    monetary_check(&r, &c, &b, &[]);
}

fn monetary_edited_bundle(
    edit: impl Fn(CommerceActionV1, &mut CommerceActionSettingsV1),
) -> PreparedCommerceBundleV1 {
    let mut source = preparation_source();
    for (action, settings) in &mut source.action_settings {
        edit(*action, settings);
    }
    recommit(&mut source);
    prepare_bundle_v1(&source, &[7; 32]).unwrap()
}

macro_rules! monetary_currency_cases {
    ($name:ident, $action:ident) => {
        #[test]
        fn $name() {
            let (mut r, c, b) = monetary_fixture(CommerceActionV1::$action, 5000);
            monetary_check(&r, &c, &b, &[]);
            r.currency = Some("EUR".into());
            monetary_check(&r, &c, &b, &[ReasonV1::E_CONSTRAINT_VIOLATION]);
            r.currency = Some("ZZZ".into());
            monetary_check(&r, &c, &b, &[ReasonV1::E_CONSTRAINT_VIOLATION]);
            let b = monetary_edited_bundle(|_, s| s.currencies.push("EUR".into()));
            r.currency = Some("EUR".into());
            monetary_check(&r, &c, &b, &[]);
        }
    };
}
monetary_currency_cases!(monetary_refund_currency_membership, RefundCreate);
monetary_currency_cases!(monetary_discount_currency_membership, DiscountCreate);

#[test]
fn monetary_refund_conflicting_balance_is_not_zero() {
    let (mut r, mut c, b) = monetary_fixture(CommerceActionV1::RefundCreate, 0);
    r.amount_minor = Some(0);
    for (captured, prior) in [(1, 2), (0, i64::MAX), (i64::MAX - 1, i64::MAX)] {
        c.evidence.captured_minor = Some(captured);
        c.evidence.prior_refunds_minor = Some(prior);
        monetary_check(&r, &c, &b, &[ReasonV1::E_EVIDENCE_CONFLICT]);
    }
    c.evidence.captured_minor = Some(i64::MAX);
    c.evidence.prior_refunds_minor = Some(i64::MAX);
    monetary_check(&r, &c, &b, &[]);
}

#[test]
fn monetary_action_selects_its_own_settings() {
    let (mut r, c, _) = monetary_fixture(CommerceActionV1::RefundCreate, 5000);
    let b = monetary_edited_bundle(|action, s| {
        if action == CommerceActionV1::RefundCreate {
            s.amount_ceiling_minor = Some(1000);
        } else if action == CommerceActionV1::DiscountCreate {
            s.amount_ceiling_minor = Some(2000);
            s.currencies = vec!["EUR".into()];
        }
    });
    monetary_check(&r, &c, &b, &[]);
    r.action = CommerceActionV1::DiscountCreate;
    r.amount_minor = Some(2000);
    r.currency = Some("EUR".into());
    monetary_check(&r, &c, &b, &[]);
    r.action = CommerceActionV1::RefundCreate;
    monetary_check(
        &r,
        &c,
        &b,
        &[ReasonV1::E_AMOUNT_LIMIT, ReasonV1::E_CONSTRAINT_VIOLATION],
    );
}

#[test]
fn monetary_missing_ceiling_never_defaults_to_unlimited() {
    for action in [
        CommerceActionV1::RefundCreate,
        CommerceActionV1::DiscountCreate,
    ] {
        let (r, c, _) = monetary_fixture(action, 5000);
        let b = monetary_edited_bundle(|_, s| {
            s.enabled = false;
            s.amount_ceiling_minor = None;
        });
        monetary_check(&r, &c, &b, &[ReasonV1::E_POLICY_UNCONFIGURED]);
    }
}

#[test]
fn monetary_empty_currency_set_never_defaults_to_any_currency() {
    for action in [
        CommerceActionV1::RefundCreate,
        CommerceActionV1::DiscountCreate,
    ] {
        let (r, c, _) = monetary_fixture(action, 5000);
        let b = monetary_edited_bundle(|_, s| {
            s.enabled = false;
            s.currencies.clear();
        });
        monetary_check(&r, &c, &b, &[ReasonV1::E_POLICY_UNCONFIGURED]);
    }
}

#[test]
fn monetary_disabled_settings_classify_without_enabling() {
    let (mut r, c, _) = monetary_fixture(CommerceActionV1::DiscountCreate, 5000);
    let b = monetary_edited_bundle(|_, s| {
        s.enabled = false;
        s.require_monetary_review = false;
        s.required_evidence.clear();
    });
    monetary_check(&r, &c, &b, &[]);
    r.amount_minor = None;
    monetary_check(&r, &c, &b, &[ReasonV1::E_UNDERSPECIFIED_ACTION]);
}

#[test]
fn monetary_causes_are_complete_ordered_and_deduplicated() {
    let (mut r, mut c, b) = monetary_fixture(CommerceActionV1::RefundCreate, 1000);
    r.amount_minor = Some(1001);
    r.currency = Some("ZZZ".into());
    c.evidence.captured_minor = Some(1);
    c.evidence.prior_refunds_minor = Some(2);
    monetary_check(
        &r,
        &c,
        &b,
        &[
            ReasonV1::E_EVIDENCE_CONFLICT,
            ReasonV1::E_AMOUNT_LIMIT,
            ReasonV1::E_CONSTRAINT_VIOLATION,
        ],
    );
    c.evidence.prior_refunds_minor = Some(0);
    monetary_check(
        &r,
        &c,
        &b,
        &[ReasonV1::E_AMOUNT_LIMIT, ReasonV1::E_CONSTRAINT_VIOLATION],
    );
    let b = monetary_edited_bundle(|_, s| {
        s.enabled = false;
        s.amount_ceiling_minor = None;
        s.currencies.clear();
    });
    r.amount_minor = None;
    r.currency = None;
    c.evidence.captured_minor = None;
    c.evidence.prior_refunds_minor = None;
    monetary_check(
        &r,
        &c,
        &b,
        &[
            ReasonV1::E_UNDERSPECIFIED_ACTION,
            ReasonV1::E_MISSING_EVIDENCE,
            ReasonV1::E_POLICY_UNCONFIGURED,
        ],
    );
}

fn budget_fixture(
    action: CommerceActionV1,
) -> (
    CommerceRequestV1,
    DecisionContextV1,
    PreparedCommerceBundleV1,
) {
    let b = prepare_bundle_v1(&preparation_source(), &[7; 32]).unwrap();
    let (mut r, mut c) = bound_frame(&b);
    r.action = action;
    if matches!(
        action,
        CommerceActionV1::RefundCreate | CommerceActionV1::DiscountCreate
    ) {
        r.amount_minor = Some(1000);
        r.currency = Some("USD".into());
    } else {
        r.amount_minor = None;
        r.currency = None;
    }
    c.now_ms = 1000;
    c.budget = CommerceBudgetV1 {
        profile_id: r.profile_id.clone(),
        shop_id: r.shop_id.clone(),
        subject_id: r.subject_id.clone(),
        action,
        currency: r.currency.clone(),
        window_start_ms: 0,
        window_length_ms: 60000,
        count_used: 1,
        value_used_minor: 2000,
        revision: "budget-revision".into(),
    };
    let b = email_migration_bundle(b, action);
    email_rebind_policy(&r, &mut c, &b);
    (r, c, b)
}

fn budget_check(
    r: &CommerceRequestV1,
    c: &DecisionContextV1,
    b: &PreparedCommerceBundleV1,
    causes: &[ReasonV1],
) {
    assert_eq!(
        validate_budget_constraints_v1(r, c, b),
        if causes.is_empty() {
            Ok(())
        } else {
            Err(causes.to_vec())
        }
    );
}

macro_rules! budget_count_cases {
    ($name:ident, $action:ident) => {
        #[test]
        fn $name() {
            let (r, mut c, b) = budget_fixture(CommerceActionV1::$action);
            for used in [0, 1, 2, 3, 65537, i64::MAX] {
                c.budget.count_used = used;
                budget_check(
                    &r,
                    &c,
                    &b,
                    if used >= 3 {
                        &[ReasonV1::E_COUNT_LIMIT]
                    } else {
                        &[]
                    },
                );
            }
        }
    };
}
budget_count_cases!(budget_count_refund, RefundCreate);
budget_count_cases!(budget_count_address, OrderAddressUpdate);
budget_count_cases!(budget_count_cancel, OrderCancel);
budget_count_cases!(budget_count_discount, DiscountCreate);
budget_count_cases!(budget_count_email, CustomerEmailSend);

macro_rules! budget_value_cases {
    ($name:ident, $action:ident) => {
        #[test]
        fn $name() {
            let (mut r, mut c, b) = budget_fixture(CommerceActionV1::$action);
            for (used, amount, fails) in [
                (0, 0, false),
                (4000, 999, false),
                (4000, 1000, false),
                (4000, 1001, true),
                (0, 5001, true),
                (5000, 0, false),
                (5001, 0, true),
                (i64::MAX, 1, true),
            ] {
                c.budget.value_used_minor = used;
                r.amount_minor = Some(amount);
                budget_check(
                    &r,
                    &c,
                    &b,
                    if fails {
                        &[ReasonV1::E_VALUE_LIMIT]
                    } else {
                        &[]
                    },
                );
            }
        }
    };
}
budget_value_cases!(budget_value_refund, RefundCreate);
budget_value_cases!(budget_value_discount, DiscountCreate);

#[test]
fn budget_count_maximum_is_checked_and_inclusive() {
    let (r, mut c, _) = budget_fixture(CommerceActionV1::RefundCreate);
    let mut source = preparation_source();
    source.action_settings[0].1.count_ceiling = Some(i64::MAX);
    recommit(&mut source);
    let b = prepare_bundle_v1(&source, &[7; 32]).unwrap();
    c.budget.count_used = i64::MAX - 1;
    budget_check(&r, &c, &b, &[]);
    c.budget.count_used = i64::MAX;
    budget_check(&r, &c, &b, &[ReasonV1::E_COUNT_LIMIT]);
    // A shortened integer representation collapses this used count to 1.
    c.budget.count_used = 65537;
    source.action_settings[0].1.count_ceiling = Some(3);
    recommit(&mut source);
    let b = prepare_bundle_v1(&source, &[7; 32]).unwrap();
    budget_check(&r, &c, &b, &[ReasonV1::E_COUNT_LIMIT]);
}

#[test]
fn budget_value_maximum_is_checked_and_inclusive() {
    let (mut r, mut c, _) = budget_fixture(CommerceActionV1::RefundCreate);
    let mut source = preparation_source();
    source.action_settings[0].1.value_ceiling_minor = Some(i64::MAX);
    recommit(&mut source);
    let b = prepare_bundle_v1(&source, &[7; 32]).unwrap();
    for (used, amount, fails) in [
        (i64::MAX, 0, false),
        (i64::MAX - 1, 1, false),
        (0, i64::MAX, false),
        (i64::MAX, 1, true),
        (1, i64::MAX, true),
    ] {
        c.budget.value_used_minor = used;
        r.amount_minor = Some(amount);
        budget_check(
            &r,
            &c,
            &b,
            if fails {
                &[ReasonV1::E_VALUE_LIMIT]
            } else {
                &[]
            },
        );
    }
}

macro_rules! budget_value_off_target {
    ($name:ident, $action:ident) => {
        #[test]
        fn $name() {
            let (mut r, mut c, _) = budget_fixture(CommerceActionV1::$action);
            let mut source = preparation_source();
            for (action, s) in &mut source.action_settings {
                if *action == r.action {
                    s.value_ceiling_minor = None;
                }
            }
            recommit(&mut source);
            let b = prepare_bundle_v1(&source, &[7; 32]).unwrap();
            let b = email_migration_bundle(b, r.action);
            email_rebind_policy(&r, &mut c, &b);
            c.budget.value_used_minor = i64::MAX;
            budget_check(&r, &c, &b, &[]);
            r.amount_minor = Some(i64::MAX);
            budget_check(&r, &c, &b, &[]);
        }
    };
}
budget_value_off_target!(budget_value_address_off_target, OrderAddressUpdate);
budget_value_off_target!(budget_value_cancel_off_target, OrderCancel);
budget_value_off_target!(budget_value_email_off_target, CustomerEmailSend);

#[test]
fn budget_zero_count_refuses_first_operation() {
    let (r, mut c, _) = budget_fixture(CommerceActionV1::RefundCreate);
    let mut source = preparation_source();
    source.action_settings[0].1.count_ceiling = Some(0);
    recommit(&mut source);
    let b = prepare_bundle_v1(&source, &[7; 32]).unwrap();
    c.budget.count_used = 0;
    budget_check(&r, &c, &b, &[ReasonV1::E_COUNT_LIMIT]);
}

#[test]
fn budget_zero_value_accepts_zero_and_refuses_positive() {
    let (mut r, mut c, _) = budget_fixture(CommerceActionV1::RefundCreate);
    let mut source = preparation_source();
    source.action_settings[0].1.value_ceiling_minor = Some(0);
    recommit(&mut source);
    let b = prepare_bundle_v1(&source, &[7; 32]).unwrap();
    c.budget.value_used_minor = 0;
    r.amount_minor = Some(0);
    budget_check(&r, &c, &b, &[]);
    r.amount_minor = Some(1);
    budget_check(&r, &c, &b, &[ReasonV1::E_VALUE_LIMIT]);
}

macro_rules! budget_missing_input {
    ($name:ident, $field:ident) => {
        #[test]
        fn $name() {
            let (mut r, c, b) = budget_fixture(CommerceActionV1::RefundCreate);
            r.$field = None;
            budget_check(&r, &c, &b, &[ReasonV1::E_UNDERSPECIFIED_ACTION]);
        }
    };
}
budget_missing_input!(budget_amount_required, amount_minor);
budget_missing_input!(budget_currency_required, currency);

macro_rules! budget_missing_limit {
    ($name:ident, $field:ident) => {
        #[test]
        fn $name() {
            let (r, c, _) = budget_fixture(CommerceActionV1::RefundCreate);
            let mut source = preparation_source();
            let s = &mut source.action_settings[0].1;
            s.enabled = false;
            s.$field = None;
            recommit(&mut source);
            let b = prepare_bundle_v1(&source, &[7; 32]).unwrap();
            budget_check(&r, &c, &b, &[ReasonV1::E_POLICY_UNCONFIGURED]);
        }
    };
}
budget_missing_limit!(budget_count_configuration_defence_in_depth, count_ceiling);
budget_missing_limit!(
    budget_value_configuration_defence_in_depth,
    value_ceiling_minor
);

#[test]
fn budget_uses_named_action_settings() {
    let (r, c, _) = budget_fixture(CommerceActionV1::DiscountCreate);
    let mut source = preparation_source();
    source.action_settings[0].1.count_ceiling = Some(0);
    source.action_settings[0].1.value_ceiling_minor = Some(0);
    recommit(&mut source);
    let b = prepare_bundle_v1(&source, &[7; 32]).unwrap();
    budget_check(&r, &c, &b, &[]);
}

#[test]
fn budget_representation_precedes_semantic_arithmetic() {
    let (mut r, mut c, b) = budget_fixture(CommerceActionV1::RefundCreate);
    c.budget.count_used = -1;
    c.budget.value_used_minor = i64::MAX;
    budget_check(&r, &c, &b, &[ReasonV1::E_MALFORMED_REQUEST]);
    c.budget.count_used = 1;
    c.budget.value_used_minor = -1;
    budget_check(&r, &c, &b, &[ReasonV1::E_MALFORMED_REQUEST]);
    c.budget.value_used_minor = 2000;
    r.operation_id.clear();
    budget_check(&r, &c, &b, &[ReasonV1::E_MALFORMED_REQUEST]);
}

macro_rules! budget_scope_case {
    ($name:ident, $field:ident, $bad:expr) => {
        #[test]
        fn $name() {
            let (r, mut c, b) = budget_fixture(CommerceActionV1::RefundCreate);
            budget_check(&r, &c, &b, &[]);
            c.budget.$field = $bad;
            budget_check(&r, &c, &b, &[ReasonV1::E_BINDING_MISMATCH]);
        }
    };
}
budget_scope_case!(budget_profile_binding, profile_id, "another-profile".into());
budget_scope_case!(budget_shop_binding, shop_id, "another-shop".into());
budget_scope_case!(budget_subject_binding, subject_id, "another-subject".into());
budget_scope_case!(budget_action_binding, action, CommerceActionV1::OrderCancel);
budget_scope_case!(budget_window_length_binding, window_length_ms, 60001);

#[test]
fn budget_monetary_currency_binding() {
    for action in [
        CommerceActionV1::RefundCreate,
        CommerceActionV1::DiscountCreate,
    ] {
        let (r, mut c, b) = budget_fixture(action);
        budget_check(&r, &c, &b, &[]);
        for currency in [None, Some("EUR".into())] {
            c.budget.currency = currency;
            budget_check(&r, &c, &b, &[ReasonV1::E_BINDING_MISMATCH]);
        }
    }
}

#[test]
fn budget_nonmonetary_currency_scope() {
    for action in [
        CommerceActionV1::OrderAddressUpdate,
        CommerceActionV1::OrderCancel,
        CommerceActionV1::CustomerEmailSend,
    ] {
        let (mut r, mut c, b) = budget_fixture(action);
        r.currency = Some("USD".into()); // Optional request money does not create monetary budget scope.
        budget_check(&r, &c, &b, &[]);
        c.budget.currency = Some("USD".into());
        budget_check(&r, &c, &b, &[ReasonV1::E_BINDING_MISMATCH]);
    }
}

#[test]
fn budget_window_is_half_open_without_rollover() {
    let (r, mut c, b) = budget_fixture(CommerceActionV1::RefundCreate);
    c.budget.window_start_ms = 100;
    for (now, causes) in [
        (99, vec![ReasonV1::E_MALFORMED_REQUEST]),
        (100, vec![]),
        (60099, vec![]),
        (60100, vec![ReasonV1::E_STALE_STATE]),
        (60101, vec![ReasonV1::E_STALE_STATE]),
    ] {
        c.now_ms = now;
        budget_check(&r, &c, &b, &causes);
        assert_eq!(c.budget.window_start_ms, 100);
        assert_eq!(c.budget.window_length_ms, 60000);
        assert_eq!(c.budget.count_used, 1);
        assert_eq!(c.budget.value_used_minor, 2000);
    }
}

#[test]
fn budget_window_end_must_fit_time_domain() {
    let (r, mut c, b) = budget_fixture(CommerceActionV1::RefundCreate);
    let maximum = u64::try_from(i64::MAX).unwrap();
    c.budget.window_start_ms = maximum - 60000;
    c.now_ms = maximum - 1;
    budget_check(&r, &c, &b, &[]);
    c.now_ms = maximum;
    budget_check(&r, &c, &b, &[ReasonV1::E_STALE_STATE]);
    c.budget.window_start_ms += 1;
    budget_check(&r, &c, &b, &[ReasonV1::E_MALFORMED_REQUEST]);
    c.budget.window_start_ms = maximum;
    budget_check(&r, &c, &b, &[ReasonV1::E_MALFORMED_REQUEST]);
    c.budget.window_length_ms = u64::MAX;
    budget_check(&r, &c, &b, &[ReasonV1::E_MALFORMED_REQUEST]); // Representation short-circuit.
}

#[test]
fn budget_zero_length_cannot_match_configured_window() {
    let (r, mut c, b) = budget_fixture(CommerceActionV1::RefundCreate);
    c.budget.window_length_ms = 0;
    budget_check(&r, &c, &b, &[ReasonV1::E_BINDING_MISMATCH]);
}

#[test]
fn budget_disabled_window_configuration_has_no_default() {
    let (r, c, _) = budget_fixture(CommerceActionV1::RefundCreate);
    for window in [None, Some(0)] {
        let mut source = preparation_source();
        let s = &mut source.action_settings[0].1;
        s.enabled = false;
        s.budget_window_ms = window;
        recommit(&mut source);
        let b = prepare_bundle_v1(&source, &[7; 32]).unwrap();
        budget_check(&r, &c, &b, &[ReasonV1::E_POLICY_UNCONFIGURED]);
    }
}

#[test]
fn budget_complete_causes_are_sorted_and_deduplicated() {
    let (r, mut c, b) = budget_fixture(CommerceActionV1::RefundCreate);
    c.budget.profile_id = "other".into();
    c.budget.shop_id = "other".into();
    c.budget.currency = Some("EUR".into());
    c.budget.window_length_ms = 59999;
    c.now_ms = 59999;
    c.budget.count_used = 3;
    c.budget.value_used_minor = 5000;
    budget_check(
        &r,
        &c,
        &b,
        &[
            ReasonV1::E_BINDING_MISMATCH,
            ReasonV1::E_STALE_STATE,
            ReasonV1::E_COUNT_LIMIT,
            ReasonV1::E_VALUE_LIMIT,
        ],
    );
    c.budget.window_start_ms = 60000;
    budget_check(
        &r,
        &c,
        &b,
        &[
            ReasonV1::E_MALFORMED_REQUEST,
            ReasonV1::E_BINDING_MISMATCH,
            ReasonV1::E_COUNT_LIMIT,
            ReasonV1::E_VALUE_LIMIT,
        ],
    );
}

#[test]
fn budget_unconfigured_causes_deduplicate() {
    let (r, c, _) = budget_fixture(CommerceActionV1::RefundCreate);
    let mut source = preparation_source();
    let s = &mut source.action_settings[0].1;
    s.enabled = false;
    s.count_ceiling = None;
    s.value_ceiling_minor = None;
    s.budget_window_ms = None;
    recommit(&mut source);
    let b = prepare_bundle_v1(&source, &[7; 32]).unwrap();
    budget_check(&r, &c, &b, &[ReasonV1::E_POLICY_UNCONFIGURED]);
}

#[test]
fn budget_snapshot_classification_is_not_permission_or_authentication() {
    let (r, mut c, b) = budget_fixture(CommerceActionV1::RefundCreate);
    c.authenticated_subject = "another".into();
    c.evidence.provenance = ProvenanceStateV1::Invalid;
    c.evidence.captured_minor = None;
    c.evidence.prior_refunds_minor = None;
    c.evidence.line_items_eligible = Some(false);
    c.budget.revision = "unverified-revision".into();
    c.review.grant = None;
    c.review.attestation = None;
    budget_check(&r, &c, &b, &[]);
    assert_eq!(c.budget.revision, "unverified-revision");
}

fn window_fixture(
    action: CommerceActionV1,
    limit: Option<u64>,
    enabled: bool,
    routes: Vec<ReviewRouteV1>,
) -> (
    CommerceRequestV1,
    DecisionContextV1,
    PreparedCommerceBundleV1,
) {
    let mut source = preparation_source();
    for (a, s) in &mut source.action_settings {
        s.order_age_limit_seconds = if *a == CommerceActionV1::RefundCreate {
            Some(1000)
        } else {
            None
        };
        if *a == action {
            s.order_age_limit_seconds = limit;
            s.enabled = enabled;
            s.review_routes = routes.clone();
        }
    }
    recommit(&mut source);
    let b = prepare_bundle_v1(&source, &[7; 32]).unwrap();
    let (mut r, mut c) = bound_frame(&b);
    r.action = action;
    c.evidence.order_age_seconds = Some(100);
    c.evidence.discount_conflict = Some(false);
    let b = email_migration_bundle(b, action);
    email_rebind_policy(&r, &mut c, &b);
    (r, c, b)
}
fn window_check(
    r: &CommerceRequestV1,
    c: &DecisionContextV1,
    b: &PreparedCommerceBundleV1,
    expected: &[ReasonV1],
) {
    assert_eq!(
        validate_order_window_and_discount_v1(r, c, b),
        if expected.is_empty() {
            Ok(())
        } else {
            Err(expected.to_vec())
        }
    );
}
macro_rules! window_age_cells {
    ($boundary:ident, $missing:ident, $action:ident) => {
        #[test]
        fn $boundary() {
            let (r, mut c, b) = window_fixture(CommerceActionV1::$action, Some(100), true, vec![]);
            for (age, fails) in [
                (0, false),
                (99, false),
                (100, false),
                (101, true),
                (65537, true),
            ] {
                c.evidence.order_age_seconds = Some(age);
                window_check(
                    &r,
                    &c,
                    &b,
                    if fails {
                        &[ReasonV1::E_ORDER_WINDOW]
                    } else {
                        &[]
                    },
                );
            }
        }
        #[test]
        fn $missing() {
            let (r, mut c, b) = window_fixture(CommerceActionV1::$action, Some(100), true, vec![]);
            // required_evidence contains SourceRevision only. The configured age limit
            // adds a mandatory age fact even without OrderAgeSeconds in that list.
            assert!(
                !b.action_settings()
                    .iter()
                    .find(|(a, _)| *a == r.action)
                    .unwrap()
                    .1
                    .required_evidence
                    .iter()
                    .any(|(f, _)| *f == EvidenceFieldV1::OrderAgeSeconds)
            );
            c.evidence.order_age_seconds = None;
            window_check(&r, &c, &b, &[ReasonV1::E_MISSING_EVIDENCE]);
        }
    };
}
window_age_cells!(
    window_refund_boundaries,
    window_refund_missing_age,
    RefundCreate
);
window_age_cells!(
    window_address_boundaries,
    window_address_missing_age,
    OrderAddressUpdate
);
window_age_cells!(
    window_cancel_boundaries,
    window_cancel_missing_age,
    OrderCancel
);
window_age_cells!(
    window_discount_boundaries,
    window_discount_missing_age,
    DiscountCreate
);
window_age_cells!(
    window_email_boundaries,
    window_email_missing_age,
    CustomerEmailSend
);

macro_rules! window_no_limit_cells {
    ($name:ident, $action:ident) => {
        #[test]
        fn $name() {
            let (r, mut c, b) = window_fixture(CommerceActionV1::$action, None, true, vec![]);
            for age in [None, Some(0), Some(i64::MAX as u64)] {
                c.evidence.order_age_seconds = age;
                window_check(&r, &c, &b, &[]);
            }
        }
    };
}
window_no_limit_cells!(window_address_without_limit, OrderAddressUpdate);
window_no_limit_cells!(window_cancel_without_limit, OrderCancel);
window_no_limit_cells!(window_discount_without_limit, DiscountCreate);
window_no_limit_cells!(window_email_without_limit, CustomerEmailSend);

#[test]
fn window_refund_missing_limit_defence_in_depth() {
    let (r, mut c, b) = window_fixture(CommerceActionV1::RefundCreate, None, false, vec![]);
    window_check(&r, &c, &b, &[ReasonV1::E_POLICY_UNCONFIGURED]);
    c.evidence.order_age_seconds = None;
    window_check(
        &r,
        &c,
        &b,
        &[
            ReasonV1::E_MISSING_EVIDENCE,
            ReasonV1::E_POLICY_UNCONFIGURED,
        ],
    );
    // Enabled refunds cannot construct this prepared configuration.
    let mut source = preparation_source();
    source.action_settings[0].1.order_age_limit_seconds = None;
    assert_eq!(
        bundle_digest_v1(&source),
        Err(BundleErrorV1::SettingsInvalid)
    );
}

#[test]
fn window_zero_limit_is_explicit_and_inclusive() {
    for action in [
        CommerceActionV1::RefundCreate,
        CommerceActionV1::OrderAddressUpdate,
        CommerceActionV1::OrderCancel,
        CommerceActionV1::DiscountCreate,
        CommerceActionV1::CustomerEmailSend,
    ] {
        let (r, mut c, b) = window_fixture(action, Some(0), true, vec![]);
        c.evidence.order_age_seconds = Some(0);
        window_check(&r, &c, &b, &[]);
        c.evidence.order_age_seconds = Some(1);
        window_check(&r, &c, &b, &[ReasonV1::E_ORDER_WINDOW]);
        c.evidence.order_age_seconds = None;
        window_check(&r, &c, &b, &[ReasonV1::E_MISSING_EVIDENCE]);
    }
}

#[test]
fn window_maximum_domain_and_no_unsigned_truncation() {
    let max = i64::MAX as u64;
    let (r, mut c, b) = window_fixture(CommerceActionV1::RefundCreate, Some(max), true, vec![]);
    c.evidence.order_age_seconds = Some(max);
    window_check(&r, &c, &b, &[]);
    let (r, c, b) = window_fixture(CommerceActionV1::RefundCreate, Some(max - 1), true, vec![]);
    let mut c = c;
    c.evidence.order_age_seconds = Some(max);
    window_check(&r, &c, &b, &[ReasonV1::E_ORDER_WINDOW]);
}

#[test]
fn window_discount_conflict_cells() {
    let (r, mut c, b) = window_fixture(CommerceActionV1::DiscountCreate, None, true, vec![]);
    for (value, reasons) in [
        (Some(false), &[][..]),
        (Some(true), &[ReasonV1::E_DISCOUNT_CONFLICT][..]),
        (None, &[ReasonV1::E_MISSING_EVIDENCE][..]),
    ] {
        c.evidence.discount_conflict = value;
        window_check(&r, &c, &b, reasons);
    }
}
macro_rules! window_discount_off_target {
    ($name:ident, $action:ident) => {
        #[test]
        fn $name() {
            let (r, mut c, b) = window_fixture(CommerceActionV1::$action, Some(100), true, vec![]);
            for conflict in [None, Some(false), Some(true)] {
                c.evidence.discount_conflict = conflict;
                window_check(&r, &c, &b, &[]);
            }
        }
    };
}
window_discount_off_target!(window_conflict_refund_off_target, RefundCreate);
window_discount_off_target!(window_conflict_address_off_target, OrderAddressUpdate);
window_discount_off_target!(window_conflict_cancel_off_target, OrderCancel);
window_discount_off_target!(window_conflict_email_off_target, CustomerEmailSend);

#[test]
fn window_complete_causes_are_registry_ordered() {
    let (r, mut c, b) = window_fixture(CommerceActionV1::DiscountCreate, Some(100), true, vec![]);
    c.evidence.order_age_seconds = Some(101);
    c.evidence.discount_conflict = Some(true);
    window_check(
        &r,
        &c,
        &b,
        &[ReasonV1::E_ORDER_WINDOW, ReasonV1::E_DISCOUNT_CONFLICT],
    );
    c.evidence.order_age_seconds = None;
    window_check(
        &r,
        &c,
        &b,
        &[ReasonV1::E_MISSING_EVIDENCE, ReasonV1::E_DISCOUNT_CONFLICT],
    );
}
#[test]
fn window_conflict_absence_sorts_a_later_discovered_evidence_cause() {
    let (r, mut c, b) = window_fixture(CommerceActionV1::DiscountCreate, Some(100), true, vec![]);
    c.evidence.order_age_seconds = Some(101);
    c.evidence.discount_conflict = None;
    window_check(
        &r,
        &c,
        &b,
        &[ReasonV1::E_MISSING_EVIDENCE, ReasonV1::E_ORDER_WINDOW],
    );
}

#[test]
fn window_missing_causes_deduplicate() {
    let (r, mut c, b) = window_fixture(CommerceActionV1::DiscountCreate, Some(100), true, vec![]);
    c.evidence.order_age_seconds = None;
    c.evidence.discount_conflict = None;
    window_check(&r, &c, &b, &[ReasonV1::E_MISSING_EVIDENCE]);
}
#[test]
fn window_routes_and_reviews_do_not_override() {
    let (r, mut c, b) = window_fixture(
        CommerceActionV1::DiscountCreate,
        Some(100),
        true,
        vec![ReviewRouteV1::OrderWindow, ReviewRouteV1::DiscountConflict],
    );
    assert!(c.review.grant.is_some() && c.review.attestation.is_some());
    c.evidence.order_age_seconds = Some(101);
    c.evidence.discount_conflict = Some(true);
    window_check(
        &r,
        &c,
        &b,
        &[ReasonV1::E_ORDER_WINDOW, ReasonV1::E_DISCOUNT_CONFLICT],
    );
}
#[test]
fn window_disabled_action_still_classifies() {
    let (r, mut c, b) = window_fixture(CommerceActionV1::DiscountCreate, Some(100), false, vec![]);
    c.evidence.order_age_seconds = Some(101);
    c.evidence.discount_conflict = Some(true);
    window_check(
        &r,
        &c,
        &b,
        &[ReasonV1::E_ORDER_WINDOW, ReasonV1::E_DISCOUNT_CONFLICT],
    );
}
#[test]
fn window_request_representation_precedes_semantics() {
    let (mut r, mut c, b) =
        window_fixture(CommerceActionV1::DiscountCreate, Some(100), true, vec![]);
    r.operation_id.clear();
    c.evidence.order_age_seconds = Some(101);
    c.evidence.discount_conflict = Some(true);
    window_check(&r, &c, &b, &[ReasonV1::E_MALFORMED_REQUEST]);
}
#[test]
fn window_context_representation_precedes_semantics() {
    let (r, mut c, b) = window_fixture(CommerceActionV1::DiscountCreate, Some(100), true, vec![]);
    c.now_ms = u64::MAX;
    c.evidence.order_age_seconds = Some(101);
    c.evidence.discount_conflict = Some(true);
    window_check(&r, &c, &b, &[ReasonV1::E_MALFORMED_REQUEST]);
}
#[test]
fn window_settings_are_selected_by_action() {
    // The refund entry has limit 1000; this action's entry has limit 100.
    let (r, mut c, b) = window_fixture(
        CommerceActionV1::OrderAddressUpdate,
        Some(100),
        true,
        vec![],
    );
    c.evidence.order_age_seconds = Some(101);
    window_check(&r, &c, &b, &[ReasonV1::E_ORDER_WINDOW]);
}
#[test]
fn window_success_is_not_permission_or_snapshot_validation() {
    let (r, mut c, b) = window_fixture(CommerceActionV1::DiscountCreate, Some(100), false, vec![]);
    c.evidence.order_age_seconds = Some(100);
    c.evidence.discount_conflict = Some(false);
    c.authenticated_subject = "other".into();
    c.evidence.provenance = ProvenanceStateV1::Invalid;
    c.evidence.fetched_at_ms = 0;
    c.now_ms = 10000;
    c.review.grant = None;
    c.review.attestation = None;
    let before = c.clone();
    window_check(&r, &c, &b, &[]);
    assert_eq!(c, before);
}

#[test]
fn window_full_unsigned_age_is_not_a_malformed_timestamp() {
    // Age uses the full unsigned number encoding; configured limits use the
    // signed-64-bit time representation. A valid age may exceed every limit.
    let (r, mut c, b) = window_fixture(
        CommerceActionV1::RefundCreate,
        Some(i64::MAX as u64),
        true,
        vec![],
    );
    c.evidence.order_age_seconds = Some(u64::MAX);
    window_check(&r, &c, &b, &[ReasonV1::E_ORDER_WINDOW]);
    let (r, mut c, b) = window_fixture(CommerceActionV1::OrderCancel, None, true, vec![]);
    c.evidence.order_age_seconds = Some(u64::MAX);
    window_check(&r, &c, &b, &[]);
}

fn requirements_fixture(
    edit: impl Fn(&mut CommerceActionSettingsV1),
) -> (
    CommerceRequestV1,
    DecisionContextV1,
    PreparedCommerceBundleV1,
) {
    let (request, mut context, bundle) = review_fixture(|s| {
        s.require_provenance = false;
        s.attestation_enabled = false;
        s.require_monetary_review = false;
        s.review_routes.clear();
        s.grant_max_lifetime_ms = Some(1000);
        s.attestation_max_lifetime_ms = Some(1000);
        s.review_request_timeout_ms = Some(1000);
        s.reviewer_role_policy_id = Some("reviewers".into());
        edit(s);
    });
    context.evidence.provenance = ProvenanceStateV1::Missing;
    (request, context, bundle)
}

fn requirements_assert(
    request: &CommerceRequestV1,
    context: &DecisionContextV1,
    bundle: &PreparedCommerceBundleV1,
    used: &[&str],
    missing: &[ReviewKindV1],
) {
    let resolution = resolve_review_requirements_v1(request, context, bundle).unwrap();
    let ids: Vec<_> = resolution.review_ids().iter().map(String::as_str).collect();
    assert_eq!(ids, used);
    assert_eq!(resolution.required_kinds(), missing);
}

#[test]
fn requirements_optional_valid_facts_are_not_used() {
    let (r, c, b) = requirements_fixture(|_| {});
    requirements_assert(&r, &c, &b, &[], &[]);
    let mut absent = c.clone();
    absent.review.grant = None;
    absent.review.attestation = None;
    absent.review_request = None;
    requirements_assert(&r, &absent, &b, &[], &[]);
}

#[test]
fn requirements_grant_does_not_satisfy_attestation() {
    let (r, mut c, b) = requirements_fixture(|s| {
        s.require_provenance = true;
        s.attestation_enabled = true;
    });
    requirements_assert(&r, &c, &b, &["attestation"], &[]);
    c.review.attestation = None;
    requirements_assert(&r, &c, &b, &[], &[ReviewKindV1::Attestation]);
}

#[test]
fn requirements_attestation_does_not_satisfy_grant() {
    let (r, mut c, b) = requirements_fixture(|s| s.require_monetary_review = true);
    requirements_assert(&r, &c, &b, &["review"], &[]);
    c.review.grant = None;
    requirements_assert(&r, &c, &b, &[], &[ReviewKindV1::Grant]);
}

#[test]
fn requirements_both_missing_kinds_are_sorted_not_permission() {
    let (r, mut c, b) = requirements_fixture(|s| {
        s.require_provenance = true;
        s.attestation_enabled = true;
        s.require_monetary_review = true;
    });
    c.review.grant = None;
    c.review.attestation = None;
    requirements_assert(
        &r,
        &c,
        &b,
        &[],
        &[ReviewKindV1::Grant, ReviewKindV1::Attestation],
    );
}

#[test]
fn requirements_used_ids_are_sorted_by_raw_utf8() {
    let (r, mut c, b) = requirements_fixture(|s| {
        s.require_provenance = true;
        s.attestation_enabled = true;
        s.require_monetary_review = true;
    });
    for (grant, attestation, expected) in [
        ("z", "a", ["a", "z"]),
        ("é", "Z", ["Z", "é"]),
        ("e\u{301}", "é", ["e\u{301}", "é"]),
    ] {
        c.review.grant.as_mut().unwrap().id = grant.into();
        c.review.attestation.as_mut().unwrap().id = attestation.into();
        requirements_assert(&r, &c, &b, &expected, &[]);
    }
}

#[test]
fn requirements_one_satisfied_and_one_missing_stays_separate() {
    let (r, mut c, b) = requirements_fixture(|s| {
        s.require_provenance = true;
        s.attestation_enabled = true;
        s.require_monetary_review = true;
    });
    let attestation = c.review.attestation.take();
    requirements_assert(&r, &c, &b, &["review"], &[ReviewKindV1::Attestation]);
    c.review.attestation = attestation;
    c.review.grant = None;
    requirements_assert(&r, &c, &b, &["attestation"], &[ReviewKindV1::Grant]);
}

#[test]
fn requirements_missing_provenance_without_attestation_route_is_rejected() {
    let (r, c, b) = requirements_fixture(|s| s.require_provenance = true);
    let error = resolve_review_requirements_v1(&r, &c, &b).unwrap_err();
    assert_eq!(error.provenance(), ProvenanceStateV1::Missing);
    assert_eq!(error.causes(), &[ReasonV1::E_PROVENANCE_UNVERIFIABLE]);
    assert!(!error.requires_terminal_deny()); // This method classifies provenance only.
}

#[test]
fn requirements_trusted_provenance_needs_no_attestation() {
    let (r, mut c, b) = requirements_fixture(|s| {
        s.require_provenance = true;
        s.attestation_enabled = true;
    });
    c.evidence.provenance = ProvenanceStateV1::TrustedBoundUnused;
    requirements_assert(&r, &c, &b, &[], &[]);
    c.review.attestation = None;
    requirements_assert(&r, &c, &b, &[], &[]);
}

#[test]
fn requirements_attestation_enablement_alone_is_not_a_requirement() {
    let (r, mut c, b) = requirements_fixture(|s| s.attestation_enabled = true);
    requirements_assert(&r, &c, &b, &[], &[]);
    c.review.attestation = None;
    requirements_assert(&r, &c, &b, &[], &[]);
}

macro_rules! requirements_grant_flag_case {
    ($name:ident, $action:expr) => {
        #[test]
        fn $name() {
            let (mut r, mut c, b) = requirements_fixture(|s| s.require_monetary_review = true);
            r.action = $action;
            let binding = request_binding_digest_v1(&r).unwrap();
            c.evidence.provenance_binding_digest = Some(binding);
            for fact in [&mut c.review.grant, &mut c.review.attestation]
                .into_iter()
                .flatten()
            {
                fact.binding_digest = binding;
            }
            c.review_request.as_mut().unwrap().binding_digest = binding;
            let b = email_migration_bundle(b, r.action);
            email_rebind_policy(&r, &mut c, &b);
            requirements_assert(&r, &c, &b, &["review"], &[]);
            c.review.grant = None;
            requirements_assert(&r, &c, &b, &[], &[ReviewKindV1::Grant]);
        }
    };
}
requirements_grant_flag_case!(
    requirements_refund_explicit_grant_flag,
    CommerceActionV1::RefundCreate
);
requirements_grant_flag_case!(
    requirements_address_explicit_grant_flag,
    CommerceActionV1::OrderAddressUpdate
);
requirements_grant_flag_case!(
    requirements_cancel_explicit_grant_flag,
    CommerceActionV1::OrderCancel
);
requirements_grant_flag_case!(
    requirements_discount_explicit_grant_flag,
    CommerceActionV1::DiscountCreate
);
requirements_grant_flag_case!(
    requirements_email_explicit_grant_flag,
    CommerceActionV1::CustomerEmailSend
);

#[test]
fn requirements_same_reviewer_only_denies_when_both_requirements_apply() {
    let (r, mut c, b) = requirements_fixture(|s| {
        s.require_provenance = true;
        s.attestation_enabled = true;
        s.require_monetary_review = true;
    });
    c.review.attestation.as_mut().unwrap().reviewer_id = "reviewer".into();
    assert_eq!(
        resolve_review_requirements_v1(&r, &c, &b)
            .unwrap_err()
            .causes(),
        &[ReasonV1::E_REVIEWER_SEPARATION]
    );
    c.evidence.provenance = ProvenanceStateV1::TrustedBoundUnused;
    requirements_assert(&r, &c, &b, &["review"], &[]);
}

#[test]
fn requirements_duplicate_optional_ids_are_malformed_not_repaired() {
    let (r, mut c, b) = requirements_fixture(|_| {});
    c.review.attestation.as_mut().unwrap().id = c.review.grant.as_ref().unwrap().id.clone();
    assert_eq!(
        resolve_review_requirements_v1(&r, &c, &b)
            .unwrap_err()
            .causes(),
        &[ReasonV1::E_MALFORMED_REQUEST]
    );
}

#[test]
fn requirements_duplicate_required_ids_are_malformed_not_deduped() {
    let (r, mut c, b) = requirements_fixture(|s| {
        s.require_provenance = true;
        s.attestation_enabled = true;
        s.require_monetary_review = true;
    });
    c.review.attestation.as_mut().unwrap().id = "review".into();
    assert_eq!(
        resolve_review_requirements_v1(&r, &c, &b)
            .unwrap_err()
            .causes(),
        &[ReasonV1::E_MALFORMED_REQUEST]
    );
}

fn requirements_defective_fact(grant: bool) {
    let (r, c, b) = requirements_fixture(|_| {}); // Optional, but every fact still validates.
    for case in 0..7 {
        let mut changed = c.clone();
        let fact = if grant {
            changed.review.grant.as_mut().unwrap()
        } else {
            changed.review.attestation.as_mut().unwrap()
        };
        let expected = match case {
            0 => {
                fact.authorized = false;
                ReasonV1::E_APPROVAL_INVALID
            }
            1 => {
                fact.expires_at_ms = c.now_ms;
                ReasonV1::E_APPROVAL_EXPIRED
            }
            2 => {
                fact.consumed = true;
                ReasonV1::E_REPLAY
            }
            3 => {
                fact.binding_digest[0] ^= 1;
                ReasonV1::E_BINDING_MISMATCH
            }
            4 => {
                fact.policy_digest[0] ^= 1;
                ReasonV1::E_BINDING_MISMATCH
            }
            5 => {
                fact.kind = if grant {
                    ReviewKindV1::Attestation
                } else {
                    ReviewKindV1::Grant
                };
                ReasonV1::E_MALFORMED_REQUEST
            }
            _ => {
                fact.issued_at_ms = c.now_ms + 1;
                ReasonV1::E_MALFORMED_REQUEST
            }
        };
        assert_eq!(
            resolve_review_requirements_v1(&r, &changed, &b)
                .unwrap_err()
                .causes(),
            &[expected],
            "grant={grant}, case={case}"
        );
    }
}

#[test]
fn requirements_invalid_optional_grant_is_never_absence() {
    requirements_defective_fact(true);
}
#[test]
fn requirements_invalid_optional_attestation_is_never_absence() {
    requirements_defective_fact(false);
}

#[test]
fn requirements_invalid_required_fact_does_not_manufacture_missing_kind() {
    let (r, mut c, b) = requirements_fixture(|s| s.require_monetary_review = true);
    c.review.grant.as_mut().unwrap().authorized = false;
    assert_eq!(
        resolve_review_requirements_v1(&r, &c, &b)
            .unwrap_err()
            .causes(),
        &[ReasonV1::E_APPROVAL_INVALID]
    );
}

#[test]
fn requirements_pending_expiry_denies_even_after_valid_reviews() {
    let (r, mut c, b) = requirements_fixture(|s| s.require_monetary_review = true);
    requirements_assert(&r, &c, &b, &["review"], &[]);
    c.review_request.as_mut().unwrap().expires_at_ms = c.now_ms;
    assert_eq!(
        resolve_review_requirements_v1(&r, &c, &b)
            .unwrap_err()
            .causes(),
        &[ReasonV1::E_APPROVAL_EXPIRED]
    );
}

#[test]
fn requirements_invalid_and_consumed_provenance_retain_terminal_veto() {
    let (r, c, b) = requirements_fixture(|s| {
        s.require_provenance = true;
        s.attestation_enabled = true;
    });
    for (state, reason) in [
        (
            ProvenanceStateV1::Invalid,
            ReasonV1::E_PROVENANCE_UNVERIFIABLE,
        ),
        (ProvenanceStateV1::Consumed, ReasonV1::E_REPLAY),
    ] {
        let mut changed = c.clone();
        changed.evidence.provenance = state;
        let error = resolve_review_requirements_v1(&r, &changed, &b).unwrap_err();
        assert!(error.requires_terminal_deny());
        assert_eq!(error.provenance(), state);
        assert_eq!(error.causes(), &[reason]);
    }
}

#[test]
fn requirements_attestation_does_not_upgrade_or_mutate_provenance() {
    let (r, c, b) = requirements_fixture(|s| {
        s.require_provenance = true;
        s.attestation_enabled = true;
    });
    let before = c.clone();
    requirements_assert(&r, &c, &b, &["attestation"], &[]);
    assert_eq!(c, before);
    assert_eq!(
        validate_provenance_and_reviews_v1(&r, &c, &b),
        Ok(ProvenanceStateV1::Missing)
    );
}

#[test]
fn requirements_independent_causes_are_sorted_and_deduplicated() {
    let (r, mut c, b) = requirements_fixture(|s| {
        s.require_provenance = true;
        s.require_monetary_review = true;
    });
    c.review.attestation.as_mut().unwrap().id = "review".into();
    c.review.grant.as_mut().unwrap().kind = ReviewKindV1::Attestation;
    c.review.grant.as_mut().unwrap().authorized = false;
    c.review.attestation.as_mut().unwrap().authorized = false;
    assert_eq!(
        resolve_review_requirements_v1(&r, &c, &b)
            .unwrap_err()
            .causes(),
        &[
            ReasonV1::E_MALFORMED_REQUEST,
            ReasonV1::E_PROVENANCE_UNVERIFIABLE,
            ReasonV1::E_APPROVAL_INVALID
        ]
    );
}

#[test]
fn requirements_representation_short_circuits_with_state_retained() {
    let (r, c, b) = requirements_fixture(|s| s.require_provenance = true);
    for state in [ProvenanceStateV1::Invalid, ProvenanceStateV1::Missing] {
        let mut changed = c.clone();
        changed.now_ms = u64::MAX;
        changed.evidence.provenance = state;
        changed.review.grant.as_mut().unwrap().authorized = false;
        let error = resolve_review_requirements_v1(&r, &changed, &b).unwrap_err();
        assert_eq!(error.causes(), &[ReasonV1::E_MALFORMED_REQUEST]);
        assert_eq!(error.provenance(), state);
        assert_eq!(
            error.requires_terminal_deny(),
            state == ProvenanceStateV1::Invalid
        );
    }
}

#[test]
fn requirements_settings_selection_is_per_action() {
    let mut source = preparation_source();
    source.schema_text = NATIVE_SCHEMA.into();
    for (action, settings) in &mut source.action_settings {
        settings.grant_max_lifetime_ms = Some(1000);
        settings.attestation_max_lifetime_ms = Some(1000);
        settings.review_request_timeout_ms = Some(1000);
        settings.require_provenance = false;
        settings.attestation_enabled = false;
        settings.review_routes.clear();
        settings.require_monetary_review = *action == CommerceActionV1::OrderCancel;
    }
    recommit(&mut source);
    let b = prepare_bundle_v1(&source, &[7; 32]).unwrap();
    let (mut r, mut c) = bound_frame(&b);
    c.review.attestation = None;
    c.now_ms = 50;
    requirements_assert(&r, &c, &b, &[], &[]);
    r.action = CommerceActionV1::OrderCancel;
    let binding = request_binding_digest_v1(&r).unwrap();
    c.evidence.provenance_binding_digest = Some(binding);
    c.review.grant.as_mut().unwrap().binding_digest = binding;
    c.review_request.as_mut().unwrap().binding_digest = binding;
    requirements_assert(&r, &c, &b, &["review"], &[]);
}

#[test]
fn requirements_disabled_and_review_routes_do_not_override_requirements() {
    let (r, c, b) = requirements_fixture(|s| {
        s.enabled = false;
        s.require_monetary_review = true;
        s.review_routes = vec![ReviewRouteV1::OrderWindow, ReviewRouteV1::DiscountConflict];
    });
    requirements_assert(&r, &c, &b, &["review"], &[]);
    let mut missing = c.clone();
    missing.review.grant = None;
    requirements_assert(&r, &missing, &b, &[], &[ReviewKindV1::Grant]);
}

fn routing_fixture(
    action: CommerceActionV1,
    edit: impl Fn(&mut CommerceActionSettingsV1),
) -> (
    CommerceRequestV1,
    DecisionContextV1,
    PreparedCommerceBundleV1,
) {
    let (mut r, mut c, b) = requirements_fixture(edit);
    r.action = action;
    c.review.grant = None;
    c.review.attestation = None;
    c.review_request = None;
    c.evidence.order_age_seconds = Some(100);
    c.evidence.discount_conflict = Some(false);
    c.evidence.provenance_binding_digest = None;
    let b = email_migration_bundle(b, action);
    email_rebind_policy(&r, &mut c, &b);
    (r, c, b)
}

fn routing_fact(r: &CommerceRequestV1, c: &DecisionContextV1, kind: ReviewKindV1) -> ReviewFactV1 {
    ReviewFactV1 {
        id: if kind == ReviewKindV1::Grant {
            "grant"
        } else {
            "attestation"
        }
        .into(),
        reviewer_id: if kind == ReviewKindV1::Grant {
            "grant-reviewer"
        } else {
            "attest-reviewer"
        }
        .into(),
        authorized: true,
        issued_at_ms: 0,
        expires_at_ms: 1000,
        binding_digest: request_binding_digest_v1(r).unwrap(),
        policy_digest: c.policy_digest,
        consumed: false,
        kind,
    }
}

fn routing_pending(r: &CommerceRequestV1, c: &DecisionContextV1) -> ReviewRequestV1 {
    ReviewRequestV1 {
        id: "pending".into(),
        created_at_ms: 10,
        expires_at_ms: 200,
        binding_digest: request_binding_digest_v1(r).unwrap(),
        policy_digest: c.policy_digest,
    }
}

fn routing_causes(
    r: &CommerceRequestV1,
    c: &DecisionContextV1,
    b: &PreparedCommerceBundleV1,
    expected: &[ReasonV1],
) {
    let error = resolve_review_routes_v1(r, c, b).unwrap_err();
    assert_eq!(error.causes(), expected);
    assert_eq!(error.provenance(), c.evidence.provenance);
}

#[test]
fn routing_order_window_default_deny_even_with_valid_grant() {
    let (r, mut c, b) = routing_fixture(CommerceActionV1::RefundCreate, |_| {});
    c.evidence.order_age_seconds = Some(1001);
    routing_causes(&r, &c, &b, &[ReasonV1::E_ORDER_WINDOW]);
    c.review.grant = Some(routing_fact(&r, &c, ReviewKindV1::Grant));
    routing_causes(&r, &c, &b, &[ReasonV1::E_ORDER_WINDOW]);
}

#[test]
fn routing_discount_default_deny_even_with_valid_grant() {
    let (r, mut c, b) = routing_fixture(CommerceActionV1::DiscountCreate, |_| {});
    c.evidence.discount_conflict = Some(true);
    c.review.grant = Some(routing_fact(&r, &c, ReviewKindV1::Grant));
    routing_causes(&r, &c, &b, &[ReasonV1::E_DISCOUNT_CONFLICT]);
}

#[test]
fn routing_order_window_requires_grant_only_when_triggered() {
    let (r, mut c, b) = routing_fixture(CommerceActionV1::RefundCreate, |s| {
        s.review_routes = vec![ReviewRouteV1::OrderWindow]
    });
    let pass = resolve_review_routes_v1(&r, &c, &b).unwrap();
    assert!(pass.required_kinds().is_empty());
    assert!(pass.routed_causes().is_empty());
    assert!(pass.escalation().is_none());
    c.review.grant = Some(routing_fact(&r, &c, ReviewKindV1::Grant));
    assert!(
        resolve_review_routes_v1(&r, &c, &b)
            .unwrap()
            .review_ids()
            .is_empty()
    );
    c.review.grant = None;
    c.evidence.order_age_seconds = Some(1001);
    let result = resolve_review_routes_v1(&r, &c, &b).unwrap();
    assert_eq!(result.required_kinds(), &[ReviewKindV1::Grant]);
    assert_eq!(result.routed_causes(), &[ReasonV1::E_ORDER_WINDOW]);
}

#[test]
fn routing_discount_requires_grant_only_when_triggered() {
    let (r, mut c, b) = routing_fixture(CommerceActionV1::DiscountCreate, |s| {
        s.review_routes = vec![ReviewRouteV1::DiscountConflict]
    });
    assert!(
        resolve_review_routes_v1(&r, &c, &b)
            .unwrap()
            .required_kinds()
            .is_empty()
    );
    c.evidence.discount_conflict = Some(true);
    let result = resolve_review_routes_v1(&r, &c, &b).unwrap();
    assert_eq!(result.required_kinds(), &[ReviewKindV1::Grant]);
    assert_eq!(result.routed_causes(), &[ReasonV1::E_DISCOUNT_CONFLICT]);
}

#[test]
fn routing_wrong_route_cannot_repair_window_or_conflict() {
    for action in [
        CommerceActionV1::RefundCreate,
        CommerceActionV1::DiscountCreate,
    ] {
        let wrong = if action == CommerceActionV1::RefundCreate {
            ReviewRouteV1::DiscountConflict
        } else {
            ReviewRouteV1::OrderWindow
        };
        let (r, mut c, b) = routing_fixture(action, |s| s.review_routes = vec![wrong]);
        let reason = if action == CommerceActionV1::RefundCreate {
            c.evidence.order_age_seconds = Some(1001);
            ReasonV1::E_ORDER_WINDOW
        } else {
            c.evidence.discount_conflict = Some(true);
            ReasonV1::E_DISCOUNT_CONFLICT
        };
        c.review.grant = Some(routing_fact(&r, &c, ReviewKindV1::Grant));
        routing_causes(&r, &c, &b, &[reason]);
    }
}

#[test]
fn routing_valid_grant_satisfies_route_and_preserves_raw_causes() {
    let (r, mut c, b) = routing_fixture(CommerceActionV1::DiscountCreate, |s| {
        s.review_routes = vec![ReviewRouteV1::OrderWindow, ReviewRouteV1::DiscountConflict]
    });
    c.evidence.order_age_seconds = Some(1001);
    c.evidence.discount_conflict = Some(true);
    c.review.grant = Some(routing_fact(&r, &c, ReviewKindV1::Grant));
    let before = c.clone();
    let result = resolve_review_routes_v1(&r, &c, &b).unwrap();
    assert_eq!(result.review_ids(), &["grant"]);
    assert!(result.required_kinds().is_empty());
    assert!(result.escalation().is_none());
    assert_eq!(
        result.routed_causes(),
        &[ReasonV1::E_ORDER_WINDOW, ReasonV1::E_DISCOUNT_CONFLICT]
    );
    assert_eq!(c, before);
    assert_eq!(
        validate_order_window_and_discount_v1(&r, &c, &b),
        Err(vec![
            ReasonV1::E_ORDER_WINDOW,
            ReasonV1::E_DISCOUNT_CONFLICT
        ])
    );
}

#[test]
fn routing_multiple_routes_and_flag_need_only_one_grant() {
    let (r, mut c, b) = routing_fixture(CommerceActionV1::DiscountCreate, |s| {
        s.require_monetary_review = true;
        s.review_routes = vec![ReviewRouteV1::OrderWindow, ReviewRouteV1::DiscountConflict];
    });
    c.evidence.order_age_seconds = Some(1001);
    c.evidence.discount_conflict = Some(true);
    let missing = resolve_review_routes_v1(&r, &c, &b).unwrap();
    assert_eq!(missing.required_kinds(), &[ReviewKindV1::Grant]);
    assert_eq!(
        missing.escalation().unwrap().required_kinds,
        vec![ReviewKindV1::Grant]
    );
    c.review.grant = Some(routing_fact(&r, &c, ReviewKindV1::Grant));
    assert_eq!(
        resolve_review_routes_v1(&r, &c, &b).unwrap().review_ids(),
        &["grant"]
    );
}

#[test]
fn routing_missing_field_is_not_a_review_route() {
    let (r, mut c, b) = routing_fixture(CommerceActionV1::DiscountCreate, |s| {
        s.review_routes = vec![ReviewRouteV1::OrderWindow, ReviewRouteV1::DiscountConflict]
    });
    c.evidence.order_age_seconds = None;
    c.evidence.discount_conflict = None;
    routing_causes(&r, &c, &b, &[ReasonV1::E_MISSING_EVIDENCE]);
}

#[test]
fn routing_simultaneous_blocking_fault_retains_routed_diagnostic() {
    let (r, mut c, b) = routing_fixture(CommerceActionV1::DiscountCreate, |s| {
        s.review_routes = vec![ReviewRouteV1::OrderWindow]
    });
    c.evidence.order_age_seconds = Some(1001);
    c.evidence.discount_conflict = None;
    routing_causes(
        &r,
        &c,
        &b,
        &[ReasonV1::E_MISSING_EVIDENCE, ReasonV1::E_ORDER_WINDOW],
    );
}

#[test]
fn routing_optional_invalid_fact_is_not_a_missing_requirement() {
    let (r, mut c, b) = routing_fixture(CommerceActionV1::RefundCreate, |s| {
        s.review_routes = vec![ReviewRouteV1::OrderWindow]
    });
    c.evidence.order_age_seconds = Some(1001);
    c.review.attestation = Some(routing_fact(&r, &c, ReviewKindV1::Attestation));
    c.review.attestation.as_mut().unwrap().authorized = false;
    routing_causes(
        &r,
        &c,
        &b,
        &[ReasonV1::E_ORDER_WINDOW, ReasonV1::E_APPROVAL_INVALID],
    );
}

#[test]
fn routing_invalid_or_consumed_provenance_remains_terminal() {
    let (r, mut c, b) = routing_fixture(CommerceActionV1::RefundCreate, |s| {
        s.review_routes = vec![ReviewRouteV1::OrderWindow]
    });
    c.evidence.order_age_seconds = Some(1001);
    c.evidence.provenance_binding_digest = Some(request_binding_digest_v1(&r).unwrap());
    c.review.grant = Some(routing_fact(&r, &c, ReviewKindV1::Grant));
    for (state, reason) in [
        (
            ProvenanceStateV1::Invalid,
            ReasonV1::E_PROVENANCE_UNVERIFIABLE,
        ),
        (ProvenanceStateV1::Consumed, ReasonV1::E_REPLAY),
    ] {
        c.evidence.provenance = state;
        let err = resolve_review_routes_v1(&r, &c, &b).unwrap_err();
        assert!(err.requires_terminal_deny());
        assert!(err.causes().contains(&reason));
        assert!(err.causes().contains(&ReasonV1::E_ORDER_WINDOW));
        assert_eq!(err.provenance(), state);
    }
}

#[test]
fn routing_effective_grant_and_attestation_separate_without_money_flag() {
    let (r, mut c, b) = routing_fixture(CommerceActionV1::RefundCreate, |s| {
        s.require_provenance = true;
        s.attestation_enabled = true;
        s.review_routes = vec![ReviewRouteV1::OrderWindow];
    });
    c.evidence.order_age_seconds = Some(1001);
    c.review.grant = Some(routing_fact(&r, &c, ReviewKindV1::Grant));
    c.review.attestation = Some(routing_fact(&r, &c, ReviewKindV1::Attestation));
    let success = resolve_review_routes_v1(&r, &c, &b).unwrap();
    assert_eq!(success.review_ids(), &["attestation", "grant"]);
    c.review.attestation.as_mut().unwrap().reviewer_id =
        c.review.grant.as_ref().unwrap().reviewer_id.clone();
    routing_causes(
        &r,
        &c,
        &b,
        &[ReasonV1::E_ORDER_WINDOW, ReasonV1::E_REVIEWER_SEPARATION],
    );
}

#[test]
fn routing_optional_same_reviewer_is_not_separation_requirement() {
    let (r, mut c, b) = routing_fixture(CommerceActionV1::RefundCreate, |s| {
        s.review_routes = vec![ReviewRouteV1::OrderWindow]
    });
    c.evidence.order_age_seconds = Some(1001);
    c.review.grant = Some(routing_fact(&r, &c, ReviewKindV1::Grant));
    c.review.attestation = Some(routing_fact(&r, &c, ReviewKindV1::Attestation));
    c.review.attestation.as_mut().unwrap().reviewer_id =
        c.review.grant.as_ref().unwrap().reviewer_id.clone();
    assert_eq!(
        resolve_review_routes_v1(&r, &c, &b).unwrap().review_ids(),
        &["grant"]
    );
}

#[test]
fn routing_duplicate_artifact_ids_reject_before_canonicalization() {
    let (r, mut c, b) = routing_fixture(CommerceActionV1::RefundCreate, |s| {
        s.review_routes = vec![ReviewRouteV1::OrderWindow]
    });
    c.evidence.order_age_seconds = Some(1001);
    c.review.grant = Some(routing_fact(&r, &c, ReviewKindV1::Grant));
    c.review.attestation = Some(routing_fact(&r, &c, ReviewKindV1::Attestation));
    c.review.attestation.as_mut().unwrap().id = "grant".into();
    routing_causes(
        &r,
        &c,
        &b,
        &[ReasonV1::E_MALFORMED_REQUEST, ReasonV1::E_ORDER_WINDOW],
    );
}

#[test]
fn routing_both_missing_kinds_and_used_ids_are_canonical() {
    let (r, mut c, b) = routing_fixture(CommerceActionV1::RefundCreate, |s| {
        s.require_provenance = true;
        s.attestation_enabled = true;
        s.review_routes = vec![ReviewRouteV1::OrderWindow];
    });
    c.evidence.order_age_seconds = Some(1001);
    let result = resolve_review_routes_v1(&r, &c, &b).unwrap();
    assert_eq!(
        result.required_kinds(),
        &[ReviewKindV1::Grant, ReviewKindV1::Attestation]
    );
    assert_eq!(
        result.escalation().unwrap().required_kinds,
        vec![ReviewKindV1::Grant, ReviewKindV1::Attestation]
    );
    c.review.grant = Some(routing_fact(&r, &c, ReviewKindV1::Grant));
    c.review.attestation = Some(routing_fact(&r, &c, ReviewKindV1::Attestation));
    c.review.grant.as_mut().unwrap().id = "é".into();
    c.review.attestation.as_mut().unwrap().id = "Z".into();
    assert_eq!(
        resolve_review_routes_v1(&r, &c, &b).unwrap().review_ids(),
        &["Z", "é"]
    );
    // Attestation is collected by the inner resolver before a route-only Grant.
    // Reverse the ID assignment so that collection order cannot satisfy sorting.
    c.review.grant.as_mut().unwrap().id = "Z".into();
    c.review.attestation.as_mut().unwrap().id = "é".into();
    assert_eq!(
        resolve_review_routes_v1(&r, &c, &b).unwrap().review_ids(),
        &["Z", "é"]
    );
}

#[test]
fn routing_new_deadline_is_exact_and_does_not_modify_context() {
    let (r, c, b) = routing_fixture(CommerceActionV1::CustomerEmailSend, |s| {
        s.require_monetary_review = true
    });
    let original = c.clone();
    let result = resolve_review_routes_v1(&r, &c, &b).unwrap();
    assert_eq!(
        result.escalation(),
        Some(&EscalationRequirementV1 {
            requested_at_ms: 50,
            expires_at_ms: 1050,
            review_request_id: None,
            required_kinds: vec![ReviewKindV1::Grant]
        })
    );
    assert_eq!(c, original);
}

#[test]
fn routing_existing_shorter_deadline_and_id_never_extend() {
    let (r, mut c, b) = routing_fixture(CommerceActionV1::CustomerEmailSend, |s| {
        s.require_monetary_review = true
    });
    c.review_request = Some(routing_pending(&r, &c));
    for now in [10, 50, 199] {
        c.now_ms = now;
        let result = resolve_review_routes_v1(&r, &c, &b).unwrap();
        let e = result.escalation().unwrap();
        assert_eq!((e.requested_at_ms, e.expires_at_ms), (10, 200));
        assert_eq!(e.review_request_id.as_deref(), Some("pending"));
    }
}

#[test]
fn routing_expired_pending_denies_even_when_grant_fulfilled() {
    let (r, mut c, b) = routing_fixture(CommerceActionV1::CustomerEmailSend, |s| {
        s.require_monetary_review = true
    });
    c.review_request = Some(routing_pending(&r, &c));
    c.review.grant = Some(routing_fact(&r, &c, ReviewKindV1::Grant));
    c.now_ms = 199;
    assert!(
        resolve_review_routes_v1(&r, &c, &b)
            .unwrap()
            .escalation()
            .is_none()
    );
    for now in [200, 201] {
        c.now_ms = now;
        routing_causes(&r, &c, &b, &[ReasonV1::E_APPROVAL_EXPIRED]);
    }
}

#[test]
fn routing_pending_binding_policy_and_interval_are_validated() {
    let (r, mut c, b) = routing_fixture(CommerceActionV1::CustomerEmailSend, |s| {
        s.require_monetary_review = true
    });
    c.review_request = Some(routing_pending(&r, &c));
    for field in 0..5 {
        let mut changed = c.clone();
        let p = changed.review_request.as_mut().unwrap();
        let reason = match field {
            0 => {
                p.binding_digest[0] ^= 1;
                ReasonV1::E_BINDING_MISMATCH
            }
            1 => {
                p.policy_digest[0] ^= 1;
                ReasonV1::E_BINDING_MISMATCH
            }
            2 => {
                p.created_at_ms = 51;
                ReasonV1::E_MALFORMED_REQUEST
            }
            3 => {
                p.expires_at_ms = p.created_at_ms;
                ReasonV1::E_MALFORMED_REQUEST
            }
            _ => {
                p.expires_at_ms = 1011;
                ReasonV1::E_APPROVAL_INVALID
            }
        };
        routing_causes(&r, &changed, &b, &[reason]);
    }
}

#[test]
fn routing_absent_and_zero_timeout_are_unconfigured_defence_in_depth() {
    for timeout in [None, Some(0)] {
        let (r, c, b) = routing_fixture(CommerceActionV1::CustomerEmailSend, |s| {
            s.enabled = false;
            s.require_monetary_review = true;
            s.review_request_timeout_ms = timeout;
        });
        routing_causes(&r, &c, &b, &[ReasonV1::E_POLICY_UNCONFIGURED]);
    }
}

#[test]
fn routing_new_deadline_signed_domain_is_inclusive_not_wrapping() {
    let (r, mut c, b) = routing_fixture(CommerceActionV1::CustomerEmailSend, |s| {
        s.require_monetary_review = true;
        s.review_request_timeout_ms = Some(1);
    });
    c.now_ms = i64::MAX as u64 - 1;
    assert_eq!(
        resolve_review_routes_v1(&r, &c, &b)
            .unwrap()
            .escalation()
            .unwrap()
            .expires_at_ms,
        i64::MAX as u64
    );
    c.now_ms = i64::MAX as u64;
    routing_causes(&r, &c, &b, &[ReasonV1::E_MALFORMED_REQUEST]);
}

#[test]
fn routing_unrepresentable_time_short_circuits_without_subtraction() {
    let (r, mut c, b) = routing_fixture(CommerceActionV1::CustomerEmailSend, |s| {
        s.require_monetary_review = true
    });
    c.now_ms = u64::MAX;
    c.evidence.provenance = ProvenanceStateV1::Invalid;
    let error = resolve_review_routes_v1(&r, &c, &b).unwrap_err();
    assert_eq!(error.causes(), &[ReasonV1::E_MALFORMED_REQUEST]);
    assert!(error.requires_terminal_deny());
}

#[test]
fn routing_fulfilled_requirements_and_unused_pending_have_no_escalation() {
    let (r, mut c, b) = routing_fixture(CommerceActionV1::CustomerEmailSend, |_| {});
    c.review_request = Some(routing_pending(&r, &c));
    c.review.grant = Some(routing_fact(&r, &c, ReviewKindV1::Grant));
    let result = resolve_review_routes_v1(&r, &c, &b).unwrap();
    assert!(result.review_ids().is_empty());
    assert!(result.required_kinds().is_empty());
    assert!(result.escalation().is_none());
    c.now_ms = 200;
    routing_causes(&r, &c, &b, &[ReasonV1::E_APPROVAL_EXPIRED]);
}

#[test]
fn routing_review_success_is_not_enablement_or_hard_predicate_permission() {
    let (r, mut c, b) = routing_fixture(CommerceActionV1::OrderCancel, |s| s.enabled = false);
    c.evidence.cancellation_eligible = Some(false);
    assert!(resolve_review_routes_v1(&r, &c, &b).is_ok());
    assert_eq!(
        validate_action_eligibility_v1(&r, &c),
        Err(ReasonV1::E_CONSTRAINT_VIOLATION)
    );
}

#[test]
fn routing_invalid_grant_cannot_satisfy_enabled_route() {
    let (r, mut c, b) = routing_fixture(CommerceActionV1::RefundCreate, |s| {
        s.review_routes = vec![ReviewRouteV1::OrderWindow];
    });
    c.evidence.order_age_seconds = Some(1001);
    c.review.grant = Some(routing_fact(&r, &c, ReviewKindV1::Grant));
    for defect in 0..4 {
        let mut changed = c.clone();
        let grant = changed.review.grant.as_mut().unwrap();
        let expected = match defect {
            0 => {
                grant.authorized = false;
                vec![ReasonV1::E_ORDER_WINDOW, ReasonV1::E_APPROVAL_INVALID]
            }
            1 => {
                grant.consumed = true;
                vec![ReasonV1::E_REPLAY, ReasonV1::E_ORDER_WINDOW]
            }
            2 => {
                grant.expires_at_ms = changed.now_ms;
                vec![ReasonV1::E_ORDER_WINDOW, ReasonV1::E_APPROVAL_EXPIRED]
            }
            _ => {
                grant.binding_digest[0] ^= 1;
                vec![ReasonV1::E_BINDING_MISMATCH, ReasonV1::E_ORDER_WINDOW]
            }
        };
        routing_causes(&r, &changed, &b, &expected);
    }
}

#[test]
fn routing_predicate_changes_recompute_used_grant_on_same_snapshot_contract() {
    let (r, mut c, b) = routing_fixture(CommerceActionV1::RefundCreate, |s| {
        s.review_routes = vec![ReviewRouteV1::OrderWindow];
    });
    c.evidence.order_age_seconds = Some(1001);
    c.review.grant = Some(routing_fact(&r, &c, ReviewKindV1::Grant));
    assert_eq!(
        resolve_review_routes_v1(&r, &c, &b).unwrap().review_ids(),
        &["grant"]
    );
    c.evidence.order_age_seconds = Some(1000);
    let resolved = resolve_review_routes_v1(&r, &c, &b).unwrap();
    assert!(resolved.review_ids().is_empty());
    assert!(resolved.routed_causes().is_empty());
}

#[test]
fn routing_settings_are_selected_for_requested_action() {
    let mut source = preparation_source();
    source.schema_text = NATIVE_SCHEMA.into();
    for (action, s) in &mut source.action_settings {
        s.require_provenance = false;
        s.attestation_enabled = false;
        s.require_monetary_review = false;
        s.review_routes = if *action == CommerceActionV1::DiscountCreate {
            vec![ReviewRouteV1::DiscountConflict]
        } else {
            vec![]
        };
    }
    recommit(&mut source);
    let b = prepare_bundle_v1(&source, &[7; 32]).unwrap();
    let (mut r, mut c) = bound_frame(&b);
    r.action = CommerceActionV1::DiscountCreate;
    c.now_ms = 50;
    c.review.grant = None;
    c.review.attestation = None;
    c.review_request = None;
    c.evidence.provenance = ProvenanceStateV1::Missing;
    c.evidence.provenance_binding_digest = None;
    c.evidence.order_age_seconds = Some(100);
    c.evidence.discount_conflict = Some(true);
    let result = resolve_review_routes_v1(&r, &c, &b).unwrap();
    assert_eq!(result.required_kinds(), &[ReviewKindV1::Grant]);
    assert_eq!(result.routed_causes(), &[ReasonV1::E_DISCOUNT_CONFLICT]);
}

#[test]
fn routing_simultaneous_error_causes_are_sorted_and_deduplicated() {
    let (r, mut c, b) = routing_fixture(CommerceActionV1::RefundCreate, |s| {
        s.require_provenance = true;
        s.attestation_enabled = true;
        s.require_monetary_review = true;
        s.review_routes = vec![ReviewRouteV1::OrderWindow];
    });
    c.evidence.order_age_seconds = Some(1001);
    c.review.grant = Some(routing_fact(&r, &c, ReviewKindV1::Grant));
    c.review.attestation = Some(routing_fact(&r, &c, ReviewKindV1::Attestation));
    c.review.grant.as_mut().unwrap().authorized = false;
    let a = c.review.attestation.as_mut().unwrap();
    a.authorized = false;
    a.reviewer_id = "grant-reviewer".into();
    routing_causes(
        &r,
        &c,
        &b,
        &[
            ReasonV1::E_ORDER_WINDOW,
            ReasonV1::E_REVIEWER_SEPARATION,
            ReasonV1::E_APPROVAL_INVALID,
        ],
    );
}

#[test]
fn requirements_success_is_not_other_predicates_or_permission() {
    let (r, mut c, b) = requirements_fixture(|_| {});
    c.evidence.line_items_eligible = Some(false);
    c.evidence.fetched_at_ms = c.now_ms + 1;
    c.evidence.source_revision = "changed".into();
    c.approved_evidence_revision = Some("previous".into());
    requirements_assert(&r, &c, &b, &[], &[]);
    assert!(validate_action_eligibility_v1(&r, &c).is_err());
    assert!(validate_evidence_snapshot_v1(&r, &c, &b).is_err());
}

fn email_template() -> EmailTemplateV1 {
    EmailTemplateV1 {
        template_id: "template-a".into(),
        subject: "Order update".into(),
        body: "Hello, customer.".into(),
    }
}
fn email_catalog(profile: &str, shop: &str) -> EmailTemplatePolicySourceV1 {
    EmailTemplatePolicySourceV1 {
        profile_id: profile.into(),
        shop_id: shop.into(),
        templates: vec![email_template()],
    }
}
fn email_migration_bundle(
    b: PreparedCommerceBundleV1,
    action: CommerceActionV1,
) -> PreparedCommerceBundleV1 {
    if action == CommerceActionV1::CustomerEmailSend {
        prepare_email_policy_v1(b, &email_catalog("a", "a"))
            .unwrap()
            .bundle()
            .clone()
    } else {
        b
    }
}
fn email_rebind_policy(
    r: &CommerceRequestV1,
    c: &mut DecisionContextV1,
    b: &PreparedCommerceBundleV1,
) {
    c.policy_digest = *b.digest();
    c.registry_digest = *b.registry_digest();
    let digest = request_binding_digest_v1(r).unwrap();
    if c.evidence.provenance_binding_digest.is_some() {
        c.evidence.provenance_binding_digest = Some(digest);
    }
    for fact in [&mut c.review.grant, &mut c.review.attestation]
        .into_iter()
        .flatten()
    {
        fact.policy_digest = *b.digest();
        fact.binding_digest = digest;
    }
    if let Some(pending) = &mut c.review_request {
        pending.policy_digest = *b.digest();
        pending.binding_digest = digest;
    }
}
fn email_payload() -> serde_json::Value {
    serde_json::json!({"order_id":"a","recipient":"customer@example.test","template_id":"template-a", "draft_id":"draft-a","subject":"Order update","body":"Hello, customer.","recipient_count":1})
}
fn email_commit_payload(
    r: &mut CommerceRequestV1,
    value: &serde_json::Value,
) -> EmailPayloadSourceV1 {
    use sha2::{Digest, Sha256};
    let bytes = serde_json::to_vec(value).unwrap();
    r.payload_digest = Sha256::digest(&bytes).into();
    r.payload_length = u32::try_from(bytes.len()).unwrap();
    EmailPayloadSourceV1 { bytes }
}
fn email_commit_raw(r: &mut CommerceRequestV1, bytes: Vec<u8>) -> EmailPayloadSourceV1 {
    use sha2::{Digest, Sha256};
    r.payload_digest = Sha256::digest(&bytes).into();
    r.payload_length = u32::try_from(bytes.len()).unwrap();
    EmailPayloadSourceV1 { bytes }
}
fn email_fixture() -> (
    CommerceRequestV1,
    DecisionContextV1,
    PreparedEmailPolicyV1,
    EmailPayloadSourceV1,
) {
    let policy = prepare_email_policy_v1(native_bundle(true), &email_catalog("a", "a")).unwrap();
    let (mut r, mut c) = native_frame(policy.bundle());
    r.action = CommerceActionV1::CustomerEmailSend;
    let payload = email_commit_payload(&mut r, &email_payload());
    c.evidence.recipient_count = Some(1);
    c.evidence.derived_recipient_digest =
        Some(recipient_digest_v1(&r, "customer@example.test", 1).unwrap());
    c.evidence.approved_template_digest =
        Some(email_template_digest_v1(&email_template()).unwrap());
    email_rebind_policy(&r, &mut c, policy.bundle());
    (r, c, policy, payload)
}
fn email_assert(
    r: &CommerceRequestV1,
    c: &DecisionContextV1,
    p: &PreparedEmailPolicyV1,
    payload: &EmailPayloadSourceV1,
    expected: &[ReasonV1],
) {
    assert_eq!(
        validate_email_bindings_v1(r, c, p, Some(payload)),
        if expected.is_empty() {
            Ok(())
        } else {
            Err(expected.to_vec())
        }
    );
}

// Each role is separately registered. No policy permission is asserted by an Ok.
fn email_guard_fixture(
    annex: bool,
    action: CommerceActionV1,
    mismatch: bool,
) -> (
    CommerceRequestV1,
    DecisionContextV1,
    PreparedCommerceBundleV1,
) {
    let (mut r, mut c, b) = routing_fixture(CommerceActionV1::OrderCancel, |s| {
        s.require_provenance = false;
        s.require_monetary_review = false;
    });
    r.action = action;
    c.budget.action = action;
    c.budget.window_length_ms = 60000;
    c.evidence.recipient_count = Some(1);
    let b = if annex {
        prepare_email_policy_v1(
            b,
            &email_catalog(if mismatch { "different" } else { "a" }, "a"),
        )
        .unwrap()
        .bundle()
        .clone()
    } else {
        b
    };
    email_rebind_policy(&r, &mut c, &b);
    (r, c, b)
}
fn email_guard_invoke(
    name: &str,
    r: &CommerceRequestV1,
    c: &DecisionContextV1,
    b: &PreparedCommerceBundleV1,
) -> Result<(), Vec<ReasonV1>> {
    match name {
        "validate_request_bindings_v1" => {
            validate_request_bindings_v1(r, c, b).map_err(|e| vec![e])
        }
        "validate_monetary_constraints_v1" => validate_monetary_constraints_v1(r, c, b),
        "validate_budget_constraints_v1" => validate_budget_constraints_v1(r, c, b),
        "validate_order_window_and_discount_v1" => validate_order_window_and_discount_v1(r, c, b),
        "validate_evidence_snapshot_v1" => validate_evidence_snapshot_v1(r, c, b),
        "validate_review_artifacts_v1" => validate_review_artifacts_v1(r, c, b),
        "validate_provenance_and_reviews_v1" => validate_provenance_and_reviews_v1(r, c, b)
            .map(|_| ())
            .map_err(|e| e.causes().to_vec()),
        "resolve_review_requirements_v1" => resolve_review_requirements_v1(r, c, b)
            .map(|_| ())
            .map_err(|e| e.causes().to_vec()),
        "resolve_review_routes_v1" => resolve_review_routes_v1(r, c, b)
            .map(|_| ())
            .map_err(|e| e.causes().to_vec()),
        "build_cedar_request_v1" => build_cedar_request_v1(r, c, b)
            .map(|_| ())
            .map_err(|e| vec![e]),
        _ => panic!("unclassified consumer"),
    }
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__validate_request_bindings_v1__base_email_refused() {
    let (r, c, b) = email_guard_fixture(false, CommerceActionV1::CustomerEmailSend, false);
    assert_eq!(
        email_guard_invoke("validate_request_bindings_v1", &r, &c, &b),
        Err(vec![ReasonV1::E_POLICY_UNCONFIGURED])
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__validate_request_bindings_v1__annex_email_accepted() {
    let (r, c, b) = email_guard_fixture(true, CommerceActionV1::CustomerEmailSend, false);
    assert_eq!(
        email_guard_invoke("validate_request_bindings_v1", &r, &c, &b),
        Ok(())
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__validate_request_bindings_v1__base_other_action_accepted() {
    let (r, c, b) = email_guard_fixture(false, CommerceActionV1::OrderCancel, false);
    assert_eq!(
        email_guard_invoke("validate_request_bindings_v1", &r, &c, &b),
        Ok(())
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__validate_request_bindings_v1__annex_scope_rejected() {
    let (r, c, b) = email_guard_fixture(true, CommerceActionV1::CustomerEmailSend, true);
    assert_eq!(
        email_guard_invoke("validate_request_bindings_v1", &r, &c, &b),
        Err(vec![ReasonV1::E_BINDING_MISMATCH])
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__validate_monetary_constraints_v1__base_email_refused() {
    let (r, c, b) = email_guard_fixture(false, CommerceActionV1::CustomerEmailSend, false);
    assert_eq!(
        email_guard_invoke("validate_monetary_constraints_v1", &r, &c, &b),
        Err(vec![ReasonV1::E_POLICY_UNCONFIGURED])
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__validate_monetary_constraints_v1__annex_email_accepted() {
    let (r, c, b) = email_guard_fixture(true, CommerceActionV1::CustomerEmailSend, false);
    assert_eq!(
        email_guard_invoke("validate_monetary_constraints_v1", &r, &c, &b),
        Ok(())
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__validate_monetary_constraints_v1__base_other_action_accepted() {
    let (r, c, b) = email_guard_fixture(false, CommerceActionV1::OrderCancel, false);
    assert_eq!(
        email_guard_invoke("validate_monetary_constraints_v1", &r, &c, &b),
        Ok(())
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__validate_monetary_constraints_v1__annex_scope_rejected() {
    let (r, c, b) = email_guard_fixture(true, CommerceActionV1::CustomerEmailSend, true);
    assert_eq!(
        email_guard_invoke("validate_monetary_constraints_v1", &r, &c, &b),
        Err(vec![ReasonV1::E_BINDING_MISMATCH])
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__validate_budget_constraints_v1__base_email_refused() {
    let (r, c, b) = email_guard_fixture(false, CommerceActionV1::CustomerEmailSend, false);
    assert_eq!(
        email_guard_invoke("validate_budget_constraints_v1", &r, &c, &b),
        Err(vec![ReasonV1::E_POLICY_UNCONFIGURED])
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__validate_budget_constraints_v1__annex_email_accepted() {
    let (r, c, b) = email_guard_fixture(true, CommerceActionV1::CustomerEmailSend, false);
    assert_eq!(
        email_guard_invoke("validate_budget_constraints_v1", &r, &c, &b),
        Ok(())
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__validate_budget_constraints_v1__base_other_action_accepted() {
    let (r, c, b) = email_guard_fixture(false, CommerceActionV1::OrderCancel, false);
    assert_eq!(
        email_guard_invoke("validate_budget_constraints_v1", &r, &c, &b),
        Ok(())
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__validate_budget_constraints_v1__annex_scope_rejected() {
    let (r, c, b) = email_guard_fixture(true, CommerceActionV1::CustomerEmailSend, true);
    assert_eq!(
        email_guard_invoke("validate_budget_constraints_v1", &r, &c, &b),
        Err(vec![ReasonV1::E_BINDING_MISMATCH])
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__validate_order_window_and_discount_v1__base_email_refused() {
    let (r, c, b) = email_guard_fixture(false, CommerceActionV1::CustomerEmailSend, false);
    assert_eq!(
        email_guard_invoke("validate_order_window_and_discount_v1", &r, &c, &b),
        Err(vec![ReasonV1::E_POLICY_UNCONFIGURED])
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__validate_order_window_and_discount_v1__annex_email_accepted() {
    let (r, c, b) = email_guard_fixture(true, CommerceActionV1::CustomerEmailSend, false);
    assert_eq!(
        email_guard_invoke("validate_order_window_and_discount_v1", &r, &c, &b),
        Ok(())
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__validate_order_window_and_discount_v1__base_other_action_accepted() {
    let (r, c, b) = email_guard_fixture(false, CommerceActionV1::OrderCancel, false);
    assert_eq!(
        email_guard_invoke("validate_order_window_and_discount_v1", &r, &c, &b),
        Ok(())
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__validate_order_window_and_discount_v1__annex_scope_rejected() {
    let (r, c, b) = email_guard_fixture(true, CommerceActionV1::CustomerEmailSend, true);
    assert_eq!(
        email_guard_invoke("validate_order_window_and_discount_v1", &r, &c, &b),
        Err(vec![ReasonV1::E_BINDING_MISMATCH])
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__validate_evidence_snapshot_v1__base_email_refused() {
    let (r, c, b) = email_guard_fixture(false, CommerceActionV1::CustomerEmailSend, false);
    assert_eq!(
        email_guard_invoke("validate_evidence_snapshot_v1", &r, &c, &b),
        Err(vec![ReasonV1::E_POLICY_UNCONFIGURED])
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__validate_evidence_snapshot_v1__annex_email_accepted() {
    let (r, c, b) = email_guard_fixture(true, CommerceActionV1::CustomerEmailSend, false);
    assert_eq!(
        email_guard_invoke("validate_evidence_snapshot_v1", &r, &c, &b),
        Ok(())
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__validate_evidence_snapshot_v1__base_other_action_accepted() {
    let (r, c, b) = email_guard_fixture(false, CommerceActionV1::OrderCancel, false);
    assert_eq!(
        email_guard_invoke("validate_evidence_snapshot_v1", &r, &c, &b),
        Ok(())
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__validate_evidence_snapshot_v1__annex_scope_rejected() {
    let (r, c, b) = email_guard_fixture(true, CommerceActionV1::CustomerEmailSend, true);
    assert_eq!(
        email_guard_invoke("validate_evidence_snapshot_v1", &r, &c, &b),
        Err(vec![ReasonV1::E_BINDING_MISMATCH])
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__validate_review_artifacts_v1__base_email_refused() {
    let (r, c, b) = email_guard_fixture(false, CommerceActionV1::CustomerEmailSend, false);
    assert_eq!(
        email_guard_invoke("validate_review_artifacts_v1", &r, &c, &b),
        Err(vec![ReasonV1::E_POLICY_UNCONFIGURED])
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__validate_review_artifacts_v1__annex_email_accepted() {
    let (r, c, b) = email_guard_fixture(true, CommerceActionV1::CustomerEmailSend, false);
    assert_eq!(
        email_guard_invoke("validate_review_artifacts_v1", &r, &c, &b),
        Ok(())
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__validate_review_artifacts_v1__base_other_action_accepted() {
    let (r, c, b) = email_guard_fixture(false, CommerceActionV1::OrderCancel, false);
    assert_eq!(
        email_guard_invoke("validate_review_artifacts_v1", &r, &c, &b),
        Ok(())
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__validate_review_artifacts_v1__annex_scope_rejected() {
    let (r, c, b) = email_guard_fixture(true, CommerceActionV1::CustomerEmailSend, true);
    assert_eq!(
        email_guard_invoke("validate_review_artifacts_v1", &r, &c, &b),
        Err(vec![ReasonV1::E_BINDING_MISMATCH])
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__validate_provenance_and_reviews_v1__base_email_refused() {
    let (r, c, b) = email_guard_fixture(false, CommerceActionV1::CustomerEmailSend, false);
    assert_eq!(
        email_guard_invoke("validate_provenance_and_reviews_v1", &r, &c, &b),
        Err(vec![ReasonV1::E_POLICY_UNCONFIGURED])
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__validate_provenance_and_reviews_v1__annex_email_accepted() {
    let (r, c, b) = email_guard_fixture(true, CommerceActionV1::CustomerEmailSend, false);
    assert_eq!(
        email_guard_invoke("validate_provenance_and_reviews_v1", &r, &c, &b),
        Ok(())
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__validate_provenance_and_reviews_v1__base_other_action_accepted() {
    let (r, c, b) = email_guard_fixture(false, CommerceActionV1::OrderCancel, false);
    assert_eq!(
        email_guard_invoke("validate_provenance_and_reviews_v1", &r, &c, &b),
        Ok(())
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__validate_provenance_and_reviews_v1__annex_scope_rejected() {
    let (r, c, b) = email_guard_fixture(true, CommerceActionV1::CustomerEmailSend, true);
    assert_eq!(
        email_guard_invoke("validate_provenance_and_reviews_v1", &r, &c, &b),
        Err(vec![ReasonV1::E_BINDING_MISMATCH])
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__resolve_review_requirements_v1__base_email_refused() {
    let (r, c, b) = email_guard_fixture(false, CommerceActionV1::CustomerEmailSend, false);
    assert_eq!(
        email_guard_invoke("resolve_review_requirements_v1", &r, &c, &b),
        Err(vec![ReasonV1::E_POLICY_UNCONFIGURED])
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__resolve_review_requirements_v1__annex_email_accepted() {
    let (r, c, b) = email_guard_fixture(true, CommerceActionV1::CustomerEmailSend, false);
    assert_eq!(
        email_guard_invoke("resolve_review_requirements_v1", &r, &c, &b),
        Ok(())
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__resolve_review_requirements_v1__base_other_action_accepted() {
    let (r, c, b) = email_guard_fixture(false, CommerceActionV1::OrderCancel, false);
    assert_eq!(
        email_guard_invoke("resolve_review_requirements_v1", &r, &c, &b),
        Ok(())
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__resolve_review_requirements_v1__annex_scope_rejected() {
    let (r, c, b) = email_guard_fixture(true, CommerceActionV1::CustomerEmailSend, true);
    assert_eq!(
        email_guard_invoke("resolve_review_requirements_v1", &r, &c, &b),
        Err(vec![ReasonV1::E_BINDING_MISMATCH])
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__resolve_review_routes_v1__base_email_refused() {
    let (r, c, b) = email_guard_fixture(false, CommerceActionV1::CustomerEmailSend, false);
    assert_eq!(
        email_guard_invoke("resolve_review_routes_v1", &r, &c, &b),
        Err(vec![ReasonV1::E_POLICY_UNCONFIGURED])
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__resolve_review_routes_v1__annex_email_accepted() {
    let (r, c, b) = email_guard_fixture(true, CommerceActionV1::CustomerEmailSend, false);
    assert_eq!(
        email_guard_invoke("resolve_review_routes_v1", &r, &c, &b),
        Ok(())
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__resolve_review_routes_v1__base_other_action_accepted() {
    let (r, c, b) = email_guard_fixture(false, CommerceActionV1::OrderCancel, false);
    assert_eq!(
        email_guard_invoke("resolve_review_routes_v1", &r, &c, &b),
        Ok(())
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__resolve_review_routes_v1__annex_scope_rejected() {
    let (r, c, b) = email_guard_fixture(true, CommerceActionV1::CustomerEmailSend, true);
    assert_eq!(
        email_guard_invoke("resolve_review_routes_v1", &r, &c, &b),
        Err(vec![ReasonV1::E_BINDING_MISMATCH])
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__build_cedar_request_v1__base_email_refused() {
    let (r, c, b) = email_guard_fixture(false, CommerceActionV1::CustomerEmailSend, false);
    assert_eq!(
        email_guard_invoke("build_cedar_request_v1", &r, &c, &b),
        Err(vec![ReasonV1::E_POLICY_UNCONFIGURED])
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__build_cedar_request_v1__annex_email_accepted() {
    let (r, c, b) = email_guard_fixture(true, CommerceActionV1::CustomerEmailSend, false);
    assert_eq!(
        email_guard_invoke("build_cedar_request_v1", &r, &c, &b),
        Ok(())
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__build_cedar_request_v1__base_other_action_accepted() {
    let (r, c, b) = email_guard_fixture(false, CommerceActionV1::OrderCancel, false);
    assert_eq!(
        email_guard_invoke("build_cedar_request_v1", &r, &c, &b),
        Ok(())
    );
}

#[test]
#[allow(non_snake_case)] // Contract-derived control identity uses double separators.
fn email_guard__build_cedar_request_v1__annex_scope_rejected() {
    let (r, c, b) = email_guard_fixture(true, CommerceActionV1::CustomerEmailSend, true);
    assert_eq!(
        email_guard_invoke("build_cedar_request_v1", &r, &c, &b),
        Err(vec![ReasonV1::E_BINDING_MISMATCH])
    );
}

#[test]
fn email_binding_declared_domain_vectors() {
    let (mut r, _, _) = frame();
    r.profile_id = "profile-a".into();
    r.shop_id = "shop-a".into();
    r.resource_id = "order-a".into();
    assert_eq!(
        hex(&recipient_digest_v1(&r, "customer@example.test", 1).unwrap()),
        "91d98292d348f45e3eebb7f6de0016229f891e7d20ae035ea750612f263b9dc6"
    );
    assert_eq!(
        hex(&email_template_digest_v1(&email_template()).unwrap()),
        "53c692da7fee438545843d772f8f0f2d9e72a37c7b1129a990ce185fb12d341e"
    );
    assert_eq!(
        hex(&email_policy_digest_v1(&[17; 32], &email_catalog("profile-a", "shop-a")).unwrap()),
        "493f4dbbdd3cf49044d2d22cd30c0f52c8bbc86859d818239093812d510a8096"
    );
}

#[test]
fn email_binding_exact_payload_identity_and_whitespace() {
    let (mut r, c, p, payload) = email_fixture();
    email_assert(&r, &c, &p, &payload, &[]);
    let mut changed = payload.clone();
    changed.bytes.push(b' ');
    email_assert(&r, &c, &p, &changed, &[ReasonV1::E_BINDING_MISMATCH]);
    r.payload_length += 1;
    email_assert(&r, &c, &p, &changed, &[ReasonV1::E_BINDING_MISMATCH]);
    let changed = email_commit_raw(&mut r, changed.bytes);
    email_assert(&r, &c, &p, &changed, &[]);
    r.payload_digest[0] ^= 1;
    email_assert(&r, &c, &p, &changed, &[ReasonV1::E_BINDING_MISMATCH]);
}
#[test]
fn email_binding_strict_projection_no_repair() {
    let (mut r, c, p, _) = email_fixture();
    let mut variants = vec![
        b"[]".to_vec(),
        b"null".to_vec(),
        b"{}".to_vec(),
        b"{\"order_id\":\"a\",\"order_id\":\"b\"}".to_vec(),
        b"\xff".to_vec(),
    ];
    for field in [
        "order_id",
        "recipient",
        "template_id",
        "draft_id",
        "subject",
        "body",
        "recipient_count",
    ] {
        let mut value = email_payload();
        value.as_object_mut().unwrap().remove(field);
        variants.push(serde_json::to_vec(&value).unwrap());
        value = email_payload();
        value[field] = serde_json::Value::Null;
        variants.push(serde_json::to_vec(&value).unwrap());
    }
    let mut value = email_payload();
    value["extra"] = true.into();
    variants.push(serde_json::to_vec(&value).unwrap());
    for count in [
        serde_json::json!(-1),
        serde_json::json!(4294967296u64),
        serde_json::json!(1.0),
        serde_json::json!("1"),
    ] {
        value = email_payload();
        value["recipient_count"] = count;
        variants.push(serde_json::to_vec(&value).unwrap());
    }
    let mut trailing = serde_json::to_vec(&email_payload()).unwrap();
    trailing.extend_from_slice(b" {}");
    variants.push(trailing);
    let good = serde_json::to_string(&email_payload()).unwrap();
    variants.push(
        good.replacen("{", "{\"recipient\":\"other\",", 1)
            .into_bytes(),
    );
    for bytes in variants {
        let payload = email_commit_raw(&mut r, bytes);
        email_assert(&r, &c, &p, &payload, &[ReasonV1::E_MALFORMED_REQUEST]);
    }
}
#[test]
fn email_binding_whole_payload_bound_and_missing() {
    let (mut r, c, p, _) = email_fixture();
    email_assert(
        &r,
        &c,
        &p,
        &EmailPayloadSourceV1 { bytes: vec![] },
        &[ReasonV1::E_MALFORMED_REQUEST],
    );
    let original = serde_json::to_vec(&email_payload()).unwrap();
    for length in [65535, 65536, 65537] {
        let mut bytes = original.clone();
        bytes.resize(length, b' ');
        let payload = email_commit_raw(&mut r, bytes);
        email_assert(
            &r,
            &c,
            &p,
            &payload,
            if length > 65536 {
                &[ReasonV1::E_MALFORMED_REQUEST]
            } else {
                &[]
            },
        );
    }
}
#[test]
fn email_binding_recipient_digest_and_scope_operands() {
    let (r, c, p, payload) = email_fixture();
    let expected = recipient_digest_v1(&r, "customer@example.test", 1).unwrap();
    for field in 0..5 {
        let mut changed = r.clone();
        let mut recipient = "customer@example.test";
        let mut count = 1;
        match field {
            0 => changed.profile_id.push('x'),
            1 => changed.shop_id.push('x'),
            2 => changed.resource_id.push('x'),
            3 => recipient = "other@example.test",
            _ => count = 2,
        }
        assert_ne!(
            recipient_digest_v1(&changed, recipient, count).unwrap(),
            expected,
            "recipient operand {field}"
        );
    }
    let mut changed = c.clone();
    changed.evidence.derived_recipient_digest = Some([42; 32]);
    email_assert(&r, &changed, &p, &payload, &[ReasonV1::E_BINDING_MISMATCH]);
    changed.evidence.derived_recipient_digest = None;
    email_assert(&r, &changed, &p, &payload, &[ReasonV1::E_MISSING_EVIDENCE]);
    let mut value = email_payload();
    value["recipient"] = "other@example.test".into();
    let mut changed = r.clone();
    let payload = email_commit_payload(&mut changed, &value);
    email_assert(&changed, &c, &p, &payload, &[ReasonV1::E_BINDING_MISMATCH]);
    let mut rebound = c.clone();
    rebound.evidence.derived_recipient_digest =
        Some(recipient_digest_v1(&changed, "other@example.test", 1).unwrap());
    email_assert(&changed, &rebound, &p, &payload, &[]);
}
#[test]
fn email_binding_resource_comparison_independent_of_frame() {
    let (mut r, c, p, _) = email_fixture();
    let mut value = email_payload();
    value["order_id"] = "other-order".into();
    let payload = email_commit_payload(&mut r, &value);
    email_assert(&r, &c, &p, &payload, &[ReasonV1::E_BINDING_MISMATCH]);
}
#[test]
fn email_binding_template_digest_and_catalog_operands() {
    let (mut r, c, p, _) = email_fixture();
    let expected = email_template_digest_v1(&email_template()).unwrap();
    for field in ["template_id", "subject", "body"] {
        let mut value = email_payload();
        value[field] = "unapproved".into();
        let payload = email_commit_payload(&mut r, &value);
        email_assert(
            &r,
            &c,
            &p,
            &payload,
            &[
                ReasonV1::E_BINDING_MISMATCH,
                ReasonV1::E_CONSTRAINT_VIOLATION,
            ],
        );
        let changed = EmailTemplateV1 {
            template_id: value["template_id"].as_str().unwrap().into(),
            subject: value["subject"].as_str().unwrap().into(),
            body: value["body"].as_str().unwrap().into(),
        };
        let digest = email_template_digest_v1(&changed).unwrap();
        assert_ne!(digest, expected, "template operand {field}");
        let mut rebound = c.clone();
        rebound.evidence.approved_template_digest = Some(digest);
        email_assert(
            &r,
            &rebound,
            &p,
            &payload,
            &[ReasonV1::E_CONSTRAINT_VIOLATION],
        );
    }
}
#[test]
fn email_binding_template_evidence_required() {
    let (r, mut c, p, payload) = email_fixture();
    c.evidence.approved_template_digest = None;
    email_assert(&r, &c, &p, &payload, &[ReasonV1::E_MISSING_EVIDENCE]);
    c.evidence.approved_template_digest = Some([99; 32]);
    email_assert(&r, &c, &p, &payload, &[ReasonV1::E_BINDING_MISMATCH]);
}
#[test]
fn email_binding_count_checks_independent_and_complete_causes() {
    let (mut r, mut c, p, payload) = email_fixture();
    c.evidence.recipient_count = None;
    email_assert(&r, &c, &p, &payload, &[ReasonV1::E_MISSING_EVIDENCE]);
    c.evidence.recipient_count = Some(2);
    email_assert(&r, &c, &p, &payload, &[ReasonV1::E_EVIDENCE_CONFLICT]);
    for count in [0, 2, 65537, u32::MAX] {
        let mut value = email_payload();
        value["recipient_count"] = count.into();
        let payload = email_commit_payload(&mut r, &value);
        c.evidence.recipient_count = Some(count);
        c.evidence.derived_recipient_digest =
            Some(recipient_digest_v1(&r, "customer@example.test", count).unwrap());
        email_assert(&r, &c, &p, &payload, &[ReasonV1::E_CONSTRAINT_VIOLATION]);
    }
    let mut value = email_payload();
    value["body"] = "not approved".into();
    value["recipient_count"] = 2.into();
    let payload = email_commit_payload(&mut r, &value);
    c.evidence.recipient_count = Some(1);
    c.evidence.derived_recipient_digest = None;
    c.evidence.approved_template_digest = None;
    email_assert(
        &r,
        &c,
        &p,
        &payload,
        &[
            ReasonV1::E_MISSING_EVIDENCE,
            ReasonV1::E_EVIDENCE_CONFLICT,
            ReasonV1::E_CONSTRAINT_VIOLATION,
        ],
    );
}
#[test]
fn email_binding_draft_payload_only_and_wrong_action() {
    let (mut r, c, p, payload) = email_fixture();
    let mut value = email_payload();
    value["draft_id"] = "other-draft".into();
    let bytes = serde_json::to_vec(&value).unwrap();
    email_assert(
        &r,
        &c,
        &p,
        &EmailPayloadSourceV1 { bytes },
        &[ReasonV1::E_BINDING_MISMATCH],
    );
    let rebound = email_commit_payload(&mut r, &value);
    email_assert(&r, &c, &p, &rebound, &[]);
    r.action = CommerceActionV1::OrderCancel;
    email_assert(&r, &c, &p, &payload, &[ReasonV1::E_MALFORMED_REQUEST]);
}
#[test]
fn email_binding_effective_policy_all_supplied_artifacts() {
    let (mut r, mut c, p, payload) = email_fixture();
    let (_, bound, _) = review_fixture(|_| {});
    c.review = bound.review;
    c.review_request = bound.review_request;
    c.evidence.provenance_binding_digest = Some([0; 32]);
    email_rebind_policy(&r, &mut c, p.bundle());
    email_assert(&r, &c, &p, &payload, &[]);
    for field in 0..5 {
        let mut changed = c.clone();
        match field {
            0 => changed.policy_digest = *p.base_digest(),
            1 => changed.review.grant.as_mut().unwrap().policy_digest = *p.base_digest(),
            2 => changed.review.attestation.as_mut().unwrap().policy_digest = *p.base_digest(),
            3 => changed.review_request.as_mut().unwrap().policy_digest = *p.base_digest(),
            _ => changed.evidence.provenance_binding_digest = Some([0; 32]),
        };
        email_assert(&r, &changed, &p, &payload, &[ReasonV1::E_BINDING_MISMATCH]);
    }
    let mut catalog = email_catalog("a", "a");
    catalog.templates.push(EmailTemplateV1 {
        template_id: "other".into(),
        ..email_template()
    });
    let p2 = prepare_email_policy_v1(native_bundle(true), &catalog).unwrap();
    assert_ne!(p.digest(), p2.digest());
    email_assert(&r, &c, &p2, &payload, &[ReasonV1::E_BINDING_MISMATCH]);
    email_rebind_policy(&r, &mut c, p2.bundle());
    email_assert(&r, &c, &p2, &payload, &[]);
    r.profile_id = "wrong".into();
    email_assert(&r, &c, &p2, &payload, &[ReasonV1::E_BINDING_MISMATCH]);
}
#[test]
fn email_binding_representation_precedence_and_scope() {
    let (mut r, c, p, payload) = email_fixture();
    r.operation_id.clear();
    email_assert(&r, &c, &p, &payload, &[ReasonV1::E_MALFORMED_REQUEST]);
    let (r, c, _, payload) = email_fixture();
    let scoped =
        prepare_email_policy_v1(native_bundle(true), &email_catalog("different", "a")).unwrap();
    let mut rebound = c;
    email_rebind_policy(&r, &mut rebound, scoped.bundle());
    email_assert(
        &r,
        &rebound,
        &scoped,
        &payload,
        &[ReasonV1::E_BINDING_MISMATCH],
    );
    let base = native_bundle(true);
    let mut context = rebound;
    context.policy_digest = [0; 32];
    assert_eq!(
        validate_request_bindings_v1(&r, &context, &base),
        Err(ReasonV1::E_BINDING_MISMATCH)
    );
}
#[test]
fn email_binding_catalog_canonicalization_versions_and_nesting() {
    let mut catalog = email_catalog("a", "a");
    catalog.templates.push(EmailTemplateV1 {
        body: "second approved version".into(),
        ..email_template()
    });
    let digest = email_policy_digest_v1(&[17; 32], &catalog).unwrap();
    catalog.templates.reverse();
    assert_eq!(email_policy_digest_v1(&[17; 32], &catalog).unwrap(), digest);
    let p = prepare_email_policy_v1(native_bundle(true), &catalog).unwrap();
    assert_ne!(p.base_digest(), p.digest());
    assert_eq!(
        prepare_email_policy_v1(p.bundle().clone(), &catalog).unwrap_err(),
        BundleErrorV1::SettingsInvalid
    );
    catalog.templates.push(catalog.templates[0].clone());
    assert_eq!(
        email_policy_digest_v1(&[17; 32], &catalog),
        Err(BundleErrorV1::SettingsInvalid)
    );
    catalog.templates.clear();
    assert_eq!(
        email_policy_digest_v1(&[17; 32], &catalog),
        Err(BundleErrorV1::SettingsInvalid)
    );
    for i in 0..64 {
        catalog.templates.push(EmailTemplateV1 {
            template_id: format!("t{i}"),
            ..email_template()
        });
    }
    assert!(email_policy_digest_v1(&[17; 32], &catalog).is_ok());
    catalog.templates.push(email_template());
    assert_eq!(
        email_policy_digest_v1(&[17; 32], &catalog),
        Err(BundleErrorV1::StructureLimitExceeded)
    );
}
#[test]
fn email_binding_approved_rendered_version_is_positive() {
    let mut catalog = email_catalog("a", "a");
    catalog.templates.push(EmailTemplateV1 {
        body: "second approved version".into(),
        ..email_template()
    });
    let p = prepare_email_policy_v1(native_bundle(true), &catalog).unwrap();
    let (mut r, mut c, _, _) = email_fixture();
    email_rebind_policy(&r, &mut c, p.bundle());
    let mut value = email_payload();
    value["body"] = "second approved version".into();
    let payload = email_commit_payload(&mut r, &value);
    c.evidence.approved_template_digest =
        Some(email_template_digest_v1(&catalog.templates[1]).unwrap());
    email_assert(&r, &c, &p, &payload, &[]);
}
#[test]
fn email_binding_identifier_and_recipient_byte_boundaries() {
    let (mut r, c, p, _) = email_fixture();
    for field in ["order_id", "template_id", "draft_id", "recipient"] {
        let cap = if field == "recipient" { 1024 } else { 128 };
        for value in [
            String::new(),
            "x".repeat(cap + 1),
            "é".repeat(cap / 2 + 1),
            "x\n".into(),
            "x\r".into(),
            "x\t".into(),
            "x\0".into(),
            "x\u{0085}".into(),
        ] {
            let mut body = email_payload();
            body[field] = value.into();
            let payload = email_commit_payload(&mut r, &body);
            email_assert(&r, &c, &p, &payload, &[ReasonV1::E_MALFORMED_REQUEST]);
        }
        for value in ["x".repeat(cap), "é".repeat(cap / 2)] {
            let mut body = email_payload();
            body[field] = value.into();
            let payload = email_commit_payload(&mut r, &body);
            let causes = validate_email_bindings_v1(&r, &c, &p, Some(&payload))
                .err()
                .unwrap_or_default();
            assert!(
                !causes.contains(&ReasonV1::E_MALFORMED_REQUEST),
                "valid {field}"
            );
        }
    }
    for id in [String::new(), "x".repeat(129), "é".repeat(65), "x\n".into()] {
        let mut catalog = email_catalog("a", "a");
        catalog.profile_id = id;
        assert_eq!(
            email_policy_digest_v1(&[0; 32], &catalog),
            Err(BundleErrorV1::SettingsInvalid)
        );
    }
}
#[test]
fn email_binding_content_exact_controls_and_limits() {
    for field in ["subject", "body"] {
        for value in ["x\n", "x\r", "x\t", "x\0", "e\u{301}", "é"] {
            let mut catalog = email_catalog("a", "a");
            if field == "subject" {
                catalog.templates[0].subject = value.into();
            } else {
                catalog.templates[0].body = value.into();
            }
            let p = prepare_email_policy_v1(native_bundle(true), &catalog).unwrap();
            let (mut r, mut c, _, _) = email_fixture();
            let mut body = email_payload();
            body[field] = value.into();
            let payload = email_commit_payload(&mut r, &body);
            email_rebind_policy(&r, &mut c, p.bundle());
            c.evidence.approved_template_digest =
                Some(email_template_digest_v1(&catalog.templates[0]).unwrap());
            email_assert(&r, &c, &p, &payload, &[]);
        }
        for value in [String::new(), "x".repeat(1025), "é".repeat(513)] {
            let mut t = email_template();
            if field == "subject" {
                t.subject = value;
            } else {
                t.body = value;
            }
            assert_eq!(
                email_template_digest_v1(&t),
                Err(ReasonV1::E_MALFORMED_REQUEST)
            );
        }
        for value in ["x".repeat(1024), "é".repeat(512)] {
            let mut t = email_template();
            if field == "subject" {
                t.subject = value;
            } else {
                t.body = value;
            }
            assert!(email_template_digest_v1(&t).is_ok());
        }
    }
    let a = EmailTemplateV1 {
        subject: "ab".into(),
        body: "c".into(),
        ..email_template()
    };
    let b = EmailTemplateV1 {
        subject: "a".into(),
        body: "bc".into(),
        ..email_template()
    };
    assert_ne!(
        email_template_digest_v1(&a).unwrap(),
        email_template_digest_v1(&b).unwrap()
    );
}
#[test]
fn email_binding_policy_identity_all_operands_and_catalog_order() {
    let catalog = email_catalog("a", "a");
    let digest = email_policy_digest_v1(&[17; 32], &catalog).unwrap();
    assert_ne!(email_policy_digest_v1(&[18; 32], &catalog).unwrap(), digest);
    for field in 0..5 {
        let mut c = catalog.clone();
        match field {
            0 => c.profile_id.push('x'),
            1 => c.shop_id.push('x'),
            2 => c.templates[0].template_id.push('x'),
            3 => c.templates[0].subject.push('x'),
            _ => c.templates[0].body.push('x'),
        };
        assert_ne!(
            email_policy_digest_v1(&[17; 32], &c).unwrap(),
            digest,
            "policy operand {field}"
        );
    }
    let mut c = catalog;
    c.templates.push(EmailTemplateV1 {
        template_id: "é".into(),
        ..email_template()
    });
    c.templates.push(EmailTemplateV1 {
        template_id: "Z".into(),
        ..email_template()
    });
    let d = email_policy_digest_v1(&[17; 32], &c).unwrap();
    c.templates.reverse();
    assert_eq!(email_policy_digest_v1(&[17; 32], &c).unwrap(), d);
}

// Deliberately bounded lexical discovery, not a general Rust type resolver.
// Resolve the declared alias/wrapper examples, and inventory the real direct
// signatures. Private helpers and policy-wrapper consumers get separate roles.
fn email_discovered_consumers(source: &str, indirect_types: &[&str]) -> Vec<String> {
    // Handle simple, explicitly shaped aliases/wrappers; this is not general
    // name resolution, macro expansion or a compiler call graph.
    let mut bundle_types = vec![
        "PreparedCommerceBundleV1",
        "PreparedEmailPolicyV1",
        "CommercePolicyInputV1",
    ];
    bundle_types.extend_from_slice(indirect_types);
    for alias in source.split("type ").skip(1) {
        if let Some((name, rest)) = alias.split_once('=') {
            let target = rest.split(';').next().unwrap_or_default();
            if bundle_types.iter().any(|ty| target.contains(ty)) {
                bundle_types.push(name.trim());
            }
        }
    }
    for record in source.split("struct ").skip(1) {
        if let Some((name, rest)) = record.split_once('{') {
            let fields = rest.split('}').next().unwrap_or_default();
            if bundle_types.iter().any(|ty| fields.contains(ty)) {
                bundle_types.push(name.trim());
            }
        }
    }
    source
        .split("fn ")
        .skip(1)
        .filter_map(|item| {
            let (signature, _) = item.split_once('{')?;
            let (name, _) = signature.split_once('(')?;
            let request = signature.contains("CommerceRequestV1");
            let bundle = bundle_types.iter().any(|ty| signature.contains(ty));
            (request && bundle).then(|| name.trim().to_owned())
        })
        .collect()
}
#[test]
fn email_guard_candidate_inventory_and_unclassified_alias_wrapper() {
    let source = include_str!("../src/commerce/decision.rs");
    let mut actual = email_discovered_consumers(source, &[]);
    actual.sort();
    let mut expected = vec![
        "validate_request_bindings_v1",
        "validate_monetary_constraints_v1",
        "validate_budget_constraints_v1",
        "validate_order_window_and_discount_v1",
        "validate_evidence_snapshot_v1",
        "validate_review_artifacts_v1",
        "validate_provenance_and_reviews_v1",
        "resolve_review_requirements_v1",
        "resolve_review_routes_v1",
        "build_cedar_request_v1",
        "validate_email_policy_flavor_v1",
        "build_cedar_entities_v1",
        "validate_email_bindings_v1",
        "evaluate_permissions_v1",
        "evaluate_commerce_v1",
        // cfg(test)-only fixture constructor; no caller request or runtime entry.
        "permissions_test_fixture",
        // Same cfg(test) constructor with a caller-selected policy string; builds
        // a newly hashed/prepared synthetic bundle, not a runtime request path.
        "permissions_test_fixture_with_policy",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect::<Vec<_>>();
    expected.sort();
    assert_eq!(
        actual, expected,
        "new consumers need enrollment and evidence, not name-derived exclusion"
    );
    for (extra, types) in [
        (
            "fn unguarded_extra(r: &CommerceRequestV1, b: &PreparedCommerceBundleV1) -> Result<(), ReasonV1> { Ok(()) }",
            vec![],
        ),
        (
            "type BundleAlias = PreparedCommerceBundleV1; fn unguarded_alias(r: &CommerceRequestV1, b: &BundleAlias) -> Result<(), ReasonV1> { Ok(()) }",
            vec!["BundleAlias"],
        ),
        (
            "struct BundleHolder { value: PreparedCommerceBundleV1 } fn unguarded_wrapper(r: &CommerceRequestV1, b: &BundleHolder) -> Result<(), ReasonV1> { Ok(()) }",
            vec!["BundleHolder"],
        ),
    ] {
        let amended = format!("{source}\n{extra}");
        let found = email_discovered_consumers(&amended, &types);
        let automatic = email_discovered_consumers(&amended, &[]);
        assert!(automatic.iter().any(|name| !expected.contains(name)));
        assert!(found.iter().any(|name| !expected.contains(name)));
    }
}

#[test]
fn email_binding_recipient_count_lossless_and_delimiters() {
    let (mut r, _, _) = frame();
    let one = recipient_digest_v1(&r, "customer", 1).unwrap();
    assert_ne!(recipient_digest_v1(&r, "customer", 65537).unwrap(), one);
    assert_ne!(recipient_digest_v1(&r, "customer", u32::MAX).unwrap(), one);
    r.profile_id = "ab".into();
    r.shop_id = "c".into();
    let first = recipient_digest_v1(&r, "customer", 1).unwrap();
    r.profile_id = "a".into();
    r.shop_id = "bc".into();
    assert_ne!(recipient_digest_v1(&r, "customer", 1).unwrap(), first);
}

#[test]
fn email_binding_payload_length_independent_of_hash() {
    let (mut r, c, p, payload) = email_fixture();
    r.payload_length += 1;
    email_assert(&r, &c, &p, &payload, &[ReasonV1::E_BINDING_MISMATCH]);
    r.payload_length -= 2;
    email_assert(&r, &c, &p, &payload, &[ReasonV1::E_BINDING_MISMATCH]);
}

#[test]
fn email_binding_content_golden_and_no_normalization() {
    let t = EmailTemplateV1 {
        template_id: "template-a".into(),
        subject: "Line\n\0".into(),
        body: "Body\r\té".into(),
    };
    assert_eq!(
        hex(&email_template_digest_v1(&t).unwrap()),
        "244b2414c7a54de153aac4acf4ec58fb56626ecd6621687588240f22c57acb63"
    );
    for (left, right) in [("e\u{301}", "é"), ("x\r\n", "x\n"), ("x\n", "x")] {
        let a = EmailTemplateV1 {
            body: left.into(),
            ..email_template()
        };
        let b = EmailTemplateV1 {
            body: right.into(),
            ..email_template()
        };
        assert_ne!(
            email_template_digest_v1(&a).unwrap(),
            email_template_digest_v1(&b).unwrap()
        );
    }
}

#[test]
fn email_binding_absent_input_is_not_supplied_malformed_input() {
    let (r, c, p, _) = email_fixture();
    assert_eq!(
        validate_email_bindings_v1(&r, &c, &p, None),
        Err(vec![ReasonV1::E_MISSING_EVIDENCE])
    );
    let supplied = EmailPayloadSourceV1 { bytes: vec![] };
    assert_eq!(
        validate_email_bindings_v1(&r, &c, &p, Some(&supplied)),
        Err(vec![ReasonV1::E_MALFORMED_REQUEST])
    );
}

fn permissions_fixture(
    policy: &str,
    enabled: bool,
) -> (
    CommerceRequestV1,
    DecisionContextV1,
    PreparedCommerceBundleV1,
) {
    let mut source = preparation_source();
    source.schema_text = NATIVE_SCHEMA.into();
    source.permissions_text = policy.into();
    for (_, settings) in &mut source.action_settings {
        settings.enabled = enabled;
    }
    recommit(&mut source);
    let bundle = prepare_bundle_v1(&source, &[7; 32]).unwrap();
    let (request, context) = bound_frame(&bundle);
    (request, context, bundle)
}

macro_rules! permissions_action {
    ($name:ident, $action:ident) => {
        #[test]
        fn $name() {
            let (mut r, mut c, b) =
                permissions_fixture("permit(principal, action, resource);", true);
            r.action = CommerceActionV1::$action;
            let b = email_migration_bundle(b, r.action);
            email_rebind_policy(&r, &mut c, &b);
            assert_eq!(evaluate_permissions_v1(&r, &c, &b), Ok(()));
        }
    };
}
permissions_action!(permissions_refund_error_free_permit, RefundCreate);
permissions_action!(permissions_address_error_free_permit, OrderAddressUpdate);
permissions_action!(permissions_cancel_error_free_permit, OrderCancel);
permissions_action!(permissions_discount_error_free_permit, DiscountCreate);
permissions_action!(permissions_email_annex_error_free_permit, CustomerEmailSend);

#[test]
fn permissions_no_matching_permit() {
    let (r, c, b) =
        permissions_fixture("permit(principal, action, resource) when { false };", true);
    assert_eq!(
        evaluate_permissions_v1(&r, &c, &b),
        Err(vec![ReasonV1::E_POLICY_FORBID])
    );
}
#[test]
fn permissions_empty_policy_set() {
    let (r, c, b) = permissions_fixture("", true);
    assert_eq!(
        evaluate_permissions_v1(&r, &c, &b),
        Err(vec![ReasonV1::E_POLICY_FORBID])
    );
}
#[test]
fn permissions_forbid_overrides_matching_permit() {
    let (r, c, b) = permissions_fixture(
        "permit(principal, action, resource); forbid(principal, action, resource);",
        true,
    );
    assert_eq!(
        evaluate_permissions_v1(&r, &c, &b),
        Err(vec![ReasonV1::E_POLICY_FORBID])
    );
}
#[test]
fn permissions_error_free_allow_does_not_hide_error() {
    let (r, c, b) = permissions_fixture(
        "permit(principal, action, resource); permit(principal, action, resource) when { 9223372036854775807 + 1 == 0 };",
        true,
    );
    let inputs = build_cedar_request_v1(&r, &c, &b).unwrap();
    let raw = cedar_policy::Authorizer::new().is_authorized(
        inputs.request(),
        b.policies(),
        inputs.entities(),
    );
    assert_eq!(raw.decision(), cedar_policy::Decision::Allow);
    assert_eq!(raw.diagnostics().errors().count(), 1);
    assert_eq!(
        evaluate_permissions_v1(&r, &c, &b),
        Err(vec![ReasonV1::E_INTERNAL_EVALUATION])
    );
}
#[test]
fn permissions_error_and_actual_deny_are_both_retained() {
    let (r, c, b) = permissions_fixture(
        "forbid(principal, action, resource); permit(principal, action, resource) when { 9223372036854775807 + 1 == 0 };",
        true,
    );
    assert_eq!(
        evaluate_permissions_v1(&r, &c, &b),
        Err(vec![
            ReasonV1::E_INTERNAL_EVALUATION,
            ReasonV1::E_POLICY_FORBID
        ])
    );
}
#[test]
fn permissions_error_without_valid_permit_is_not_allow() {
    let (r, c, b) = permissions_fixture(
        "permit(principal, action, resource) when { 9223372036854775807 + 1 == 0 };",
        true,
    );
    assert_eq!(
        evaluate_permissions_v1(&r, &c, &b),
        Err(vec![
            ReasonV1::E_INTERNAL_EVALUATION,
            ReasonV1::E_POLICY_FORBID
        ])
    );
}
#[test]
fn permissions_multiple_errors_are_one_registry_cause() {
    let (r, c, b) = permissions_fixture(
        "permit(principal, action, resource); permit(principal, action, resource) when { 9223372036854775807 + 1 == 0 }; forbid(principal, action, resource) when { 9223372036854775807 + 1 == 0 };",
        true,
    );
    assert_eq!(
        evaluate_permissions_v1(&r, &c, &b),
        Err(vec![ReasonV1::E_INTERNAL_EVALUATION])
    );
}
#[test]
fn permissions_disabled_refusal_is_not_counterfactual_policy_deny() {
    let (r, c, b) = permissions_fixture("forbid(principal, action, resource);", false);
    assert_eq!(
        evaluate_permissions_v1(&r, &c, &b),
        Err(vec![ReasonV1::E_POLICY_UNCONFIGURED])
    );
}
#[test]
fn permissions_representation_refusal_precedes_binding_and_configuration() {
    let (mut r, mut c, b) = permissions_fixture("permit(principal, action, resource);", false);
    r.subject_id.clear();
    c.authenticated_shop = "different".into();
    assert_eq!(
        evaluate_permissions_v1(&r, &c, &b),
        Err(vec![ReasonV1::E_MALFORMED_REQUEST])
    );
}
#[test]
fn permissions_binding_refusal_precedes_disabled_setting() {
    let (r, mut c, b) = permissions_fixture("permit(principal, action, resource);", false);
    c.authenticated_subject = "different".into();
    assert_eq!(
        evaluate_permissions_v1(&r, &c, &b),
        Err(vec![ReasonV1::E_BINDING_MISMATCH])
    );
}
#[test]
fn permissions_valid_optional_artifacts_are_bound_before_cedar() {
    let (r, c, b) = permissions_fixture("permit(principal, action, resource);", true);
    for field in 0..4 {
        let mut changed = c.clone();
        match field {
            0 => changed.evidence.provenance_binding_digest = Some([42; 32]),
            1 => changed.review.grant.as_mut().unwrap().binding_digest[0] ^= 1,
            2 => changed.review.attestation.as_mut().unwrap().policy_digest[0] ^= 1,
            _ => changed.review_request.as_mut().unwrap().binding_digest[0] ^= 1,
        }
        assert_eq!(
            evaluate_permissions_v1(&r, &changed, &b),
            Err(vec![ReasonV1::E_BINDING_MISMATCH]),
            "artifact {field}"
        );
    }
}
#[test]
fn permissions_success_is_not_commerce_or_email_acceptance() {
    let (r, mut c, b) = permissions_fixture("permit(principal, action, resource);", true);
    c.evidence.provenance = ProvenanceStateV1::Invalid;
    c.evidence.line_items_eligible = Some(false);
    c.review.grant.as_mut().unwrap().authorized = false;
    c.budget.count_used = i64::MAX;
    assert_eq!(evaluate_permissions_v1(&r, &c, &b), Ok(()));
    assert!(validate_action_eligibility_v1(&r, &c).is_err());
    assert!(validate_provenance_and_reviews_v1(&r, &c, &b).is_err());
    assert!(validate_budget_constraints_v1(&r, &c, &b).is_err());
    let (r, mut c, p, payload) = email_fixture();
    c.evidence.derived_recipient_digest = Some([42; 32]);
    assert_eq!(evaluate_permissions_v1(&r, &c, p.bundle()), Ok(()));
    assert!(validate_email_bindings_v1(&r, &c, &p, Some(&payload)).is_err());
}

#[test]
#[allow(non_snake_case)] // Adopted per-consumer slot naming convention.
fn email_guard__evaluate_permissions_v1__base_email_refused() {
    let (r, c, b) = email_guard_fixture(false, CommerceActionV1::CustomerEmailSend, false);
    assert_eq!(
        evaluate_permissions_v1(&r, &c, &b),
        Err(vec![ReasonV1::E_POLICY_UNCONFIGURED])
    );
}
#[test]
#[allow(non_snake_case)]
fn email_guard__evaluate_permissions_v1__annex_email_accepted() {
    let (r, c, b) = email_guard_fixture(true, CommerceActionV1::CustomerEmailSend, false);
    assert_eq!(evaluate_permissions_v1(&r, &c, &b), Ok(()));
}
#[test]
#[allow(non_snake_case)]
fn email_guard__evaluate_permissions_v1__base_other_action_accepted() {
    let (r, c, b) = email_guard_fixture(false, CommerceActionV1::OrderCancel, false);
    assert_eq!(evaluate_permissions_v1(&r, &c, &b), Ok(()));
}
#[test]
#[allow(non_snake_case)]
fn email_guard__evaluate_permissions_v1__annex_scope_rejected() {
    let (r, c, b) = email_guard_fixture(true, CommerceActionV1::CustomerEmailSend, true);
    assert_eq!(
        evaluate_permissions_v1(&r, &c, &b),
        Err(vec![ReasonV1::E_BINDING_MISMATCH])
    );
}

#[test]
fn permissions_enablement_is_selected_for_requested_action() {
    let mut source = preparation_source();
    source.schema_text = NATIVE_SCHEMA.into();
    for (action, settings) in &mut source.action_settings {
        settings.enabled = *action == CommerceActionV1::OrderCancel;
    }
    recommit(&mut source);
    let b = prepare_bundle_v1(&source, &[7; 32]).unwrap();
    let (mut r, mut c) = bound_frame(&b);
    r.action = CommerceActionV1::OrderCancel;
    email_rebind_policy(&r, &mut c, &b);
    assert_eq!(evaluate_permissions_v1(&r, &c, &b), Ok(()));
    r.action = CommerceActionV1::RefundCreate;
    email_rebind_policy(&r, &mut c, &b);
    assert_eq!(
        evaluate_permissions_v1(&r, &c, &b),
        Err(vec![ReasonV1::E_POLICY_UNCONFIGURED])
    );
}

// Expected bodies below are authored from the contract, never captured evaluator output.
fn composition_fixture(
    action: CommerceActionV1,
    policy: &str,
    edit: impl Fn(&mut CommerceActionSettingsV1),
) -> (
    CommerceRequestV1,
    DecisionContextV1,
    PreparedCommerceBundleV1,
) {
    let mut source = preparation_source();
    source.schema_text = NATIVE_SCHEMA.into();
    source.permissions_text = policy.into();
    for (_, s) in &mut source.action_settings {
        s.require_provenance = false;
        s.attestation_enabled = false;
        edit(s);
    }
    recommit(&mut source);
    let b = prepare_bundle_v1(&source, &[7; 32]).unwrap();
    let (mut r, mut c, _) = frame();
    r.action = action;
    let monetary = matches!(
        action,
        CommerceActionV1::RefundCreate | CommerceActionV1::DiscountCreate
    );
    r.amount_minor = monetary.then_some(100);
    r.currency = monetary.then(|| "USD".into());
    c.now_ms = 50;
    c.evidence.fetched_at_ms = 50;
    c.evidence.captured_minor = Some(1000);
    c.evidence.prior_refunds_minor = Some(0);
    c.evidence.order_age_seconds = Some(100);
    c.evidence.line_items_eligible = Some(true);
    c.evidence.any_fulfillment = Some(false);
    c.evidence.cancellation_eligible = Some(true);
    c.evidence.discount_conflict = Some(false);
    c.evidence.recipient_count = Some(1);
    c.budget.action = action;
    c.budget.currency = r.currency.clone();
    c.budget.window_length_ms = 60000;
    email_rebind_policy(&r, &mut c, &b);
    (r, c, b)
}
fn composition_email_fixture() -> (
    CommerceRequestV1,
    DecisionContextV1,
    PreparedEmailPolicyV1,
    EmailPayloadSourceV1,
) {
    let (mut r, mut c, b) = composition_fixture(
        CommerceActionV1::CustomerEmailSend,
        "permit(principal, action, resource);",
        |_| {},
    );
    let p = prepare_email_policy_v1(b, &email_catalog("a", "a")).unwrap();
    let payload = email_commit_payload(&mut r, &email_payload());
    c.evidence.derived_recipient_digest =
        Some(recipient_digest_v1(&r, "customer@example.test", 1).unwrap());
    c.evidence.approved_template_digest =
        Some(email_template_digest_v1(&email_template()).unwrap());
    email_rebind_policy(&r, &mut c, p.bundle());
    (r, c, p, payload)
}
fn composition_expected(
    r: &CommerceRequestV1,
    c: &DecisionContextV1,
    b: &PreparedCommerceBundleV1,
    outcome: CommerceOutcomeV1,
    reasons: &[ReasonV1],
    ids: &[&str],
    escalation: Option<EscalationRequirementV1>,
) -> CommerceDecisionV1 {
    let monetary = matches!(
        r.action,
        CommerceActionV1::RefundCreate | CommerceActionV1::DiscountCreate
    );
    let constraints = (outcome == CommerceOutcomeV1::Allow).then(|| CommerceConstraintsV1 {
        profile_id: r.profile_id.clone(),
        shop_id: r.shop_id.clone(),
        subject_id: r.subject_id.clone(),
        operation_id: r.operation_id.clone(),
        resource_id: r.resource_id.clone(),
        action: r.action,
        payload_digest: r.payload_digest,
        amount_ceiling_minor: if monetary { r.amount_minor } else { None },
        currency: if monetary { r.currency.clone() } else { None },
        evidence_revision: c.evidence.source_revision.clone(),
        policy_digest: *b.digest(),
        registry_digest: *b.registry_digest(),
        review_ids: ids.iter().map(|id| (*id).into()).collect(),
        evaluation_time_ms: c.now_ms,
    });
    let mut expected = CommerceDecisionV1 {
        outcome,
        primary_reason: reasons
            .first()
            .copied()
            .unwrap_or(ReasonV1::OK_POLICY_PERMIT),
        diagnostics: reasons.iter().skip(1).copied().collect(),
        constraints,
        escalation,
        decision_digest: [0; 32],
        policy_digest: *b.digest(),
        registry_digest: *b.registry_digest(),
        evidence_digest: c.evidence.evidence_digest,
    };
    // This independently constructs the body; shared encoding checks digest wiring,
    // not independent cryptographic derivation. Existing fixed vectors anchor encoding.
    expected.decision_digest = decision_digest_v1(r, c, b.digest(), &expected).unwrap();
    expected
}
fn composition_assert(
    r: &CommerceRequestV1,
    c: &DecisionContextV1,
    b: &PreparedCommerceBundleV1,
    outcome: CommerceOutcomeV1,
    reasons: &[ReasonV1],
    ids: &[&str],
    escalation: Option<EscalationRequirementV1>,
) {
    let expected = composition_expected(r, c, b, outcome, reasons, ids, escalation);
    assert_eq!(
        evaluate_commerce_v1(r, c, &CommercePolicyInputV1::Base(b)),
        Ok(expected)
    );
}
macro_rules! composition_positive {
    ($name:ident,$action:ident) => {
        #[test]
        fn $name() {
            let (r, c, b) = composition_fixture(
                CommerceActionV1::$action,
                "permit(principal, action, resource);",
                |_| {},
            );
            composition_assert(&r, &c, &b, CommerceOutcomeV1::Allow, &[], &[], None);
        }
    };
}
composition_positive!(composition_refund_allow, RefundCreate);
composition_positive!(composition_address_allow, OrderAddressUpdate);
composition_positive!(composition_cancel_allow, OrderCancel);
composition_positive!(composition_discount_allow, DiscountCreate);
#[test]
fn composition_email_allow() {
    let (r, c, p, payload) = composition_email_fixture();
    let expected =
        composition_expected(&r, &c, p.bundle(), CommerceOutcomeV1::Allow, &[], &[], None);
    assert_eq!(
        evaluate_commerce_v1(
            &r,
            &c,
            &CommercePolicyInputV1::Email {
                policy: &p,
                payload: Some(&payload)
            }
        ),
        Ok(expected)
    );
}
#[test]
fn composition_representation_error_has_no_decision() {
    let (r, c, b) = composition_fixture(
        CommerceActionV1::RefundCreate,
        "permit(principal, action, resource);",
        |_| {},
    );
    let mut bad = r.clone();
    bad.currency = Some("usd".into());
    assert_eq!(
        evaluate_commerce_v1(&bad, &c, &CommercePolicyInputV1::Base(&b)),
        Err(ReasonV1::E_MALFORMED_REQUEST)
    );
    let mut bad = c;
    bad.now_ms = u64::MAX;
    assert_eq!(
        evaluate_commerce_v1(&r, &bad, &CommercePolicyInputV1::Base(&b)),
        Err(ReasonV1::E_MALFORMED_REQUEST)
    );
}
#[test]
fn composition_disabled_is_not_policy_deny() {
    let (r, c, b) = composition_fixture(
        CommerceActionV1::RefundCreate,
        "forbid(principal, action, resource);",
        |s| s.enabled = false,
    );
    composition_assert(
        &r,
        &c,
        &b,
        CommerceOutcomeV1::Deny,
        &[ReasonV1::E_POLICY_UNCONFIGURED],
        &[],
        None,
    );
}
#[test]
fn composition_identity_binding_denies() {
    let (r, mut c, b) = composition_fixture(
        CommerceActionV1::RefundCreate,
        "permit(principal, action, resource);",
        |_| {},
    );
    c.authenticated_subject = "other".into();
    composition_assert(
        &r,
        &c,
        &b,
        CommerceOutcomeV1::Deny,
        &[ReasonV1::E_BINDING_MISMATCH],
        &[],
        None,
    );
}
#[test]
fn composition_evidence_freshness_denies() {
    let (r, mut c, b) = composition_fixture(
        CommerceActionV1::RefundCreate,
        "permit(principal, action, resource);",
        |_| {},
    );
    c.now_ms = 1051;
    c.evidence.fetched_at_ms = 50;
    composition_assert(
        &r,
        &c,
        &b,
        CommerceOutcomeV1::Deny,
        &[ReasonV1::E_MISSING_EVIDENCE],
        &[],
        None,
    );
}
#[test]
fn composition_action_eligibility_denies() {
    let (r, mut c, b) = composition_fixture(
        CommerceActionV1::RefundCreate,
        "permit(principal, action, resource);",
        |_| {},
    );
    c.evidence.line_items_eligible = Some(false);
    composition_assert(
        &r,
        &c,
        &b,
        CommerceOutcomeV1::Deny,
        &[ReasonV1::E_CONSTRAINT_VIOLATION],
        &[],
        None,
    );
}
#[test]
fn composition_monetary_denies() {
    let (mut r, mut c, b) = composition_fixture(
        CommerceActionV1::RefundCreate,
        "permit(principal, action, resource);",
        |_| {},
    );
    r.amount_minor = Some(1001);
    email_rebind_policy(&r, &mut c, &b);
    composition_assert(
        &r,
        &c,
        &b,
        CommerceOutcomeV1::Deny,
        &[ReasonV1::E_AMOUNT_LIMIT],
        &[],
        None,
    );
}
#[test]
fn composition_budget_denies() {
    let (r, mut c, b) = composition_fixture(
        CommerceActionV1::RefundCreate,
        "permit(principal, action, resource);",
        |_| {},
    );
    c.budget.count_used = 3;
    composition_assert(
        &r,
        &c,
        &b,
        CommerceOutcomeV1::Deny,
        &[ReasonV1::E_COUNT_LIMIT],
        &[],
        None,
    );
}
#[test]
fn composition_cedar_denies() {
    let (r, c, b) = composition_fixture(
        CommerceActionV1::RefundCreate,
        "forbid(principal, action, resource);",
        |_| {},
    );
    composition_assert(
        &r,
        &c,
        &b,
        CommerceOutcomeV1::Deny,
        &[ReasonV1::E_POLICY_FORBID],
        &[],
        None,
    );
}
#[test]
fn composition_cedar_errors_override_established_policy_deny() {
    let (r, mut c, b) = composition_fixture(
        CommerceActionV1::RefundCreate,
        "permit(principal, action, resource) when { (9223372036854775807 + 1) == 0 };",
        |_| {},
    );
    c.evidence.line_items_eligible = Some(false);
    composition_assert(
        &r,
        &c,
        &b,
        CommerceOutcomeV1::Deny,
        &[
            ReasonV1::E_INTERNAL_EVALUATION,
            ReasonV1::E_POLICY_FORBID,
            ReasonV1::E_CONSTRAINT_VIOLATION,
        ],
        &[],
        None,
    );
}
#[test]
fn composition_missing_provenance_escalates_only_enabled_attestation() {
    let (r, c, b) = composition_fixture(
        CommerceActionV1::RefundCreate,
        "permit(principal, action, resource);",
        |s| {
            s.require_provenance = true;
            s.attestation_enabled = true;
        },
    );
    composition_assert(
        &r,
        &c,
        &b,
        CommerceOutcomeV1::Escalate,
        &[ReasonV1::E_PROVENANCE_UNVERIFIABLE],
        &[],
        Some(EscalationRequirementV1 {
            requested_at_ms: 50,
            expires_at_ms: 1050,
            review_request_id: None,
            required_kinds: vec![ReviewKindV1::Attestation],
        }),
    );
    let (r, c, b) = composition_fixture(
        CommerceActionV1::RefundCreate,
        "permit(principal, action, resource);",
        |s| s.require_provenance = true,
    );
    composition_assert(
        &r,
        &c,
        &b,
        CommerceOutcomeV1::Deny,
        &[ReasonV1::E_PROVENANCE_UNVERIFIABLE],
        &[],
        None,
    );
}
#[test]
fn composition_invalid_bound_provenance_is_terminal() {
    let (r, mut c, b) = composition_fixture(
        CommerceActionV1::RefundCreate,
        "permit(principal, action, resource);",
        |s| {
            s.require_provenance = true;
            s.attestation_enabled = true;
        },
    );
    c.evidence.provenance = ProvenanceStateV1::Invalid;
    c.evidence.provenance_binding_digest = Some(request_binding_digest_v1(&r).unwrap());
    c.review.attestation = Some(routing_fact(&r, &c, ReviewKindV1::Attestation));
    composition_assert(
        &r,
        &c,
        &b,
        CommerceOutcomeV1::Deny,
        &[ReasonV1::E_PROVENANCE_UNVERIFIABLE],
        &[],
        None,
    );
}
#[test]
fn composition_consumed_provenance_is_terminal() {
    let (r, mut c, b) = composition_fixture(
        CommerceActionV1::RefundCreate,
        "permit(principal, action, resource);",
        |_| {},
    );
    c.evidence.provenance = ProvenanceStateV1::Consumed;
    c.evidence.provenance_binding_digest = Some(request_binding_digest_v1(&r).unwrap());
    composition_assert(
        &r,
        &c,
        &b,
        CommerceOutcomeV1::Deny,
        &[ReasonV1::E_REPLAY],
        &[],
        None,
    );
}
#[test]
fn composition_unused_invalid_review_stays_terminal() {
    let (r, mut c, b) = composition_fixture(
        CommerceActionV1::RefundCreate,
        "permit(principal, action, resource);",
        |_| {},
    );
    c.review.grant = Some(routing_fact(&r, &c, ReviewKindV1::Grant));
    c.review.grant.as_mut().unwrap().authorized = false;
    composition_assert(
        &r,
        &c,
        &b,
        CommerceOutcomeV1::Deny,
        &[ReasonV1::E_APPROVAL_INVALID],
        &[],
        None,
    );
}
#[test]
fn composition_used_reviews_only_and_exact_requested_amount() {
    let (r, mut c, b) = composition_fixture(
        CommerceActionV1::RefundCreate,
        "permit(principal, action, resource);",
        |s| {
            s.require_provenance = true;
            s.attestation_enabled = true;
            s.require_monetary_review = true;
        },
    );
    c.review.grant = Some(routing_fact(&r, &c, ReviewKindV1::Grant));
    c.review.attestation = Some(routing_fact(&r, &c, ReviewKindV1::Attestation));
    composition_assert(
        &r,
        &c,
        &b,
        CommerceOutcomeV1::Allow,
        &[],
        &["attestation", "grant"],
        None,
    );
    c.review.attestation.as_mut().unwrap().reviewer_id = "grant-reviewer".into();
    composition_assert(
        &r,
        &c,
        &b,
        CommerceOutcomeV1::Deny,
        &[ReasonV1::E_REVIEWER_SEPARATION],
        &[],
        None,
    );
}
#[test]
fn composition_unused_valid_review_omitted_but_digest_commits_it() {
    let (r, mut c, b) = composition_fixture(
        CommerceActionV1::RefundCreate,
        "permit(principal, action, resource);",
        |_| {},
    );
    let first = evaluate_commerce_v1(&r, &c, &CommercePolicyInputV1::Base(&b)).unwrap();
    c.review.grant = Some(routing_fact(&r, &c, ReviewKindV1::Grant));
    composition_assert(&r, &c, &b, CommerceOutcomeV1::Allow, &[], &[], None);
    let next = evaluate_commerce_v1(&r, &c, &CommercePolicyInputV1::Base(&b)).unwrap();
    assert_ne!(first.decision_digest, next.decision_digest);
    assert_eq!(first.constraints, next.constraints);
}
#[test]
fn composition_order_route_requires_then_uses_valid_grant() {
    let (r, mut c, b) = composition_fixture(
        CommerceActionV1::RefundCreate,
        "permit(principal, action, resource);",
        |s| s.review_routes = vec![ReviewRouteV1::OrderWindow],
    );
    c.evidence.order_age_seconds = Some(1001);
    composition_assert(
        &r,
        &c,
        &b,
        CommerceOutcomeV1::Escalate,
        &[ReasonV1::E_ORDER_WINDOW, ReasonV1::E_APPROVAL_REQUIRED],
        &[],
        Some(EscalationRequirementV1 {
            requested_at_ms: 50,
            expires_at_ms: 1050,
            review_request_id: None,
            required_kinds: vec![ReviewKindV1::Grant],
        }),
    );
    c.review.grant = Some(routing_fact(&r, &c, ReviewKindV1::Grant));
    composition_assert(&r, &c, &b, CommerceOutcomeV1::Allow, &[], &["grant"], None);
}
#[test]
fn composition_unconfigured_window_is_deny() {
    let (r, mut c, b) = composition_fixture(
        CommerceActionV1::RefundCreate,
        "permit(principal, action, resource);",
        |_| {},
    );
    c.evidence.order_age_seconds = Some(1001);
    c.review.grant = Some(routing_fact(&r, &c, ReviewKindV1::Grant));
    composition_assert(
        &r,
        &c,
        &b,
        CommerceOutcomeV1::Deny,
        &[ReasonV1::E_ORDER_WINDOW],
        &[],
        None,
    );
}
#[test]
fn composition_hard_fault_reinstates_satisfied_route_cause() {
    let (r, mut c, b) = composition_fixture(
        CommerceActionV1::RefundCreate,
        "permit(principal, action, resource);",
        |s| s.review_routes = vec![ReviewRouteV1::OrderWindow],
    );
    c.evidence.order_age_seconds = Some(1001);
    c.review.grant = Some(routing_fact(&r, &c, ReviewKindV1::Grant));
    c.budget.count_used = 3;
    composition_assert(
        &r,
        &c,
        &b,
        CommerceOutcomeV1::Deny,
        &[ReasonV1::E_COUNT_LIMIT, ReasonV1::E_ORDER_WINDOW],
        &[],
        None,
    );
}
#[test]
fn composition_cedar_denial_cannot_escalate() {
    let (r, mut c, b) = composition_fixture(
        CommerceActionV1::RefundCreate,
        "forbid(principal, action, resource);",
        |s| s.review_routes = vec![ReviewRouteV1::OrderWindow],
    );
    c.evidence.order_age_seconds = Some(1001);
    composition_assert(
        &r,
        &c,
        &b,
        CommerceOutcomeV1::Deny,
        &[ReasonV1::E_POLICY_FORBID, ReasonV1::E_ORDER_WINDOW],
        &[],
        None,
    );
}
#[test]
fn composition_pending_deadline_retained_then_expires_with_facts() {
    let (r, mut c, b) = composition_fixture(
        CommerceActionV1::RefundCreate,
        "permit(principal, action, resource);",
        |s| s.require_monetary_review = true,
    );
    c.review_request = Some(routing_pending(&r, &c));
    composition_assert(
        &r,
        &c,
        &b,
        CommerceOutcomeV1::Escalate,
        &[ReasonV1::E_APPROVAL_REQUIRED],
        &[],
        Some(EscalationRequirementV1 {
            requested_at_ms: 10,
            expires_at_ms: 200,
            review_request_id: Some("pending".into()),
            required_kinds: vec![ReviewKindV1::Grant],
        }),
    );
    c.now_ms = 199;
    c.review.grant = Some(routing_fact(&r, &c, ReviewKindV1::Grant));
    composition_assert(&r, &c, &b, CommerceOutcomeV1::Allow, &[], &["grant"], None);
    c.now_ms = 200;
    composition_assert(
        &r,
        &c,
        &b,
        CommerceOutcomeV1::Deny,
        &[ReasonV1::E_APPROVAL_EXPIRED],
        &[],
        None,
    );
}
#[test]
fn composition_multifault_primary_complete_deduplicated_diagnostics() {
    let (r, mut c, b) = composition_fixture(
        CommerceActionV1::RefundCreate,
        "forbid(principal, action, resource);",
        |_| {},
    );
    c.evidence.captured_minor = None;
    c.evidence.prior_refunds_minor = None;
    c.evidence.line_items_eligible = Some(false);
    c.budget.count_used = 3;
    composition_assert(
        &r,
        &c,
        &b,
        CommerceOutcomeV1::Deny,
        &[
            ReasonV1::E_MISSING_EVIDENCE,
            ReasonV1::E_POLICY_FORBID,
            ReasonV1::E_COUNT_LIMIT,
            ReasonV1::E_CONSTRAINT_VIOLATION,
        ],
        &[],
        None,
    );
}
#[test]
fn composition_email_wrong_recipient_and_unapproved_content() {
    let (mut r, mut c, p, _) = composition_email_fixture();
    let mut value = email_payload();
    value["recipient"] = "other@example.test".into();
    let payload = email_commit_payload(&mut r, &value);
    email_rebind_policy(&r, &mut c, p.bundle());
    let expected = composition_expected(
        &r,
        &c,
        p.bundle(),
        CommerceOutcomeV1::Deny,
        &[ReasonV1::E_BINDING_MISMATCH],
        &[],
        None,
    );
    assert_eq!(
        evaluate_commerce_v1(
            &r,
            &c,
            &CommercePolicyInputV1::Email {
                policy: &p,
                payload: Some(&payload)
            }
        ),
        Ok(expected)
    );
    value = email_payload();
    value["body"] = "Unapproved body".into();
    let payload = email_commit_payload(&mut r, &value);
    email_rebind_policy(&r, &mut c, p.bundle());
    c.evidence.approved_template_digest = Some(
        email_template_digest_v1(&EmailTemplateV1 {
            body: "Unapproved body".into(),
            ..email_template()
        })
        .unwrap(),
    );
    let expected = composition_expected(
        &r,
        &c,
        p.bundle(),
        CommerceOutcomeV1::Deny,
        &[ReasonV1::E_CONSTRAINT_VIOLATION],
        &[],
        None,
    );
    assert_eq!(
        evaluate_commerce_v1(
            &r,
            &c,
            &CommercePolicyInputV1::Email {
                policy: &p,
                payload: Some(&payload)
            }
        ),
        Ok(expected)
    );
}
#[test]
fn composition_email_absent_payload_and_marker_escape_refused() {
    let (r, c, p, _) = composition_email_fixture();
    let expected = composition_expected(
        &r,
        &c,
        p.bundle(),
        CommerceOutcomeV1::Deny,
        &[ReasonV1::E_MISSING_EVIDENCE],
        &[],
        None,
    );
    assert_eq!(
        evaluate_commerce_v1(
            &r,
            &c,
            &CommercePolicyInputV1::Email {
                policy: &p,
                payload: None
            }
        ),
        Ok(expected)
    );
    let expected = composition_expected(
        &r,
        &c,
        p.bundle(),
        CommerceOutcomeV1::Deny,
        &[ReasonV1::E_POLICY_UNCONFIGURED],
        &[],
        None,
    );
    assert_eq!(
        evaluate_commerce_v1(&r, &c, &CommercePolicyInputV1::Base(p.bundle())),
        Ok(expected)
    );
}
#[test]
#[allow(non_snake_case)] // Stable email correspondence slot identifiers.
fn email_guard__evaluate_commerce_v1__base_email_refused() {
    let (r, c, b) = composition_fixture(
        CommerceActionV1::CustomerEmailSend,
        "permit(principal, action, resource);",
        |_| {},
    );
    composition_assert(
        &r,
        &c,
        &b,
        CommerceOutcomeV1::Deny,
        &[ReasonV1::E_POLICY_UNCONFIGURED],
        &[],
        None,
    );
}
#[test]
#[allow(non_snake_case)] // Stable email correspondence slot identifiers.
fn email_guard__evaluate_commerce_v1__annex_email_accepted() {
    composition_email_allow();
}
#[test]
#[allow(non_snake_case)] // Stable email correspondence slot identifiers.
fn email_guard__evaluate_commerce_v1__base_other_action_accepted() {
    composition_cancel_allow();
}
#[test]
#[allow(non_snake_case)] // Stable email correspondence slot identifiers.
fn email_guard__evaluate_commerce_v1__annex_scope_rejected() {
    let (r, mut c, _, payload) = composition_email_fixture();
    let (_, _, base) = composition_fixture(
        CommerceActionV1::CustomerEmailSend,
        "permit(principal, action, resource);",
        |_| {},
    );
    let p = prepare_email_policy_v1(base, &email_catalog("other", "a")).unwrap();
    email_rebind_policy(&r, &mut c, p.bundle());
    let expected = composition_expected(
        &r,
        &c,
        p.bundle(),
        CommerceOutcomeV1::Deny,
        &[ReasonV1::E_BINDING_MISMATCH],
        &[],
        None,
    );
    assert_eq!(
        evaluate_commerce_v1(
            &r,
            &c,
            &CommercePolicyInputV1::Email {
                policy: &p,
                payload: Some(&payload)
            }
        ),
        Ok(expected)
    );
}

#[test]
fn composition_time_identity_not_narrowed_or_reset() {
    let (r, mut c, b) = composition_fixture(
        CommerceActionV1::OrderCancel,
        "permit(principal, action, resource);",
        |_| {},
    );
    c.now_ms = 65537;
    c.evidence.fetched_at_ms = 65537;
    c.budget.window_start_ms = 65000;
    composition_assert(&r, &c, &b, CommerceOutcomeV1::Allow, &[], &[], None);
}
