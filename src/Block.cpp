#include "jdx/Block.hpp"
#include "jdx/LdrUtils.hpp"

#include <algorithm>
#include <array>
#include <climits>
#include <cstring>
#include <limits>

sciformats::jdx::Block::Block(std::istream& iStream)
    : m_istream{iStream}
{
    auto firstLine = util::readLine(m_istream);
    if (!util::isLdrStart(firstLine))
    {
        throw std::runtime_error("Malformed LDR start: " + firstLine);
    }
    auto [label, title] = util::parseLdrStart(firstLine);
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
        const auto line = util::readLine(m_istream);

        if (!util::isLdrStart(line))
        {
            // continuation of previous LDR
            if (!label.has_value())
            {
                // account for special case that a $$ comment immediately
                // follows a nested block
                auto [preCommentValue, comment]
                    = util::stripLineComment(line);
                util::trim(preCommentValue);
                // if not this special case, give up
                if (!preCommentValue.empty())
                {
                    throw std::runtime_error(
                        "Unexpected content found in block \"" + title
                        + std::string{"\": "}.append(line));
                }
            }
            else if (!value.empty() && value.back() == '=')
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
                        "Duplicate LDR in Block \"" + title
                        + std::string{"\": "}.append(line));
                }
                m_ldrs.emplace_back(label.value(), value);
            }
        }

        // parse new LDR
        std::tie(label, value) = util::parseLdrStart(line);
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
                    "Multiple XYDATA LDRs encountered in block: \"" + title);
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
                    "Multiple RADATA LDRs encountered in block: \"" + title);
            }
            auto raData = RaData(label.value(), value, m_istream, m_ldrs);
            m_raData.emplace(std::move(raData));
        }
        else if ("XYPOINTS" == label)
        {
            if (getXyPoints())
            {
                // duplicate
                throw std::runtime_error(
                    "Multiple XYPOINTS LDRs encountered in block: \"" + title);
            }
            auto xyPoints = XyPoints(label.value(), value, m_istream, m_ldrs);
            m_xyPoints.emplace(std::move(xyPoints));
        }
        else if ("PEAKTABLE" == label)
        {
            if (getPeakTable())
            {
                // duplicate
                throw std::runtime_error(
                    "Multiple PEAK TABLE LDRs encountered in block: \""
                    + title);
            }
            auto peakTable = PeakTable(label.value(), value, m_istream);
            m_peakTable.emplace(std::move(peakTable));
        }
        // TODO: add special treatment for data LDRs (e.g. PEAK ASSIGNMENTS,
        // NTUPLES, ...), DONE: XYDATA, RADATA, XYPOINTS, PEAK TABLE
    }
    if ("END" != label)
    {
        throw std::runtime_error(
            "Unexpected end of block. No END label found: \"" + title);
    }
}

std::optional<const sciformats::jdx::Ldr> sciformats::jdx::Block::getLdr(
    const std::string& label) const
{
    return util::findLdr(m_ldrs, label);
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

const std::optional<sciformats::jdx::XyPoints>&
sciformats::jdx::Block::getXyPoints() const
{
    return m_xyPoints;
}

const std::optional<sciformats::jdx::PeakTable>&
sciformats::jdx::Block::getPeakTable() const
{
    return m_peakTable;
}
