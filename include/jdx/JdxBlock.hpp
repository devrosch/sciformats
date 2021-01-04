#ifndef LIBJDX_JDXBLOCK_HPP
#define LIBJDX_JDXBLOCK_HPP

#include "jdx/JdxLdr.hpp"
#include "jdx/XyData.hpp"
#include "jdx/RaData.hpp"
#include "jdx/RaParameters.hpp"
#include "jdx/XyParameters.hpp"

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
     * @brief Constructs a JdxBlock from istream.
     * @param iStream Input stream with JCAMP-DX data. The stream position
     * is assumed to be at the start of the first line of the block (containing
     * the TITLE LDR). The inputStream is expected to exist for the lifetime of
     * this object.
     */
    explicit JdxBlock(std::istream& iStream);
    /**
     * @brief Provides the labeled data records (LDRs) of the JdxBlock.
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
    [[nodiscard]] const std::vector<JdxLdr>& getLdrs() const;
    /**
     * @brief Provides a labeled data record (LDR) from the block. The same
     * exclusions as for getLdrs() apply.
     * @param label The label of the LDR.
     * @return The LDR for the given label if it exists in the block,
     * std::nullopt otherwise.
     */
    [[nodiscard]] std::optional<const JdxLdr> getLdr(
        const std::string& label) const;
    /**
     * @brief Provides the nested JdxBlocks of the JdxBlock.
     * @return JDXBlocks that are nested in this (LINK) block.
     */
    [[nodiscard]] const std::vector<JdxBlock>& getBlocks() const;
    /**
     * @brief Provides the labeled data records (LDRs) of the
     * JdxBlock that are comments (i.e. "##= <comment>").
     * @return The comment contents. The content of a comment is the text
     * following the "=" without initial blank character if any. E.g. the
     * comment "##= abc" has content "abc".
     */
    [[nodiscard]] const std::vector<std::string>& getLdrComments() const;
    /**
     * @brief Provides parameters specific to XYDATA.
     * @return The parameters.
     */
    [[nodiscard]] const std::optional<XyParameters>&
    getXyDataParameters() const;
    /**
     * @brief Provides the XYDATA record if available.
     * @return XYDATA record.
     */
    [[nodiscard]] const std::optional<XyData>& getXyData() const;
    /**
     * @brief Provides parameters specific to RADATA.
     * @return The parameters.
     */
    [[nodiscard]] const std::optional<RaParameters>&
    getRaDataParameters() const;
    /**
     * @brief Provides the RADATA record if available.
     * @return RADATA record.
     */
    [[nodiscard]] const std::optional<RaData>& getRaData() const;

private:
    std::istream& m_istream;
    std::vector<JdxLdr> m_ldrs;
    std::vector<std::string> m_ldrComments;
    std::vector<JdxBlock> m_blocks;
    std::optional<XyParameters> m_xyParameters;
    std::optional<RaParameters> m_raParameters;
    std::optional<XyData> m_xyData;
    std::optional<RaData> m_raData;

    /**
     * @brief Constructs a JdxBlock from first line value and istream.
     * @param title The value of the first line of the block, i.e. the content
     * of the line following the "##TITLE=" label.
     * @param iStream Input stream with JCAMP-DX data. The stream position
     * is assumed to be at the start of the second line (the line following the
     * TITLE line) of the block. The inputStream is expected to exist for the
     * lifetime of this object.
     */
    JdxBlock(const std::string& title, std::istream& iStream);
    void parseInput(const std::string& title);
    static XyParameters parseXyParameters(const std::vector<JdxLdr>& ldrs);
    static RaParameters parseRaParameters(const std::vector<JdxLdr>& ldrs);
    static std::optional<const JdxLdr> findLdr(
        const std::vector<JdxLdr>& ldrs, const std::string& label);
    static std::optional<std::string> findLdrValue(
        const std::vector<JdxLdr>& ldrs, const std::string& label);
};
} // namespace sciformats::jdx

#endif // LIBJDX_JDXBLOCK_HPP
