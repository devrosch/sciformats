module.exports = {
  env: {
    browser: true,
    es2021: true
  },
  extends: [
    'airbnb-base',
    'airbnb-typescript/base',
  ],
  overrides: [
  ],
  parserOptions: {
    ecmaVersion: 'latest',
    sourceType: 'module',
    project: './tsconfig.json',
  },
  rules: {
    // turn off for debugging purposes for now
    'no-console': 'off',
    'class-methods-use-this': [
      'error', {
        'exceptMethods': [
          'connectedCallback',
          'disconnectedCallback',
          'adoptedCallback',
          'attributeChangedCallback'
        ]
      }
    ],
    // allow ForOfStatement
    // https://github.com/airbnb/javascript/issues/1271
    'no-restricted-syntax': [
      'error',
      'ForInStatement',
      'LabeledStatement',
      'WithStatement',
    ],
    'prefer-destructuring' : 'off',
  },
}
