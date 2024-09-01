const config = {
  singleQuote: true,
  overrides: [
    {
      files: [
        "**/*.json",
        "**/*.html",
        "**/*.css",
        "**/*.config.js",
        "**/*.config.mjs",
        "**/*.yaml",
        "**/*.yml",
      ],
      options: {
        singleQuote: false,
      },
    },
  ],
};

export default config;
