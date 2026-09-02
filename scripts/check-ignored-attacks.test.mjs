import assert from "node:assert/strict";
import test from "node:test";

import {
  EXPECTED_IGNORED_ATTACKS,
  REQUIRED_ACTIVE_CONFORMANCE_TESTS,
  compareIgnoredAttacks,
  compareRequiredActiveTests,
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

test("accepts the required conformance test only when listed and active", () => {
  assert.deepEqual(compareRequiredActiveTests(REQUIRED_ACTIVE_CONFORMANCE_TESTS, []), {
    missing: [],
    ignored: [],
  });
});

test("rejects a missing required conformance test", () => {
  assert.deepEqual(compareRequiredActiveTests([], []), {
    missing: REQUIRED_ACTIVE_CONFORMANCE_TESTS,
    ignored: [],
  });
});

test("rejects an ignored required conformance test", () => {
  assert.deepEqual(
    compareRequiredActiveTests(REQUIRED_ACTIVE_CONFORMANCE_TESTS, REQUIRED_ACTIVE_CONFORMANCE_TESTS),
    {
      missing: [],
      ignored: REQUIRED_ACTIVE_CONFORMANCE_TESTS,
    },
  );
});
