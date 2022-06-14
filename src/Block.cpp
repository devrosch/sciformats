#include "jdx/Block.hpp"
#include "util/LdrUtils.hpp"

#include <algorithm>
#include <array>
#include <climits>
#include <cstring>
#include <limits>

sciformats::jdx::Block::Block(std::istream& iStream)
    : m_istream{iStream}
{
    auto firstLine = util::readLine(m_istream);
    auto titleFirstLine = validateInput(firstLine);
    parseInput(titleFirstLine);
}

sciformats::jdx::Block::Block(const std::string& title, std::istream& iStream)
    : m_istream{iStream}
{
    parseInput(title);
}

std::optional<const sciformats::jdx::StringLdr> sciformats::jdx::Block::getLdr(
    const std::string& label) const
{
    return util::findLdr(m_ldrs, label);
}

const std::vector<sciformats::jdx::StringLdr>&
sciformats::jdx::Block::getLdrs() const
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

const std::optional<sciformats::jdx::PeakAssignments>&
sciformats::jdx::Block::getPeakAssignments() const
{
    return m_peakAssignments;
}

std::string sciformats::jdx::Block::validateInput(const std::string& firstLine)
{
    if (!util::isLdrStart(firstLine))
    {
        throw std::runtime_error("Malformed LDR start: " + firstLine);
    }
    auto [label, value] = util::parseLdrStart(firstLine);
    if (label != "TITLE")
    {
        throw std::runtime_error(
            "Malformed Block start, wrong label: " + firstLine);
    }
    return value;
}

void sciformats::jdx::Block::parseInput(const std::string& titleValue)
{
    std::string title = titleValue;
    std::optional<std::string> nextLine = parseStringValue(title);
    m_ldrs.emplace_back("TITLE", title);

    while (nextLine.has_value())
    {
        auto [label, value] = util::parseLdrStart(nextLine.value());
        if (!isSpecialLabel(label))
        {
            // LDR is a regular LDR
            nextLine = parseStringValue(value);
            if (getLdr(label))
            {
                // reference implementation seems to overwrite LDR with
                // duplicate, but spec (JCAMP-DX IR 3.2) says
                // a duplicate LDR is illegal in a block => throw
                throw std::runtime_error("Duplicate LDR in Block \"" + title
                                         + std::string{"\": "}.append(label));
            }
            m_ldrs.emplace_back(label, value);
        }
        else if (label.empty())
        {
            // LDR start is an LDR comment "##="
            nextLine = parseStringValue(value);
            m_ldrComments.push_back(value);
        }
        else if ("END" == label)
        {
            // end of block
            break;
        }
        else if ("TITLE" == label)
        {
            // nested block
            auto block = Block(value, m_istream);
            m_blocks.push_back(std::move(block));
            nextLine = moveToNextLdr();
        }
        else if ("XYDATA" == label)
        {
            if (getXyData())
            {
                // duplicate
                throw std::runtime_error(
                    "Multiple XYDATA LDRs encountered in block: \"" + title);
            }
            auto xyData = XyData(label, value, m_istream, m_ldrs);
            m_xyData.emplace(std::move(xyData));
            nextLine = moveToNextLdr();
        }
        else if ("RADATA" == label)
        {
            if (getRaData())
            {
                // duplicate
                throw std::runtime_error(
                    "Multiple RADATA LDRs encountered in block: \"" + title);
            }
            auto raData = RaData(label, value, m_istream, m_ldrs);
            m_raData.emplace(std::move(raData));
            nextLine = moveToNextLdr();
        }
        else if ("XYPOINTS" == label)
        {
            if (getXyPoints())
            {
                // duplicate
                throw std::runtime_error(
                    "Multiple XYPOINTS LDRs encountered in block: \"" + title);
            }
            auto xyPoints = XyPoints(label, value, m_istream, m_ldrs);
            m_xyPoints.emplace(std::move(xyPoints));
            nextLine = moveToNextLdr();
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
            auto peakTable = PeakTable(label, value, m_istream);
            m_peakTable.emplace(std::move(peakTable));
            nextLine = moveToNextLdr();
        }
        else if ("PEAKASSIGNMENTS" == label)
        {
            if (getPeakAssignments())
            {
                // duplicate
                throw std::runtime_error(
                    "Multiple PEAK ASSIGNMENTS LDRs encountered in block: \""
                    + title);
            }
            auto peakAssignments = PeakAssignments(label, value, m_istream);
            m_peakAssignments.emplace(std::move(peakAssignments));
            nextLine = moveToNextLdr();
        }
        else
        {
            // TODO: add special treatment for data LDRs (e.g. NTUPLES, ...),
            // DONE: XYDATA, RADATA, XYPOINTS, PEAK TABLE, PEAK ASSIGNMENTS
            std::string msg = "Unsupported LDR \"";
            msg += label;
            msg += "\"in block: \"";
            msg += title;
            throw std::runtime_error(msg);
        }
    }

    auto lastParsedLabel = util::parseLdrStart(nextLine.value()).first;
    if ("END" != lastParsedLabel)
    {
        throw std::runtime_error(
            "Unexpected end of block. No END label found for block: \""
            + title);
    }
}

std::optional<const std::string> sciformats::jdx::Block::moveToNextLdr()
{
    std::optional<std::string> line{std::nullopt};
    while (!m_istream.eof())
    {
        line = util::readLine(m_istream);
        if (util::isLdrStart(line.value()))
        {
            break;
        }
        // account for special case that a $$ comment immediately
        // follows a nested block
        auto [preCommentValue, comment] = util::stripLineComment(line.value());
        util::trim(preCommentValue);
        // if not this special case, give up
        if (!preCommentValue.empty())
        {
            throw std::runtime_error(
                "Unexpected content found in block \""
                + getLdr("TITLE").value().getLabel()
                + std::string{"\": "}.append(line.value()));
        }
    }

    return line;
}

bool sciformats::jdx::Block::isSpecialLabel(const std::string& label)
{
    return std::any_of(s_specialLdrs.cbegin(), s_specialLdrs.cend(),
        [&label](const char* specialLabel) { return specialLabel == label; });
}

std::optional<const std::string> sciformats::jdx::Block::parseStringValue(
    std::string& value)
{
    while (!m_istream.eof())
    {
        const auto line = util::readLine(m_istream);
        if (util::isLdrStart(line))
        {
            return line;
        }
        auto [content, comment] = util::stripLineComment(line);
        if (!content.empty() && value.back() == '=')
        {
            // account for terminal "=" as non line breaking marker
            value.pop_back();
            value.append(line);
        }
        else
        {
            value.append('\n' + line);
        }
    }
    return std::nullopt;
}
