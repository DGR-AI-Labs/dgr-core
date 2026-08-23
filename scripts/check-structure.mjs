// Structural / governance check — Phase 1 enforcement proof.
//
// This CI job is expected to pass. It verifies that the governance spine is
// present and that the README retains the required project disclaimers. The
// separate Rust job runs the currently active conformance suite, which is
// expected green and is being prepared as a required merge gate by CORE-005.
// This script intentionally does not run or redefine that suite.
import { readFileSync, existsSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const problems = [];

const requiredPaths = [
  ".specify/memory/constitution.md",
  ".specify/templates/spec-template.md",
  ".specify/templates/plan-template.md",
  ".specify/templates/tasks-template.md",
  "specs/0001-enforcement-spec.md",
  "tests/bypass/README.md",
  "tests/bypass/01-no-token.test.mjs",
  "tests/bypass/02-expired-replayed-token.test.mjs",
  "tests/bypass/03-missing-justification.test.mjs",
  "tests/bypass/04-ambiguous-evidence.test.mjs",
  "tests/bypass/05-gate-throws.test.mjs",
  "src/gate.mjs",
  ".github/workflows/ci.yml",
  ".github/ISSUE_TEMPLATE/bug.yml",
  "CLAUDE.md",
  "AGENTS.md",
  "README.md",
  "LICENSE",
];

for (const rel of requiredPaths) {
  if (!existsSync(join(root, rel))) problems.push(`missing required file: ${rel}`);
}

// README must retain the project disclaimers (matched case-insensitively).
const readmePath = join(root, "README.md");
if (existsSync(readmePath)) {
  const readme = readFileSync(readmePath, "utf8").toLowerCase();
  const mustContain = [
    "experimental",
    "not production-ready",
    "no guarantees",
    "please do not share",
  ];
  for (const phrase of mustContain) {
    if (!readme.includes(phrase)) {
      problems.push(`README.md must contain the disclaimer phrase: "${phrase}"`);
    }
  }
}

if (problems.length > 0) {
  console.error("Structural check FAILED:");
  for (const p of problems) console.error(`  - ${p}`);
  process.exit(1);
}

console.log(`Structural check passed: ${requiredPaths.length} governance files present.`);
