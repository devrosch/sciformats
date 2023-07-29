/* eslint-disable import/no-duplicates */
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import PeakTable from 'model/PeakTable';
import './DataPeaks'; // for side effects
import DataPeaks from './DataPeaks';

const element = 'sf-data-peaks';
const peakTable: PeakTable = {
  columnNames: [
    { key: 'col0', value: 'Column 0' },
    { key: 'col1', value: 'Column 1' },
    { key: 'col2', value: 'Column 2' },
  ],
  peaks: [
    new Map([['col0', 'Cell 00'], ['col1', 'Cell 01'], ['col2', 'Cell 02']]),
    new Map([/* no col0 */['col1', 'Cell11'], ['col2', 'Cell 12']]),
    new Map([['col0', 'Cell 20'], /* no col1 */['col2', 'Cell 22']]),
    new Map([['col0', 'Cell 30'], ['col1', 'Cell 31']/* no col2 */]),
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
  peakData: PeakTable,
  document: Document,
  expectRender: boolean,
) => {
  const table = document.querySelector(`${element} table`) as HTMLTableElement;
  expect(table).not.toBeNull();

  const header = table.querySelector('thead > tr') as HTMLElement;
  const rows = table.querySelectorAll('tbody > tr');

  if (expectRender) {
    // table header
    expect(header).not.toBeNull();
    const columnHeaders = header.querySelectorAll('th');
    expect(columnHeaders.length).toBe(peakData.columnNames.length);
    for (let i = 0; i < peakData.columnNames.length; i += 1) {
      expect(columnHeaders[i].textContent).toBe(peakData.columnNames[i].value);
    }

    // table body
    expect(rows.length).toBe(peakData.peaks.length);
    for (let i = 0; i < peakData.peaks.length; i += 1) {
      const cells = rows[i].querySelectorAll('td');
      expect(cells.length).toBe(peakData.columnNames.length);
      for (let j = 0; j < peakData.columnNames.length; j += 1) {
        const columnKey = peakData.columnNames[j].key;
        const expectedCellValue = peakData.peaks[i].has(columnKey) ? peakData.peaks[i].get(columnKey) : '';
        expect(cells[j].textContent).toBe(expectedCellValue);
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

test('sf-data-peaks renders', async () => {
  document.body.innerHTML = `<${element}/>`;
  expect(document.body.innerHTML).toContain('table');

  const peaks = document.body.querySelector(element) as DataPeaks;
  peaks.data = peakTable;
  checkDataIsRendered(peakTable, document, true);
});

test('sf-data-peaks reacts to sf-tree-node-(de)selected events', async () => {
  const dataPeaks = new DataPeaks();
  document.body.append(dataPeaks);
  const channel = CustomEventsMessageBus.getDefaultChannel();
  checkDataIsRendered(peakTable, document, false);

  channel.dispatch('sf-tree-node-selected', {
    url: urlChild2,
    data: null,
    peakTable,
    parameters: null,
  });
  checkDataIsRendered(peakTable, document, true);
  expect(dataPeaks.data).toEqual(peakTable);

  channel.dispatch('sf-tree-node-deselected', { url: urlChild2 });
  checkDataIsRendered(peakTable, document, false);
});

test('sf-data-peaks reacts to sf-tree-node-data-updated events', async () => {
  const dataPeaks = new DataPeaks();
  document.body.append(dataPeaks);
  const channel = CustomEventsMessageBus.getDefaultChannel();
  checkDataIsRendered(peakTable, document, false);

  const emptyPeakTable: PeakTable = {
    columnNames: [],
    peaks: [],
  };

  channel.dispatch('sf-tree-node-selected', {
    url: urlChild2,
    data: null,
    peakTable: emptyPeakTable,
    parameters: null,
  });
  checkDataIsRendered(emptyPeakTable, document, true);
  let peakData = dataPeaks.data;
  expect(peakData).toEqual(emptyPeakTable);

  channel.dispatch('sf-tree-node-data-updated', {
    url: urlChild2,
    data: null,
    peakTable,
    parameters: null,
  });

  checkDataIsRendered(peakTable, document, true);
  peakData = dataPeaks.data;
  expect(peakData).toEqual(peakTable);

  channel.dispatch('sf-tree-node-deselected', { url: urlChild2 });
  checkDataIsRendered(peakTable, document, false);
});
