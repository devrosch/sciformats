#ifndef LIBJDX_JDXBLOCK_HPP
#define LIBJDX_JDXBLOCK_HPP

#include <cstdint>
#include <fstream>
#include <istream>
#include <map>
#include <optional>
#include <sstream>
#include <vector>

namespace sciformats::jdx
{
/**
 * @brief A JCAMP-DX block.
 */
class JdxBlock
{
public:
    /**
     * @brief JdxBlock Constructs from istream.
     * @param inputStream Input stream with JCAMP-DX data. The stream position
     * is assumed to be at the start of the first line of the block (containing
     * the TITLE LDR). The inputStream is expected to exist for the lifetime of
     * this object.
     */
    explicit JdxBlock(std::istream& inputStream);
    /**
     * @brief JdxBlock Constructs from first line value and istream.
     * @param title The value of the first line of the block, i.e. the content
     * of the line following the `##TITLE=` label.
     * @param inputStream Input stream with JCAMP-DX data. The stream position
     * is assumed to be at the start of the second line (the line following the
     * TITLE line) of the block. The inputStream is expected to exist for the
     * lifetime of this object.
     */
    JdxBlock(const std::string& title, std::istream& inputStream);
    [[nodiscard]] const std::map<std::string, std::string>& getLdrs() const;
    [[nodiscard]] const std::vector<JdxBlock>& getBlocks() const;
    [[nodiscard]] const std::vector<std::string>& getLdrComments() const;

private:
    std::istream& m_istream;
    std::map<std::string, std::string> m_ldrs;
    std::vector<std::string> m_ldrComments;
    std::vector<JdxBlock> m_blocks;

    void parseInput();
};
} // namespace sciformats::jdx

#endif // LIBJDX_JDXBLOCK_HPP
