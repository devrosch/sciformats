#ifndef LIBJDX_JDXDATAPARSER_HPP
#define LIBJDX_JDXDATAPARSER_HPP

#include <optional>
#include <string>
#include <vector>

namespace sciformats::jdx
{
/**
 * @brief A JCAMP-DX labelled data record (LDR).
 */
class JdxDataParser
{
public:
    std::vector<double> static readXppYYData(std::istream& istream);
    std::pair<std::vector<double>, bool> static readXppYYLine(
        std::string& line, const std::optional<double>& yValueCheck);
    std::pair<std::vector<double>, bool> static readValues(
        std::string& encodedValues);

private:
    static bool isTokenStart(std::string encodedValues, size_t index);
    static bool isTokenDelimiter(std::string encodedValues, size_t index);
    static std::optional<char> getAsciiDigitValue(char c);
    static std::optional<char> getSqzDigitValue(char c);
    static std::optional<char> getDifDigitValue(char c);
    static std::optional<char> getDupDigitValue(char c);
};
} // namespace sciformats::jdx

#endif // LIBJDX_JDXDATAPARSER_HPP
