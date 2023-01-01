// required for Plotly
global.URL.createObjectURL = jest.fn();
// for Plotly, not strictly required, but avoids inconsequential error message
global.HTMLCanvasElement.prototype.getContext = jest.fn();