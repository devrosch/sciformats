#include "stub/StubNode.hpp"

#ifdef __EMSCRIPTEN__
#include <emscripten/bind.h>
#endif

std::string sciformats::sciwrap::stub::StubNode::getName() const
{
    return "A Node";
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

std::shared_ptr<sciformats::sciwrap::model::Node> sciformats::sciwrap::stub::StubNode::getSingleChild()
{
    std::shared_ptr<StubNode> ptr0 = std::make_shared<StubNode>(StubNode());
    return ptr0;
}

#ifdef __EMSCRIPTEN__
EMSCRIPTEN_BINDINGS(StubNode) {
    using namespace sciformats::sciwrap::model;
    using namespace sciformats::sciwrap::stub;
    using namespace emscripten;
    class_<Node>("Node")
        .smart_ptr<std::shared_ptr<Node>>("Node")
//        .allow_subclass<StubNode>("StubNode")
    ;
    // see: https://github.com/emscripten-core/emscripten/issues/627
    class_<StubNode, base<Node>>("StubNode")
        .constructor<>()
        .smart_ptr<std::shared_ptr<StubNode>>("StubNode")
        .property("name", &StubNode::getName)
        // embind fails mapping getChildNodes to property
        .function("getChildNodes", &StubNode::getChildNodes)
//        .property("childNodes", &StubNode::getChildNodes)
//        .function("getName", &StubNode::getName)
        .function("getSingleChild", &StubNode::getSingleChild)
    ;
    // cannot use unique_ptr in embind
    // see: https://stackoverflow.com/questions/31814092/cant-use-vector-of-unique-ptr-in-emscripten-bindings
    register_vector<std::shared_ptr<Node>>("vector<std::shared_ptr<Node>>");
}
#endif
