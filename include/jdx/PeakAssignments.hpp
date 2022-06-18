#ifndef LIBJDX_PEAKASSIGNMENTS_HPP
#define LIBJDX_PEAKASSIGNMENTS_HPP

#include "jdx/DataLdr.hpp"
#include "jdx/PeakAssignment.hpp"
#include "jdx/TabularData.hpp"
#include "jdx/TextReader.hpp"

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
    PeakAssignments(
        const std::string& label, std::string variableList, TextReader& reader);
    /**
     * @brief Provides the parsed peak assignments.
     * @return The list of peak assignments.
     */
    std::vector<PeakAssignment> getData();
    /**
     * @brief Definition of peak width (and other kernel) functions.
     *
     * Comment $$ in line(s) following LDR start may contain peak width and
     * other peak kernel functions
     *
     * @return Textual description of peak width function.
     */
    std::optional<std::string> getWidthFunction();

private:
    static constexpr const char* s_peakAssignentsLabel = "PEAKASSIGNMENTS";
    static constexpr std::array<const char*, 2> s_peakAssignentsVariableLists
        = {"(XYA)", "(XYWA)"};

    size_t getNumVariables();
};
} // namespace sciformats::jdx

#endif // LIBJDX_PEAKASSIGNMENTS_HPP
