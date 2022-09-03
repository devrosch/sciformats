#ifndef LIBJDX_NTUPLESATTRIBUTES_HPP
#define LIBJDX_NTUPLESATTRIBUTES_HPP

#include "jdx/StringLdr.hpp"

#include <optional>
#include <vector>

namespace sciformats::jdx
{
/**
 * @brief A collection of attributes describing NTUPLES data.
 */
struct NTuplesAttributes
{
public:
    /**
     * @brief ##VAR_NAME.
     */
    std::string varName;
    /**
     * @brief ##SYMBOL.
     */
    std::string symbol;
    /**
     * @brief ##VAR_TYPE.
     */
    std::optional<std::string> varType;
    /**
     * @brief ##VAR_FORM.
     */
    std::optional<std::string> varForm;
    /**
     * @brief ##VAR_DIM.
     */
    std::optional<uint64_t> varDim; // optional, may be blank for mass spectra
    /**
     * @brief ##UNITS.
     */
    std::optional<std::string> units;
    /**
     * @brief ##FIRST.
     */
    std::optional<double> first;
    /**
     * @brief ##LAST.
     */
    std::optional<double> last;
    /**
     * @brief ##MIN.
     */
    std::optional<double> min;
    /**
     * @brief ##MAX.
     */
    std::optional<double> max;
    /**
     * @brief ##FACTOR.
     */
    std::optional<double> factor;
    /**
     * @brief Additional application specific LDRs.
     */
    std::vector<StringLdr> applicationAttributes;
};
} // namespace sciformats::jdx

#endif // LIBJDX_NTUPLESATTRIBUTES_HPP
