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

std::vector<std::pair<double, double>> sciformats::jdx::XyPoints::getData()
{
    auto varList = getVariableList();
    if (s_xyPointsVariableLists.at(0) == varList)
    {
        return XyBase::getXYXYData(Data2D::VariableList::XYXY);
    }
    if (s_xyPointsVariableLists.at(1) == varList)
    {
        return XyBase::getXYXYData(Data2D::VariableList::XRXR);
    }
    if (s_xyPointsVariableLists.at(2) == varList)
    {
        return XyBase::getXYXYData(Data2D::VariableList::XIXI);
    }
    throw ParseException("Unsupported variable list for XYPOINTS: " + varList);
}
