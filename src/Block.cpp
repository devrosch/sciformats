#include "jdx/Block.hpp"
#include "util/LdrUtils.hpp"
#include "util/StringUtils.hpp"

sciformats::jdx::Block::Block(std::unique_ptr<std::istream> streamPtr)
    : m_streamPtr{std::move(streamPtr)}
    , m_istream{*m_streamPtr}
{
    auto firstLine = util::readLine(m_istream);
    auto titleFirstLine = parseFirstLine(firstLine);
    parseInput(titleFirstLine);
}

sciformats::jdx::Block::Block(std::istream& iStream)
    : m_streamPtr{nullptr}
    , m_istream{iStream}
{
    auto firstLine = util::readLine(m_istream);
    auto titleFirstLine = parseFirstLine(firstLine);
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

std::string sciformats::jdx::Block::parseFirstLine(const std::string& firstLine)
{
    if (!util::isLdrStart(firstLine))
    {
        throw BlockParseException("Malformed LDR start: " + firstLine);
    }
    auto [label, value] = util::parseLdrStart(firstLine);
    if (label != s_blockStartLabel)
    {
        throw BlockParseException(
            "Malformed Block start, wrong label: " + firstLine);
    }
    return value;
}

void sciformats::jdx::Block::parseInput(const std::string& titleValue)
{
    std::string title = titleValue;
    std::optional<std::string> nextLine = parseStringValue(title);
    m_ldrs.emplace_back(s_blockStartLabel, title);

    while (nextLine.has_value())
    {
        // "auto [label, value] = util::parseLdrStart(nextLine.value());" cannot
        // be used as lambdas (below) cannot capture these variables
        // see:
        // https://stackoverflow.com/questions/46114214/lambda-implicit-capture-fails-with-variable-declared-from-structured-binding
        std::string label;
        std::string value;
        std::tie(label, value) = util::parseLdrStart(nextLine.value());
        if (!isSpecialLabel(label))
        {
            // LDR is a regular LDR
            if (getLdr(label))
            {
                // reference implementation seems to overwrite LDR with
                // duplicate, but spec (JCAMP-DX IR 3.2) says
                // a duplicate LDR is illegal in a block => throw
                throw BlockParseException("Multiple", label, title);
            }
            nextLine = parseStringValue(value);
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
        else if (s_blockStartLabel == label)
        {
            // nested block
            auto block = Block(value, m_istream);
            m_blocks.push_back(std::move(block));
            nextLine = moveToNextLdr();
        }
        else if ("XYDATA" == label)
        {
            nextLine = addLdr<XyData>(title, "XYDATA", m_xyData,
                [&]() { return XyData(label, value, m_istream, m_ldrs); });
        }
        else if ("RADATA" == label)
        {
            nextLine = addLdr<RaData>(title, "RADATA", m_raData,
                [&]() { return RaData(label, value, m_istream, m_ldrs); });
        }
        else if ("XYPOINTS" == label)
        {
            nextLine = addLdr<XyPoints>(title, "XYPOINTS", m_xyPoints,
                [&]() { return XyPoints(label, value, m_istream, m_ldrs); });
        }
        else if ("PEAKTABLE" == label)
        {
            nextLine = addLdr<PeakTable>(title, "PEAKTABLE", m_peakTable,
                [&]() { return PeakTable(label, value, m_istream); });
        }
        else if ("PEAKASSIGNMENTS" == label)
        {
            nextLine = addLdr<PeakAssignments>(title, "PEAKASSIGNMENTS",
                m_peakAssignments,
                [&]() { return PeakAssignments(label, value, m_istream); });
        }
        else
        {
            // TODO: add special treatment for data LDRs (e.g. NTUPLES, ...),
            // DONE: XYDATA, RADATA, XYPOINTS, PEAK TABLE, PEAK ASSIGNMENTS
            throw BlockParseException("Unsupported", label, title);
        }
    }

    auto lastParsedLabel = util::parseLdrStart(nextLine.value()).first;
    if ("END" != lastParsedLabel)
    {
        throw BlockParseException("No", "END", title);
    }
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

bool sciformats::jdx::Block::isSpecialLabel(const std::string& label)
{
    return std::any_of(s_specialLdrs.cbegin(), s_specialLdrs.cend(),
        [&label](const char* specialLabel) { return specialLabel == label; });
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
            throw BlockParseException(
                "Unexpected content found in block \""
                + getLdr(s_blockStartLabel).value().getValue()
                + std::string{"\": "}.append(line.value()));
        }
    }

    return line;
}
