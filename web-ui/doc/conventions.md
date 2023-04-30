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

### Structure

Each custom element has a `template` constant that holds the HTML template for its child elements.

Apart from the regular custom element lifecycle methods (connectedCallback, disconnectedCallback, adoptedCallback, attributeChangedCallback), each custom element has the following two methods:
* `init()`: Performs initializations such as setting up the inner state of the component (e.g. based on set attributes) and to apply the template of child elements. Init needs to guard itself against making initializations multiple times, either by checking if the initialization outcome is already present or by holding info on whether initialization has been performed in the components state. init() must not be called in the components constructor. init() must be called at the start of both lifecycle methods connectedCallback() and attributeChangedCallback() as at least one of them is called at first use of the component.
* `render()`: Performs any updates to the component's UI after initialization. Whenever any update needs to be performed, the state of the component needs to be set up accordingly and then a call to render() needs to be made.

### Menu

Because extending `<li>` and `<ul>` elements is not supported by Safari, and the [ElementInternals](https://developer.mozilla.org/en-US/docs/Web/API/ElementInternals) API with `ARIAMixin` are not supported by Safari and only partially supported by Firefox, a11y is implemented through automatically setting `role` attributes for menu elements (insipred by this [example](https://www.w3.org/WAI/ARIA/apg/example-index/menubar/menubar-navigation)). However, any explicitly set `role` attributes will be overwritten.

## Events

### Component Events

The following custom events are dispatched or listened to by components.

| Event                       | Details             | Dispatchers | Listeners                                                     |
| --------------------------- | ------------------- | ----------- | ------------------------------------------------------------- |
| sf-tree-node-selected       | `NodeData`          | TreeNode    | Tree, TreeNode, ParametersPanel, DataTable, DataChart, Footer |
| sf-tree-node-deselected     | { url: `URL` }      | TreeNode    | Tree, TreeNode, ParametersPanel, DataTable, DataChart, Footer |
| sf-tree-node-data-updated   | `NodeData`          | TreeNode    | ParametersPanel, DataTable, DataChart                         |
| sf-file-open-requested      | { files: `File[]` } | NavBar      | Tree                                                          |
| sf-file-close-requested     | `null`              | NavBar      | Tree                                                          |
| sf-file-close-all-requested | `null`              | NavBar      | Tree                                                          |

Additional events are generated by components. Event handlers for events follow this naming convention:
* `onXXX()`: Handlers for events that are generated by or target the component directly, e.g. `onClick()`.
* `handleXXX()`: Handlers for events, including the above custom events, that are generated by or target other components, e.g. `handleTreeNodeSelected()`.

### Worker Events

The following events are used to communicate with the web worker that encapsulates the parsing library.

The communication is asynchronous. The worker expects a `WorkerRequest`object and returns a `WorkerResponse` object that each have the structure:
```
{
  name: string,
  correlationId: string,
  detail: any,
}
```
The correlationId can be used to match requests to responses. The worker sends exactly one response for each request. The event names and details vary as outlined in the table below. Note: URLs are passed as strings as they cannot be default serialized.

| Request Name   | Request Details                   | Response Name       | Response Details                  | Main Thread Object |
| -------------- | --------------------------------- | ------------------- | --------------------------------- | ------------------ |
| status         | null                              | status              | `WorkerStatus`                    | ???                |
| scan           | `WorkerFileInfo`                  | scanned             | { recognized: `boolean` }         | ParserRepository   |
| open           | `WorkerFileInfo`                  | opened              | `WorkerFileUrl`                   | ParserRepository   |
| close          | `WorkerFileUrl`                   | closed              | `WorkerFileUrl`                   | ???                |
| read           | `WorkerFileUrl`                   | read                | `WorkerNodeData`                  | Parser             |

For any request, in case of error, an error message is sent instead of the response from above table.

| Response Name       | Response Details                  |
| ------------------- | --------------------------------- |
| error               | `string`                          |
