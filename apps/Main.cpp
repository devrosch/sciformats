#include <iostream>

int main()
{
    std::cout << "C++: main() executed\n";
    // noop, required for emscripten, so that *.js and *.wasm files get
    // geenrated for library configured as dependency see:
    // https://stackoverflow.com/questions/34234446/cmake-is-it-possible-to-build-an-executable-from-only-static-libraries-and-no-s
}
