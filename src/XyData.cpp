#include "jdx/XyData.hpp"
#include "jdx/JdxDataParser.hpp"
#include "jdx/JdxLdrParser.hpp"

sciformats::jdx::XyData::XyData(
    std::istream& iStream, const std::vector<JdxLdr>& ldrs)
    : Data2D(iStream)
{
    validateInput(getLabel(), getVariableList());
    m_parameters = parseParameters(ldrs);
    skipToNextLdr(iStream);
}

sciformats::jdx::XyData::XyData(const std::string& label,
    const std::string& variableList, std::istream& iStream,
    const std::vector<JdxLdr>& ldrs)
    : Data2D(label, variableList, iStream)
{
    validateInput(label, variableList);
    m_parameters = parseParameters(ldrs);
    skipToNextLdr(iStream);
}

const sciformats::jdx::XyParameters&
sciformats::jdx::XyData::getParameters() const
{
    return m_parameters;
}

std::vector<std::pair<double, double>> sciformats::jdx::XyData::getData()
{
    auto variableList = getVariableList();
    if (variableList == s_xppYYVariableList)
    {
        return Data2D::getData(m_parameters.firstX, m_parameters.lastX,
            m_parameters.xFactor, m_parameters.yFactor, m_parameters.nPoints,
            Data2D::DataEncoding::XppYY);
    }
    if (variableList == s_xyVariableList)
    {
        return Data2D::getData(m_parameters.firstX, m_parameters.lastX,
            m_parameters.xFactor, m_parameters.yFactor, m_parameters.nPoints,
            Data2D::DataEncoding::XyXy);
    }
    throw std::runtime_error(
        "Illegal variable list for XYDATA encountered: " + variableList);
}

void sciformats::jdx::XyData::validateInput(
    const std::string& label, const std::string& variableList)
{
    if (label != "XYDATA")
    {
        throw std::runtime_error(
            "Illegal label at XYDATA start encountered: " + label);
    }
    if (variableList != s_xppYYVariableList && variableList != s_xyVariableList)
    {
        throw std::runtime_error(
            "Illegal variable list for XYDATA encountered: " + variableList);
    }
}

sciformats::jdx::XyParameters sciformats::jdx::XyData::parseParameters(
    const std::vector<JdxLdr>& ldrs)
{
    // required
    // string
    auto xUnits = JdxLdrParser::findLdrValue(ldrs, "XUNITS");
    auto yUnits = JdxLdrParser::findLdrValue(ldrs, "YUNITS");
    // double
    auto firstX = JdxLdrParser::findLdrValue(ldrs, "FIRSTX");
    auto lastX = JdxLdrParser::findLdrValue(ldrs, "LASTX");
    auto xFactor = JdxLdrParser::findLdrValue(ldrs, "XFACTOR");
    auto yFactor = JdxLdrParser::findLdrValue(ldrs, "YFACTOR");
    auto nPoints = JdxLdrParser::findLdrValue(ldrs, "NPOINTS");
    // optional
    // double
    auto firstY = JdxLdrParser::findLdrValue(ldrs, "FIRSTY");
    auto maxX = JdxLdrParser::findLdrValue(ldrs, "MAXX");
    auto minX = JdxLdrParser::findLdrValue(ldrs, "MINX");
    auto maxY = JdxLdrParser::findLdrValue(ldrs, "MAXY");
    auto minY = JdxLdrParser::findLdrValue(ldrs, "MINY");
    auto resolution = JdxLdrParser::findLdrValue(ldrs, "RESOLUTION");
    auto deltaX = JdxLdrParser::findLdrValue(ldrs, "DELTAX");

    std::string missing{};
    missing += xUnits.has_value() ? "" : " XUNITS";
    missing += yUnits.has_value() ? "" : " YUNITS";
    missing += firstX.has_value() ? "" : " FIRSTX";
    missing += lastX.has_value() ? "" : " LASTX";
    missing += xFactor.has_value() ? "" : " XFACTOR";
    missing += yFactor.has_value() ? "" : " YFACTOR";
    missing += nPoints.has_value() ? "" : " NPOINTS";

    if (!missing.empty())
    {
        throw std::runtime_error(
            "Required LDR(s) missing for XYDATA: {" + missing + " }");
    }

    // we're parsing NPOINTS as unsigned long and assigning to unint_64
    static_assert(std::numeric_limits<unsigned long>::max()
                      // NOLINTNEXTLINE(misc-redundant-expression)
                      <= std::numeric_limits<uint64_t>::max(),
        "unsigned long max larger than uint_64_t max");

    // parse values
    XyParameters parms;
    parms.xUnits = xUnits.value();
    parms.yUnits = yUnits.value();
    parms.firstX = std::stod(firstX.value());
    parms.lastX = std::stod(lastX.value());
    parms.xFactor = std::stod(xFactor.value());
    parms.yFactor = std::stod(yFactor.value());
    parms.nPoints = std::stoul(nPoints.value());
    parms.firstY = firstY.has_value()
                       ? std::optional<double>(std::stod(firstY.value()))
                       : std::nullopt;
    parms.maxX = maxX.has_value()
                     ? std::optional<double>(std::stod(maxX.value()))
                     : std::nullopt;
    parms.minX = minX.has_value()
                     ? std::optional<double>(std::stod(minX.value()))
                     : std::nullopt;
    parms.maxY = maxY.has_value()
                     ? std::optional<double>(std::stod(maxY.value()))
                     : std::nullopt;
    parms.minY = minY.has_value()
                     ? std::optional<double>(std::stod(minY.value()))
                     : std::nullopt;
    parms.resolution = resolution.has_value() ? std::optional<double>(
                           std::stod(resolution.value()))
                                              : std::nullopt;
    parms.deltaX = deltaX.has_value()
                       ? std::optional<double>(std::stod(deltaX.value()))
                       : std::nullopt;
    return parms;
}
