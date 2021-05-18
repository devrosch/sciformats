#include "model/Node.hpp"
#include "stub/StubNode.hpp"

#ifdef __EMSCRIPTEN__
#include <emscripten/bind.h>
#endif

std::string sciformats::sciwrap::stub::StubNode::getName() const
{
    return "A Stub Node";
}

std::vector<std::pair<std::string, std::string>> sciformats::sciwrap::stub::StubNode::getParameters()
{
    auto vec = std::vector<std::pair<std::string, std::string>>();
    auto param0 = std::pair<std::string, std::string>{"key0", "value0"};
    auto param1 = std::pair<std::string, std::string>{"key1", "value1"};
    auto param2 = std::pair<std::string, std::string>{"key2", "value2"};
    vec.push_back(param0);
    vec.push_back(param1);
    vec.push_back(param2);
    return vec;
}

std::vector<std::shared_ptr<sciformats::sciwrap::model::Node>> sciformats::sciwrap::stub::StubNode::getChildNodes()
{
    auto children = std::vector<std::shared_ptr<Node>>();
    std::shared_ptr<Node> ptr0 = std::make_shared<StubNode>(StubNode());
    std::shared_ptr<Node> ptr1 = std::make_shared<StubNode>(StubNode());
    std::shared_ptr<Node> ptr2 = std::make_shared<StubNode>(StubNode());
    children.push_back(std::move(ptr0));
    children.push_back(std::move(ptr1));
    children.push_back(std::move(ptr2));
    return children;
}

#ifdef __EMSCRIPTEN__
EMSCRIPTEN_BINDINGS(StubNode) {
    using namespace sciformats::sciwrap::model;
    using namespace sciformats::sciwrap::stub;
    using namespace emscripten;
    // see: https://github.com/emscripten-core/emscripten/issues/627
    class_<StubNode, base<Node>>("StubNode")
        .constructor<>()
        .property("name", &StubNode::getName)
        .function("getParameters", &StubNode::getParameters)
        // embind fails mapping getChildNodes to property
        .function("getChildNodes", &StubNode::getChildNodes)
    ;
}
#endif
