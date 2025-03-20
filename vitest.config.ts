import { defineConfig } from "vitest/config";
import { config as dotenvConfig } from "dotenv";
import tsconfigPaths from "vite-tsconfig-paths";
import path from "path";

dotenvConfig({
  path: ".env",
});

export default defineConfig({
  test: {
    globals: true,
    environment: "node",
    include: ["./tests/**/*.test.(ts|tsx)"],
    coverage: {
      include: ["./tests"],
      exclude: ["**/index.ts"],
      thresholds: {
        statements: 100,
        branches: 100,
        functions: 100,
        lines: 100,
      },
      skipFull: true,
    },
  },
  // plugins: [
  //   tsconfigPaths({
  //     loose: true,
  //   }),
  // ],
  // resolve: {
  //   alias: {
  //     "@": path.resolve(__dirname, "packages"),
  //     "@solana": path.resolve(__dirname, "node_modules/@solana"),
  //     "@coral-xyz": path.resolve(__dirname, "node_modules/@coral-xyz"),
  //   },
  // },
});
