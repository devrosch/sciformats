#include "stub/StubNode.hpp"
#include "model/KeyValueParam.hpp"
#include "model/Node.hpp"

#ifdef __EMSCRIPTEN__
#include <emscripten/bind.h>
#endif

std::string sciformats::sciwrap::stub::StubNode::getName() const
{
    return "A Stub Node";
}

std::vector<sciformats::sciwrap::model::KeyValueParam>
sciformats::sciwrap::stub::StubNode::getParams()
{
    auto vec = std::vector<sciformats::sciwrap::model::KeyValueParam>();
    vec.push_back({"key0", "value0"});
    vec.push_back({"key1", "value1"});
    vec.push_back({"key2", "value2"});
    return vec;
}

std::vector<std::shared_ptr<sciformats::sciwrap::model::Node>>
sciformats::sciwrap::stub::StubNode::getChildNodes()
{
    auto children = std::vector<std::shared_ptr<Node>>();
    std::shared_ptr<Node> ptr0 = std::make_shared<StubNode>();
    std::shared_ptr<Node> ptr1 = std::make_shared<StubNode>();
    std::shared_ptr<Node> ptr2 = std::make_shared<StubNode>();
    children.push_back(std::move(ptr0));
    children.push_back(std::move(ptr1));
    children.push_back(std::move(ptr2));
    return children;
}

#ifdef __EMSCRIPTEN__
EMSCRIPTEN_BINDINGS(StubNode)
{
    using namespace sciformats::sciwrap::model;
    using namespace sciformats::sciwrap::stub;
    using namespace emscripten;
    // see: https://github.com/emscripten-core/emscripten/issues/627
    class_<StubNode, base<Node>>("StubNode")
        .constructor<>()
        .property("name", &StubNode::getName)
        // embind fails mapping getParams() or getChildNodes() to a property
        .function("getParams", &StubNode::getParams)
        .function("getChildNodes", &StubNode::getChildNodes);
}
#endif
