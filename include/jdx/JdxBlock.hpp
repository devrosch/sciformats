#ifndef LIBJDX_JDXBLOCK_HPP
#define LIBJDX_JDXBLOCK_HPP

#include <cstdint>
#include <fstream>
#include <istream>
#include <optional>
#include <sstream>
#include <vector>

namespace sciformats::jdx
{
/**
 * @brief The JdxDataSet class represents JCAMP-DX data.
 */
class JdxBlock
{
public:
    /**
     * @brief sciformats::io::JdxBlock::JdxBlock Constructs from file.
     * @param filePath Path to the file.
     * @param endian Default endianness of data.
     */
    explicit JdxBlock(const std::string& filePath);

    /**
     * @brief sciformats::io::JdxBlock::JdxBlock Constructs from istream.
     * @param inputStream Input stream with binary data.
     */
    explicit JdxBlock(std::istream& inputStream);

private:
    std::optional<std::ifstream> m_ifstream;
    std::istream& m_istream;
};
} // namespace sciformats::jdx

#endif // LIBJDX_JDXBLOCK_HPP
