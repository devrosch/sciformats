#include "jdx/JdxBlock.hpp"
#include "jdx/JdxLdrParser.hpp"
#include "jdx/JdxXyData.hpp"

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
            continue;
        }
        if ("XYDATA" == label)
        {
            if (getXyData())
            {
                // duplicate
                throw std::runtime_error(
                    "Multiple XYDATA LDRs encountered in block: \""
                    + getLdr("TITLE").value().getValue());
            }
            auto firstX = getFirstX();
            auto lastX = getLastX();
            auto xFactor = getXFactor();
            auto yFactor = getYFactor();
            auto nPoints = getNPoints();
            if (!firstX.has_value() || !lastX.has_value()
                || !xFactor.has_value() || !yFactor.has_value()
                || !nPoints.has_value())
            {
                std::string missing{};
                missing += firstX.has_value() ? "" : " FIRSTX";
                missing += lastX.has_value() ? "" : " LASTX";
                missing += xFactor.has_value() ? "" : " XFACTOR";
                missing += yFactor.has_value() ? "" : " YFACTOR";
                missing += nPoints.has_value() ? "" : " NPOINTS";
                throw std::runtime_error(
                    "Required LDR(s) missing for XYDATA: {" + missing + " }");
            }

            // we're using unsigned long NPOINTS in a function expecting size_t
            static_assert(std::numeric_limits<unsigned long>::max()
                              // NOLINTNEXTLINE(misc-redundant-expression)
                              <= std::numeric_limits<size_t>::max(),
                "unsigned long max larger than size_t max");

            auto xyData = JdxXyData(label.value(), value, m_istream,
                firstX.value(), lastX.value(), xFactor.value(), yFactor.value(),
                nPoints.value());
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

// TODO: combine getXXX() for doubles
std::optional<double> sciformats::jdx::JdxBlock::getFirstX() const
{
    auto ldr = getLdr("FIRSTX");
    return ldr.has_value()
               ? std::optional<double>(std::stod(ldr.value().getValue()))
               : std::nullopt;
}

std::optional<double> sciformats::jdx::JdxBlock::getLastX() const
{
    auto ldr = getLdr("LASTX");
    return ldr.has_value()
               ? std::optional<double>(std::stod(ldr.value().getValue()))
               : std::nullopt;
}

std::optional<double> sciformats::jdx::JdxBlock::getXFactor() const
{
    auto ldr = getLdr("XFACTOR");
    return ldr.has_value()
               ? std::optional<double>(std::stod(ldr.value().getValue()))
               : std::nullopt;
}

std::optional<double> sciformats::jdx::JdxBlock::getYFactor() const
{
    auto ldr = getLdr("YFACTOR");
    return ldr.has_value()
               ? std::optional<double>(std::stod(ldr.value().getValue()))
               : std::nullopt;
}

std::optional<unsigned long> sciformats::jdx::JdxBlock::getNPoints() const
{
    auto ldr = getLdr("NPOINTS");
    return ldr.has_value() ? std::optional<unsigned long>(
               std::stoul(ldr.value().getValue()))
                           : std::nullopt;
}

const std::optional<sciformats::jdx::JdxXyData>&
sciformats::jdx::JdxBlock::getXyData() const
{
    return m_xyData;
}
