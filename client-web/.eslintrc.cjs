module.exports = {
  extends: [
    "eslint:recommended",
    "plugin:@typescript-eslint/recommended",
    "plugin:@typescript-eslint/recommended-requiring-type-checking",
    "plugin:@typescript-eslint/strict",
  ],
  parser: "@typescript-eslint/parser",
  parserOptions: {
    project: true,
    tsconfigRootDir: __dirname,
  },
  plugins: ["@typescript-eslint", "unused-imports"],
  root: true,
  globals: {
    fetch: "readonly",
    setTimeout: "readonly",
  },
  rules: {
    "no-redeclare": ["error", { builtinGlobals: false }],
    "unused-imports/no-unused-imports": "error",
  },
}
