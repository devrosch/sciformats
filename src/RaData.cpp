#include "jdx/RaData.hpp"
#include "jdx/ParseException.hpp"
#include "jdx/RaParameters.hpp"
#include "util/DataParser.hpp"
#include "util/LdrUtils.hpp"

#include <limits>

sciformats::jdx::RaData::RaData(const std::string& label,
    const std::string& variableList, const std::vector<StringLdr>& ldrs,
    TextReader& reader, std::optional<std::string>& nextLine)
    : Data2D(label, variableList, reader)
{
    validateInput(label, variableList, s_raDataLabel,
        std::vector<std::string>{s_raDataVariableList});
    m_parameters = parseParameters(ldrs);
    util::skipToNextLdr(reader, nextLine, true);
}

const sciformats::jdx::RaParameters&
sciformats::jdx::RaData::getParameters() const
{
    return m_parameters;
}

std::vector<std::pair<double, double>> sciformats::jdx::RaData::getData()
{
    return Data2D::parseXppYYData(getLabel(), getReader(), m_parameters.firstR,
        m_parameters.lastR, m_parameters.rFactor, m_parameters.nPoints);
}

sciformats::jdx::RaParameters sciformats::jdx::RaData::parseParameters(
    const std::vector<StringLdr>& ldrs)
{
    // required
    // string
    auto rUnits = util::findLdrValue(ldrs, "RUNITS");
    auto aUnits = util::findLdrValue(ldrs, "AUNITS");
    // double
    auto firstR = util::findLdrValue(ldrs, "FIRSTR");
    auto lastR = util::findLdrValue(ldrs, "LASTR");
    auto rFactor = util::findLdrValue(ldrs, "RFACTOR");
    auto aFactor = util::findLdrValue(ldrs, "AFACTOR");
    auto nPoints = util::findLdrValue(ldrs, "NPOINTS");
    // optional
    // double
    auto firstA = util::findLdrValue(ldrs, "FIRSTA");
    auto maxA
        = util::findLdrValue(ldrs, "MAXA"); // required, according to standard
    auto minA
        = util::findLdrValue(ldrs, "MINA"); // required, according to standard
    auto resolution = util::findLdrValue(ldrs, "RESOLUTION");
    auto deltaR = util::findLdrValue(ldrs, "DELTAR");
    auto zdp = util::findLdrValue(ldrs, "ZDP");
    // string
    auto alias = util::findLdrValue(ldrs, "ALIAS");

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
        throw ParseException(
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
    parms.zdp = zdp.has_value() ? std::optional<double>(std::stod(zdp.value()))
                                : std::nullopt;
    parms.alias = alias.has_value() ? std::optional<std::string>(alias.value())
                                    : std::nullopt;
    return parms;
}
