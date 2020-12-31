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
    std::vector<double> static readXppYYLine(
        std::string& line, const std::optional<double>& yValueCheck);
    std::vector<double> static readValues(std::string& encodedValues);
private:
    static bool isTokenStart(std::string encodedValues, size_t index);
    static bool isTokenDelimiter(std::string encodedValues, size_t index);
    static std::optional<char> getAsciiDigitValue(const char c);
    static std::optional<char> getSqzDigitValue(const char c);
    static std::optional<char> getDifDigitValue(const char c);
    static std::optional<char> getDupDigitValue(const char c);
};
} // namespace sciformats::jdx

#endif // LIBJDX_JDXDATAPARSER_HPP
