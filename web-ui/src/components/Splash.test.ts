/* eslint-disable no-duplicate-imports */
import './Splash'; // for side effects
import Splash from './Splash';

const element = 'sf-splash';

afterEach(() => {
  // make sure disconnectedCallback() is called during test
  document.body.innerHTML = '';
  jest.restoreAllMocks();
});

test('sf-splash supresses cancel', async () => {
  document.body.innerHTML = `<${element}/>`;
  const splash = document.body.querySelector(element) as Splash;
  const dialog = splash.querySelector('dialog') as HTMLElement;

  // inspired by: https://www.freecodecamp.org/news/how-to-write-better-tests-for-drag-and-drop-operations-in-the-browser-f9a131f0b281/
  const createEvent = (type: string, properties = {}) => {
    const event = new Event(type, { bubbles: true });
    Object.assign(event, properties);
    return event;
  };

  const stopPropagationMock = jest.fn();
  const preventDefaultMock = jest.fn();
  const cancelEvent = createEvent('cancel', { stopPropagation: stopPropagationMock, preventDefault: preventDefaultMock });

  dialog.dispatchEvent(cancelEvent);

  expect(stopPropagationMock).toHaveBeenCalled();
  expect(preventDefaultMock).toHaveBeenCalled();
});
