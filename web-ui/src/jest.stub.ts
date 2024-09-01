// required for Plotly
global.URL.createObjectURL = jest.fn();
// for Plotly, not strictly required, but avoids inconsequential error message
global.HTMLCanvasElement.prototype.getContext = jest.fn();
// required to render tree
global.crypto.randomUUID = jest.fn(
  () => 'aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee',
);
