let answer: number = 42;

self.onmessage = ({ data: { question: string } }) => {
  self.postMessage({
    answer: answer++,
  });
};
