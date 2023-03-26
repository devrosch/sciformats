#include "jdx/XyPoints.hpp"
#include "jdx/ParseException.hpp"
#include "jdx/XyBase.hpp"

sciformats::jdx::XyPoints::XyPoints(const std::string& label,
    const std::string& variableList, const std::vector<StringLdr>& ldrs,
    TextReader& reader, std::optional<std::string>& nextLine)
    : XyBase(label, variableList, ldrs, s_xyPointsLabel,
        std::vector<std::string>{
            s_xyPointsVariableLists.begin(), s_xyPointsVariableLists.end()},
        reader, nextLine)
{
}

std::vector<std::pair<double, double>>
sciformats::jdx::XyPoints::getData() const
{
    auto varList = getVariableList();
    if (std::any_of(s_xyPointsVariableLists.begin(),
            s_xyPointsVariableLists.end(),
            [&varList](const std::string& s) { return s == varList; }))
    {
        return XyBase::getXYXYData();
    }
    throw ParseException("Unsupported variable list for XYPOINTS: " + varList);
}
