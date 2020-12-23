#ifndef LIBJDX_JDXPARSER_HPP
#define LIBJDX_JDXPARSER_HPP

#include "jdx/JdxBlock.hpp"

#include <cstdint>
#include <fstream>
#include <istream>
#include <optional>
#include <sstream>
#include <vector>

namespace sciformats::jdx
{
/**
 * @brief The JdxParser class provides mechanisms for reading JCAMP-DX data.
 */
class JdxParser
{
public:
    /**
     * @brief sciformats::jdx::JdxParser::canParse Shallow check if the data can be parsed, e.g. by checking the file extension or magic bytes.
     * @param filePath Path to the file.
     * @param inputStream Input stream with binary data.
     */
    bool static canParse(const std::string& filePath, std::istream& inputStream);

    /**
     * @brief sciformats::jdx::JdxParser::parse .
     * @param inputStream Input stream with binary data.
     * @param activateExceptions Activate exceptions for input_stream.
     */
    JdxBlock static parse(
        std::istream& inputStream, bool activateExceptions = true);

private:
    std::vector<std::string> m_acceptedExtensions{"jdx", "dx", "jcm"};
};
} // namespace sciformats::jdx

#endif // LIBJDX_JDXPARSER_HPP
