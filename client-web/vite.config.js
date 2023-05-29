/// <reference types="vitest" />
import { defineConfig } from "vite"
import ViteRsw from "vite-plugin-rsw"

export default defineConfig({
  plugins: [ViteRsw()],
  test: {
    setupFiles: "vitest.setup.ts",
  },
})
