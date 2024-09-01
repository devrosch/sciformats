/* eslint-disable no-duplicate-imports */
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import './DataData'; // for side effects
import DataData from './DataData';

const element = 'sf-data-data';
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
  dataArray: { x: number; y: number }[],
  document: Document,
  expectRender: boolean,
) => {
  const textarea = document.querySelector(
    `${element} textarea`,
  ) as HTMLTextAreaElement;
  expect(textarea).not.toBeNull();
  const value = textarea.value;
  for (const dataPoint of dataArray) {
    if (expectRender) {
      expect(value).toContain(String(dataPoint.x));
      expect(value).toContain(String(dataPoint.y));
    } else {
      expect(value).not.toContain(String(dataPoint.x));
      expect(value).not.toContain(String(dataPoint.y));
    }
  }
};

afterEach(() => {
  // make sure disconnectedCallback() is called during test
  document.body.innerHTML = '';
});

test('sf-data-data renders', async () => {
  document.body.innerHTML = `<${element}/>`;
  expect(document.body.innerHTML).toContain('textarea');

  const table = document.body.querySelector(element) as DataData;
  table.data = data;
  expect(document.body.innerHTML).toContain('textarea');
  checkDataIsRendered(data, document, true);
});

test('sf-data-data reacts to sf-tree-node-(de)selected events', async () => {
  const table = new DataData();
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

test('sf-data-data reacts to sf-tree-node-data-updated events', async () => {
  const table = new DataData();
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
