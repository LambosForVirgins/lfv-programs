import { defineWorkspace } from "vitest/config";
import tsconfigPaths from "vite-tsconfig-paths";
import path from "path";

export default defineWorkspace([
  {
    // add "extends" to merge two configs together
    // extends: './vite.config.js',
    test: {
      name: "programs",
      environment: "node",
      include: ["./tests/**/*.test.(ts|tsx)"],
      globals: true,
      setupFiles: "./setup.ts",
      css: false,
    },
    plugins: [tsconfigPaths()],
    resolve: {
      alias: {
        "@/*": path.resolve(__dirname, "./"),
      },
    },
  },
  {
    test: {
      name: "utilities",
      environment: "node",
      include: ["./packages/**/*.test.(ts|tsx)"],
      globals: true,
      setupFiles: "./setup.ts",
      css: false,
    },
    plugins: [tsconfigPaths()],
    resolve: {
      alias: {
        "@/*": path.resolve(__dirname, "./"),
      },
    },
  },
]);
