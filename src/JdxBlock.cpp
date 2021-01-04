#include "jdx/JdxBlock.hpp"
#include "jdx/JdxLdrParser.hpp"

#include <algorithm>
#include <array>
#include <climits>
#include <cstring>
#include <limits>

sciformats::jdx::JdxBlock::JdxBlock(std::istream& iStream)
    : m_istream{iStream}
{
    auto firstLine = JdxLdrParser::readLine(m_istream);
    if (!JdxLdrParser::isLdrStart(firstLine))
    {
        throw std::runtime_error(
            std::string{"Malformed LDR start: "} + firstLine);
    }
    auto [label, title] = JdxLdrParser::parseLdrStart(firstLine);
    if (label != "TITLE")
    {
        throw std::runtime_error(
            std::string{"Malformed Block start, wrong label: "} + firstLine);
    }
    parseInput(title);
}

sciformats::jdx::JdxBlock::JdxBlock(
    const std::string& title, std::istream& iStream)
    : m_istream{iStream}
{
    parseInput(title);
}

void sciformats::jdx::JdxBlock::parseInput(const std::string& title)
{
    std::optional<std::string> label = "TITLE";
    std::string value = title;
    while (!m_istream.eof())
    {
        auto line = JdxLdrParser::readLine(m_istream);

        if (!JdxLdrParser::isLdrStart(line))
        {
            // should be continuation of previous LDR
            if (!label.has_value())
            {
                throw std::runtime_error(
                    std::string{"Unexpected content found in block \""}
                    + getLdr("TITLE").value().getValue() + "\": " + line);
            }
            // TODO: account for terminal "=" as non line breaking marker
            value.append('\n' + line);
            continue;
        }

        // start of new LDR
        // previous LDR completed
        if (label.has_value())
        {
            if (label.value().empty())
            {
                // last LDR start was an LDR comment "##="
                m_ldrComments.push_back(value);
            }
            else
            {
                // last LDR was a regular LDR
                if (getLdr(label.value()))
                {
                    // reference implementation seems to overwrite LDR with
                    // duplicate, but spec (JCAMP-DX IR 3.2) says
                    // a duplicate LDR is illegal in a block => throw
                    throw std::runtime_error(
                        std::string{"Duplicate LDR in Block \""}
                        + getLdr("TITLE").value().getValue() + "\": " + line);
                }
                m_ldrs.emplace_back(label.value(), value);
            }
        }

        // parse new LDR
        std::tie(label, value) = JdxLdrParser::parseLdrStart(line);
        // cover special cases
        if ("END" == label)
        {
            // end of block
            break;
        }
        if ("TITLE" == label)
        {
            // nested block
            auto block = JdxBlock(value, m_istream);
            m_blocks.push_back(std::move(block));
            label = std::nullopt;
        }
        else if ("XYDATA" == label)
        {
            if (getXyData())
            {
                // duplicate
                throw std::runtime_error(
                    "Multiple XYDATA LDRs encountered in block: \""
                    + getLdr("TITLE").value().getValue());
            }

            auto parameters = parseXyParameters(getLdrs());
            m_xyParameters = parameters;
            auto xyData
                = XyData(label.value(), value, m_istream, parameters);
            m_xyData.emplace(std::move(xyData));
        }
        else if ("RADATA" == label)
        {
            if (getRaData())
            {
                // duplicate
                throw std::runtime_error(
                    "Multiple RADATA LDRs encountered in block: \""
                    + getLdr("TITLE").value().getValue());
            }

            auto parameters = parseRaParameters(getLdrs());
            m_raParameters = parameters;
            auto raData
                = RaData(label.value(), value, m_istream, parameters);
            m_raData.emplace(std::move(raData));
        }
        // TODO: add special treatment for data LDRs (e.g. XYDATA,
        // XYPOINTS, RADATA, PEAK TABLE, PEAK ASSIGNMENTS, NTUPLES, ...)
    }
    if ("END" != label)
    {
        throw std::runtime_error(
            "Unexpected end of block. No END label found: \""
            + getLdr("TITLE").value().getValue());
    }
}

std::optional<const sciformats::jdx::JdxLdr> sciformats::jdx::JdxBlock::getLdr(
    const std::string& label) const
{
    return findLdr(m_ldrs, label);
}

const std::vector<sciformats::jdx::JdxLdr>&
sciformats::jdx::JdxBlock::getLdrs() const
{
    return m_ldrs;
}

const std::vector<sciformats::jdx::JdxBlock>&
sciformats::jdx::JdxBlock::getBlocks() const
{
    return m_blocks;
}

const std::vector<std::string>&
sciformats::jdx::JdxBlock::getLdrComments() const
{
    return m_ldrComments;
}

const std::optional<sciformats::jdx::XyData>&
sciformats::jdx::JdxBlock::getXyData() const
{
    return m_xyData;
}

const std::optional<sciformats::jdx::XyParameters>&
sciformats::jdx::JdxBlock::getXyDataParameters() const
{
    return m_xyParameters;
}

const std::optional<sciformats::jdx::RaData>&
sciformats::jdx::JdxBlock::getRaData() const
{
    return m_raData;
}

const std::optional<sciformats::jdx::RaParameters>&
sciformats::jdx::JdxBlock::getRaDataParameters() const
{
    return m_raParameters;
}

sciformats::jdx::XyParameters sciformats::jdx::JdxBlock::parseXyParameters(
    const std::vector<JdxLdr>& ldrs)
{
    // required
    // string
    auto xUnits = findLdrValue(ldrs, "XUNITS");
    auto yUnits = findLdrValue(ldrs, "YUNITS");
    // double
    auto firstX = findLdrValue(ldrs, "FIRSTX");
    auto lastX = findLdrValue(ldrs, "LASTX");
    auto xFactor = findLdrValue(ldrs, "XFACTOR");
    auto yFactor = findLdrValue(ldrs, "YFACTOR");
    auto nPoints = findLdrValue(ldrs, "NPOINTS");
    // optional
    // double
    auto firstY = findLdrValue(ldrs, "FIRSTY");
    auto maxX = findLdrValue(ldrs, "MAXX");
    auto minX = findLdrValue(ldrs, "MINX");
    auto maxY = findLdrValue(ldrs, "MAXY");
    auto minY = findLdrValue(ldrs, "MINY");
    auto resolution = findLdrValue(ldrs, "RESOLUTION");
    auto deltaX = findLdrValue(ldrs, "DELTAX");

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

sciformats::jdx::RaParameters sciformats::jdx::JdxBlock::parseRaParameters(
    const std::vector<JdxLdr>& ldrs)
{
    // required
    // string
    auto rUnits = findLdrValue(ldrs, "RUNITS");
    auto aUnits = findLdrValue(ldrs, "AUNITS");
    // double
    auto firstR = findLdrValue(ldrs, "FIRSTR");
    auto lastR = findLdrValue(ldrs, "LASTR");
    auto rFactor = findLdrValue(ldrs, "RFACTOR");
    auto aFactor = findLdrValue(ldrs, "AFACTOR");
    auto nPoints = findLdrValue(ldrs, "NPOINTS");
    // optional
    // double
    auto firstA = findLdrValue(ldrs, "FIRSTA");
    auto maxA = findLdrValue(ldrs, "MAXA"); // required, according to standard
    auto minA = findLdrValue(ldrs, "MINA"); // required, according to standard
    auto resolution = findLdrValue(ldrs, "RESOLUTION");
    auto deltaR = findLdrValue(ldrs, "DELTAR");
    auto zdp = findLdrValue(ldrs, "ZDP");
    // string
    auto alias = findLdrValue(ldrs, "ALIAS");

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

std::optional<const sciformats::jdx::JdxLdr> sciformats::jdx::JdxBlock::findLdr(
    const std::vector<JdxLdr>& ldrs, const std::string& label)
{
    std::string normalizedLabel = "##" + label + "=";
    // TODO: make normalizeLdrLabel() more generic
    sciformats::jdx::JdxLdrParser::normalizeLdrLabel(normalizedLabel);
    normalizedLabel = normalizedLabel.substr(2, normalizedLabel.size() - 3);
    auto it = std::find_if(
        ldrs.begin(), ldrs.end(), [&normalizedLabel](const JdxLdr& ldr) {
            return ldr.getLabel() == normalizedLabel;
        });

    if (it != ldrs.end())
    {
        return *it;
    }
    return std::nullopt;
}

std::optional<std::string> sciformats::jdx::JdxBlock::findLdrValue(
    const std::vector<JdxLdr>& ldrs, const std::string& label)
{
    auto ldr = JdxBlock::findLdr(ldrs, label);
    return ldr.has_value() ? std::optional<std::string>(ldr.value().getValue())
                           : std::optional<std::string>(std::nullopt);
}
