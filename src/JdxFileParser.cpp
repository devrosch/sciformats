#include "jdx/JdxFileParser.hpp"
#include "jdx/JdxBlockNode.hpp"
#include "jdx/JdxParser.hpp"
#include "model/Node.hpp"

#include <fstream>
#include <iostream>
#include <sstream>

#ifdef __EMSCRIPTEN__
#include <emscripten/bind.h>
#endif

bool sciformats::sciwrap::jdx::JdxFileParser::isRecognized(
    const std::string& path)
{
    std::cout << "C++: JdxFileParser.isRecognized(): " << path << '\n';
    std::ifstream input{path};
    auto isRecognized = sciformats::jdx::JdxParser::canParse(path, input);
    return isRecognized;
}

std::unique_ptr<sciformats::sciwrap::model::Node>
sciformats::sciwrap::jdx::JdxFileParser::parse(const std::string& path)
{
    std::cout << "C++: JdxFileParser.parse(): " << path << '\n';
    auto streamPtr = std::make_unique<std::ifstream>(path);
    // root node is owner of istream
    std::unique_ptr<model::Node> node
        = std::make_unique<JdxBlockNode>(std::move(streamPtr));
    return node;
}

#ifdef __EMSCRIPTEN__
EMSCRIPTEN_BINDINGS(JdxFileParser)
{
    using namespace sciformats::sciwrap::model;
    using namespace sciformats::sciwrap::jdx;
    using namespace emscripten;
    class_<JdxFileParser, base<FileParser>>("JdxFileParser")
        .smart_ptr_constructor(
            "JdxFileParser", &std::make_shared<JdxFileParser>)
        .function("isRecognized", &JdxFileParser::isRecognized)
        .function("parse", &JdxFileParser::parse);
}
#endif
