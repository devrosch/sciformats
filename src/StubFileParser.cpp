#include "stub/StubFileParser.hpp"
#include "model/Node.hpp"
#include "stub/StubNode.hpp"

#include <fstream>
#include <iostream>
#include <sstream>

#ifdef __EMSCRIPTEN__
#include <emscripten/bind.h>
#endif

bool sciformats::sciwrap::stub::StubFileParser::isRecognized(
    const std::string& path)
{
    // for loading data in JS, see:
    // https://stackoverflow.com/questions/47313403/passing-client-files-to-webassembly-from-the-front-end
    // also:
    // https://stackoverflow.com/questions/61496876/how-can-i-load-a-file-from-a-html-input-into-emscriptens-memfs-file-system
    // also:
    // https://stackoverflow.com/questions/59128901/reading-large-user-provided-file-from-emscripten-chunk-at-a-time
    std::cout << "C++: StubFileParser.isRecognized(): " << path << '\n';
    // for alternatives, see
    // https://stackoverflow.com/questions/12774207/fastest-way-to-check-if-a-file-exist-using-standard-c-c11-c
    std::ifstream input{path};
    auto ret = input.good();

    if (input.good())
    {
        std::cout << "C++: ----------------------\n";
        std::cout << "C++: initial streampos: " << input.tellg() << '\n';

        input.seekg(0, std::ios_base::end);
        std::streamoff size = input.tellg();
        input.seekg(0, std::ios_base::beg);
        std::cout << "C++: file size: " << size << '\n';

        std::cout << "C++: stream content():";
        while (!input.eof())
        {
            std::cout << ' ' << input.get();
        }
        std::cout << '\n';

        input.clear();
        input.seekg(0, std::ios_base::beg);
        std::string str((std::istreambuf_iterator<char>(input)),
            std::istreambuf_iterator<char>());
        std::cout << "C++: stream content as string(): " << str << '\n';

        std::cout << "C++: final streampos: " << input.tellg() << '\n';
        std::cout << "C++: ----------------------\n";
    }

    return ret;
}

std::unique_ptr<sciformats::sciwrap::model::Node>
sciformats::sciwrap::stub::StubFileParser::parse(const std::string& path)
{
    std::cout << "C++: StubFileParser.parse(): " << path << '\n';
    std::unique_ptr<model::Node> node = std::make_unique<StubNode>(StubNode());
    return node;
}

#ifdef __EMSCRIPTEN__
EMSCRIPTEN_BINDINGS(StubFileParser)
{
    using namespace sciformats::sciwrap::model;
    using namespace sciformats::sciwrap::stub;
    using namespace emscripten;
    class_<StubFileParser, base<FileParser>>("StubFileParser")
        //        .constructor<>()
        .smart_ptr_constructor(
            "StubFileParser", &std::make_shared<StubFileParser>)
        .function("isRecognized", &StubFileParser::isRecognized)
        .function("parse", &StubFileParser::parse);
}
#endif
