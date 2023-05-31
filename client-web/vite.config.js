/// <reference types="vitest" />
import { defineConfig } from "vite"
import ViteRsw from "vite-plugin-rsw"

export default defineConfig({
  plugins: [ViteRsw()],
  define: {
    "import.meta.vitest": "undefined",
  },
  test: {
    setupFiles: "vitest.setup.ts",
    includeSource: ["src/**/*.ts"],
  },
})
