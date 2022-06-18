#include "jdx/XyPoints.hpp"
#include "jdx/XyBase.hpp"

sciformats::jdx::XyPoints::XyPoints(const std::string& label,
    const std::string& variableList, TextReader& reader,
    const std::vector<StringLdr>& ldrs)
    : XyBase(label, variableList, reader, ldrs, s_xyPointsLabel,
        s_xyPointsVariableList)
{
}

std::vector<std::pair<double, double>> sciformats::jdx::XyPoints::getData()
{
    return XyBase::getData(Array2DData::DataEncoding::XyXy);
}
