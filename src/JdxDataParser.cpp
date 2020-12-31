#include "jdx/JdxDataParser.hpp"
#include "jdx/JdxLdrParser.hpp"

#include <cmath>
#include <limits>
#include <regex>

std::vector<double> sciformats::jdx::JdxDataParser::readXppYYData(
    std::istream& istream)
{
    static_assert(
        std::numeric_limits<double>::has_quiet_NaN, "No quiet NaN available.");

    // read (X++(Y..Y)) data
    std::vector<double> yValues;
    std::string line;
    std::streamoff pos = istream.tellg();
    std::optional<double> yValueCheck = std::nullopt;
    while (!sciformats::jdx::JdxLdrParser::isLdrStart(
        line = sciformats::jdx::JdxLdrParser::readLine(istream)))
    {
        // save position to move back if next readLine() encouinters LDR start
        pos = istream.tellg();
        // pre-process line
        auto [data, comment]
            = sciformats::jdx::JdxLdrParser::stripLineComment(line);
        sciformats::jdx::JdxLdrParser::trim(data);
        // read Y values from line
        std::vector<double> lineYValues = readXppYYLine(data, yValueCheck);
        if (yValueCheck.has_value())
        {
            // y value is duplicated in new line, trust new value
            yValues.pop_back();
        }
        // append line values to yValues
        yValues.insert(
            std::end(yValues), std::begin(lineYValues), std::end(lineYValues));
        // if last and second to last values are defined, use last as y check
        if (lineYValues.empty()
            || (lineYValues.size() == 1 && std::isnan(lineYValues.back()))
            || (lineYValues.size() >= 2
                && (std::isnan(lineYValues.back())
                    || std::isnan(lineYValues.at(lineYValues.size() - 2)))))
        {
            yValueCheck = std::nullopt;
        }
        else
        {
            yValueCheck = lineYValues.back();
        }
    }
    // next LDR encountered => all data read => move back to start of next LDR
    istream.seekg(pos);

    return yValues;
}

std::vector<double> sciformats::jdx::JdxDataParser::readXppYYLine(
    std::string& line, const std::optional<double>& yValueCheck)
{
    // read (X++(Y..Y)) data line
    auto values = readValues(line);
    if (!values.empty())
    {
        // remove initial x value (not required for (X++(Y..Y)) encoded data)
        values.erase(values.begin());
        // TODO: maybe perform some kind of X value check
    }
    if (yValueCheck.has_value() && !values.empty())
    {
        // first y value is a duplicate, check if roughly the same
        if (fabs(values.front() - yValueCheck.value()) >= 1)
        {
            throw std::runtime_error(
                std::string{"Y value check failed in line: "} + line);
        }
    }
    return values;
}

std::vector<double> sciformats::jdx::JdxDataParser::readValues(
        std::string& encodedValues)
{
    // output
    std::vector<double> yValues{};
    // state
    enum class TokenType
    {
        Affn,
        Sqz,
        Dif,
        Dup,
    };
    std::string tokenString{};
    TokenType tokenType = TokenType::Affn;
    std::optional<double> previousTokenValue{};
    TokenType previousTokenType = TokenType::Affn;
    // loop
    size_t index = 0;
    while (index <= encodedValues.size())
    {        
        auto isDelim = isTokenDelimiter(encodedValues, index);
        auto isStart = isTokenStart(encodedValues, index);
        if (isStart || isDelim)
        {
            if ((tokenType == TokenType::Dup || tokenType == TokenType::Dif)
                && !previousTokenValue.has_value())
            {
                auto message = tokenType == TokenType::Dup
                    ? std::string{"DUP"} : std::string{"DIF"};
                message += " token without preceding token"
                    " encountered in sequence: ";
                message += encodedValues;
                throw std::runtime_error(message);
            }
            if ((tokenType == TokenType::Dup && previousTokenValue.has_value()
                && previousTokenType == TokenType::Dup))
            {
                throw std::runtime_error(
                    std::string{"DUP token with preceding DUP token encountered "
                        "in sequence: "}
                            + encodedValues);
            }

            if (!tokenString.empty())
            {
                // a complete token has been captured
                if (tokenType == TokenType::Dup)
                {
                    auto numRepeats = std::stol(tokenString);
                    for (auto i{0}; i < numRepeats - 1; i++)
                    {
                        if (previousTokenType == TokenType::Dif)
                        {
                            auto lastValue = yValues.back();
                            auto nextValue = lastValue + previousTokenValue.value();
                            yValues.push_back(nextValue);
                        }
                        else
                        {
                            yValues.push_back(yValues.back());
                        }
                    }
                    previousTokenValue = numRepeats;
                }
                else
                {
                    // TODO: also account for ? values
                    auto value = std::stod(tokenString);
                    if (tokenType == TokenType::Dif)
                    {
                        auto lastValue = yValues.back();
                        auto nextValue = lastValue + value;
                        yValues.push_back(nextValue);
                    }
                    else
                    {
                        yValues.push_back(value);
                    }
                    previousTokenValue = value;
                }
                previousTokenType = tokenType;
                tokenString.clear();
                tokenType = TokenType::Affn;
            }
        }

        if (isStart)
        {
            // start recording new token
            auto c = encodedValues.at(index);
            if (auto sqzDigit = getSqzDigitValue(c))
            {
                // replace SQZ char (first char) with (signed) value
                tokenString = std::to_string(sqzDigit.value());
                tokenType = TokenType::Sqz;
            }
            else if (auto difDigit = getDifDigitValue(c))
            {
                // replace DIF char (first char) with (signed) value
                tokenString = std::to_string(difDigit.value());
                tokenType = TokenType::Dif;
            }
            else if (auto dupDigit = getDupDigitValue(c))
            {
                // replace DUP char (first char) with unsigned value
                tokenString = std::to_string(dupDigit.value());
                tokenType = TokenType::Dup;
            }
            else
            {
                // must be plain AFFN or PAC (or illegal)
                tokenString = c;
                tokenType = TokenType::Affn;
            }
        }
        else if (!isDelim)
        {
            // non start digit of token
            auto c = encodedValues.at(index);
            tokenString.push_back(c);
        }
        index++;
    }
    return yValues;
}

bool sciformats::jdx::JdxDataParser::isTokenDelimiter(std::string encodedValues, size_t index)
{
    if (index >= encodedValues.size())
    {
        return true;
    }
    char c = encodedValues.at(index);
    auto static isspace = [](unsigned char ch) {
        return static_cast<bool>(std::isspace(ch));
    };
    return isspace(static_cast<unsigned char>(c)) || c == ';' || c == ',';
}

bool sciformats::jdx::JdxDataParser::isTokenStart(std::string encodedValues, size_t index)
{
    if (index >= encodedValues.size())
    {
        return false;
    }
    const static std::regex regex{"^[eE][+-]{0,1}\\d{2,3}[;,\\s]{0,1}.*"};
    const static std::regex altRegex{"^[eE][+-]{0,1}\\d{1,3}[;,\\s].*"};
    char c = encodedValues.at(index);
    if ((getAsciiDigitValue(c).has_value() || c == '.')
        && (index == 0 || isTokenDelimiter(encodedValues, index - 1)))
    {
        return true;
    }
    if (c == 'E' || c == 'e')
    {
        // could be either an exponent or SQZ digit (E==+5, e==-5)
        // apply heuristic to provide answer
        auto substr = encodedValues.substr(index, 6);
        return !std::regex_match(substr, regex)
            && !std::regex_match(substr, altRegex);
    }
    if (c == '+' || c == '-')
    {
        if (index == 0)
        {
            return true;
        }
        // could be either a sign of an exponent or PAC start digit
        // apply heuristic to provide answer
        auto substr = encodedValues.substr(index - 1, 6);
        return !std::regex_match(substr, regex)
            && !std::regex_match(substr, altRegex);
    }
    if (getSqzDigitValue(c).has_value() || getDifDigitValue(c).has_value()
            || getDupDigitValue(c).has_value())
    {
        return true;
    }
    return false;
}

std::optional<char> sciformats::jdx::JdxDataParser::getAsciiDigitValue(const char c)
{
    static const std::string asciiDigits = "0123456789";
    auto pos = asciiDigits.find(c);
    return pos == std::string::npos
            ? std::nullopt : std::make_optional(static_cast<char>(pos));
}

std::optional<char> sciformats::jdx::JdxDataParser::getSqzDigitValue(const char c)
{
    static const std::string positiveSqzDigits = "@ABCDEFGHI";
    auto pos = positiveSqzDigits.find(c);
    if (pos != std::string::npos)
    {
        return std::make_optional(static_cast<char>(pos));
    }
    static const std::string negativeSqzDigits = "abcdefghi";
    pos = negativeSqzDigits.find(c);
    return pos == std::string::npos
            ? std::nullopt : std::make_optional(static_cast<char>(-pos - 1));
}

std::optional<char> sciformats::jdx::JdxDataParser::getDifDigitValue(const char c)
{
    static const std::string positiveDifDigits = "%JKLMNOPQR";
    auto pos = positiveDifDigits.find(c);
    if (pos != std::string::npos)
    {
        return std::make_optional(static_cast<char>(pos));
    }
    static const std::string negativeDifDigits = "jklmnopqr";
    pos = negativeDifDigits.find(c);
    return pos == std::string::npos
            ? std::nullopt : std::make_optional(static_cast<char>(-pos - 1));
}

std::optional<char> sciformats::jdx::JdxDataParser::getDupDigitValue(const char c)
{
    static const std::string positiveDupDigits = "STUVWXYZs";
    auto pos = positiveDupDigits.find(c);
    return pos == std::string::npos
            ? std::nullopt : std::make_optional(static_cast<char>(pos + 1));
}
