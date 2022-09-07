#include "jdx/XyData.hpp"
#include "jdx/XyBase.hpp"

sciformats::jdx::XyData::XyData(const std::string& label,
    const std::string& variableList, TextReader& reader,
    const std::vector<StringLdr>& ldrs, std::optional<std::string>& nextLine)
    : XyBase(label, variableList, reader, ldrs, s_xyDataLabel,
        s_xyDataVariableList, nextLine)
{
}

std::vector<std::pair<double, double>> sciformats::jdx::XyData::getData()
{
    return XyBase::getData(Data2D::VariableList::XppYY);
}
