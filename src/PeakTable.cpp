#include "jdx/PeakTable.hpp"
#include "jdx/LdrParser.hpp"
#include "jdx/Peak.hpp"

#include <algorithm>
#include <istream>
#include <tuple>

// TODO: duplicate of constructor in Data2D
sciformats::jdx::PeakTable::PeakTable(std::istream& istream)
    : m_istream{istream}
    , m_streamDataPos{istream.tellg()}
{
    std::tie(m_label, m_variableList) = readFirstLine(istream);
    m_streamDataPos = istream.tellg();
    validateInput(m_label, m_variableList, s_peakTableLabel,
        std::vector<std::string>{
            s_peakTableXyVariableList, s_peakTableXywVariableList});
    skipToNextLdr(istream);
}

// TODO: duplicate of constructor in Data2D
sciformats::jdx::PeakTable::PeakTable(
    std::string label, std::string variableList, std::istream& istream)
    : m_istream{istream}
    , m_streamDataPos{istream.tellg()}
    , m_label{std::move(label)}
    , m_variableList{std::move(variableList)}
{
    validateInput(m_label, m_variableList, s_peakTableLabel,
        std::vector<std::string>{
            s_peakTableXyVariableList, s_peakTableXywVariableList});
    skipToNextLdr(istream);
}

// TODO: duplicate of skipToNextLdr() in Data2D
void sciformats::jdx::PeakTable::skipToNextLdr(std::istream& iStream)
{
    while (!iStream.eof())
    {
        std::istream::pos_type pos = iStream.tellg();
        std::string line = sciformats::jdx::LdrParser::readLine(iStream);
        if (sciformats::jdx::LdrParser::isLdrStart(line))
        {
            // move back to start of LDR
            iStream.seekg(pos);
            break;
        }
    }
}

// TODO: duplicate of readFirstLine() in Data2D
std::pair<std::string, std::string> sciformats::jdx::PeakTable::readFirstLine(
    std::istream& istream)
{
    auto pos = istream.tellg();
    auto line = LdrParser::readLine(istream);
    if (!LdrParser::isLdrStart(line))
    {
        // reset for consistent state
        istream.seekg(pos);
        throw std::runtime_error(
            "Cannot parse PEAK TABLE. Stream position not at LDR start: "
            + line);
    }
    auto [label, variableList] = LdrParser::parseLdrStart(line);
    LdrParser::stripLineComment(variableList);
    LdrParser::trim(variableList);

    return {label, variableList};
}

// TODO: similar to validateInput() in Data2D
void sciformats::jdx::PeakTable::validateInput(const std::string& label,
    const std::string& variableList, const std::string& expectedLabel,
    const std::vector<std::string>& expectedVariableLists)
{
    if (label != expectedLabel)
    {
        throw std::runtime_error("Illegal label at " + expectedLabel
                                 + " start encountered: " + label);
    }
    if (std::none_of(expectedVariableLists.begin(), expectedVariableLists.end(),
            [&variableList](const std::string& expectedVariableList) {
                return variableList == expectedVariableList;
            }))
    {
        throw std::runtime_error("Illegal variable list for " + label
                                 + " encountered: " + variableList);
    }
}

std::vector<sciformats::jdx::Peak> sciformats::jdx::PeakTable::getData()
{
    // TODO: parse potential peak width and other peak kernel functions given as
    // comment $$ in line(s) following LDR start
    auto streamPos = m_istream.eof() ? std::nullopt : std::optional<std::streampos>(m_istream.tellg());
    try {
        m_istream.seekg(m_streamDataPos);
        auto numComponents = m_variableList == s_peakTableXyVariableList ? 2U : 3U;
        std::string line;
        std::vector<sciformats::jdx::Peak> peaks;
        while (!m_istream.eof()
               && !sciformats::jdx::LdrParser::isLdrStart(
                   line = sciformats::jdx::LdrParser::readLine(m_istream)))
        {
            // assume that a group (i.e. peak) does not span multiple lines
            size_t pos = 0;
            while (auto peak = nextPeak(line, pos, numComponents))
            {
                peaks.push_back(peak.value());
            }
        }
        if (streamPos)
        {
            m_istream.seekg(streamPos.value());
        }
        return peaks;
    } catch (...) {
        // TODO: duplicate code in Data2D
        try
        {
            if (streamPos)
            {
                m_istream.seekg(streamPos.value());
            }
        }
        catch (...)
        {
        }
        throw;
    }
}

std::optional<sciformats::jdx::Peak> sciformats::jdx::PeakTable::nextPeak(
    const std::string& line, size_t& pos, size_t numComponents)
{
    std::vector<double> components{};
    for (auto i = 0U; i < numComponents; ++i)
    {
        const auto prevPos = pos;
        auto isNewGroup = skipToNextToken(line, pos);
        if (isNewGroup && i != 0U)
        {
            throw std::runtime_error(
                "Missing peak component encountered in line " + line
                + " at position: " + std::to_string(prevPos));
        }
        if (!isNewGroup && i == 0U)
        {
            throw std::runtime_error(
                "Excess peak component encountered in line " + line
                + " at position: " + std::to_string(prevPos));
        }
        if (isNewGroup && i == 0U && pos >= line.size())
        {
            // no (more) peaks in line
            return std::nullopt;
        }
        auto token = nextToken(line, pos);
        if (!token.has_value())
        {
            throw std::runtime_error(
                "Missing peak component encountered in line " + line
                + " at position: " + std::to_string(prevPos));
        }
        auto value = std::stod(token.value());
        components.push_back(value);
    }

    if (numComponents == 2)
    {
        return Peak{components[0], components[1], std::nullopt};
    }
    if (numComponents == 3)
    {
        return Peak{components[0], components[1], components[2]};
    }
    throw std::runtime_error(
        "Unexpected number of peak components encountered in line " + line
        + ": " + std::to_string(components.size()));
}

bool sciformats::jdx::PeakTable::skipToNextToken(
    const std::string& line, size_t& pos)
{
    bool componentSeparatorFound = false;
    while (pos < line.size() && isTokenDelimiter(line, pos))
    {
        if (line.at(pos) == ',')
        {
            componentSeparatorFound = true;
        }
        ++pos;
    }
    return !componentSeparatorFound;
}

std::optional<std::string> sciformats::jdx::PeakTable::nextToken(
    const std::string& line, size_t& pos)
{
    if (pos >= line.size())
    {
        return std::nullopt;
    }
    auto first = pos;
    while (pos < line.size() && !isTokenDelimiter(line, pos))
    {
        ++pos;
    }
    auto token = line.substr(first, pos - first);
    return token;
}

bool sciformats::jdx::PeakTable::isTokenDelimiter(
    const std::string& line, size_t& pos)
{
    if (pos >= line.size())
    {
        return true;
    }
    const char c = line.at(pos);
    return LdrParser::isSpace(c) || c == ';' || c == ',';
}
