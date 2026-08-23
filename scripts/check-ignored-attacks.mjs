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

function main() {
  const result = spawnSync(
    "cargo",
    [
      "test",
      "--manifest-path",
      "tests/bypass-rust/Cargo.toml",
      "--test",
      "attack_set",
      "--locked",
      "--",
      "--list",
      "--ignored",
      "--format",
      "terse",
    ],
    { encoding: "utf8" },
  );

  if (result.error) {
    console.error(`ignored-set enumeration could not start: ${result.error.message}`);
    process.exitCode = 1;
    return;
  }

  if (result.status !== 0) {
    process.stderr.write(result.stderr);
    console.error(`ignored-set enumeration failed with exit code ${result.status}`);
    process.exitCode = 1;
    return;
  }

  const actual = parseIgnoredTests(result.stdout);
  const { unexpected, missing } = compareIgnoredAttacks(actual);

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

  console.log(`ignored-set guard passed: ${actual.join(", ")}`);
}

const invokedPath = process.argv[1] ? resolve(process.argv[1]) : "";
if (invokedPath === fileURLToPath(import.meta.url)) {
  main();
}
