#ifndef LIBJDX_LDRUTILS_HPP
#define LIBJDX_LDRUTILS_HPP

#include "jdx/StringLdr.hpp"
#include "jdx/TextReader.hpp"

#include <optional>
#include <vector>

/**
 * @brief Helper functions for parsing JCAMP-DX labeled data records (LDRs).
 */
namespace sciformats::jdx::util
{
bool isLdrStart(const std::string& line);
std::string normalizeLdrStart(const std::string& ldr);
std::string normalizeLdrLabel(const std::string& label);
std::pair<std::string, std::string> parseLdrStart(const std::string& ldrStart);
std::pair<std::string, std::optional<std::string>> stripLineComment(
    const std::string& line);
std::optional<const StringLdr> findLdr(
    const std::vector<StringLdr>& ldrs, const std::string& label);
std::optional<std::string> findLdrValue(
    const std::vector<StringLdr>& ldrs, const std::string& label);
std::optional<std::string>& skipToNextLdr(TextReader& reader, std::optional<std::string>& nextLine, bool skipPureCommentsOnly = false);
} // namespace sciformats::jdx::utils

#endif // LIBJDX_LDRUTILS_HPP
