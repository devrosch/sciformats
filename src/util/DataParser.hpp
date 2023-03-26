#ifndef LIBJDX_DATAPARSER_HPP
#define LIBJDX_DATAPARSER_HPP

#include "jdx/TextReader.hpp"

#include <optional>
#include <string>
#include <vector>

namespace sciformats::jdx::util
{
/**
 * @brief Helper functions for parsing JCAMP-DX data.
 */
class DataParser
{
public:
    static std::vector<double> readXppYYData(TextReader& reader);
    static std::vector<std::pair<double, double>> readXyXyData(
        TextReader& reader);
    static std::pair<std::vector<double>, bool> readValues(
        std::string& encodedValues);

private:
    enum class TokenType
    {
        Affn,
        Sqz,
        Dif,
        Dup,
        Missing,
    };
    static std::pair<std::vector<double>, bool> readXppYYLine(
        std::string& line, const std::optional<double>& yValueCheck);
    static std::optional<std::string> nextToken(
        const std::string& line, size_t& pos);
    static TokenType toAffn(std::string& token);
    static bool isTokenStart(std::string encodedValues, size_t index);
    static bool isTokenDelimiter(std::string encodedValues, size_t index);
    static std::optional<char> getAsciiDigitValue(char c);
    static std::optional<char> getSqzDigitValue(char c);
    static std::optional<char> getDifDigitValue(char c);
    static std::optional<char> getDupDigitValue(char c);
};
} // namespace sciformats::jdx

#endif // LIBJDX_DATAPARSER_HPP
