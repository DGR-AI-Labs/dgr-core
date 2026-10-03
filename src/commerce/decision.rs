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
        self.option(&r.currency, |e, s| e.string(s))?;
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
        self.option(&v.currency, |e, s| e.string(s))?;
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
        self.option(&v.currency, |e, s| e.string(s))?;
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
