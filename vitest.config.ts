import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

// Tests run against TypeScript source (no build step needed) by aliasing the
// workspace package names to their source entrypoints. The published packages
// still build via `tsc -b`; this alias only affects the test runner.
export default defineConfig({
  resolve: {
    alias: {
      "@dgr/core": fileURLToPath(new URL("./packages/core/src/index.ts", import.meta.url)),
      "@dgr/openclaw": fileURLToPath(
        new URL("./packages/adapter-openclaw/src/index.ts", import.meta.url),
      ),
    },
  },
  test: {
    include: ["packages/**/*.{test,spec}.ts", "tests/**/*.{test,spec}.ts"],
    environment: "node",
  },
});
