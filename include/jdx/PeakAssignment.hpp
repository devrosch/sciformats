#ifndef LIBJDX_PEAKASSIGNMENT_HPP
#define LIBJDX_PEAKASSIGNMENT_HPP

#include <optional>
#include <string>

namespace sciformats::jdx
{
/**
 * @brief A JCAMP-DX peak assignment, i.e. one item in PEAK ASSIGNMENTS.
 */
struct PeakAssignment
{
public:
    /**
     * @brief Peak position.
     */
    double x;
    /**
     * @brief Intensity.
     */
    std::optional<double> y; // standard is ambiguous whether this is optional
    /**
     * @brief Width.
     */
    std::optional<double> w;
    /**
     * @brief The peak assignment string.
     */
    std::string a;

    PeakAssignment(const std::string& stringValue, size_t numVariables);
    static bool isPeakAssignmentStart(const std::string& stringValue);
    static bool isPeakAssignmentEnd(const std::string& stringValue);
private:
    /**
     * @brief parseNextToken Parses the next element of the PEAK ASSIGNMENT.
     * @param stringValue The string representing the PEAK ASSIGNMENT.
     * @param position The position in the PEAK ASSIGNMENT the last parsing operation ended with. It will be updated with the position after the next token separator (comma or closing parenthesis).
     * @return The next token, if any.
     */
    static std::optional<std::string> parseNextToken(const std::string& stringValue, size_t& position);
    static std::string parseStringToken(const std::string& stringValue, size_t& position);
};
} // namespace sciformats::jdx

#endif // LIBJDX_PEAKASSIGNMENT_HPP
