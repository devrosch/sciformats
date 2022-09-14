#ifndef LIBJDX_PEAKASSIGNMENTS_HPP
#define LIBJDX_PEAKASSIGNMENTS_HPP

#include "jdx/DataLdr.hpp"
#include "jdx/PeakAssignment.hpp"
#include "jdx/TabularData.hpp"
#include "jdx/TextReader.hpp"

#include <array>
#include <optional>
#include <string>
#include <vector>

namespace sciformats::jdx
{
/**
 * @brief A JCAMP-DX PEAK ASSIGNMENTS record.
 */
class PeakAssignments : public TabularData
{
public:
    /**
     * @brief PeakAssignments Constructs PeakAssignments.
     * @param label The label of the first line of the record, i.e.
     * "PEAKASSIGNMENTS".
     * @param variableList The value of the first line of the record
     * representing the structure of the data, e.g., "(XYA)".
     * @param reader Text reader with JCAMP-DX data. The reader position is
     * assumed to be at the start of the second line (the line following the
     * line containing the label) of the record. The reader is expected to exist
     * for the lifetime of this object.
     * @param nextLine The first line of the LDR, i.e., the one containing the
     * label. Will contain the line following the record or nullopt if the end
     * of the reader has been reached.
     */
    PeakAssignments(const std::string& label, std::string variableList,
        TextReader& reader, std::optional<std::string>& nextLine);

    /**
     * @brief Provides the parsed peak assignments.
     * @return The list of peak assignments.
     */
    std::vector<PeakAssignment> getData();

private:
    static constexpr const char* s_peakAssignentsLabel = "PEAKASSIGNMENTS";
    static constexpr std::array<const char*, 4> s_peakAssignentsVariableLists
        = {"(XYA)", "(XYWA)", "(XYMA)", "(XYMWA)"};
};
} // namespace sciformats::jdx

#endif // LIBJDX_PEAKASSIGNMENTS_HPP
