#ifndef LIBJDX_BLOCK_HPP
#define LIBJDX_BLOCK_HPP

#include "jdx/Ldr.hpp"
#include "jdx/PeakTable.hpp"
#include "jdx/RaData.hpp"
#include "jdx/XyData.hpp"
#include "jdx/XyPoints.hpp"

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
 * @brief A JCAMP-DX block. Can be a link or data block.
 */
class Block
{
public:
    /**
     * @brief Constructs a Block from istream.
     * @param iStream Input stream with JCAMP-DX data. The stream position
     * is assumed to be at the start of the first line of the block (containing
     * the TITLE LDR). The inputStream is expected to exist for the lifetime of
     * this object.
     */
    explicit Block(std::istream& iStream);
    /**
     * @brief Provides the labeled data records (LDRs) of the Block.
     * This does \em not include the following LDRs:
     * - comments ("##=")
     * - data ("##XYDATA=", "##XYPOINTS=", "##PEAK TABLE=", "##PEAK
     * ASSIGNMENTS=", "##RADATA=", "##NTUPLES=")
     *
     * Use the specialized member functions to retrieve the respective data.
     *
     * @return The LDRs in this block. The key is the label without "##" and "="
     * and the value is the content (without initial blank character if any).
     * E.g. the LDR "##TITLE= abc" has label "TITLE" and content "abc".
     */
    [[nodiscard]] const std::vector<Ldr>& getLdrs() const;
    /**
     * @brief Provides a labeled data record (LDR) from the block. The same
     * exclusions as for getLdrs() apply.
     * @param label The label of the LDR. Search will use normalized form of
     * label, e.g. "Title" and "TI TLE" will both find the "TITLE" LDR.
     * @return The LDR for the given label if it exists in the block,
     * std::nullopt otherwise.
     */
    [[nodiscard]] std::optional<const Ldr> getLdr(
        const std::string& label) const;
    /**
     * @brief Provides the nested Blocks of the Block.
     * @return Blocks that are nested in this (LINK) block.
     */
    [[nodiscard]] const std::vector<Block>& getBlocks() const;
    /**
     * @brief Provides the labeled data records (LDRs) of the
     * Block that are comments (i.e. "##= <comment>").
     * @return The comment contents. The content of a comment is the text
     * following the "=" without initial blank character if any. E.g. the
     * comment "##= abc" has content "abc".
     */
    [[nodiscard]] const std::vector<std::string>& getLdrComments() const;
    /**
     * @brief Provides the XYDATA record if available.
     * @return XYDATA record.
     */
    [[nodiscard]] const std::optional<XyData>& getXyData() const;
    /**
     * @brief Provides the RADATA record if available.
     * @return RADATA record.
     */
    [[nodiscard]] const std::optional<RaData>& getRaData() const;
    /**
     * @brief Provides the XYPOINTS record if available.
     * @return XYPOINTS record.
     */
    [[nodiscard]] const std::optional<XyPoints>& getXyPoints() const;
    /**
     * @brief Provides the PEAK TABLE record if available.
     * @return PEAK TABLE record.
     */
    [[nodiscard]] const std::optional<PeakTable>& getPeakTable() const;

private:
    std::istream& m_istream;
    std::vector<Ldr> m_ldrs;
    std::vector<std::string> m_ldrComments;
    std::vector<Block> m_blocks;
    std::optional<XyData> m_xyData;
    std::optional<RaData> m_raData;
    std::optional<XyPoints> m_xyPoints;
    std::optional<PeakTable> m_peakTable;

    /**
     * @brief Constructs a Block from first line value and istream.
     * @param title The value of the first line of the block, i.e. the content
     * of the line following the "##TITLE=" label.
     * @param iStream Input stream with JCAMP-DX data. The stream position
     * is assumed to be at the start of the second line (the line following the
     * TITLE line) of the block. The inputStream is expected to exist for the
     * lifetime of this object.
     */
    Block(const std::string& title, std::istream& iStream);
    void parseInput(const std::string& title);
};
} // namespace sciformats::jdx

#endif // LIBJDX_BLOCK_HPP
