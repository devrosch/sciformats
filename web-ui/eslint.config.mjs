// @ts-check
import eslint from "@eslint/js";
import tseslint from "typescript-eslint";
import stylistic from "@stylistic/eslint-plugin";

export default tseslint.config(
  eslint.configs.recommended,
  tseslint.configs.recommended,
  // tseslint.configs.recommendedTypeChecked,
  // {
  //   languageOptions: {
  //     parserOptions: {
  //       projectService: true,
  //       tsconfigRootDir: import.meta.dirname,
  //     },
  //   },
  // },
  tseslint.configs.strict,
  tseslint.configs.stylistic,
  // tseslint.configs.strictTypeChecked,
  // tseslint.configs.stylisticTypeChecked,
  {
    plugins: {
      "@stylistic": stylistic,
    },
    rules: {
      "@typescript-eslint/no-explicit-any": "off",
      "@typescript-eslint/ban-ts-comment": "off",
      "no-plusplus": "error",
      "no-param-reassign": "error",
      "class-methods-use-this": "error",
      "no-duplicate-imports": "error",
      "no-await-in-loop": "error",
      "@typescript-eslint/no-non-null-assertion": "off",
      "@stylistic/semi": "error",
      "@stylistic/comma-dangle": ["error", "always-multiline"],
      "@stylistic/space-before-function-paren": [
        "error",
        {
          anonymous: "never",
          named: "never",
          asyncArrow: "always",
        },
      ],
    },
  },
);
