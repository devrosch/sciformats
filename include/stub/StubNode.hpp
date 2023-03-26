#ifndef LIBSCIWRAP_STUBNODE_HPP
#define LIBSCIWRAP_STUBNODE_HPP

#include "model/KeyValueParam.hpp"
#include "model/Node.hpp"

namespace sciformats::sciwrap::stub
{
class StubNode : public model::Node
{
public:
    [[nodiscard]] std::string getName() const override;
    std::vector<model::KeyValueParam> getParams() override;
    std::optional<std::vector<model::Point2D>> getData() override;
    std::vector<std::shared_ptr<model::Node>> getChildNodes() override;

private:
};
} // sciformats::sciwrap::stub

#endif // LIBSCIWRAP_STUBNODE_HPP
