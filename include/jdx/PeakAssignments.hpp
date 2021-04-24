#ifndef LIBJDX_PEAKASSIGNMENTS_HPP
#define LIBJDX_PEAKASSIGNMENTS_HPP

#include "jdx/PeakAssignment.hpp"

#include <vector>

namespace sciformats::jdx
{
/**
 * @brief A JCAMP-DX PEAK TABLE record.
 * LDRs.
 */
class PeakAssignments
{
public:
    explicit PeakAssignments(std::istream& iStream);
    PeakAssignments(const std::string& label, const std::string& variableList,
        std::istream& iStream);
    /**
     * @brief Provides the parsed peak assignments.
     * @return The list of peak assignments.
     */
    std::vector<PeakAssignment> getData();

private:
    static constexpr const char* s_peakTableLabel = "PEAKASSIGNMENTS";
    static constexpr const char* s_peakTableXyVariableList = "(XYA)";
    static constexpr const char* s_peakTableXywVariableList = "(XYWA)";
};
} // namespace sciformats::jdx

#endif // LIBJDX_PEAKASSIGNMENTS_HPP
