# Coding conventions

## General

* Code sits in the "src" directory.
* Directory names are all lower case.
* Custom events are named using kebab-case with "sf" prefix. Example: "sf-node-selected".

## Imports

### Paths

Importing other modules that are not in the same or nested directory (e.g., sibling or parent directories) is done by specifying the module root and relative path to that instead of the relative path to the current module to make reorganizing module paths a little bit easier. Example: `import { isSameUrl } from 'util/UrlUtils';`instead of `import { isSameUrl } from '../../util/UrlUtils';`.

However, for this to work, the module root directories have to be specified:
* For Webpack in webpack.config.js - resolve.alias.
* For Typescript in tsconfig.json - compilerOptions.paths.
* For Jest in package.json - jest.moduleNameMapper.

### Side effects

Each web component fpr the UI sits in its own module and contains a statement that will register it when being importet. However, TypeScript performs tree shaking and removes the initialization code from modules when regularly importing them ([GitHub Issue](https://github.com/microsoft/TypeScript/issues/9191)). The workaround for this is to use a special import syntax ([Typescript Handbook](https://www.typescriptlang.org/docs/handbook/modules.html#import-a-module-for-side-effects-only)). If types from the module are required, this results in two imports for the same module. However, for this an ESLint rule needs to be disabled. Example:

    /* eslint-disable import/no-duplicates */
    import './Parameter'; // for side effects
    import Parameter from './Parameter';

Using TypeScript's [importsNotUsedAsValues](https://www.typescriptlang.org/tsconfig#importsNotUsedAsValues) and Webpack's [sideEffects](https://webpack.js.org/guides/tree-shaking/) config options instead did not succeed.


## Events

The following custom events are dispatched or listened to by components.

| Event                   | Details                     | Dispatchers | Listeners                 |
| ----------------------- | --------------------------- | ----------- | ------------------------- |
| sf-tree-node-selected   | { url: `URL to data: URL` } | TreeNode    | TreeNode, ParametersPanel |
| sf-tree-node-deselected | { url: `URL to data: URL` } | TreeNode    | TreeNode, ParametersPanel |
