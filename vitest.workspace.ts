import { defineWorkspace } from "vitest/config";
import path from "path";

export default defineWorkspace([
  {
    // add "extends" to merge two configs together
    // extends: './vite.config.js',
    test: {
      include: ["packages/**/*.test.{ts,js}"],
      name: "packages",
      environment: "node",
      globals: true,
      setupFiles: "./scripts/vitest.setup.ts",
      css: false,
    },
    resolve: {
      alias: {
        "~": path.resolve(__dirname, "./"),
      },
    },
  },
]);
