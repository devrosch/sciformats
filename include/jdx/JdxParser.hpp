#ifndef LIBJDX_JDXPARSER_HPP
#define LIBJDX_JDXPARSER_HPP

#include "jdx/Block.hpp"

#include <istream>
#include <string>

namespace sciformats::jdx
{
/**
 * @brief A parser for JCAMP-DX data.
 */
class JdxParser
{
public:
    /**
     * @brief Shallow check if the data can be parsed, e.g. by checking the file
     * extension or magic bytes.
     * @param filePath Path to the file.
     * @param iStream Binary input stream.
     * @return A tentative indication whether this parser can process the file.
     */
    bool static canParse(const std::string& filePath, std::istream& iStream);

    /**
     * @brief Parses the data.
     * @param iStream Binary input stream.
     * @param activateExceptions Activate exceptions for input_stream. Defaults
     * to "true".
     * @return A Block representing the data.
     */
    Block static parse(std::istream& iStream, bool activateExceptions = true);

private:
    static constexpr std::array<const char*, 3> s_acceptedExtensions
        = {".jdx", ".dx", ".jcm"};
};
} // namespace sciformats::jdx

#endif // LIBJDX_JDXPARSER_HPP
