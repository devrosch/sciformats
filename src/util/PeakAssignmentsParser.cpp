#include "util/PeakAssignmentsParser.hpp"
#include "jdx/ParseException.hpp"
#include "jdx/PeakAssignment.hpp"
#include "util/LdrUtils.hpp"
#include "util/StringUtils.hpp"

#include <algorithm>

sciformats::jdx::util::PeakAssignmentsParser::PeakAssignmentsParser(
    TextReader& reader, size_t numVariables)
    : m_reader{reader}
    , m_numVariables{numVariables}
{
}

sciformats::jdx::PeakAssignment
sciformats::jdx::util::PeakAssignmentsParser::next()
{
    auto nextAssignmentString = readNextAssignmentString();
    if (!nextAssignmentString)
    {
        throw ParseException("No next peak assignment found at: "
                             + std::to_string(m_reader.tellg()));
    }
    auto nextAssignment = createPeakAssignment(nextAssignmentString.value());
    return nextAssignment;
}

bool sciformats::jdx::util::PeakAssignmentsParser::hasNext()
{
    if (m_reader.eof())
    {
        return false;
    }
    auto readerPos = m_reader.tellg();
    auto nextAssignmentString = readNextAssignmentString();
    // TODO: optimize
    m_reader.seekg(readerPos);
    return nextAssignmentString.has_value();
}

std::optional<std::string>
sciformats::jdx::util::PeakAssignmentsParser::readNextAssignmentString()
{
    std::string peakAssignmentString{};
    // find start
    while (!m_reader.eof())
    {
        std::streampos pos = m_reader.tellg();
        auto line = m_reader.readLine();
        auto [lineStart, comment] = util::stripLineComment(line);
        util::trim(lineStart);
        if (isPeakAssignmentStart(lineStart))
        {
            peakAssignmentString.append(lineStart);
            break;
        }
        if (util::isLdrStart(lineStart))
        {
            // PEAKASSIGNMENT LDR ended, no peak assignments
            m_reader.seekg(pos);
            return std::nullopt;
        }
        if (!lineStart.empty())
        {
            throw ParseException(
                "Illegal string found in peak assignment: " + line);
        }
    }
    if (isPeakAssignmentEnd(peakAssignmentString))
    {
        return peakAssignmentString;
    }
    // read to end of current peak assignment
    while (!m_reader.eof())
    {
        std::streampos pos = m_reader.tellg();
        auto line = m_reader.readLine();
        auto [lineStart, comment] = util::stripLineComment(line);
        util::trim(lineStart);

        if (util::isLdrStart(lineStart))
        {
            // PEAKASSIGNMENT LDR ended before end of last peak assignment
            m_reader.seekg(pos);
            throw ParseException(
                "No closing parenthesis found for peak assignment: "
                + peakAssignmentString);
        }
        peakAssignmentString.append(" ");
        peakAssignmentString.append(lineStart);
        if (isPeakAssignmentEnd(lineStart))
        {
            return peakAssignmentString;
        }
        if (m_reader.eof() || util::isLdrStart(lineStart))
        {
            // PEAKASSIGNMENT LDR ended before end of last peak assignment
            m_reader.seekg(pos);
            throw ParseException(
                "No closing parenthesis found for peak assignment: "
                + peakAssignmentString);
        }
    }
    throw ParseException(
        "File ended before closing parenthesis was found for peak assignment: "
        + peakAssignmentString);
}

bool sciformats::jdx::util::PeakAssignmentsParser::isPeakAssignmentStart(
    const std::string& stringValue)
{
    std::string value{stringValue};
    util::trimLeft(value);
    return !value.empty() && value.at(0) == '(';
}

bool sciformats::jdx::util::PeakAssignmentsParser::isPeakAssignmentEnd(
    const std::string& stringValue)
{
    std::string value{stringValue};
    util::trimRight(value);
    return !value.empty() && value.back() == ')';
}

sciformats::jdx::PeakAssignment
sciformats::jdx::util::PeakAssignmentsParser::createPeakAssignment(
    const std::string& stringValue) const
{
    auto [lineStart, comment] = util::stripLineComment(stringValue);
    util::trim(lineStart);
    if (!isPeakAssignmentStart(stringValue)
        || !isPeakAssignmentEnd(stringValue))
    {
        throw ParseException("Illegal peak assignment string: " + stringValue);
    }
    size_t pos = 1;
    auto token0 = parseNextPeakAssignmentToken(stringValue, pos);
    auto token1 = parseNextPeakAssignmentToken(stringValue, pos);
    auto token2 = pos < stringValue.length()
                      ? parseNextPeakAssignmentToken(stringValue, pos)
                      : std::nullopt;
    if (m_numVariables <= 3 && pos < stringValue.length())
    {
        throw ParseException(
            "Illegal peak assignment string. Illegal number of tokens: "
            + stringValue);
    }
    auto token3 = pos < stringValue.length()
                      ? parseNextPeakAssignmentToken(stringValue, pos)
                      : std::nullopt;
    if (m_numVariables <= 4 && pos < stringValue.length())
    {
        throw ParseException(
            "Illegal peak assignment string. Illegal number of tokens: "
            + stringValue);
    }

    PeakAssignment peakAssignment{};
    if (m_numVariables == 3)
    {
        if (token2)
        {
            // 3 tokens
            peakAssignment.x = token0.value().empty()
                                   ? std::numeric_limits<double>::quiet_NaN()
                                   : strtod(token0.value().data(), nullptr);
            peakAssignment.y = token1.value().empty()
                                   ? std::numeric_limits<double>::quiet_NaN()
                                   : strtod(token1.value().data(), nullptr);
            peakAssignment.a = token2 ? token2.value() : "";
        }
        else
        {
            // 2 tokens
            peakAssignment.x = token0.value().empty()
                                   ? std::numeric_limits<double>::quiet_NaN()
                                   : strtod(token0.value().data(), nullptr);
            peakAssignment.a = token1.value();
        }
    }
    else if (m_numVariables == 4)
    {
        // 2, 3 or 4 tokens
        if (token2 && token3)
        {
            // 4 tokens
            peakAssignment.x = token0.value().empty()
                                   ? std::numeric_limits<double>::quiet_NaN()
                                   : strtod(token0.value().data(), nullptr);
            peakAssignment.y = token1.value().empty()
                                   ? std::numeric_limits<double>::quiet_NaN()
                                   : strtod(token1.value().data(), nullptr);
            peakAssignment.w = token2.value().empty()
                                   ? std::numeric_limits<double>::quiet_NaN()
                                   : strtod(token2.value().data(), nullptr);
            peakAssignment.a = token3.value();
        }
        else if (token2)
        {
            // 3 tokens
            throw ParseException("Ambiguous peak assignment (second "
                                 "variable Y or W) for four variables: "
                                 + lineStart);
        }
        else
        {
            // 2 tokens
            peakAssignment.x = token0.value().empty()
                                   ? std::numeric_limits<double>::quiet_NaN()
                                   : strtod(token0.value().data(), nullptr);
            peakAssignment.a = token1.value();
        }
    }
    else
    {
        throw ParseException("Unsupported number of variables: "
                             + std::to_string(m_numVariables));
    }
    return peakAssignment;
}

std::optional<std::string>
sciformats::jdx::util::PeakAssignmentsParser::parseNextPeakAssignmentToken(
    const std::string& stringValue, size_t& position)
{
    auto isTokenDelimiter = [](const std::string& string, size_t pos) {
        return string.at(pos) == ',' || string.at(pos) == ')';
    };
    auto isNonWhitespace = [](char c) { return !util::isSpace(c); };
    std::string token{};
    if (position == 0 && stringValue.at(0) == '(')
    {
        // skip leading '('
        position++;
    }
    while (position < stringValue.length()
           && !isTokenDelimiter(stringValue, position))
    {
        if (stringValue.at(position) == '<')
        {
            // string token
            if (std::any_of(token.begin(), token.end(), isNonWhitespace))
            {
                throw ParseException(
                    "Non whitespace characters before string token at: "
                    + stringValue);
            }
            position++;
            token = parsePeakAssignmentStringToken(stringValue, position);
            // consume whitespace characters after string end delimiter
            while (position < stringValue.length()
                   && !isTokenDelimiter(stringValue, position))
            {
                if (isNonWhitespace(stringValue.at(position)))
                {
                    throw ParseException(
                        "Non whitespace character after string token at: "
                        + stringValue);
                }
                position++;
            }
            break;
        }
        if (stringValue.at(position) == '>')
        {
            throw ParseException(
                "Missing opening angle bracket at: " + stringValue);
        }
        token += stringValue.at(position);
        position++;
    }
    if (position >= stringValue.length())
    {
        throw ParseException(
            "No delimiter encountered at end of peak assignment token: "
            + stringValue);
    }
    position++;
    util::trim(token);
    return token;
}

std::string
sciformats::jdx::util::PeakAssignmentsParser::parsePeakAssignmentStringToken(
    const std::string& stringValue, size_t& position)
{
    std::string token{};
    while (position < stringValue.length() && stringValue.at(position) != '>')
    {
        token += stringValue.at(position);
        position++;
    }
    if (position >= stringValue.length())
    {
        throw ParseException(
            "No delimiter encountered at end of peak assignment string token: "
            + stringValue);
    }
    position++;
    return token;
}
