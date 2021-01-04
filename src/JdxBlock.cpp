#include "jdx/JdxBlock.hpp"
#include "jdx/JdxLdrParser.hpp"
#include "jdx/JdxXyData.hpp"
#include "jdx/RaParameters.hpp"
#include "jdx/XyParameters.hpp"

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
                = JdxXyData(label.value(), value, m_istream, parameters);
            m_xyData.emplace(xyData);
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
    std::string normalizedLabel = "##" + label + "=";
    // TODO: make normalizeLdrLabel() more generic
    sciformats::jdx::JdxLdrParser::normalizeLdrLabel(normalizedLabel);
    normalizedLabel = normalizedLabel.substr(2, normalizedLabel.size() - 3);
    auto it = std::find_if(
        m_ldrs.begin(), m_ldrs.end(), [&normalizedLabel](const JdxLdr& ldr) {
            return ldr.getLabel() == normalizedLabel;
        });

    if (it != m_ldrs.end())
    {
        return *it;
    }
    return std::nullopt;
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

const std::optional<sciformats::jdx::JdxXyData>&
sciformats::jdx::JdxBlock::getXyData() const
{
    return m_xyData;
}

const std::optional<sciformats::jdx::XyParameters>&
sciformats::jdx::JdxBlock::getXyDataParameters() const
{
    return m_xyParameters;
}

sciformats::jdx::XyParameters sciformats::jdx::JdxBlock::parseXyParameters(
    const std::vector<JdxLdr>& ldrs)
{
    auto findLdr = [ldrs](const std::string& label) {
        auto it = std::find_if(ldrs.begin(), ldrs.end(),
            [&label](const JdxLdr& ldr) { return ldr.getLabel() == label; });
        return it == ldrs.end() ? std::optional<std::string>(std::nullopt)
                                : std::optional<std::string>((*it).getValue());
    };

    // required
    // string
    auto xUnits = findLdr("XUNITS");
    auto yUnits = findLdr("YUNITS");
    // double
    auto firstX = findLdr("FIRSTX");
    auto lastX = findLdr("LASTX");
    auto xFactor = findLdr("XFACTOR");
    auto yFactor = findLdr("YFACTOR");
    auto nPoints = findLdr("NPOINTS");
    // optional
    // double
    auto firstY = findLdr("FIRSTY");
    auto maxX = findLdr("MAXX");
    auto minX = findLdr("MINX");
    auto maxY = findLdr("MAXY");
    auto minY = findLdr("MINY");

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
    parms.lastX = std::stod(firstX.value());
    parms.xFactor = std::stod(firstX.value());
    parms.yFactor = std::stod(firstX.value());
    parms.nPoints = std::stoul(firstX.value());
    parms.firstY = firstY.has_value()
                       ? std::optional<double>(std::stod(firstY.value()))
                       : std::nullopt;
    parms.maxX = maxX.has_value()
                     ? std::optional<double>(std::stod(maxX.value()))
                     : std::nullopt;
    parms.minX = maxX.has_value()
                     ? std::optional<double>(std::stod(minX.value()))
                     : std::nullopt;
    parms.maxY = maxX.has_value()
                     ? std::optional<double>(std::stod(maxY.value()))
                     : std::nullopt;
    parms.minY = maxX.has_value()
                     ? std::optional<double>(std::stod(minY.value()))
                     : std::nullopt;

    return parms;
}
