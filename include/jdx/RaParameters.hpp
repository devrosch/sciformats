#ifndef LIBJDX_RAPARAMETERS_HPP
#define LIBJDX_RAPARAMETERS_HPP

#include <optional>

namespace sciformats::jdx
{
/**
 * @brief JCAMP-DX spectral parameters describing an RADATA record.
 *
 * It is poorly defined in the standard which elements are used for RADATA and
 * which are required and which are optional. The choice here is governed by
 * technical needs for parsing/displaying.
 */
struct RaParameters
{
    std::string rUnits; // not required for parsing but for displaying
    std::string aUnits; // not required for parsing but for displaying
    double firstR;
    double lastR;
    std::optional<double> maxA; // required, according to standard
    std::optional<double> minA; // required, according to standard
    double rFactor;
    double aFactor;
    uint64_t nPoints;
    double firstA;
    std::optional<double> resolution;
    std::optional<double> deltaR;
    std::optional<double> zdp;
    std::optional<std::string> alias; // standard says type is AFFN, but gives
                                      // "1/1" and "1/2" as examples
    // in addition, XUNITS, YUNITS, FIRSTX, LASTX, DELTAX are given in examples
    // in the standard with not quite clear meaning
};
} // namespace sciformats::jdx

#endif // LIBJDX_RAPARAMETERS_HPP
