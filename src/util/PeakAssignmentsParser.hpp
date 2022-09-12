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
     * @brief Next peak assignment.
     * @note Assumes that a peak assignment tuple always starts on a new line,
     * but may span multiple lines.
     * @return The next peak assignment.
     * @throws ParseException If there is no next peak assignment.
     */
    PeakAssignment next();

    /**
     * @brief Whether there is another peak assignment.
     * @return True if there is another peak assignment, false otherwise.
     */
    bool hasNext();

private:
    TextReader& m_reader;
    size_t m_numVariables;

    // tuple
    std::optional<std::string> nextTuple();
    static bool isTupleStart(const std::string& stringValue);
    static bool isTupleEnd(const std::string& stringValue);
    // assignment
    [[nodiscard]] sciformats::jdx::PeakAssignment createPeakAssignment(
        const std::string& tuple) const;
};
}

#endif /* LIBJDX_PEAKASSIGNMENTSPARSER_HPP */
