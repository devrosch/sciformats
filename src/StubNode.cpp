#include "stub/StubNode.hpp"

#ifdef __EMSCRIPTEN__
#include <emscripten/bind.h>

using namespace emscripten;
#endif

std::string sciformats::sciwrap::stub::StubNode::getName() const
{
    return "A Node";
}

std::vector<std::unique_ptr<sciformats::sciwrap::model::Node>> sciformats::sciwrap::stub::StubNode::getChildNodes()
{
    auto children = std::vector<std::unique_ptr<Node>>();
    std::unique_ptr<Node> ptr0 = std::make_unique<StubNode>(StubNode());
    std::unique_ptr<Node> ptr1 = std::make_unique<StubNode>(StubNode());
    std::unique_ptr<Node> ptr2 = std::make_unique<StubNode>(StubNode());
    children.push_back(std::move(ptr0));
    children.push_back(std::move(ptr1));
    children.push_back(std::move(ptr2));
    return children;
}

#ifdef __EMSCRIPTEN__
using namespace sciformats::sciwrap::stub;

EMSCRIPTEN_BINDINGS(StubNode) {
  class_<StubNode>("StubNode")
    .constructor<>()
    .property("name", &StubNode::getName)
//    .function("getName", &StubNode::getName)
    ;
}
#endif
