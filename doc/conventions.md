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

## Custom elements

UI components are implemented through custom elements ("web components"). As Safari does not support customized built-in elements ([GitHub](https://github.com/WebKit/standards-positions/issues/97), [Bugzilla](https://bugs.webkit.org/show_bug.cgi?id=182671)), only autonomous elements are used.

For easier styling, only light DOM (no shadow DOM) is used in custom elements.

### Menu

Because extending `<li>` and `<ul>` elements is not supported by Safari, and the [ElementInternals](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals) API with `ARIAMixin` are not supported by Safaria and only partially supported by Firefox, a11y is implemented through automatically setting `role` attributes for menu elements (insipred by this [example](https://www.w3.org/WAI/ARIA/apg/example-index/menubar/menubar-navigation)). However, any explicitly set `role` attributes will be overwritten.

## Events

The following custom events are dispatched or listened to by components.

| Event                       | Details             | Dispatchers | Listeners                                                     |
| --------------------------- | ------------------- | ----------- | ------------------------------------------------------------- |
| sf-tree-node-selected       | `NodeData`          | TreeNode    | Tree, TreeNode, ParametersPanel, DataTable, DataChart, Footer |
| sf-tree-node-deselected     | { url: `URL` }      | TreeNode    | Tree, TreeNode, ParametersPanel, DataTable, DataChart, Footer |
| sf-tree-node-data-read      | `NodeData`          | TreeNode    |                                                               |
| sf-file-open-requested      | { files: `File[]` } | App, NavBar | Tree                                                          |
| sf-file-close-requested     | `null`              | NavBar      | Tree                                                          |
| sf-file-close-all-requested | `null`              | NavBar      | Tree                                                          |
