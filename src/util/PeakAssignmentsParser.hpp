#ifndef LIBJDX_PEAKASSIGNMENTSPARSER_HPP
#define LIBJDX_PEAKASSIGNMENTSPARSER_HPP

#include "jdx/PeakAssignment.hpp"
#include "jdx/TextReader.hpp"

namespace sciformats::jdx::util
{
/**
 * @brief A parser for PEAK ASSIGNMENTS.
 */
class PeakAssignmentsParser
{
public:
    explicit PeakAssignmentsParser(TextReader& reader, size_t numVariables);

    /**
     * @brief Next assignment item.
     * @note Assumes that a peak assignment tuple always starts on a new line,
     * but may span multiple lines.
     * @return The next peak assignment.
     */
    PeakAssignment next();

    bool hasNext();

private:
    TextReader& m_reader;
    size_t m_numVariables;

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
