import { defineWorkspace } from "vitest/config";
import path from "path";

export default defineWorkspace([
  {
    // add "extends" to merge two configs together
    // extends: './vite.config.js',
    test: {
      // include: ['tests/**/*.{browser}.test.{ts,js}'],
      name: "solana",
      environment: "node",
      globals: true,
      setupFiles: "./setup.ts",
      css: false,
    },
    resolve: {
      alias: {
        "@": path.resolve(__dirname, "./"),
      },
    },
  },
]);
