#include "jdx/XyBase.hpp"
#include "jdx/DataParser.hpp"
#include "jdx/LdrParser.hpp"

sciformats::jdx::XyBase::XyBase(std::istream& iStream,
    const std::vector<Ldr>& ldrs, std::string expectedLabel,
    std::string expectedVariableList)
    : Data2D{iStream}
    , m_expectedLabel{std::move(expectedLabel)}
    , m_expectedVariableList{std::move(expectedVariableList)}
{
    Data2D::validateInput(
        getLabel(), getVariableList(), m_expectedLabel, m_expectedVariableList);
    m_parameters = parseParameters(ldrs);
    skipToNextLdr(iStream);
}

sciformats::jdx::XyBase::XyBase(const std::string& label,
    const std::string& variableList, std::istream& iStream,
    const std::vector<Ldr>& ldrs, std::string expectedLabel,
    std::string expectedVariableList)
    : Data2D{label, variableList, iStream}
    , m_expectedLabel{std::move(expectedLabel)}
    , m_expectedVariableList{std::move(expectedVariableList)}
{
    Data2D::validateInput(
        getLabel(), getVariableList(), m_expectedLabel, m_expectedVariableList);
    m_parameters = parseParameters(ldrs);
    skipToNextLdr(iStream);
}

const sciformats::jdx::XyParameters&
sciformats::jdx::XyBase::getParameters() const
{
    return m_parameters;
}

std::vector<std::pair<double, double>> sciformats::jdx::XyBase::getData(
    Data2D::DataEncoding encoding)
{
    Data2D::validateInput(
        getLabel(), getVariableList(), m_expectedLabel, m_expectedVariableList);
    return Data2D::getData(m_parameters.firstX, m_parameters.lastX,
        m_parameters.xFactor, m_parameters.yFactor, m_parameters.nPoints,
        encoding);
}

sciformats::jdx::XyParameters sciformats::jdx::XyBase::parseParameters(
    const std::vector<Ldr>& ldrs)
{
    // required
    // string
    auto xUnits = LdrParser::findLdrValue(ldrs, "XUNITS");
    auto yUnits = LdrParser::findLdrValue(ldrs, "YUNITS");
    // double
    auto firstX = LdrParser::findLdrValue(ldrs, "FIRSTX");
    auto lastX = LdrParser::findLdrValue(ldrs, "LASTX");
    auto xFactor = LdrParser::findLdrValue(ldrs, "XFACTOR");
    auto yFactor = LdrParser::findLdrValue(ldrs, "YFACTOR");
    auto nPoints = LdrParser::findLdrValue(ldrs, "NPOINTS");
    // optional
    // double
    auto firstY = LdrParser::findLdrValue(ldrs, "FIRSTY");
    auto maxX = LdrParser::findLdrValue(ldrs, "MAXX");
    auto minX = LdrParser::findLdrValue(ldrs, "MINX");
    auto maxY = LdrParser::findLdrValue(ldrs, "MAXY");
    auto minY = LdrParser::findLdrValue(ldrs, "MINY");
    auto resolution = LdrParser::findLdrValue(ldrs, "RESOLUTION");
    auto deltaX = LdrParser::findLdrValue(ldrs, "DELTAX");

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
