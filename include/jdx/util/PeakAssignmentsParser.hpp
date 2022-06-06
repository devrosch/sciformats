#ifndef LIBJDX_PEAKASSIGNMENTSPARSER_HPP
#define LIBJDX_PEAKASSIGNMENTSPARSER_HPP

#include "jdx/PeakAssignment.hpp"

#include <variant>

namespace sciformats::jdx::util
{
/**
 * @brief A parser for PEAK ASSIGNMENTS.
 */
class PeakAssignmentsParser
{
public:
    explicit PeakAssignmentsParser(std::istream& iStream, size_t numVariables);
    /**
     * @brief Next assignment item.
     * @note Assumes that a peak assignment tuple always starts on a new line,
     * but may span multiple lines.
     * @return Either a textual description of peak width function or next peak
     * assignment.
     */
    std::variant<std::string, PeakAssignment> next();
    bool hasNext();

private:
    std::istream& m_istream;
    size_t m_numVariables;
    bool m_isPastInitialComment;

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
