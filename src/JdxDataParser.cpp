#include "jdx/JdxDataParser.hpp"
#include "jdx/JdxLdrParser.hpp"

#include <cmath>
#include <limits>
#include <regex>
#include <string>

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
        // save position to move back if next readLine() encounters LDR start
        pos = istream.tellg();
        // pre-process line
        auto [data, comment]
            = sciformats::jdx::JdxLdrParser::stripLineComment(line);
        sciformats::jdx::JdxLdrParser::trim(data);
        // read Y values from line
        auto [lineYValues, isDifEncoded] = readXppYYLine(data, yValueCheck);
        if (yValueCheck.has_value())
        {
            // y value is duplicated in new line, trust new value
            yValues.pop_back();
        }
        // append line values to yValues
        yValues.insert(
            std::end(yValues), std::begin(lineYValues), std::end(lineYValues));
        // if last and second to last values are defined, use last as y check
        if (!isDifEncoded || lineYValues.empty()
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

std::vector<std::pair<double, double>>
sciformats::jdx::JdxDataParser::readXyXyData(std::istream& istream)
{
    static_assert(
        std::numeric_limits<double>::has_quiet_NaN, "No quiet NaN available.");

    // read (XY..XY) data
    std::vector<std::pair<double, double>> xyValues;
    bool lastValueIsXOnly = false;
    std::string line;
    std::streamoff pos = istream.tellg();
    while (!sciformats::jdx::JdxLdrParser::isLdrStart(
        line = sciformats::jdx::JdxLdrParser::readLine(istream)))
    {
        // save position to move back if next readLine() encounters LDR start
        pos = istream.tellg();
        // pre-process line
        auto [data, comment]
            = sciformats::jdx::JdxLdrParser::stripLineComment(line);
        sciformats::jdx::JdxLdrParser::trim(data);
        // read xy values from line
        auto [lineValues, isDifEncoded] = readValues(data);
        // turn line values into pairs and append line values to xyValues
        for (auto value : lineValues)
        {
            if (lastValueIsXOnly)
            {
                // must be y value
                xyValues.back().second = value;
                lastValueIsXOnly = false;
                continue;
            }
            // must be x value
            if (std::isnan(value))
            {
                throw std::runtime_error(
                    "NaN value encountered as x value in line: " + line);
            }
            std::pair<double, double> xyValue{
                value, std::numeric_limits<double>::quiet_NaN()};
            lastValueIsXOnly = true;
            xyValues.push_back(xyValue);
        }
    }
    // next LDR encountered => all data read => move back to start of next LDR
    istream.seekg(pos);

    if (lastValueIsXOnly)
    {
        // uneven number of single values
        throw std::runtime_error("uneven number of values for xy data "
                                 "encountered. No y value for x value: "
                                 + std::to_string(xyValues.back().first));
    }
    return xyValues;
}

std::pair<std::vector<double>, bool> sciformats::jdx::JdxDataParser::readValues(
    std::string& encodedValues)
{
    // output
    std::vector<double> yValues{};
    bool difEncoded = false;
    // state
    // for DIF/DUP previousTokenValue not same as last yValues value
    std::optional<double> previousTokenValue{};
    TokenType previousTokenType = TokenType::Affn;
    // loop
    size_t index = 0;
    while (auto token = nextToken(encodedValues, index))
    {
        TokenType tokenType = toAffn(token.value());
        if (tokenType == TokenType::Dif)
        {
            difEncoded = true;
        }

        // check for logical errors
        if ((tokenType == TokenType::Dup || tokenType == TokenType::Dif)
            && !previousTokenValue.has_value())
        {
            throw std::runtime_error(
                tokenType == TokenType::Dup
                    ? std::string{"DUP"}
                    : std::string{"DIF"}
                          + " token without preceding token encountered in "
                            "sequence: "
                          + encodedValues);
        }
        if ((tokenType == TokenType::Dup && previousTokenValue.has_value()
                && previousTokenType == TokenType::Dup))
        {
            throw std::runtime_error(
                "DUP token with preceding DUP token encountered in sequence: "
                + encodedValues);
        }

        // process token
        if (tokenType == TokenType::Missing)
        {
            // ?
            yValues.push_back(std::numeric_limits<double>::quiet_NaN());
            previousTokenValue = std::numeric_limits<double>::quiet_NaN();
        }
        else if (tokenType == TokenType::Dup)
        {
            auto numRepeats = std::stol(token.value());
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
            auto value = std::stod(token.value());
            if (tokenType == TokenType::Dif)
            {
                if (previousTokenType == TokenType::Missing)
                {
                    throw std::runtime_error("DIF token with preceding ? token "
                                             "encountered in sequence: "
                                             + encodedValues);
                }
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
    }
    return {yValues, difEncoded};
}

std::pair<std::vector<double>, bool>
sciformats::jdx::JdxDataParser::readXppYYLine(
    std::string& line, const std::optional<double>& yValueCheck)
{
    // read (X++(Y..Y)) data line
    auto [values, difEncoded] = readValues(line);
    if (!values.empty())
    {
        // remove initial x value (not required for (X++(Y..Y)) encoded data)
        values.erase(values.begin());
        // TODO: perform X value check
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
    return {values, difEncoded};
}

std::optional<std::string> sciformats::jdx::JdxDataParser::nextToken(
    const std::string& line, size_t& pos)
{
    if (line == "460.0, ?; 461.0, 21.0")
    {
        int i = 0;
    }
    // skip delimiters
    while (pos < line.size() && isTokenDelimiter(line, pos))
    {
        pos++;
    }
    if (pos >= line.size())
    {
        return std::nullopt;
    }
    if (!isTokenStart(line, pos))
    {
        throw std::runtime_error(
            std::string{"illegal sequence encountered in line \""} + line
            + "\" at position: " + std::to_string(pos));
    }
    std::string token;
    do
    {
        token += line.at(pos++);
    } while (!isTokenDelimiter(line, pos) && !isTokenStart(line, pos));
    return token;
}

sciformats::jdx::JdxDataParser::TokenType
sciformats::jdx::JdxDataParser::toAffn(std::string& token)
{
    auto c = token.front();
    TokenType tokenType = TokenType::Affn;
    std::optional<char> firstDigit;
    if (c == '?') {
        tokenType = TokenType::Missing;
    }
    else if ((firstDigit = getSqzDigitValue(c)))
    {
        tokenType = TokenType::Sqz;
    }
    else if ((firstDigit = getDifDigitValue(c)))
    {
        tokenType = TokenType::Dif;
    }
    else if ((firstDigit = getDupDigitValue(c)))
    {
        tokenType = TokenType::Dup;
    }

    if (TokenType::Affn != tokenType && TokenType::Missing != tokenType)
    {
        // replace SQZ/DIF/DUP char (first char) with (signed) value
        token.erase(0, 1);
        token.insert(0, std::to_string(firstDigit.value()));
    }
    // must be plain AFFN or PAC (or illegal)
    return tokenType;
}

bool sciformats::jdx::JdxDataParser::isTokenDelimiter(
    std::string encodedValues, size_t index)
{
    if (index >= encodedValues.size())
    {
        return true;
    }
    char c = encodedValues.at(index);
    auto static isspace
        = [](unsigned char ch) { return static_cast<bool>(std::isspace(ch)); };
    return isspace(static_cast<unsigned char>(c)) || c == ';' || c == ',';
}

bool sciformats::jdx::JdxDataParser::isTokenStart(
    std::string encodedValues, size_t index)
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
    if (c == '?')
    {
        // "invalid" data symbol
        return true;
    }
    return false;
}

std::optional<char> sciformats::jdx::JdxDataParser::getAsciiDigitValue(char c)
{
    static const std::string asciiDigits = "0123456789";
    auto pos = asciiDigits.find(c);
    return pos == std::string::npos
               ? std::nullopt
               : std::make_optional(static_cast<char>(pos));
}

std::optional<char> sciformats::jdx::JdxDataParser::getSqzDigitValue(char c)
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
               ? std::nullopt
               : std::make_optional(static_cast<char>(-pos - 1));
}

std::optional<char> sciformats::jdx::JdxDataParser::getDifDigitValue(char c)
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
               ? std::nullopt
               : std::make_optional(static_cast<char>(-pos - 1));
}

std::optional<char> sciformats::jdx::JdxDataParser::getDupDigitValue(char c)
{
    static const std::string positiveDupDigits = "STUVWXYZs";
    auto pos = positiveDupDigits.find(c);
    return pos == std::string::npos
               ? std::nullopt
               : std::make_optional(static_cast<char>(pos + 1));
}
