#ifndef LIBJDX_PEAKUTILS_HPP
#define LIBJDX_PEAKUTILS_HPP

#include "jdx/PeakAssignment.hpp"

namespace sciformats::jdx::peakutils
{
sciformats::jdx::PeakAssignment createPeakAssignment(
    const std::string& stringValue, size_t numVariables);
bool isPeakAssignmentStart(const std::string& stringValue);
bool isPeakAssignmentEnd(const std::string& stringValue);
/**
 * @brief parseNextToken Parses the next element of the PEAK ASSIGNMENT.
 * @param stringValue The string representing the PEAK ASSIGNMENT.
 * @param position The position in the PEAK ASSIGNMENT the last parsing
 * operation ended with. It will be updated with the position after the next
 * token separator (comma or closing parenthesis).
 * @return The next token, if any.
 */
std::optional<std::string> parseNextToken(const std::string& stringValue,
    size_t& position);
std::string parseStringToken(const std::string& stringValue, size_t& position);
}

#endif /* LIBJDX_PEAKUTILS_HPP */
