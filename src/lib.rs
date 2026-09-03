//! Distributable DGR enforcement core.
//!
//! This crate contains the reviewed T0 enforcement units extracted by PROD-001.
//! Extraction changes distributability, not the bounded isolation proof. Runtime
//! interception and non-bypassability remain unproven.

pub mod founder_approval_store;
pub mod founder_approval_timeout;
pub mod founder_authored_guard;
pub mod founder_before_tool_call_floor;
pub mod founder_consumption_store;
pub mod founder_fail_closed;
pub mod founder_s2_approval_store;
pub mod founder_s2_consumption_store;
pub mod founder_token_verification;

/// The only outcomes permitted by the CORE-001 specification.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RequiredOutcome {
    Block,
    Deny,
    EscalateThenDenyOnTimeout,
    FailClosed,
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
