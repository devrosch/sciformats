#include "jdx/XyPoints.hpp"
#include "jdx/XyBase.hpp"

sciformats::jdx::XyPoints::XyPoints(const std::string& label,
    const std::string& variableList, const std::vector<StringLdr>& ldrs,
    TextReader& reader, std::optional<std::string>& nextLine)
    : XyBase(label, variableList, ldrs, s_xyPointsLabel, s_xyPointsVariableList,
        reader, nextLine)
{
}

std::vector<std::pair<double, double>> sciformats::jdx::XyPoints::getData()
{
    return XyBase::getData(Data2D::VariableList::XyXy);
}
