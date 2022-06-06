#ifndef LIBJDX_PEAKASSIGNMENTS_HPP
#define LIBJDX_PEAKASSIGNMENTS_HPP

#include "jdx/PeakAssignment.hpp"

#include <functional>
#include <vector>

namespace sciformats::jdx
{
/**
 * @brief A JCAMP-DX PEAK ASSIGNMENTS record.
 */
class PeakAssignments
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

    std::istream& m_istream;
    std::streampos m_streamDataPos;
    std::string m_label;
    std::string m_variableList;

    static void skipToNextLdr(std::istream& iStream);
    static void validateInput(const std::string& label,
        const std::string& variableList, const std::string& expectedLabel,
        const std::vector<std::string>& expectedVariableLists);
    static std::pair<std::string, std::string> readFirstLine(
        std::istream& istream);
    template<typename R>
    R callAndResetStreamPos(const std::function<R()>& func);
};
} // namespace sciformats::jdx

#endif // LIBJDX_PEAKASSIGNMENTS_HPP
