#include "jdx/JdxBlockNode.hpp"
#include "jdx/JdxParser.hpp"
#include "jdx/Block.hpp"
#include "model/KeyValueParam.hpp"
#include "model/Node.hpp"

#ifdef __EMSCRIPTEN__
#include <emscripten/bind.h>
#endif

sciformats::sciwrap::jdx::JdxBlockNode::JdxBlockNode(const sciformats::jdx::Block& block)
    : m_istream{nullptr}
    , m_block{std::nullopt}
    , m_blockRef{block}
{
}

sciformats::sciwrap::jdx::JdxBlockNode::JdxBlockNode(std::unique_ptr<std::istream> stream)
    : m_istream{std::move(stream)}
    , m_block{sciformats::jdx::JdxParser::parse(*m_istream, true)}
    , m_blockRef{m_block.value()}
{
}

std::string sciformats::sciwrap::jdx::JdxBlockNode::getName() const
{
    return m_blockRef.getLdr("TITLE").value().getValue();
}

std::vector<sciformats::sciwrap::model::KeyValueParam>
sciformats::sciwrap::jdx::JdxBlockNode::getParams()
{
    auto const& ldrs = m_blockRef.getLdrs();
    auto vec = std::vector<sciformats::sciwrap::model::KeyValueParam>();
    for (auto const& ldr : ldrs)
    {
        vec.push_back({ldr.getLabel(), ldr.getValue()});
    }
    return vec;
}

std::vector<std::shared_ptr<sciformats::sciwrap::model::Node>>
sciformats::sciwrap::jdx::JdxBlockNode::getChildNodes()
{
    auto childNodes = std::vector<std::shared_ptr<Node>>();
    for (auto const& block : m_blockRef.getBlocks())
    {
        auto blockPtr = std::make_shared<JdxBlockNode>(block);
        childNodes.push_back(blockPtr);
    }
    // TODO: populate with other node types
    return childNodes;
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
