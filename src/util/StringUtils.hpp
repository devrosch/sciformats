#ifndef LIBJDX_STRINGUTILS_HPP
#define LIBJDX_STRINGUTILS_HPP

#include <istream>

/**
 * @brief Helper functions for processing strings.
 */
namespace sciformats::jdx::util
{
std::string readLine(std::istream& istream);
void trim(std::string& s);
void trimLeft(std::string& s);
void trimRight(std::string& s);
bool isSpace(char c);
void toLower(std::string& s);
} // namespace sciformats::jdx::utils

#endif // LIBJDX_STRINGUTILS_HPP
