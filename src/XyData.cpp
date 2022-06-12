#include "jdx/XyData.hpp"
#include "jdx/XyBase.hpp"
#include "util/DataParser.hpp"
#include "util/LdrUtils.hpp"

sciformats::jdx::XyData::XyData(
    std::istream& iStream, const std::vector<StringLdr>& ldrs)
    : XyBase(iStream, ldrs, s_xyDataLabel, s_xyDataVariableList)
{
}

sciformats::jdx::XyData::XyData(const std::string& label,
    const std::string& variableList, std::istream& iStream,
    const std::vector<StringLdr>& ldrs)
    : XyBase(
        label, variableList, iStream, ldrs, s_xyDataLabel, s_xyDataVariableList)
{
}

std::vector<std::pair<double, double>> sciformats::jdx::XyData::getData()
{
    return XyBase::getData(Array2DData::DataEncoding::XppYY);
}
