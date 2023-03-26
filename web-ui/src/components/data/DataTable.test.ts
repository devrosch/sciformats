/* eslint-disable import/no-duplicates */
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import './DataTable'; // for side effects
import DataTable from './DataTable';

const element = 'sf-data-table';
const data = [
  { x: 1.1, y: 1.2 },
  { x: 2.1, y: 2.2 },
];
const urlChild2 = new URL('file:///test/path/root.txt#/child 2');

/**
 * Checks if data has been rendered.
 * @param data Data to be rendered.
 * @param document The document the data is to be rendered in.
 * @param expectRender Whether the data should or should not have been rendered.
 */
const checkDataIsRendered = (
  dataArray: { x: number, y: number }[],
  document: Document,
  expectRender: boolean,
) => {
  const html = document.body.innerHTML;
  for (const dataPoint of dataArray) {
    if (expectRender) {
      expect(html).toContain(String(dataPoint.x));
      expect(html).toContain(String(dataPoint.y));
    } else {
      expect(html).not.toContain(String(dataPoint.x));
      expect(html).not.toContain(String(dataPoint.y));
    }
  }
};

afterEach(() => {
  // make sure disconnectedCallback() is called during test
  document.body.innerHTML = '';
});

test('sf-data-table renders', async () => {
  document.body.innerHTML = `<${element}/>`;
  expect(document.body.innerHTML).toContain('table');
  expect(document.body.innerHTML).toContain('thead');

  const table = document.body.querySelector(element) as DataTable;
  table.data = data;
  expect(document.body.innerHTML).toContain('table');
  expect(document.body.innerHTML).toContain('thead');
  expect(document.body.innerHTML).toContain('tbody');
  expect(document.body.innerHTML).toContain('td');
  checkDataIsRendered(data, document, true);
});

test('sf-data-table reacts to sf-tree-node-(de)selected events', async () => {
  const table = new DataTable();
  document.body.append(table);
  const channel = CustomEventsMessageBus.getDefaultChannel();
  checkDataIsRendered(data, document, false);

  channel.dispatch('sf-tree-node-selected', {
    url: urlChild2,
    data,
    parameters: null,
  });
  checkDataIsRendered(data, document, true);
  const tableData = table.data;
  expect(tableData).toHaveLength(2);
  expect(tableData[0].x).toBeCloseTo(1.1);
  expect(tableData[0].y).toBeCloseTo(1.2);
  expect(tableData[1].x).toBeCloseTo(2.1);
  expect(tableData[1].y).toBeCloseTo(2.2);

  channel.dispatch('sf-tree-node-deselected', { url: urlChild2 });
  checkDataIsRendered(data, document, false);
});

test('sf-data-table reacts to sf-tree-node-data-updated events', async () => {
  const table = new DataTable();
  document.body.append(table);
  const channel = CustomEventsMessageBus.getDefaultChannel();
  checkDataIsRendered(data, document, false);

  channel.dispatch('sf-tree-node-selected', {
    url: urlChild2,
    data: null,
    parameters: null,
  });
  checkDataIsRendered([], document, true);
  let tableData = table.data;
  expect(tableData).toEqual([]);

  channel.dispatch('sf-tree-node-data-updated', {
    url: urlChild2,
    data,
    parameters: null,
  });

  checkDataIsRendered(data, document, true);
  tableData = table.data;
  expect(tableData).toHaveLength(2);
  expect(tableData[0].x).toBeCloseTo(1.1);
  expect(tableData[0].y).toBeCloseTo(1.2);
  expect(tableData[1].x).toBeCloseTo(2.1);
  expect(tableData[1].y).toBeCloseTo(2.2);

  channel.dispatch('sf-tree-node-deselected', { url: urlChild2 });
  checkDataIsRendered(data, document, false);
});
