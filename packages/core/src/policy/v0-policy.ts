// T1 — review required (example policy that shapes v0 behavior).
//
// v0 policy. HARD REQUIREMENT (Constitution P6 + task): it spans BOTH a payment
// action AND non-payment actions from the very first commit, so DGR can never be
// mistaken for a payment-only spend cap. Cross-action is non-negotiable.

import type { PolicyBundle } from "../types";

export const V0_POLICY: PolicyBundle = {
  version: "v0",
  rules: [
    // --- PAYMENT action ---
    {
      id: "pay-invoice",
      action: "pay_invoice",
      effect: "require-authorization",
      requires: {
        justification: true,
        minEvidence: 1,
        evidenceKinds: ["policy"],
        minConfidence: 0.6,
      },
      deferrable: false,
      description:
        "Payment: pay an invoice. Requires justification, a policy reference, and adequate confidence.",
    },
    {
      id: "make-purchase",
      action: "make_purchase",
      effect: "require-authorization",
      requires: { justification: true, minEvidence: 1, minConfidence: 0.6 },
      deferrable: false,
      description: "Payment: make a purchase. Requires justification and adequate confidence.",
    },

    // --- NON-PAYMENT actions (proves DGR governs more than money movement) ---
    {
      id: "read-secret",
      action: "read_secret",
      effect: "require-authorization",
      requires: { justification: true, minEvidence: 1, minConfidence: 0.6 },
      deferrable: false,
      description: "Non-payment: read a secret. Sensitive; requires justification and evidence.",
    },
    {
      id: "send-bulk-email",
      action: "send_bulk_email",
      effect: "require-authorization",
      requires: { justification: true, minEvidence: 1, minConfidence: 0.5 },
      deferrable: true,
      description:
        "Non-payment: send a bulk email. Deferrable — may be queued (not executed) in degraded mode.",
    },

    // --- Fail-closed default ---
    {
      id: "default-deny",
      action: "*",
      effect: "deny",
      description: "Fail-closed default: any action not explicitly governed is denied.",
    },
  ],
};
