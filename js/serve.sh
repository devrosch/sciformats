#!/usr/bin/env python

# see: https://curiousprog.com/2018/10/08/serving-webassembly-files-with-a-development-web-server/

import BaseHTTPServer, SimpleHTTPServer

port=8000
print "Running on port %d" % port

SimpleHTTPServer.SimpleHTTPRequestHandler.extensions_map['.wasm'] =    'application/wasm' 
httpd = BaseHTTPServer.HTTPServer(('localhost', port),
    SimpleHTTPServer.SimpleHTTPRequestHandler)

print "Mapping \".wasm\" to \"%s\"" % SimpleHTTPServer.SimpleHTTPRequestHandler.extensions_map['.wasm']
httpd.serve_forever()
