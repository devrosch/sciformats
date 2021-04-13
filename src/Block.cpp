#include "jdx/Block.hpp"
#include "jdx/LdrParser.hpp"

#include <algorithm>
#include <array>
#include <climits>
#include <cstring>
#include <limits>

sciformats::jdx::Block::Block(std::istream& iStream)
    : m_istream{iStream}
{
    auto firstLine = LdrParser::readLine(m_istream);
    if (!LdrParser::isLdrStart(firstLine))
    {
        throw std::runtime_error("Malformed LDR start: " + firstLine);
    }
    auto [label, title] = LdrParser::parseLdrStart(firstLine);
    if (label != "TITLE")
    {
        throw std::runtime_error(
            "Malformed Block start, wrong label: " + firstLine);
    }
    parseInput(title);
}

sciformats::jdx::Block::Block(const std::string& title, std::istream& iStream)
    : m_istream{iStream}
{
    parseInput(title);
}

void sciformats::jdx::Block::parseInput(const std::string& title)
{
    std::optional<std::string> label = "TITLE";
    std::string value = title;
    while (!m_istream.eof())
    {
        auto line = LdrParser::readLine(m_istream);

        if (!LdrParser::isLdrStart(line))
        {
            // continuation of previous LDR
            if (!label.has_value())
            {
                throw std::runtime_error("Unexpected content found in block \""
                                         + getLdr("TITLE").value().getValue()
                                         + "\": " + line);
            }
            if (!value.empty() && value.back() == '=')
            {
                // account for terminal "=" as non line breaking marker
                value.pop_back();
                value.append(line);
            }
            else
            {
                value.append('\n' + line);
            }
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
                        "Duplicate LDR in Block \""
                        + getLdr("TITLE").value().getValue() + "\": " + line);
                }
                m_ldrs.emplace_back(label.value(), value);
            }
        }

        // parse new LDR
        std::tie(label, value) = LdrParser::parseLdrStart(line);
        // cover special cases
        if ("END" == label)
        {
            // end of block
            break;
        }
        if ("TITLE" == label)
        {
            // nested block
            auto block = Block(value, m_istream);
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
        // TODO: add special treatment for data LDRs (e.g. XYPOINTS, PEAK TABLE,
        // PEAK ASSIGNMENTS, NTUPLES, ...), DONE: XYDATA, RADATA
    }
    if ("END" != label)
    {
        throw std::runtime_error(
            "Unexpected end of block. No END label found: \""
            + getLdr("TITLE").value().getValue());
    }
}

std::optional<const sciformats::jdx::Ldr> sciformats::jdx::Block::getLdr(
    const std::string& label) const
{
    return LdrParser::findLdr(m_ldrs, label);
}

const std::vector<sciformats::jdx::Ldr>& sciformats::jdx::Block::getLdrs() const
{
    return m_ldrs;
}

const std::vector<sciformats::jdx::Block>&
sciformats::jdx::Block::getBlocks() const
{
    return m_blocks;
}

const std::vector<std::string>& sciformats::jdx::Block::getLdrComments() const
{
    return m_ldrComments;
}

const std::optional<sciformats::jdx::XyData>&
sciformats::jdx::Block::getXyData() const
{
    return m_xyData;
}

const std::optional<sciformats::jdx::RaData>&
sciformats::jdx::Block::getRaData() const
{
    return m_raData;
}
