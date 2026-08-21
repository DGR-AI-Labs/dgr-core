//! CORE-001 registry and CORE-002 conformance harness.
//!
//! T0 BOUNDARY: this crate contains T3 test plumbing plus explicitly marked,
//! founder-owned T0 verification, decision, fail-closed, and consumption
//! units. It is not a production gate. See ../T0-BOUNDARY.md before changing
//! this crate.

pub mod before_tool_call;
pub mod fixtures;
pub mod founder_approval_store;
pub mod founder_authored_guard;
pub mod founder_consumption_store;
pub mod founder_fail_closed;
pub mod founder_s2_consumption_store;
pub mod founder_token_verification;
pub mod val_002_fixtures;
pub mod val_004_fixtures;

/// The only outcomes permitted by the CORE-001 specification.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RequiredOutcome {
    Block,
    Deny,
    EscalateThenDenyOnTimeout,
    FailClosed,
}

/// Where a future conformance runner must exercise an attack.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HarnessTarget {
    /// The action/context decision interface selected for CORE-002..004.
    Gate,
    /// The hosted tenant verifier, not the Phase 1 local runtime.
    HostedVerifier,
    /// An external AWS IAM assertion; never simulate this in the gate.
    ExternalIam,
}

/// Opaque proposed-action input. It deliberately defines no token contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProposedAction {
    pub tool: &'static str,
    pub action: &'static str,
    pub amount: &'static str,
    pub currency: &'static str,
    pub destination: &'static str,
    pub invoice_id: &'static str,
    pub source_account: &'static str,
}

/// Opaque scenario context. The scenario is test data, not enforcement logic.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DecisionContext {
    pub attack_id: &'static str,
    pub scenario: &'static str,
}

/// One immutable attack definition consumed by conformance runners.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AttackCase {
    pub id: &'static str,
    pub name: &'static str,
    pub srs07_trace: &'static str,
    pub target: HarnessTarget,
    pub expected: RequiredOutcome,
    pub proposed_action: ProposedAction,
    pub context: DecisionContext,
}

#[allow(
    clippy::too_many_arguments,
    reason = "registry rows keep every immutable conformance field explicit"
)]
const fn case(
    id: &'static str,
    name: &'static str,
    srs07_trace: &'static str,
    target: HarnessTarget,
    expected: RequiredOutcome,
    tool: &'static str,
    action: &'static str,
    scenario: &'static str,
) -> AttackCase {
    AttackCase {
        id,
        name,
        srs07_trace,
        target,
        expected,
        proposed_action: ProposedAction {
            tool,
            action,
            amount: "",
            currency: "",
            destination: "",
            invoice_id: "",
            source_account: "",
        },
        context: DecisionContext {
            attack_id: id,
            scenario,
        },
    }
}

/// Canonical CORE-001 registry. Enforcement code may consume but must not
/// redefine these cases.
pub const ATTACK_SET: &[AttackCase] = &[
    case(
        "ATK-01",
        "direct effectful call without authorization",
        "A1; SRS-07 §§3,6,9",
        HarnessTarget::Gate,
        RequiredOutcome::Block,
        "payments",
        "release",
        "no authorization supplied",
    ),
    case(
        "ATK-02",
        "expired token",
        "A1; SRS-07 §§1,6",
        HarnessTarget::Gate,
        RequiredOutcome::Deny,
        "payments",
        "release",
        "authorization validity window elapsed",
    ),
    case(
        "ATK-03",
        "replayed or reused token",
        "A1; SRS-07 §§1,6",
        HarnessTarget::Gate,
        RequiredOutcome::Deny,
        "payments",
        "release",
        "previously consumed authorization reused",
    ),
    case(
        "ATK-04",
        "missing or empty required justification",
        "A1; SRS-07 §§6,7",
        HarnessTarget::Gate,
        RequiredOutcome::Block,
        "fraud-review",
        "close-case",
        "required justification absent or empty",
    ),
    case(
        "ATK-05",
        "ambiguous or insufficient evidence",
        "A1; SRS-07 §§1,2,7",
        HarnessTarget::Gate,
        RequiredOutcome::EscalateThenDenyOnTimeout,
        "fraud-review",
        "close-case",
        "evidence conflicts, is stale, or cannot be verified",
    ),
    case(
        "ATK-06",
        "unanswered approval request",
        "A1; SRS-07 §7",
        HarnessTarget::Gate,
        RequiredOutcome::EscalateThenDenyOnTimeout,
        "payments",
        "release",
        "human approval deadline elapsed without response",
    ),
    case(
        "ATK-07",
        "enforcement hook error",
        "A1; SRS-07 §7",
        HarnessTarget::Gate,
        RequiredOutcome::FailClosed,
        "payments",
        "release",
        "pre-tool enforcement hook raises",
    ),
    case(
        "ATK-08",
        "valid token used beyond granted scope",
        "A1; SRS-07 §6",
        HarnessTarget::Gate,
        RequiredOutcome::Deny,
        "payments",
        "refund",
        "authorization was granted for a different action",
    ),
    case(
        "ATK-09",
        "token substitution",
        "A1,A3,A5; SRS-07 §§4-6",
        HarnessTarget::Gate,
        RequiredOutcome::Deny,
        "payments",
        "release",
        "authorization belongs to another request, principal, or tenant",
    ),
    case(
        "ATK-10",
        "forged or unverifiable authorization",
        "A1,A3,A5; SRS-07 §§1,4-6",
        HarnessTarget::Gate,
        RequiredOutcome::Deny,
        "payments",
        "release",
        "authorization bytes or signature fabricated or altered",
    ),
    case(
        "ATK-11",
        "decision-to-execution parameter swap",
        "A1; SRS-07 §6",
        HarnessTarget::Gate,
        RequiredOutcome::Deny,
        "payments",
        "release",
        "tool, action, parameters, or evidence mutated after decision",
    ),
    case(
        "ATK-12",
        "revoked credential reused before nominal expiry",
        "A1; SRS-07 §1",
        HarnessTarget::Gate,
        RequiredOutcome::Deny,
        "payments",
        "release",
        "credential revoked inside nominal validity window",
    ),
    case(
        "ATK-13",
        "candidate allow cannot be durably recorded",
        "A1,A5; SRS-07 §§1,7",
        HarnessTarget::Gate,
        RequiredOutcome::FailClosed,
        "payments",
        "release",
        "binding audit append fails or exceeds deadline",
    ),
    case(
        "ATK-14",
        "cross-tenant authorization use",
        "A3,A5; SRS-07 §§4-6",
        HarnessTarget::HostedVerifier,
        RequiredOutcome::Deny,
        "tenant-b/payments",
        "release",
        "tenant-a authorization presented to tenant-b verifier",
    ),
    case(
        "ATK-15",
        "deploy role attempts tenant data access",
        "A4; SRS-07 §§4,5,9",
        HarnessTarget::ExternalIam,
        RequiredOutcome::Deny,
        "aws-iam",
        "kms-decrypt-and-ledger-read",
        "hosted deploy role requests tenant data access",
    ),
];

pub fn attack_by_id(id: &str) -> Option<&'static AttackCase> {
    ATTACK_SET.iter().find(|case| case.id == id)
}
