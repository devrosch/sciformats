#ifndef LIBJDX_PEAKASSIGNMENT_HPP
#define LIBJDX_PEAKASSIGNMENT_HPP

#include <optional>
#include <string>

namespace sciformats::jdx
{
/**
 * @brief A JCAMP-DX peak assignment, i.e., one item in PEAK ASSIGNMENTS.
 */
struct PeakAssignment
{
    /**
     * @brief Peak position.
     */
    double x;

    /**
     * @brief Intensity.
     */
    std::optional<double> y; // standard is ambiguous whether this is optional

    /**
     * @brief Multiplicity.
     * @remark S, D, Т, Q for singlets, douЬlets, triplets, or quadruplets, М
     * for multiple, and U for unassigned. Used only for NMR.
     */
    std::optional<std::string> m;

    /**
     * @brief Width.
     */
    std::optional<double> w;

    /**
     * @brief The peak assignment string.
     */
    std::string a;
};
} // namespace sciformats::jdx

#endif // LIBJDX_PEAKASSIGNMENT_HPP
