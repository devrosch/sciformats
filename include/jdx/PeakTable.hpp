#ifndef LIBJDX_PEAKTABLE_HPP
#define LIBJDX_PEAKTABLE_HPP

#include "jdx/DataLdr.hpp"
#include "jdx/Peak.hpp"
#include "jdx/TabularData.hpp"
#include "jdx/TextReader.hpp"

#include <array>
#include <optional>
#include <string>
#include <vector>

namespace sciformats::jdx
{
/**
 * @brief A JCAMP-DX PEAK TABLE record.
 */
class PeakTable : public TabularData
{
public:
    /**
     * @brief PeakTable Constructs PeakTable.
     * @param label The label of the first line of the record, i.e. "PEAKTABLE".
     * @param variableList The value of the first line of the record
     * representing the structure of the data, e.g., "(XY..XY)".
     * @param reader Text reader with JCAMP-DX data. The reader position is
     * assumed to be at the start of the second line (the line following the
     * line containing the label) of the record. The reader is expected to exist
     * for the lifetime of this object.
     * @param nextLine The first line of the LDR, i.e., the one containing the
     * label. Will contain the line following the record or nullopt if the end
     * of the reader has been reached.
     */
    PeakTable(const std::string& label, std::string variableList,
        TextReader& reader, std::optional<std::string>& nextLine);

    /**
     * @brief Provides the parsed peak data.
     * @return The list of peaks.
     */
    [[nodiscard]] std::vector<Peak> getData() const;

private:
    static constexpr const char* s_peakTableLabel = "PEAKTABLE";
    static constexpr std::array<const char*, 3> s_peakTableVariableLists
        = {"(XY..XY)", "(XYW..XYW)", "(XYM..XYM)"};
};
} // namespace sciformats::jdx

#endif // LIBJDX_PEAKTABLE_HPP
