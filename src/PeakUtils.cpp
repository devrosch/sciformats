#include "jdx/PeakUtils.hpp"
#include "jdx/LdrParser.hpp"
#include "jdx/PeakAssignment.hpp"

#include <algorithm>
#include <limits>

sciformats::jdx::PeakAssignment
sciformats::jdx::peakutils::createPeakAssignment(const std::string& stringValue,
    size_t numVariables)
{
    auto [lineStart, comment] = LdrParser::stripLineComment(stringValue);
    LdrParser::trim(lineStart);
    if (!isPeakAssignmentStart(stringValue)
        || !isPeakAssignmentEnd(stringValue))
    {
        throw std::runtime_error(
            "Illegal peak assignment string: " + stringValue);
    }
    size_t pos = 1;
    auto token0 = parseNextToken(stringValue, pos);
    auto token1 = parseNextToken(stringValue, pos);
    auto token2 = pos < stringValue.length() ? parseNextToken(stringValue, pos)
                                             : std::nullopt;
    if (numVariables <= 3 && pos < stringValue.length())
    {
        throw std::runtime_error(
            "Illegal peak assignment string. Illegal number of tokens: "
            + stringValue);
    }
    auto token3 = pos < stringValue.length() ? parseNextToken(stringValue, pos)
                                             : std::nullopt;
    if (numVariables <= 4 && pos < stringValue.length())
    {
        throw std::runtime_error(
            "Illegal peak assignment string. Illegal number of tokens: "
            + stringValue);
    }

    PeakAssignment peakAssignment{};
    if (numVariables == 3)
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
    else if (numVariables == 4)
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
            throw std::runtime_error("Ambiguous peak assignment (second "
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
        throw std::runtime_error(
            "Unsupported number of variables: " + std::to_string(numVariables));
    }
    return peakAssignment;
}

std::optional<std::string> sciformats::jdx::peakutils::parseNextToken(
    const std::string& stringValue, size_t& position)
{
    auto isTokenDelimiter = [](const std::string& string, size_t pos)
    { return string.at(pos) == ',' || string.at(pos) == ')'; };
    auto isNonWhitespace = [](char c) { return !LdrParser::isSpace(c); };
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
                throw std::runtime_error(
                    "Non whitespace characters before string token at: "
                    + stringValue);
            }
            position++;
            token = parseStringToken(stringValue, position);
            // consume whitespace characters after string end delimiter
            while (position < stringValue.length()
                   && !isTokenDelimiter(stringValue, position))
            {
                if (isNonWhitespace(stringValue.at(position)))
                {
                    throw std::runtime_error(
                        "Non whitespace character after string token at: "
                        + stringValue);
                }
                position++;
            }
            break;
        }
        if (stringValue.at(position) == '>')
        {
            throw std::runtime_error(
                "Missing opening angle bracket at: " + stringValue);
        }
        token += stringValue.at(position);
        position++;
    }
    if (position >= stringValue.length())
    {
        throw std::runtime_error(
            "No delimiter encountered at end of peak assignment token: "
            + stringValue);
    }
    position++;
    LdrParser::trim(token);
    return token;
}

std::string sciformats::jdx::peakutils::parseStringToken(
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
        throw std::runtime_error(
            "No delimiter encountered at end of peak assignment string token: "
            + stringValue);
    }
    position++;
    return token;
}

bool sciformats::jdx::peakutils::isPeakAssignmentStart(
    const std::string& stringValue)
{
    std::string value{stringValue};
    LdrParser::trimLeft(value);
    return !value.empty() && value.at(0) == '(';
}

bool sciformats::jdx::peakutils::isPeakAssignmentEnd(
    const std::string& stringValue)
{
    std::string value{stringValue};
    LdrParser::trimRight(value);
    return !value.empty() && value.back() == ')';
}
