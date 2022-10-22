# sf-ui

An HTML/CSS/JS UI for visualizing scientific data.

You can find the application published at [GitLab Pages](https://devrosch.gitlab.io/sf-ui/).

## Build

To install the required packages run:
```
npm install
```

After that, to run the tests, run:
```
npm test
```

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

## Coding conventions

* Code sits in the "src" directory.
* Directory names are all lower case.
* Custom events are named using kebab-case with "sf" prefix. Example: "sf-node-selected".

## Author

**Robert Schiwon** - *Main developer* - [devrosch](https://gitlab.com/devrosch)

## License

Copyright (C) 2022 Robert Schiwon

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program.  If not, see <http://www.gnu.org/licenses/>.
