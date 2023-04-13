/* eslint-disable no-restricted-globals, @typescript-eslint/no-unused-vars, no-plusplus */
let answer: number = 42;

self.onmessage = ({ data: { question: string } }) => {
  self.postMessage({
    answer: answer++,
  });
};
/* eslint-enable no-restricted-globals, @typescript-eslint/no-unused-vars, no-plusplus */
