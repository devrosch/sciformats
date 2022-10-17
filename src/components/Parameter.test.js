import {} from './Parameter';

test('sf-parameter renders', async () => {
  document.body.innerHTML = `<sf-parameter key="abc" value="def"></sf-parameter>`;
  expect(document.body.innerHTML).toContain('abc');
  expect(document.body.innerHTML).toContain('def');
  // make sure disconnectedCallback() is called during test
  document.body.innerHTML = ``;
})