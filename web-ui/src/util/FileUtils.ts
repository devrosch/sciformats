/**
 * Save data to a file.
 *
 * @param fileName The (suggested) file name.
 * @param blob The data to be saved.
 */
export const saveFile = (fileName: string, blob: Blob) => {
  console.log('File save');
  // save blob via anchor element with download attribute and object URL
  const a = document.createElement('a');
  a.href = URL.createObjectURL(blob);
  a.download = fileName;
  document.body.appendChild(a);
  a.click();
  // remove element
  setTimeout(() => {
    document.body.removeChild(a);
    window.URL.revokeObjectURL(a.href);
  }, 100);
};
