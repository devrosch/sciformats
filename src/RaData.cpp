#include "jdx/RaData.hpp"
#include "jdx/JdxDataParser.hpp"
#include "jdx/JdxLdrParser.hpp"
#include "jdx/RaParameters.hpp"

sciformats::jdx::RaData::RaData(
    std::istream& iStream, const std::vector<JdxLdr>& ldrs)
    : Data2D(iStream)
{
    validateInput(getLabel(), getVariableList());
    m_parameters = parseParameters(ldrs);
    skipToNextLdr(iStream);
}

sciformats::jdx::RaData::RaData(const std::string& label,
    const std::string& variableList, std::istream& iStream,
    const std::vector<JdxLdr>& ldrs)
    : Data2D(label, variableList, iStream)
{
    validateInput(label, variableList);
    m_parameters = parseParameters(ldrs);
    skipToNextLdr(iStream);
}

const sciformats::jdx::RaParameters&
sciformats::jdx::RaData::getParameters() const
{
    return m_parameters;
}

std::vector<std::pair<double, double>> sciformats::jdx::RaData::getData()
{
    return Data2D::getData(m_parameters.firstR, m_parameters.lastR,
        m_parameters.aFactor, m_parameters.nPoints);
}

void sciformats::jdx::RaData::validateInput(
    const std::string& label, const std::string& variableList)
{
    if (label != "RADATA")
    {
        throw std::runtime_error(
            "Illegal label at RADATA start encountered: " + label);
    }
    if (variableList != "(R++(A..A))" && variableList != "(RA..RA)")
    {
        throw std::runtime_error(
            "Illegal variable list for RADATA encountered: " + variableList);
    }
}

sciformats::jdx::RaParameters sciformats::jdx::RaData::parseParameters(
    const std::vector<JdxLdr>& ldrs)
{
    // required
    // string
    auto rUnits = JdxLdrParser::findLdrValue(ldrs, "RUNITS");
    auto aUnits = JdxLdrParser::findLdrValue(ldrs, "AUNITS");
    // double
    auto firstR = JdxLdrParser::findLdrValue(ldrs, "FIRSTR");
    auto lastR = JdxLdrParser::findLdrValue(ldrs, "LASTR");
    auto rFactor = JdxLdrParser::findLdrValue(ldrs, "RFACTOR");
    auto aFactor = JdxLdrParser::findLdrValue(ldrs, "AFACTOR");
    auto nPoints = JdxLdrParser::findLdrValue(ldrs, "NPOINTS");
    // optional
    // double
    auto firstA = JdxLdrParser::findLdrValue(ldrs, "FIRSTA");
    auto maxA = JdxLdrParser::findLdrValue(
        ldrs, "MAXA"); // required, according to standard
    auto minA = JdxLdrParser::findLdrValue(
        ldrs, "MINA"); // required, according to standard
    auto resolution = JdxLdrParser::findLdrValue(ldrs, "RESOLUTION");
    auto deltaR = JdxLdrParser::findLdrValue(ldrs, "DELTAR");
    auto zdp = JdxLdrParser::findLdrValue(ldrs, "ZDP");
    // string
    auto alias = JdxLdrParser::findLdrValue(ldrs, "ALIAS");

    std::string missing{};
    missing += rUnits.has_value() ? "" : " RUNITS";
    missing += aUnits.has_value() ? "" : " AUNITS";
    missing += firstR.has_value() ? "" : " FIRSTR";
    missing += lastR.has_value() ? "" : " LASTR";
    missing += rFactor.has_value() ? "" : " RFACTOR";
    missing += aFactor.has_value() ? "" : " AFACTOR";
    missing += nPoints.has_value() ? "" : " NPOINTS";

    if (!missing.empty())
    {
        throw std::runtime_error(
            "Required LDR(s) missing for RADATA: {" + missing + " }");
    }

    // we're parsing NPOINTS as unsigned long and assigning to unint_64
    static_assert(std::numeric_limits<unsigned long>::max()
                      // NOLINTNEXTLINE(misc-redundant-expression)
                      <= std::numeric_limits<uint64_t>::max(),
        "unsigned long max larger than uint_64_t max");

    // parse values
    RaParameters parms;
    parms.rUnits = rUnits.value();
    parms.aUnits = aUnits.value();
    parms.firstR = std::stod(firstR.value());
    parms.lastR = std::stod(lastR.value());
    parms.rFactor = std::stod(rFactor.value());
    parms.aFactor = std::stod(aFactor.value());
    parms.nPoints = std::stoul(nPoints.value());
    parms.firstA = firstA.has_value()
                       ? std::optional<double>(std::stod(firstA.value()))
                       : std::nullopt;
    parms.maxA = maxA.has_value()
                     ? std::optional<double>(std::stod(maxA.value()))
                     : std::nullopt;
    parms.minA = minA.has_value()
                     ? std::optional<double>(std::stod(minA.value()))
                     : std::nullopt;
    parms.resolution = resolution.has_value() ? std::optional<double>(
                           std::stod(resolution.value()))
                                              : std::nullopt;
    parms.deltaR = deltaR.has_value()
                       ? std::optional<double>(std::stod(deltaR.value()))
                       : std::nullopt;

    return parms;
}
