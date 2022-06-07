#include "jdx/util/PeakTableParser.hpp"
#include "jdx/Peak.hpp"
#include "jdx/util/LdrUtils.hpp"

#include <algorithm>

sciformats::jdx::util::PeakTableParser::PeakTableParser(
    std::istream& iStream, unsigned int numVariables)
    : m_istream{iStream}
    , m_numVariables{numVariables}
    , m_isPastInitialComment{false}
    , m_currentLine{""}
    , m_currentPos{0}
{
}

std::variant<std::string, sciformats::jdx::Peak>
sciformats::jdx::util::PeakTableParser::next()
{
    if (!m_isPastInitialComment)
    {
        auto kernelFunction = parseKernelFunctions();
        m_isPastInitialComment = true;
        if (kernelFunction)
        {
            return kernelFunction.value();
        }
    }

    std::optional<Peak> peak = nextPeak();

    if (!peak)
    {
        throw std::runtime_error(
            "No next peak found at: " + std::to_string(m_istream.tellg()));
    }

    return peak.value();
}

bool sciformats::jdx::util::PeakTableParser::hasNext()
{
    if (m_istream.eof())
    {
        return false;
    }
    auto streamPos = m_istream.tellg();
    auto currentLine = m_currentLine;
    auto currentPos = m_currentPos;

    // TODO: does not account for leading comments as next value
    std::optional<Peak> peak = nextPeak();

    // TODO: optimize
    m_istream.seekg(streamPos);
    m_currentLine = currentLine;
    m_currentPos = currentPos;

    return peak.has_value();
}

// TODO: similar to getKernel() in PeakTable
std::optional<std::string>
sciformats::jdx::util::PeakTableParser::parseKernelFunctions()
{
    // comment $$ in line(s) following LDR start may contain peak function
    auto streamPos = m_istream.tellg();
    std::string line{};
    std::string functionDescription{};
    while (!m_istream.eof()
           && !util::isLdrStart(line = util::readLine(m_istream))
           && isPureInlineComment(line))
    {
        streamPos = m_istream.tellg();
        auto [content, comment] = util::stripLineComment(line);
        appendToDescription(comment.value(), functionDescription);
    }
    // reset stream position to start of first assignment or start of next LDR
    m_istream.seekg(streamPos);
    // return
    return functionDescription.empty()
               ? std::nullopt
               : std::optional<std::string>{functionDescription};
}

bool sciformats::jdx::util::PeakTableParser::isPureInlineComment(
    const std::string& line)
{
    auto [content, comment] = util::stripLineComment(line);
    util::trim(content);
    return content.empty() && comment.has_value();
}

void sciformats::jdx::util::PeakTableParser::appendToDescription(
    std::string comment, std::string& description)
{
    if (!description.empty())
    {
        description += '\n';
    }
    util::trim(comment);
    description.append(comment);
}

std::optional<sciformats::jdx::Peak>
sciformats::jdx::util::PeakTableParser::nextPeak()
{
    std::optional<Peak> peak{std::nullopt};
    while (!peak)
    {
        if (m_currentPos < m_currentLine.size())
        {
            peak = nextPeak(m_currentLine, m_currentPos, m_numVariables);
        }
        else
        {
            if (m_istream.eof())
            {
                break;
            }
            auto streamPos = m_istream.tellg();
            m_currentLine = util::readLine(m_istream);
            std::tie(m_currentLine, std::ignore)
                = util::stripLineComment(m_currentLine);
            //            auto [content, comment] =
            //            util::stripLineComment(m_currentLine); m_currentLine =
            //            content;
            m_currentPos = 0;
            if (util::isLdrStart(m_currentLine))
            {
                m_istream.seekg(streamPos);
                break;
            }
        }
    }

    return peak;
}

std::optional<sciformats::jdx::Peak>
sciformats::jdx::util::PeakTableParser::nextPeak(
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
                "Missing peak component encountered in line \"" + line
                + "\" at position: " + std::to_string(prevPos));
        }
        if (!isNewGroup && i == 0U)
        {
            throw std::runtime_error(
                "Excess peak component encountered in line \"" + line
                + "\" at position: " + std::to_string(prevPos));
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
                "Missing peak component encountered in line \"" + line
                + "\" at position: " + std::to_string(prevPos));
        }
        try
        {
            auto value = std::stod(token.value());
            components.push_back(value);
        }
        catch (...)
        {
            throw std::runtime_error(
                "Cannot parse value in line \"" + line
                + "\" at position: " + std::to_string(prevPos));
        }
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
        "Unexpected number of peak components encountered in line \"" + line
        + "\": " + std::to_string(components.size()));
}

bool sciformats::jdx::util::PeakTableParser::skipToNextToken(
    const std::string& line, size_t& pos)
{
    bool componentSeparatorFound = false;
    bool nonWhitespaceDelimiterFound = false;
    while (pos < line.size() && isTokenDelimiter(line, pos))
    {
        const char c = line.at(pos);
        if (c == ',' || c == ';')
        {
            if (c == ',')
            {
                componentSeparatorFound = true;
            }
            if (nonWhitespaceDelimiterFound)
            {
                throw std::runtime_error(
                    "Missing peak component encountered in line \"" + line
                    + "\" at position: " + std::to_string(pos));
            }
            nonWhitespaceDelimiterFound = true;
        }
        ++pos;
    }
    return !componentSeparatorFound;
}

std::optional<std::string> sciformats::jdx::util::PeakTableParser::nextToken(
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

bool sciformats::jdx::util::PeakTableParser::isTokenDelimiter(
    const std::string& line, size_t& pos)
{
    if (pos >= line.size())
    {
        return true;
    }
    const char c = line.at(pos);
    return util::isSpace(c) || c == ';' || c == ',';
}
