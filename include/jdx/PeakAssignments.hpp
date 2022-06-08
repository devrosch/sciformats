#ifndef LIBJDX_PEAKASSIGNMENTS_HPP
#define LIBJDX_PEAKASSIGNMENTS_HPP

#include "jdx/DataLdr.hpp"
#include "jdx/PeakAssignment.hpp"

#include <functional>
#include <vector>

namespace sciformats::jdx
{
/**
 * @brief A JCAMP-DX PEAK ASSIGNMENTS record.
 */
class PeakAssignments : public DataLdr
{
public:
    explicit PeakAssignments(std::istream& iStream);
    PeakAssignments(
        std::string label, std::string variableList, std::istream& iStream);
    /**
     * @brief Provides the parsed peak assignments.
     * @return The list of peak assignments.
     */
    std::vector<PeakAssignment> getData();
    /**
     * @brief Definition of peak width function.
     * @return Textual description of peak width function.
     */
    std::optional<std::string> getWidthFunction();

private:
    static constexpr const char* s_peakAssignentsLabel = "PEAKASSIGNMENTS";
    static constexpr const char* s_peakAssignentsXyaVariableList = "(XYA)";
    static constexpr const char* s_peakAssignentsXywaVariableList = "(XYWA)";
};
} // namespace sciformats::jdx

#endif // LIBJDX_PEAKASSIGNMENTS_HPP
