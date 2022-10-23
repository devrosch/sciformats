/* disable rule for now until more helpers are added */
/* eslint-disable import/prefer-default-export */

/**
 * Dispatches a custom event to window.
 * @param name Event name.
 * @param detail Event details.
 */
export const dispatchWindowCustomEvent = (name: string, detail: any) => {
  window.dispatchEvent(new CustomEvent(name, {
    bubbles: true,
    cancelable: true,
    composed: true,
    detail,
  }));
};
