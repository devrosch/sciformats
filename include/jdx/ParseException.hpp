#ifndef LIBJDX_PARSEEXCEPTION_HPP
#define LIBJDX_PARSEEXCEPTION_HPP

#include <stdexcept>

#include <string>

namespace sciformats::jdx
{
/**
 * @brief Indicates an exception during parsing of JCAMP-DX data.
 */
class ParseException : public std::invalid_argument
{
public:
    explicit ParseException(const std::string& what);
};
} // namespace sciformats::jdx

#endif // LIBJDX_PARSEEXCEPTION_HPP
