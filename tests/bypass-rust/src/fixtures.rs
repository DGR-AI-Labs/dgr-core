//! Opaque conformance fixtures. Labels describe test setup; they are not
//! verification results and must never be used as enforcement inputs.

use crate::before_tool_call::{BeforeToolCallRequest, EffectfulToolProbe, OpaqueCapabilityToken};
use crate::founder_approval_store::ApprovalStore;
use crate::val_002_fixtures::Val002Fixture;
use crate::{AttackCase, DecisionContext, ProposedAction};

pub const EFFECTFUL_ACTION: ProposedAction = ProposedAction {
    tool: "payments",
    action: "release",
    amount: "",
    currency: "",
    destination: "",
    invoice_id: "",
    source_account: "",
};

pub const ATK_01_CONTEXT: DecisionContext = DecisionContext {
    attack_id: "ATK-01",
    scenario: "effectful before_tool_call request without a capability token",
};

pub const VALID_TOKEN_BYTES: &[u8] = b"fixture:valid-token-candidate";
pub const EXPIRED_TOKEN_BYTES: &[u8] = b"fixture:expired-token-candidate";
pub const REPLAYED_TOKEN_BYTES: &[u8] = b"fixture:replayed-token-candidate";
pub const FORGED_TOKEN_BYTES: &[u8] = b"fixture:forged-token-candidate";
pub const OUT_OF_SCOPE_TOKEN_BYTES: &[u8] = b"fixture:valid-token-for-another-action";

pub const fn no_token_request() -> BeforeToolCallRequest<'static> {
    BeforeToolCallRequest {
        proposed_action: EFFECTFUL_ACTION,
        context: &ATK_01_CONTEXT,
        capability_token: None,
    }
}

pub const fn valid_token_request() -> BeforeToolCallRequest<'static> {
    token_request(VALID_TOKEN_BYTES)
}

pub const fn expired_token_request() -> BeforeToolCallRequest<'static> {
    token_request(EXPIRED_TOKEN_BYTES)
}

pub const fn replayed_token_request() -> BeforeToolCallRequest<'static> {
    token_request(REPLAYED_TOKEN_BYTES)
}

pub const fn forged_token_request() -> BeforeToolCallRequest<'static> {
    token_request(FORGED_TOKEN_BYTES)
}

pub const fn out_of_scope_token_request() -> BeforeToolCallRequest<'static> {
    token_request(OUT_OF_SCOPE_TOKEN_BYTES)
}

/// Selects opaque setup data for a registered attack. This is fixture routing,
/// not token classification or verification.
pub fn request_for_attack(case: &'static AttackCase) -> BeforeToolCallRequest<'static> {
    let capability_token = match case.id {
        "ATK-01" | "ATK-04" | "ATK-05" | "ATK-06" | "ATK-07" | "ATK-13" => None,
        "ATK-02" => Some(OpaqueCapabilityToken {
            bytes: EXPIRED_TOKEN_BYTES,
        }),
        "ATK-03" => Some(OpaqueCapabilityToken {
            bytes: REPLAYED_TOKEN_BYTES,
        }),
        "ATK-08" | "ATK-09" | "ATK-14" => Some(OpaqueCapabilityToken {
            bytes: OUT_OF_SCOPE_TOKEN_BYTES,
        }),
        "ATK-10" => Some(OpaqueCapabilityToken {
            bytes: FORGED_TOKEN_BYTES,
        }),
        "ATK-11" | "ATK-12" => Some(OpaqueCapabilityToken {
            bytes: VALID_TOKEN_BYTES,
        }),
        _ => None,
    };
    BeforeToolCallRequest {
        proposed_action: case.proposed_action,
        context: &case.context,
        capability_token,
    }
}

/// Wires a VAL-002 fixture's presented token bytes to an existing conformance
/// case. The fixture separately carries its complete action fields, fixed
/// clock, and expected outcome sequence. This adapter does not interpret any
/// of them or compute a guard decision.
pub fn request_for_val_002_fixture<'a>(
    case: &'static AttackCase,
    fixture: &'a Val002Fixture,
) -> BeforeToolCallRequest<'a> {
    let presented = fixture
        .request
        .expect("VAL-002 fixture must carry a presented action");

    BeforeToolCallRequest {
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
        capability_token: fixture.token.as_ref().map(|token| OpaqueCapabilityToken {
            bytes: &token.wire_bytes,
        }),
    }
}

const fn token_request(bytes: &'static [u8]) -> BeforeToolCallRequest<'static> {
    BeforeToolCallRequest {
        proposed_action: EFFECTFUL_ACTION,
        context: &ATK_01_CONTEXT,
        capability_token: Some(OpaqueCapabilityToken { bytes }),
    }
}

/// Test-only approval port for paths that must not consult approval state.
/// Any accidental call uses the founder-owned trait's fail-closed defaults.
#[derive(Debug, Default)]
pub struct FailClosedApprovalStore;

impl ApprovalStore for FailClosedApprovalStore {}

#[derive(Debug, Default)]
pub struct RecordingToolProbe {
    invocations: u32,
}

impl EffectfulToolProbe for RecordingToolProbe {
    fn invoke(&mut self, _action: &ProposedAction) {
        self.invocations += 1;
    }

    fn invocation_count(&self) -> u32 {
        self.invocations
    }
}
