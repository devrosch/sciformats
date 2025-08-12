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
import './DataChart'; // for side effects
import DataChart from './DataChart';

const element = 'sf-data-chart';
const data = [
  { x: 1.1, y: 1.2 },
  { x: 2.1, y: 2.2 },
];
const metadata = {
  'x.label': 'X Label',
  'x.unit': 'X Unit',
  'y.label': 'Y Label',
  'y.unit': 'Y Unit',
};
const urlChild2 = new URL('file:///test/path/root.txt#/child 2');

afterEach(() => {
  // make sure disconnectedCallback() is called during test
  document.body.innerHTML = '';
});

test('sf-data-chart renders', async () => {
  document.body.innerHTML = `<${element}/>`;
  expect(document.body.innerHTML).toContain('svg');
});

test('sf-data-chart plots reversed x axis', async () => {
  document.body.innerHTML = `<${element}/>`;
  const chart = document.body.querySelector(element) as DataChart;
  expect(chart).not.toBeNull();

  chart.data = {
    xyData: data,
    metadata: { 'x.reverse': 'true' },
  };

  const reportedMetadata = chart.data.metadata;
  expect(reportedMetadata['x.reverse']).toBe('true');
});

test('sf-data-chart plots sticks', async () => {
  document.body.innerHTML = `<${element}/>`;
  const chart = document.body.querySelector(element) as DataChart;
  expect(chart).not.toBeNull();

  chart.data = {
    xyData: data,
    metadata: { 'plot.style': 'sticks' },
  };

  const reportedMetadata = chart.data.metadata;
  expect(reportedMetadata['plot.style']).toBe('sticks');
});

test('sf-data-chart reacts to sf-tree-node-(de)selected events', async () => {
  const chart = new DataChart();
  document.body.append(chart);
  const channel = CustomEventsMessageBus.getDefaultChannel();

  let plotElement = document.body.querySelector('g.scatterlayer');
  expect(plotElement).toBeFalsy();

  channel.dispatch('sf-tree-node-selected', {
    url: urlChild2,
    data,
    metadata,
    parameters: null,
  });

  plotElement = document.body.querySelector('g.scatterlayer');
  expect(plotElement).toBeTruthy();
  const chartData = chart.data.xyData;
  expect(chartData).toHaveLength(2);
  expect(chartData[0].x).toBeCloseTo(1.1);
  expect(chartData[0].y).toBeCloseTo(1.2);
  expect(chartData[1].x).toBeCloseTo(2.1);
  expect(chartData[1].y).toBeCloseTo(2.2);
  expect(chart.data.metadata.xTitle).toBe(
    `${metadata['x.label']} / ${metadata['x.unit']}`,
  );
  expect(chart.data.metadata.yTitle).toBe(
    `${metadata['y.label']} / ${metadata['y.unit']}`,
  );

  channel.dispatch('sf-tree-node-deselected', { url: urlChild2 });
  plotElement = document.body.querySelector('g.scatterlayer');
  expect(plotElement).toBeFalsy();
});

test('sf-data-chart reacts to sf-tree-node-data-updated events', async () => {
  const chart = new DataChart();
  document.body.append(chart);
  const channel = CustomEventsMessageBus.getDefaultChannel();

  let plotElement = document.body.querySelector('g.scatterlayer');
  expect(plotElement).toBeFalsy();

  channel.dispatch('sf-tree-node-selected', {
    url: urlChild2,
    data: null,
    metadata: null,
    parameters: null,
  });

  plotElement = document.body.querySelector('g.scatterlayer');
  expect(plotElement).toBeFalsy();
  let chartData = chart.data.xyData;
  expect(chartData).toEqual([]);

  channel.dispatch('sf-tree-node-data-updated', {
    url: urlChild2,
    data,
    metadata,
    parameters: null,
  });

  plotElement = document.body.querySelector('g.scatterlayer');
  expect(plotElement).toBeTruthy();
  chartData = chart.data.xyData;
  expect(chartData).toHaveLength(2);
  expect(chartData[0].x).toBeCloseTo(1.1);
  expect(chartData[0].y).toBeCloseTo(1.2);
  expect(chartData[1].x).toBeCloseTo(2.1);
  expect(chartData[1].y).toBeCloseTo(2.2);
  expect(chart.data.metadata.xTitle).toBe(
    `${metadata['x.label']} / ${metadata['x.unit']}`,
  );
  expect(chart.data.metadata.yTitle).toBe(
    `${metadata['y.label']} / ${metadata['y.unit']}`,
  );
});

test('sf-data-chart renders axes titles with (partially) missing labels or units', async () => {
  const chart = new DataChart();
  document.body.append(chart);
  const channel = CustomEventsMessageBus.getDefaultChannel();

  channel.dispatch('sf-tree-node-selected', {
    url: urlChild2,
    data,
    metadata: {
      'x.label': 'X Label',
      'y.label': 'Y Label',
      // no units
    },
    parameters: null,
  });

  expect(chart.data.metadata.xTitle).toBe('X Label');
  expect(chart.data.metadata.yTitle).toBe('Y Label');

  channel.dispatch('sf-tree-node-data-updated', {
    url: urlChild2,
    data,
    metadata: {
      'x.unit': 'X Unit',
      'y.unit': 'Y Unit',
      // no labels
    },
    parameters: null,
  });

  expect(chart.data.metadata.xTitle).toBe('X Unit');
  expect(chart.data.metadata.yTitle).toBe('Y Unit');

  channel.dispatch('sf-tree-node-data-updated', {
    url: urlChild2,
    data,
    metadata: {
      // no labels
      // no units
    },
    parameters: null,
  });

  expect(chart.data.metadata.xTitle).toBe('');
  expect(chart.data.metadata.yTitle).toBe('');
});
