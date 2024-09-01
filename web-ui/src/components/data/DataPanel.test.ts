/* eslint-disable no-duplicate-imports */
import './DataPanel'; // for side effects
import DataPanel from './DataPanel';

const element = 'sf-data-panel';

afterEach(() => {
  // make sure disconnectedCallback() is called during test
  document.body.innerHTML = '';
});

test('sf-data-panel renders', async () => {
  document.body.innerHTML = `<${element}/>`;

  const panel = document.body.querySelector(element) as DataPanel;
  expect(panel.children).toHaveLength(4);
  expect(panel.children[0].id).toBe('sf-data-tabs');
  expect(panel.children[1].id).toBe('sf-data-chart-panel');
  expect(panel.children[2].id).toBe('sf-data-data-panel');
  expect(panel.children[3].id).toBe('sf-data-table-panel');

  const tabs = panel.children[0].children;
  expect(tabs).toHaveLength(3);
  expect(tabs[0].nodeName).toBe('BUTTON');
  expect(tabs[1].nodeName).toBe('BUTTON');
  expect(tabs[2].nodeName).toBe('BUTTON');

  const chart = panel.children[1].children;
  expect(chart).toHaveLength(1);
  expect(chart[0].nodeName).toBe('SF-DATA-CHART');

  const table = panel.children[2].children;
  expect(table).toHaveLength(1);
  expect(table[0].nodeName).toBe('SF-DATA-DATA');

  const peaks = panel.children[3].children;
  expect(peaks).toHaveLength(1);
  expect(peaks[0].nodeName).toBe('SF-DATA-TABLE');
});

test('sf-data-panel reacts to tab click events', async () => {
  const panel = new DataPanel();
  document.body.append(panel);
  expect(panel.children).toHaveLength(4);

  const tabs = panel.children[0].children;
  expect(tabs).toHaveLength(3);

  const chartTab = tabs[0] as HTMLButtonElement;
  const chart = panel.children[1];
  const tableTab = tabs[1] as HTMLButtonElement;
  const table = panel.children[2];
  const peaksTab = tabs[2] as HTMLButtonElement;
  const peaks = panel.children[3];

  expect(chartTab.classList).toContain('active');
  expect(chart.classList).toContain('active');
  expect(tableTab.classList).not.toContain('active');
  expect(table.classList).not.toContain('active');
  expect(peaksTab.classList).not.toContain('active');
  expect(peaks.classList).not.toContain('active');

  tableTab.click();

  expect(chartTab.classList).not.toContain('active');
  expect(chart.classList).not.toContain('active');
  expect(tableTab.classList).toContain('active');
  expect(table.classList).toContain('active');
  expect(peaksTab.classList).not.toContain('active');
  expect(peaks.classList).not.toContain('active');

  peaksTab.click();

  expect(chartTab.classList).not.toContain('active');
  expect(chart.classList).not.toContain('active');
  expect(tableTab.classList).not.toContain('active');
  expect(table.classList).not.toContain('active');
  expect(peaksTab.classList).toContain('active');
  expect(peaks.classList).toContain('active');

  chartTab.click();

  expect(chartTab.classList).toContain('active');
  expect(chart.classList).toContain('active');
  expect(tableTab.classList).not.toContain('active');
  expect(table.classList).not.toContain('active');
  expect(peaksTab.classList).not.toContain('active');
  expect(peaks.classList).not.toContain('active');
});
