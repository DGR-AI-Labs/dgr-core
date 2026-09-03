#!/usr/bin/env node

import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import { resolve } from "node:path";

// This is the single expected deferred/external set. It encodes the boundary
// between attacks proven by the active suite and attacks not yet proven here.
// Any change requires founder review; this T3 guard only observes the set.
export const EXPECTED_IGNORED_ATTACKS = Object.freeze([
  "atk_04_missing_justification",
  "atk_05_ambiguous_evidence",
  "atk_12_revoked_credential",
  "atk_14_cross_tenant_use",
  "atk_15_deploy_role_data_access",
]);

// This active conformance test is the T3 drift detector for the authoritative
// T0 ATK-06 timeout outcome. Deleting or ignoring it must fail required CI.
export const REQUIRED_ACTIVE_CONFORMANCE_TESTS = Object.freeze([
  "atk_06_sequence_is_escalated_then_registry_derived_timeout_block",
]);

export function compareIgnoredAttacks(actualNames) {
  const expected = new Set(EXPECTED_IGNORED_ATTACKS);
  const actual = new Set(actualNames);

  const unexpected = [...actual].filter((name) => !expected.has(name)).sort();
  const missing = [...expected].filter((name) => !actual.has(name)).sort();

  return { unexpected, missing };
}

export function parseIgnoredTests(output) {
  return output
    .split(/\r?\n/u)
    .map((line) => line.trim())
    .filter((line) => line.endsWith(": test"))
    .map((line) => line.slice(0, -": test".length))
    .sort();
}

export function compareRequiredActiveTests(listedNames, ignoredNames) {
  const listed = new Set(listedNames);
  const ignored = new Set(ignoredNames);

  return {
    missing: REQUIRED_ACTIVE_CONFORMANCE_TESTS.filter((name) => !listed.has(name)),
    ignored: REQUIRED_ACTIVE_CONFORMANCE_TESTS.filter((name) => ignored.has(name)),
  };
}

function enumerateTests(testTarget, extraArgs = []) {
  return spawnSync(
    "cargo",
    [
      "test",
      "--manifest-path",
      "tests/bypass-rust/Cargo.toml",
      "--test",
      testTarget,
      "--locked",
      "--",
      "--list",
      ...extraArgs,
      "--format",
      "terse",
    ],
    { encoding: "utf8" },
  );
}

function requireEnumeration(result, label) {
  if (result.error) {
    console.error(`${label} enumeration could not start: ${result.error.message}`);
    return null;
  }

  if (result.status !== 0) {
    process.stderr.write(result.stderr);
    console.error(`${label} enumeration failed with exit code ${result.status}`);
    return null;
  }

  return parseIgnoredTests(result.stdout);
}

function main() {
  const ignoredAttacks = requireEnumeration(
    enumerateTests("attack_set", ["--ignored"]),
    "ignored-set",
  );
  if (ignoredAttacks === null) {
    process.exitCode = 1;
    return;
  }

  const { unexpected, missing } = compareIgnoredAttacks(ignoredAttacks);

  if (unexpected.length > 0 || missing.length > 0) {
    if (unexpected.length > 0) {
      console.error(`unexpected ignored tests: ${unexpected.join(", ")}`);
    }
    if (missing.length > 0) {
      console.error(`expected ignored tests no longer ignored: ${missing.join(", ")}`);
    }
    process.exitCode = 1;
    return;
  }

  const listedConformance = requireEnumeration(
    enumerateTests("core_004_conformance"),
    "required-active-test",
  );
  const ignoredConformance = requireEnumeration(
    enumerateTests("core_004_conformance", ["--ignored"]),
    "required-active-test ignored-state",
  );
  if (listedConformance === null || ignoredConformance === null) {
    process.exitCode = 1;
    return;
  }

  const activeComparison = compareRequiredActiveTests(listedConformance, ignoredConformance);
  if (activeComparison.missing.length > 0 || activeComparison.ignored.length > 0) {
    if (activeComparison.missing.length > 0) {
      console.error(`required active tests missing: ${activeComparison.missing.join(", ")}`);
    }
    if (activeComparison.ignored.length > 0) {
      console.error(`required active tests are ignored: ${activeComparison.ignored.join(", ")}`);
    }
    process.exitCode = 1;
    return;
  }

  console.log(`ignored-set guard passed: ${ignoredAttacks.join(", ")}`);
  console.log(
    `required active conformance guard passed: ${REQUIRED_ACTIVE_CONFORMANCE_TESTS.join(", ")}`,
  );
}

const invokedPath = process.argv[1] ? resolve(process.argv[1]) : "";
if (invokedPath === fileURLToPath(import.meta.url)) {
  main();
}
