#include "jdx/XyBase.hpp"
#include "jdx/ParseException.hpp"
#include "util/DataParser.hpp"
#include "util/LdrUtils.hpp"

#include <limits>

sciformats::jdx::XyBase::XyBase(const std::string& label,
    const std::string& variableList, const std::vector<StringLdr>& ldrs,
    const std::string& expectedLabel,
    const std::vector<std::string>& expectedVariableLists, TextReader& reader,
    std::optional<std::string>& nextLine)
    : Data2D{label, variableList, reader}
{
    validateInput(
        getLabel(), getVariableList(), expectedLabel, expectedVariableLists);
    m_parameters = parseParameters(ldrs);
    util::skipToNextLdr(reader, nextLine, true);
}

const sciformats::jdx::XyParameters&
sciformats::jdx::XyBase::getParameters() const
{
    return m_parameters;
}

std::vector<std::pair<double, double>>
sciformats::jdx::XyBase::getXppYYData() const
{
    return Data2D::parseXppYYData(getLabel(), getReader(), m_parameters.firstX,
        m_parameters.lastX, m_parameters.yFactor, m_parameters.nPoints);
}

std::vector<std::pair<double, double>>
sciformats::jdx::XyBase::getXYXYData() const
{
    return Data2D::parseXyXyData(getLabel(), getReader(), m_parameters.xFactor,
        m_parameters.yFactor, m_parameters.nPoints);
}

sciformats::jdx::XyParameters sciformats::jdx::XyBase::parseParameters(
    const std::vector<StringLdr>& ldrs)
{
    // required
    // string
    auto xUnits = util::findLdrValue(ldrs, "XUNITS");
    auto yUnits = util::findLdrValue(ldrs, "YUNITS");
    // double
    auto firstX = util::findLdrValue(ldrs, "FIRSTX");
    auto lastX = util::findLdrValue(ldrs, "LASTX");
    auto xFactor = util::findLdrValue(ldrs, "XFACTOR");
    auto yFactor = util::findLdrValue(ldrs, "YFACTOR");
    auto nPoints = util::findLdrValue(ldrs, "NPOINTS");
    // optional
    // double
    auto firstY = util::findLdrValue(ldrs, "FIRSTY");
    auto maxX = util::findLdrValue(ldrs, "MAXX");
    auto minX = util::findLdrValue(ldrs, "MINX");
    auto maxY = util::findLdrValue(ldrs, "MAXY");
    auto minY = util::findLdrValue(ldrs, "MINY");
    auto resolution = util::findLdrValue(ldrs, "RESOLUTION");
    auto deltaX = util::findLdrValue(ldrs, "DELTAX");

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
        throw ParseException(
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
