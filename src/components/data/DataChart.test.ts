/* eslint-disable import/no-duplicates */
import CustomEventsMessageBus from 'util/CustomEventsMessageBus';
import './DataChart'; // for side effects
import DataChart from './DataChart';

const element = 'sf-data-chart';
const data = [
  { x: 1.1, y: 1.2 },
  { x: 2.1, y: 2.2 },
];
const urlChild2 = new URL('file:///test/path/root.txt#/child 2');

afterEach(() => {
  // make sure disconnectedCallback() is called during test
  document.body.innerHTML = '';
});

test('sf-data-chart renders', async () => {
  document.body.innerHTML = `<${element}/>`;
  expect(document.body.innerHTML).toContain('svg');
});

test('sf-data-chart reacts to sf-tree-node-(un)selected events', async () => {
  const chart = new DataChart();
  document.body.append(chart);
  const channel = CustomEventsMessageBus.getDefaultChannel();

  let plotElement = document.body.querySelector('g.scatterlayer');
  expect(plotElement).toBeFalsy();

  channel.dispatch('sf-tree-node-selected', {
    url: urlChild2,
    data,
    parameters: null,
  });

  plotElement = document.body.querySelector('g.scatterlayer');
  expect(plotElement).toBeTruthy();
  const chartData = chart.data;
  expect(chartData).toHaveLength(2);
  expect(chartData[0].x).toBeCloseTo(1.1);
  expect(chartData[0].y).toBeCloseTo(1.2);
  expect(chartData[1].x).toBeCloseTo(2.1);
  expect(chartData[1].y).toBeCloseTo(2.2);

  channel.dispatch('sf-tree-node-deselected', { url: urlChild2 });
  plotElement = document.body.querySelector('g.scatterlayer');
  expect(plotElement).toBeFalsy();
});
