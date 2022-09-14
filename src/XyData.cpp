#include "jdx/XyData.hpp"
#include "jdx/ParseException.hpp"
#include "jdx/XyBase.hpp"

sciformats::jdx::XyData::XyData(const std::string& label,
    const std::string& variableList, const std::vector<StringLdr>& ldrs,
    TextReader& reader, std::optional<std::string>& nextLine)
    : XyBase(label, variableList, ldrs, s_xyDataLabel,
        std::vector<std::string>{
            s_xyDataVariableLists.begin(), s_xyDataVariableLists.end()},
        reader, nextLine)
{
}

std::vector<std::pair<double, double>> sciformats::jdx::XyData::getData()
{
    auto varList = getVariableList();
    if (s_xyDataVariableLists.at(0) == varList)
    {
        return XyBase::getXppYYData(Data2D::VariableList::XppYY);
    }
    if (s_xyDataVariableLists.at(1) == varList)
    {
        return XyBase::getXppYYData(Data2D::VariableList::XppRR);
    }
    if (s_xyDataVariableLists.at(2) == varList)
    {
        return XyBase::getXppYYData(Data2D::VariableList::XppII);
    }
    throw ParseException("Unsupported variable list for XYDATA: " + varList);
}
