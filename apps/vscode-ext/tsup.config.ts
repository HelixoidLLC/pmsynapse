import { defineConfig } from "tsup";

export default defineConfig({
  entry: ["src/extension.ts"],
  outDir: "dist",
  format: ["cjs"],
  target: "node18",
  platform: "node",
  splitting: false,
  treeshake: true,
  clean: true,
  minify: process.env.NODE_ENV === "production",
  sourcemap: process.env.NODE_ENV !== "production",
  external: [
    "vscode",
    // Node.js built-ins
    "assert",
    "buffer",
    "child_process",
    "crypto",
    "events",
    "fs",
    "http",
    "https",
    "net",
    "os",
    "path",
    "stream",
    "url",
    "util",
    "zlib",
  ],
  esbuildOptions(options) {
    options.mainFields = ["module", "main"];
  },
});
