#include "stub/StubFileReader.hpp"
#include "model/Node.hpp"
#include "stub/StubNode.hpp"

#include <fstream>
#include <iostream>

#ifdef __EMSCRIPTEN__
#include <emscripten/bind.h>
#endif

bool sciformats::sciwrap::stub::StubFileReader::isResponsible(
    const std::string& path)
{
    // for loading data in JS, see:
    // https://stackoverflow.com/questions/47313403/passing-client-files-to-webassembly-from-the-front-end
    // also:
    // https://stackoverflow.com/questions/61496876/how-can-i-load-a-file-from-a-html-input-into-emscriptens-memfs-file-system
    // also:
    // https://stackoverflow.com/questions/59128901/reading-large-user-provided-file-from-emscripten-chunk-at-a-time
    std::cout << "StubFileReader.isResponsible(): " << path << '\n';
    // for alternatives, see
    // https://stackoverflow.com/questions/12774207/fastest-way-to-check-if-a-file-exist-using-standard-c-c11-c
    std::ifstream ifstream{path.c_str()};
    return ifstream.good();
}

std::unique_ptr<sciformats::sciwrap::model::Node>
sciformats::sciwrap::stub::StubFileReader::read(const std::string& path)
{
    std::cout << "StubFileReader.read(): " << path << '\n';
    std::unique_ptr<model::Node> node = std::make_unique<StubNode>(StubNode());
    return node;
}

#ifdef __EMSCRIPTEN__
EMSCRIPTEN_BINDINGS(StubFileReader)
{
    using namespace sciformats::sciwrap::model;
    using namespace sciformats::sciwrap::stub;
    using namespace emscripten;
    class_<StubFileReader, base<FileReader>>("StubFileReader")
        .constructor<>()
        .function("isResponsible", &StubFileReader::isResponsible)
        .function("read", &StubFileReader::read);
}
#endif
