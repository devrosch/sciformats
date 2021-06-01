#ifndef LIBSCIWRAP_JDXDATA2DNODE_HPP
#define LIBSCIWRAP_JDXDATA2DNODE_HPP

#include "model/Node.hpp"
#include "model/Point2D.hpp"

#include <variant>

namespace sciformats::sciwrap::jdx
{
class JdxData2DNode : public model::Node
{
public:
    explicit JdxData2DNode(std::string name, std::vector<std::pair<double, double>> data);
    [[nodiscard]] std::string getName() const override;
    std::vector<model::KeyValueParam> getParams() override;
    std::optional<std::vector<model::Point2D>> getData() override;
    std::vector<std::shared_ptr<model::Node>> getChildNodes() override;
    ~JdxData2DNode() override = default;

protected:
    static std::vector<model::Point2D> mapPairsToPoints(const std::vector<std::pair<double, double>>& data);

private:
    const std::string m_name;
    const std::vector<std::pair<double, double>> m_data;
};
} // sciformats::sciwrap::jdx

#endif // LIBSCIWRAP_JDXDATA2DNODE_HPP
