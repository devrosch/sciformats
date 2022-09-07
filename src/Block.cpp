#include "jdx/Block.hpp"
#include "util/LdrUtils.hpp"
#include "util/StringUtils.hpp"

sciformats::jdx::Block::Block(std::unique_ptr<TextReader> readerPtr)
    : m_readerPtr{std::move(readerPtr)}
    , m_reader{*m_readerPtr}
{
    auto firstLine = m_reader.readLine();
    auto titleFirstLine = parseFirstLine(firstLine);
    std::optional<std::string> nextLine;
    parseInput(titleFirstLine, nextLine);
}

sciformats::jdx::Block::Block(TextReader& reader)
    : m_readerPtr{nullptr}
    , m_reader{reader}
{
    auto firstLine = reader.readLine();
    auto titleFirstLine = parseFirstLine(firstLine);
    std::optional<std::string> nextLine;
    parseInput(titleFirstLine, nextLine);
}

sciformats::jdx::Block::Block(const std::string& title, TextReader& reader,
    std::optional<std::string>& nextLine)
    : m_readerPtr{nullptr}
    , m_reader{reader}
{
    parseInput(title, nextLine);
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

const std::optional<sciformats::jdx::NTuples>&
sciformats::jdx::Block::getNTuples() const
{
    return m_nTuples;
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

void sciformats::jdx::Block::parseInput(
    const std::string& titleValue, std::optional<std::string>& nextLine)
{
    std::string title = titleValue;
    nextLine = parseStringValue(title, m_reader);
    m_ldrs.emplace_back(s_blockStartLabel, title);

    while (nextLine.has_value())
    {
        if (util::isPureComment(nextLine.value()))
        {
            util::skipPureComments(m_reader, nextLine, true);
            continue;
        }
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
            nextLine = parseStringValue(value, m_reader);
            m_ldrs.emplace_back(label, value);
        }
        else if (label.empty())
        {
            // LDR start is an LDR comment "##="
            nextLine = parseStringValue(value, m_reader);
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
            auto block = Block(value, m_reader, nextLine);
            m_blocks.push_back(std::move(block));
        }
        else if ("XYDATA" == label)
        {
            addLdr<XyData>(title, "XYDATA", m_xyData, [&]() {
                return XyData(label, value, m_reader, m_ldrs, nextLine);
            });
        }
        else if ("RADATA" == label)
        {
            addLdr<RaData>(title, "RADATA", m_raData, [&]() {
                return RaData(label, value, m_reader, m_ldrs, nextLine);
            });
        }
        else if ("XYPOINTS" == label)
        {
            addLdr<XyPoints>(title, "XYPOINTS", m_xyPoints, [&]() {
                return XyPoints(label, value, m_reader, m_ldrs, nextLine);
            });
        }
        else if ("PEAKTABLE" == label)
        {
            addLdr<PeakTable>(title, "PEAKTABLE", m_peakTable,
                [&]() { return PeakTable(label, value, m_reader, nextLine); });
        }
        else if ("PEAKASSIGNMENTS" == label)
        {
            addLdr<PeakAssignments>(
                title, "PEAKASSIGNMENTS", m_peakAssignments, [&]() {
                    return PeakAssignments(label, value, m_reader, nextLine);
                });
        }
        else if ("NTUPLES" == label)
        {
            addLdr<NTuples>(title, "NTUPLES", m_nTuples, [&]() {
                return NTuples(label, value, m_reader, m_ldrs, nextLine);
            });
        }
        else
        {
            throw BlockParseException("Unsupported", label, title);
        }
    }

    auto lastParsedLabel = util::parseLdrStart(nextLine.value()).first;
    if ("END" != lastParsedLabel)
    {
        throw BlockParseException("No", "END", title);
    }
    // make nextline the one following the ##END= LDR
    nextLine = m_reader.eof() ? std::nullopt
                              : std::optional<std::string>{m_reader.readLine()};
}

bool sciformats::jdx::Block::isSpecialLabel(const std::string& label)
{
    return std::any_of(s_specialLdrs.cbegin(), s_specialLdrs.cend(),
        [&label](const char* specialLabel) { return specialLabel == label; });
}
