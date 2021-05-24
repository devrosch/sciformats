onmessage = function(e) {
    const f = e.data[0];

    FS.mkdir('/work');
    FS.mount(WORKERFS, { files: [f] }, '/work');

    let sfr = new Module.StubFileReader();
    console.log('JS (Service Worker): "/work/' + f.name + '" found: ' + sfr.isResponsible('/work/' + f.name));
}

self.importScripts('sciwrap_main.js');
