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
     * @return The next peak assignment, nullopt if there is none.
     * @throws ParseException If next peak assignment is malformed.
     */
    std::optional<PeakAssignment> next();

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
