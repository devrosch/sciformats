type NodeData = {
  url: URL,
  data: { x: number, y: number }[],
  parameters: { key: string, value: string }[],
  children: string[],
};

export default NodeData;