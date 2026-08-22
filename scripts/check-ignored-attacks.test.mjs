import assert from "node:assert/strict";
import test from "node:test";

import {
  EXPECTED_IGNORED_ATTACKS,
  compareIgnoredAttacks,
  parseIgnoredTests,
} from "./check-ignored-attacks.mjs";

test("accepts exactly the founder-reviewed ignored set", () => {
  assert.deepEqual(compareIgnoredAttacks(EXPECTED_IGNORED_ATTACKS), {
    unexpected: [],
    missing: [],
  });
});

test("rejects an added ignore", () => {
  const actual = [...EXPECTED_IGNORED_ATTACKS, "atk_01_bad_signature"];
  assert.deepEqual(compareIgnoredAttacks(actual), {
    unexpected: ["atk_01_bad_signature"],
    missing: [],
  });
});

test("rejects a missing ignore", () => {
  const actual = EXPECTED_IGNORED_ATTACKS.slice(1);
  assert.deepEqual(compareIgnoredAttacks(actual), {
    unexpected: [],
    missing: [EXPECTED_IGNORED_ATTACKS[0]],
  });
});

test("normalizes terse libtest enumeration", () => {
  const output = [
    "atk_14_cross_tenant_use: test",
    "",
    "atk_04_missing_justification: test",
    "",
  ].join("\n");

  assert.deepEqual(parseIgnoredTests(output), [
    "atk_04_missing_justification",
    "atk_14_cross_tenant_use",
  ]);
});
