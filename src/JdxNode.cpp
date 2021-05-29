#include "jdx/JdxNode.hpp"
#include "model/KeyValueParam.hpp"
#include "model/Node.hpp"
#include "jdx/Block.hpp"

#ifdef __EMSCRIPTEN__
#include <emscripten/bind.h>
#endif

sciformats::sciwrap::jdx::JdxNode::JdxNode(sciformats::jdx::Block block)
    : m_block{std::move(block)}
{
}

std::string sciformats::sciwrap::jdx::JdxNode::getName() const
{
    return m_block.getLdr("TITLE").value().getValue();
}

std::vector<sciformats::sciwrap::model::KeyValueParam>
sciformats::sciwrap::jdx::JdxNode::getParams()
{
    auto const& ldrs = m_block.getLdrs();
    auto vec = std::vector<sciformats::sciwrap::model::KeyValueParam>();
    for (auto const& ldr : ldrs)
    {
        vec.push_back({ldr.getLabel(), ldr.getValue()});
    }
    return vec;
}

std::vector<std::shared_ptr<sciformats::sciwrap::model::Node>>
sciformats::sciwrap::jdx::JdxNode::getChildNodes()
{
    auto children = std::vector<std::shared_ptr<Node>>();
    // TODO: populate
    return children;
}

#ifdef __EMSCRIPTEN__
EMSCRIPTEN_BINDINGS(JdxNode)
{
    using namespace sciformats::sciwrap::model;
    using namespace sciformats::sciwrap::jdx;
    using namespace emscripten;
    // see: https://github.com/emscripten-core/emscripten/issues/627
    class_<JdxNode, base<Node>>("JdxNode")
//        .constructor<>()
        .property("name", &JdxNode::getName)
        // embind fails mapping getParams() or getChildNodes() to a property
        .function("getParams", &JdxNode::getParams)
        .function("getChildNodes", &JdxNode::getChildNodes);
}
#endif
