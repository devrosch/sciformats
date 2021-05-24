# libsciwrap

Library for wrapping scientific data format parsers for ease of use.

em++ --bind -lworkerfs.js -sFORCE_FILESYSTEM=1 -sWASM=1 -Wl,--whole-archive ./src/libsciwrap.a -Wl,--no-whole-archive -sDISABLE_EXCEPTION_CATCHING=0 ../apps/Main.cpp -o ./apps/sciwrap_main.js

python -m SimpleHTTPServer 8000
