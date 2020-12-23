#ifndef LIBJDX_JDXPARSER_HPP
#define LIBJDX_JDXPARSER_HPP

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
     * @brief sciformats::io::JdxParser::JdxParser Constructs from file.
     * @param filePath Path to the file.
     * @param endian Default endianness of data.
     */
    explicit JdxParser(const std::string& filePath);

    /**
     * @brief sciformats::io::JdxParser::JdxParser Constructs from
     * istream. Does not change exceptions flags.
     * @param inputStream Input stream with binary data.
     * @param activateExceptions Activate exceptions for input_stream.
     */
    explicit JdxParser(
        std::istream& inputStream, bool activateExceptions = true);

private:
    std::optional<std::ifstream> m_ifstream;
    std::istream& m_istream;
};
} // namespace sciformats::jdx

#endif // LIBJDX_JDXPARSER_HPP
