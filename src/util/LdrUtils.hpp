#ifndef LIBJDX_LDRUTILS_HPP
#define LIBJDX_LDRUTILS_HPP

#include "jdx/StringLdr.hpp"

#include <istream>
#include <optional>
#include <vector>

/**
 * @brief Helper functions for parsing JCAMP-DX labeled data records (LDRs).
 */
namespace sciformats::jdx::util
{
std::string readLine(std::istream& istream);
bool isLdrStart(const std::string& line);
void trim(std::string& s);
void trimLeft(std::string& s);
void trimRight(std::string& s);
std::string normalizeLdrStart(const std::string& ldr);
std::string normalizeLdrLabel(const std::string& label);
std::pair<std::string, std::string> parseLdrStart(const std::string& ldrStart);
std::pair<std::string, std::optional<std::string>> stripLineComment(
    const std::string& line);
std::optional<const StringLdr> findLdr(
    const std::vector<StringLdr>& ldrs, const std::string& label);
std::optional<std::string> findLdrValue(
    const std::vector<StringLdr>& ldrs, const std::string& label);
bool isSpace(char c);
} // namespace sciformats::jdx::utils

#endif // LIBJDX_LDRUTILS_HPP
