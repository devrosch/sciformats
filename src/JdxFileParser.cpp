#include "jdx/JdxFileParser.hpp"
#include "jdx/JdxNode.hpp"
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
    std::ifstream input{path};
    auto block = sciformats::jdx::JdxParser::parse(input, true);
    //    auto blockPtr =
    //    std::make_unique<sciformats::jdx::Block>(sciformats::jdx::JdxParser::parse(input,
    //    true));
    std::unique_ptr<model::Node> node
        = std::make_unique<JdxNode>(JdxNode(block));
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
