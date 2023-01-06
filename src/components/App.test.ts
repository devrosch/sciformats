/* eslint-disable import/no-duplicates */
import 'components/menu/NavbarMatchMediaMock'; // mock window.matchMedia()
import './App'; // for side effects
import App from './App';

const element = 'sf-app';

afterEach(() => {
  // make sure disconnectedCallback() is called during test
  document.body.innerHTML = '';
});

test('sf-app renders', async () => {
  document.body.innerHTML = `<${element}/>`;
  const app = document.body.querySelector('sf-app') as App;
  expect(app).toBeTruthy();

  expect(app.children).toHaveLength(3);
  expect(app.children.item(0)?.nodeName).toBe('DIV');
  expect(app.children.item(1)?.nodeName).toBe('DIV');
  expect(app.children.item(2)?.nodeName).toBe('DIV');
});
