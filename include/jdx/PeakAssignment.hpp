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
     * @brief x Peak position.
     */
    double x;
    /**
     * @brief y Intensity.
     */
    std::optional<double> y; // standard is ambiguous whether this is optional
    /**
     * @brief w Width.
     */
    std::optional<double> w;
    /**
     * @brief a The peak assignment string.
     */
    std::string a;
};
} // namespace sciformats::jdx

#endif // LIBJDX_PEAKASSIGNMENT_HPP
