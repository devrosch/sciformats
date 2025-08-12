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

/* eslint-disable no-duplicate-imports */
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import Table from 'model/Table';
import './DataTable'; // for side effects
import DataTable from './DataTable';

const element = 'sf-data-table';
const table: Table = {
  columnNames: [
    { key: 'col0', name: 'Column 0' },
    { key: 'col1', name: 'Column 1' },
    { key: 'col2', name: 'Column 2' },
  ],
  rows: [
    {
      col0: true,
      col1: 123.456,
      col2: BigInt(123456),
    },
    {
      // no col0
      col1: 'Cell 11',
      col2: 'Cell 12',
    },
    {
      col0: 'Cell 20',
      // no col1
      col2: 'Cell 22',
    },
    {
      col0: 'Cell 30',
      col1: 'Cell 31',
      // no col2
    },
  ],
};
const urlChild2 = new URL('file:///test/path/root.txt#/child 2');

/**
 * Checks if data has been rendered.
 * @param data Data to be rendered.
 * @param document The document the data is to be rendered in.
 * @param expectRender Whether the data should or should not have been rendered.
 */
const checkDataIsRendered = (
  peakData: Table,
  document: Document,
  expectRender: boolean,
) => {
  const htmlTable = document.querySelector(
    `${element} table`,
  ) as HTMLTableElement;
  expect(htmlTable).not.toBeNull();

  const header = htmlTable.querySelector('thead > tr') as HTMLElement;
  const rows = htmlTable.querySelectorAll('tbody > tr');

  if (expectRender) {
    // table header
    expect(header).not.toBeNull();
    const columnHeaders = header.querySelectorAll('th');
    expect(columnHeaders.length).toBe(peakData.columnNames.length);
    for (let i = 0; i < peakData.columnNames.length; i += 1) {
      expect(columnHeaders[i].textContent).toBe(peakData.columnNames[i].name);
    }

    // table body
    expect(rows.length).toBe(peakData.rows.length);
    for (let i = 0; i < peakData.rows.length; i += 1) {
      const cells = rows[i].querySelectorAll('td');
      expect(cells.length).toBe(peakData.columnNames.length);
      for (let j = 0; j < peakData.columnNames.length; j += 1) {
        const columnKey = peakData.columnNames[j].key;
        const expectedCellValue = Object.prototype.hasOwnProperty.call(
          peakData.rows[i],
          columnKey,
        )
          ? peakData.rows[i][columnKey]
          : '';
        expect(cells[j].textContent).toBe(expectedCellValue.toString());
      }
    }
  } else {
    expect(header).toBeNull();
    expect(rows.length).toBe(0);
  }
};

afterEach(() => {
  // make sure disconnectedCallback() is called during test
  document.body.innerHTML = '';
});

test('sf-data-table renders', async () => {
  document.body.innerHTML = `<${element}/>`;
  expect(document.body.innerHTML).toContain('table');

  const peaks = document.body.querySelector(element) as DataTable;
  peaks.data = table;
  checkDataIsRendered(table, document, true);
});

test('sf-data-table reacts to sf-tree-node-(de)selected events', async () => {
  const dataTable = new DataTable();
  document.body.append(dataTable);
  const channel = CustomEventsMessageBus.getDefaultChannel();
  checkDataIsRendered(table, document, false);

  channel.dispatch('sf-tree-node-selected', {
    url: urlChild2,
    data: null,
    table,
    parameters: null,
  });
  checkDataIsRendered(table, document, true);
  expect(dataTable.data).toEqual(table);

  channel.dispatch('sf-tree-node-deselected', { url: urlChild2 });
  checkDataIsRendered(table, document, false);
});

test('sf-data-table reacts to sf-tree-node-data-updated events', async () => {
  const dataTable = new DataTable();
  document.body.append(dataTable);
  const channel = CustomEventsMessageBus.getDefaultChannel();
  checkDataIsRendered(table, document, false);

  const emptyPeakTable: Table = {
    columnNames: [],
    rows: [],
  };

  channel.dispatch('sf-tree-node-selected', {
    url: urlChild2,
    data: null,
    table: emptyPeakTable,
    parameters: null,
  });
  checkDataIsRendered(emptyPeakTable, document, true);
  let peakData = dataTable.data;
  expect(peakData).toEqual(emptyPeakTable);

  channel.dispatch('sf-tree-node-data-updated', {
    url: urlChild2,
    data: null,
    table,
    parameters: null,
  });

  checkDataIsRendered(table, document, true);
  peakData = dataTable.data;
  expect(peakData).toEqual(table);

  channel.dispatch('sf-tree-node-deselected', { url: urlChild2 });
  checkDataIsRendered(table, document, false);
});
