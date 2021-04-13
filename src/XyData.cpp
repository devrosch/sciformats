#include "jdx/XyData.hpp"
#include "jdx/DataParser.hpp"
#include "jdx/LdrParser.hpp"
#include "jdx/XyBase.hpp"

sciformats::jdx::XyData::XyData(
    std::istream& iStream, const std::vector<Ldr>& ldrs)
    : XyBase(iStream, ldrs, s_xyDataLabel, s_xyDataVariableList)
{
}

sciformats::jdx::XyData::XyData(const std::string& label,
    const std::string& variableList, std::istream& iStream,
    const std::vector<Ldr>& ldrs)
    : XyBase(
        label, variableList, iStream, ldrs, s_xyDataLabel, s_xyDataVariableList)
{
}

std::vector<std::pair<double, double>> sciformats::jdx::XyData::getData()
{
    return XyBase::getData(Data2D::DataEncoding::XppYY);
}
