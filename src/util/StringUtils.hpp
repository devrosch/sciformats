#ifndef LIBJDX_STRINGUTILS_HPP
#define LIBJDX_STRINGUTILS_HPP

#include <string>
#include <vector>

/**
 * @brief Helper functions for processing strings.
 */
namespace sciformats::jdx::util
{
void trim(std::string& s);
void trimLeft(std::string& s);
void trimRight(std::string& s);
bool isSpace(char c);
void toLower(std::string& s);
std::vector<std::string> split(const std::string& input,
    const std::string& delimiterRegEx, bool trimSegments = false);
} // namespace sciformats::jdx::utils

#endif // LIBJDX_STRINGUTILS_HPP
