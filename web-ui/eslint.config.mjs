import stylistic from '@stylistic/eslint-plugin';
import ts from '@typescript-eslint/eslint-plugin';
import tsParser from '@typescript-eslint/parser';

export default [
  {
    files: ["**/*.js", "**/*.jsx", "**/*.ts", "**/*.tsx"],
    languageOptions: {
      parser: tsParser,
      parserOptions: {
        ecmaFeatures: { modules: true },
        ecmaVersion: 'latest',
        project: 'tsconfig.json',
      },
    },
    plugins: {
      '@typescript-eslint': ts,
      ts,
      '@stylistic': stylistic,
    },
    rules: {
      ...ts.configs['recommended'].rules,
      ...ts.configs['eslint-recommended'].rules,
      '@typescript-eslint/no-explicit-any': 'off',
      '@typescript-eslint/ban-ts-comment': 'off',
      'no-plusplus': 'error',
      'no-param-reassign': 'error',
      'class-methods-use-this': 'error',
      'no-duplicate-imports': 'error',
      'no-await-in-loop': 'error',
      '@stylistic/semi': 'error',
      '@stylistic/comma-dangle': ['error', 'always-multiline'],
      '@stylistic/space-before-function-paren': ['error', {
        'anonymous': 'never',
        'named': 'never',
        'asyncArrow': 'always',
      }],
    }
  }
];
