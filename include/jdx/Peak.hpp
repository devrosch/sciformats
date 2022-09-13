#ifndef LIBJDX_PEAK_HPP
#define LIBJDX_PEAK_HPP

#include <optional>

namespace sciformats::jdx
{
/**
 * @brief A JCAMP-DX peak, i.e. one item in a PEAK TABLE.
 */
struct Peak
{
public:
    /**
     * @brief x Peak position.
     */
    double x{};

    /**
     * @brief y Intensity.
     */
    double y{};

    /**
     * @brief Multiplicity.
     * @remark S, D, Т, Q for singlets, douЬlets, triplets, or quadruplets, М
     * for multiple, and U for unassigned. Used only for NMR.
     */
    std::optional<std::string> m;

    /**
     * @brief w Width.
     */
    std::optional<double> w;
};
} // namespace sciformats::jdx

#endif // LIBJDX_PEAK_HPP
