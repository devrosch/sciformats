#ifndef LIBSCIWRAP_JDXBLOCKNODE_HPP
#define LIBSCIWRAP_JDXBLOCKNODE_HPP

#include "jdx/Block.hpp"
#include "model/KeyValueParam.hpp"
#include "model/Node.hpp"

namespace sciformats::sciwrap::jdx
{
class JdxBlockNode : public model::Node
{
public:
    explicit JdxBlockNode(std::unique_ptr<std::istream> stream);
    explicit JdxBlockNode(const sciformats::jdx::Block& block);
    //    JdxNode(std::unique_ptr<sciformats::jdx::Block> blockPtr);
    [[nodiscard]] std::string getName() const override;
    std::vector<model::KeyValueParam> getParams() override;
    std::vector<std::shared_ptr<model::Node>> getChildNodes() override;

private:
    std::unique_ptr<std::istream> m_istream;
    std::optional<sciformats::jdx::Block> m_block;
    const sciformats::jdx::Block& m_blockRef;
};
} // sciformats::sciwrap::jdx

#endif // LIBSCIWRAP_JDXBLOCKNODE_HPP
