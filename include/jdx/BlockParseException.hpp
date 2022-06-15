#ifndef LIBJDX_BLOCKPARSEEXCEPTION_HPP
#define LIBJDX_BLOCKPARSEEXCEPTION_HPP

#include "jdx/ParseException.hpp"

#include <string>

namespace sciformats::jdx
{
/**
 * @brief Indicates an exception during parsing of JCAMP-DX data.
 */
class BlockParseException : public ParseException
{
public:
    explicit BlockParseException(const std::string& what);
    BlockParseException(const std::string& issueMsg, const std::string& label,
        const std::string& blockTitle);
};
} // namespace sciformats::jdx

#endif // LIBJDX_BLOCKPARSEEXCEPTION_HPP
