#ifndef LIBSCIWRAP_JDXNODE_HPP
#define LIBSCIWRAP_JDXNODE_HPP

#include "model/KeyValueParam.hpp"
#include "model/Node.hpp"
#include "jdx/Block.hpp"

namespace sciformats::sciwrap::jdx
{
class JdxNode : public model::Node
{
public:
    JdxNode(sciformats::jdx::Block block);
//    JdxNode(std::unique_ptr<sciformats::jdx::Block> blockPtr);
    [[nodiscard]] std::string getName() const override;
    std::vector<model::KeyValueParam> getParams() override;
    std::vector<std::shared_ptr<model::Node>> getChildNodes() override;

private:
    sciformats::jdx::Block m_block;
};
} // sciformats::sciwrap::jdx

#endif // LIBSCIWRAP_JDXNODE_HPP
