# sf-ui

An HTML/CSS/JS UI for visualizing scientific data.

You can find the application published at [GitLab Pages](https://devrosch.gitlab.io/sf/index.html).

## Prerequisites

Minimum Node.js v15 (for @typescript-eslint). Tested with Node.js v20 only.

## Build

First, build the WASM library as described in `lib-cpp` and then import it with:
```
npm import-wasm
```

To install the required packages run:
```
npm install
```

After that, to run the tests, run:
```
npm test
```
Test coverage information can be generated with `npm test -- --coverage`.

To lint the code, run:
```
npm run lint
```

To start the development server, run:
```
npm start
```

For builing a release version, run:
```
npm run build
```

Build artifacts are placed into the `dist` directory.

## Documentation

More detailed documentation can be found in the [doc](doc) directory.

## Author

**Robert Schiwon** - [devrosch](https://gitlab.com/devrosch)

## License

Copyright (C) 2022, 2023 Robert Schiwon

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program.  If not, see <http://www.gnu.org/licenses/>.
