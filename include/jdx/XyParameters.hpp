#ifndef LIBJDX_XYPARAMETERS_HPP
#define LIBJDX_XYPARAMETERS_HPP

#include <optional>

namespace sciformats::jdx
{
/**
 * @brief JCAMP-DX spectral parameters describing an XYDATA record.
 */
struct XyParameters
{
    // not required for parsing but for displaying
    std::string xUnits; //!< Abscissa units.
    // not required for parsing but for displaying
    std::string yUnits; //!< Ordinate units.
    double firstX; //!< The first X value.
    double lastX; //!< The last X value.
    std::optional<double> maxX;
    std::optional<double> minX;
    std::optional<double> maxY;
    std::optional<double> minY;
    /**
     * @brief The factor by which to multiply raw x values to arrive at the
     * actual value.
     */
    double xFactor;
    /**
     * @brief The factor by which to multiply raw y values to arrive at the
     * actual value.
     */
    double yFactor;
    /**
     * @brief The number of xy pairs in this record.
     */
    uint64_t nPoints;
    /**
     * @brief The first actual Y value (after scaling).
     */
    std::optional<double> firstY;
    /**
     * @brief The resolution of the data.
     */
    std::optional<double> resolution;
    /**
     * @brief The x distance between adjacent data points (if constant).
     */
    std::optional<double> deltaX;
};
} // namespace sciformats::jdx

#endif // LIBJDX_XYPARAMETERS_HPP
