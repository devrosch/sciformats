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
            auto xyData = XyData(label.value(), value, m_istream, m_ldrs);
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
            auto raData = RaData(label.value(), value, m_istream, m_ldrs);
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
    return JdxLdrParser::findLdr(m_ldrs, label);
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

const std::optional<sciformats::jdx::RaData>&
sciformats::jdx::JdxBlock::getRaData() const
{
    return m_raData;
}
