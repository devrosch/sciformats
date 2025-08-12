/**
 * Copyright (c) 2025 Robert Schiwon
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

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
