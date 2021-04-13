#ifndef LIBJDX_LDRPARSER_HPP
#define LIBJDX_LDRPARSER_HPP

#include "jdx/Ldr.hpp"

#include <istream>
#include <optional>
#include <vector>

namespace sciformats::jdx
{
/**
 * @brief Helper functions for parsing JCAMP-DX labeled data records (LDRs).
 */
class LdrParser
{
public:
    std::string static readLine(std::istream& istream);
    bool static isLdrStart(const std::string& line);
    void static trim(std::string& s);
    void static trimLeft(std::string& s);
    void static trimRight(std::string& s);
    std::string static normalizeLdrStart(const std::string& ldr);
    std::string static normalizeLdrLabel(const std::string& label);
    std::pair<std::string, std::string> static parseLdrStart(
        const std::string& ldrStart);
    std::pair<std::string, std::optional<std::string>> static stripLineComment(
        const std::string& line);
    static std::optional<const Ldr> findLdr(
        const std::vector<Ldr>& ldrs, const std::string& label);
    static std::optional<std::string> findLdrValue(
        const std::vector<Ldr>& ldrs, const std::string& label);
};
} // namespace sciformats::jdx

#endif // LIBJDX_LDRPARSER_HPP
