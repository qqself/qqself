/// <reference types="vitest" />
import { defineConfig } from "vite"

export default defineConfig({
  define: {
    "import.meta.vitest": "undefined",
  },
  test: {
    includeSource: ["src/**/*.ts"],
    environment: "jsdom",
    setupFiles: ["vitest.setup.ts"],
  },
})
