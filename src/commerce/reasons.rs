//! Explicit wire tags for commerce reasons. Declaration order is not priority.
use super::CommerceOutcomeV1;

#[allow(non_camel_case_types)] // Protocol identifiers are stable wire vocabulary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u16)]
pub enum ReasonV1 {
    E_INTERNAL_EVALUATION = 0,
    E_LEDGER_UNAVAILABLE = 1,
    E_STATE_UNAVAILABLE = 2,
    E_MALFORMED_REQUEST = 3,
    E_UNAUTHENTICATED_CALLER = 4,
    E_CALLER_IDENTITY_ASSERTED = 5,
    E_UNKNOWN_ACTION = 6,
    E_UNDERSPECIFIED_ACTION = 7,
    E_FORBIDDEN_CALLER_FIELD = 8,
    E_UNTRUSTED_KEY = 9,
    E_REVOKED_KEY = 10,
    E_INVALID_SIGNATURE = 11,
    E_CAPABILITY_EXPIRED = 12,
    E_BINDING_MISMATCH = 13,
    E_REPLAY = 14,
    E_MISSING_EVIDENCE = 15,
    E_EVIDENCE_CONFLICT = 16,
    E_STALE_STATE = 17,
    E_OPERATION_UNKNOWN = 18,
    E_POLICY_UNCONFIGURED = 19,
    E_POLICY_FORBID = 20,
    E_AMOUNT_LIMIT = 21,
    E_COUNT_LIMIT = 22,
    E_VALUE_LIMIT = 23,
    E_ORDER_WINDOW = 24,
    E_DISCOUNT_CONFLICT = 25,
    E_CONSTRAINT_VIOLATION = 26,
    E_PROVENANCE_UNVERIFIABLE = 27,
    E_APPROVAL_REQUIRED = 28,
    E_APPROVAL_EXPIRED = 29,
    E_REVIEWER_SEPARATION = 30,
    OK_POLICY_PERMIT = 31,
    E_APPROVAL_INVALID = 32,
}

impl ReasonV1 {
    pub(crate) fn rank(self) -> (u8, u8) {
        match self {
            Self::E_INTERNAL_EVALUATION => (0, 0),
            Self::E_LEDGER_UNAVAILABLE => (0, 1),
            Self::E_STATE_UNAVAILABLE => (0, 2),
            Self::E_MALFORMED_REQUEST => (1, 0),
            Self::E_UNAUTHENTICATED_CALLER => (1, 1),
            Self::E_CALLER_IDENTITY_ASSERTED => (1, 2),
            Self::E_UNKNOWN_ACTION => (1, 3),
            Self::E_UNDERSPECIFIED_ACTION => (1, 4),
            Self::E_FORBIDDEN_CALLER_FIELD => (1, 5),
            Self::E_UNTRUSTED_KEY => (2, 0),
            Self::E_REVOKED_KEY => (2, 1),
            Self::E_INVALID_SIGNATURE => (2, 2),
            Self::E_CAPABILITY_EXPIRED => (2, 3),
            Self::E_BINDING_MISMATCH => (2, 4),
            Self::E_REPLAY => (2, 5),
            Self::E_MISSING_EVIDENCE => (3, 0),
            Self::E_EVIDENCE_CONFLICT => (3, 1),
            Self::E_STALE_STATE => (3, 2),
            Self::E_OPERATION_UNKNOWN => (3, 3),
            Self::E_POLICY_UNCONFIGURED => (4, 0),
            Self::E_POLICY_FORBID => (4, 1),
            Self::E_AMOUNT_LIMIT => (4, 2),
            Self::E_COUNT_LIMIT => (4, 3),
            Self::E_VALUE_LIMIT => (4, 4),
            Self::E_ORDER_WINDOW => (4, 5),
            Self::E_DISCOUNT_CONFLICT => (4, 6),
            Self::E_CONSTRAINT_VIOLATION => (4, 7),
            Self::E_PROVENANCE_UNVERIFIABLE => (5, 0),
            Self::E_APPROVAL_REQUIRED => (5, 1),
            Self::E_APPROVAL_EXPIRED => (5, 2),
            Self::E_REVIEWER_SEPARATION => (5, 3),
            Self::E_APPROVAL_INVALID => (5, 4),
            Self::OK_POLICY_PERMIT => (6, 0),
        }
    }

    pub(crate) fn permits_outcome(self, outcome: CommerceOutcomeV1) -> bool {
        use CommerceOutcomeV1::{Allow, Deny, Escalate};
        match self {
            Self::OK_POLICY_PERMIT => outcome == Allow,
            Self::E_APPROVAL_REQUIRED => outcome == Escalate,
            Self::E_ORDER_WINDOW | Self::E_DISCOUNT_CONFLICT | Self::E_PROVENANCE_UNVERIFIABLE => {
                matches!(outcome, Deny | Escalate)
            }
            _ => outcome == Deny,
        }
    }
}
