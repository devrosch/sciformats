# Coding conventions

## General

* Code sits in the "src" directory.
* Directory names are all lower case.
* Custom events are named using kebab-case with "sf" prefix. Example: "sf-node-selected".

## Imports

Importing other modules that are not in the same or nested directory (e.g., sibling or parent directories) is done by specifying the module root and relative path to that instead of the relative path to the current module to make reorganizing module paths a little bit easier. Example: `import { isSameUrl } from 'util/UrlUtils';`instead of `import { isSameUrl } from '../../util/UrlUtils';`.

However, for this to work, the module root directories have to be specified:
* For Webpack in webpack.config.js - resolve.alias.
* For Typescript in tsconfig.json - compilerOptions.paths.
* For Jest in package.json - jest.moduleNameMapper.
