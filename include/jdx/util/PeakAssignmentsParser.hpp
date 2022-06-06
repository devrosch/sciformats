#ifndef LIBJDX_PEAKASSIGNMENTSPARSER_HPP
#define LIBJDX_PEAKASSIGNMENTSPARSER_HPP

#include "jdx/PeakAssignment.hpp"

#include <variant>

namespace sciformats::jdx::util
{
class PeakAssignmentsParser
{
public:
    explicit PeakAssignmentsParser(std::istream& iStream, uint numVariables);
    /**
     * @brief Next assignment item.
     * @return Either a textual description of peak width function or next peak
     * assignment.
     */
    std::variant<std::string, PeakAssignment> next();
    bool hasNext();

private:
    std::istream& m_istream;
    uint m_numVariables;
    bool m_isPastWidthFunction;

    // width function
    std::optional<std::string> parseWidthFunction();
    static bool isPureInlineComment(const std::string& line);
    static void appendToDescription(
        std::string comment, std::string& description);
    // assignment string
    std::optional<std::string> readNextAssignmentString();
    static bool isPeakAssignmentStart(const std::string& stringValue);
    static bool isPeakAssignmentEnd(const std::string& stringValue);
    // assignment
    [[nodiscard]] sciformats::jdx::PeakAssignment createPeakAssignment(
        const std::string& stringValue) const;
    static std::optional<std::string> parseNextPeakAssignmentToken(
        const std::string& stringValue, size_t& position);
    static std::string parsePeakAssignmentStringToken(
        const std::string& stringValue, size_t& position);
};
}

#endif /* LIBJDX_PEAKASSIGNMENTSPARSER_HPP */
