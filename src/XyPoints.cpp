#include "jdx/XyPoints.hpp"
#include "jdx/DataParser.hpp"
#include "jdx/LdrUtils.hpp"
#include "jdx/XyBase.hpp"

sciformats::jdx::XyPoints::XyPoints(
    std::istream& iStream, const std::vector<Ldr>& ldrs)
    : XyBase(iStream, ldrs, s_xyPointsLabel, s_xyPointsVariableList)
{
}

sciformats::jdx::XyPoints::XyPoints(const std::string& label,
    const std::string& variableList, std::istream& iStream,
    const std::vector<Ldr>& ldrs)
    : XyBase(label, variableList, iStream, ldrs, s_xyPointsLabel,
        s_xyPointsVariableList)
{
}

std::vector<std::pair<double, double>> sciformats::jdx::XyPoints::getData()
{
    return XyBase::getData(Data2D::DataEncoding::XyXy);
}
