// DGR gate — Phase 0 FAILING STUB. Red scaffolding only.
//
// !!! DO NOT IMPLEMENT ENFORCEMENT / DECISION-CORE LOGIC HERE !!!
// The decision core is explicitly gated and forbidden in Phase 0.
// See ../.specify/memory/constitution.md (Principle 9) and
// ../specs/0001-enforcement-spec.md.
//
// This module exists ONLY so the bypass suite has a stable symbol to call.
// `decide()` deliberately throws: nothing is implemented, so no governed
// decision can be proven secure. The bypass tests assert the SECURE outcome
// (block / escalate / fail-closed) and therefore FAIL — making CI red by
// default, which is the correct, truthful, fail-closed state for Phase 0.
//
// CI for the bypass suite turns green only when REAL, human-led, reviewed
// enforcement (a later phase) makes `decide()` return the secure outcomes.
// Green must never be achieved by weakening the tests or this stub.

export class NotImplementedError extends Error {
  constructor(message) {
    super(message);
    this.name = "NotImplementedError";
  }
}

/**
 * Target contract (for the future implementation, NOT implemented in Phase 0):
 *   decide(request) -> { allow: boolean, outcome: "block"|"allow"|"escalate", reason: string }
 *
 * Phase 0 behavior: throw. There is no decision core.
 *
 * @param {unknown} _request
 * @returns {never}
 */
export function decide(_request) {
  throw new NotImplementedError(
    "DGR decision core is not implemented (Phase 0 — spine only). " +
      "This is intentional: the bypass suite must be red until real " +
      "enforcement exists. See .specify/memory/constitution.md."
  );
}
