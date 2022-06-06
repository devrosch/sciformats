#ifndef LIBJDX_PEAKTABLE_HPP
#define LIBJDX_PEAKTABLE_HPP

#include "jdx/Peak.hpp"
#include "jdx/TabularData.hpp"

#include <functional>
#include <istream>
#include <string>
#include <vector>

namespace sciformats::jdx
{
/**
 * @brief A JCAMP-DX PEAK TABLE record.
 */
class PeakTable : public TabularData
{
public:
    explicit PeakTable(std::istream& istream);
    PeakTable(
        std::string label, std::string variableList, std::istream& istream);
    /**
     * @brief Provides the parsed peak data.
     * @return The list of peaks.
     */
    std::vector<Peak> getData();
    /**
     * @brief Definition of peak width and other peak kernel functions.
     * @return Textual description of kernel functions.
     */
    std::optional<std::string> getKernel();

private:
    static constexpr const char* s_peakTableLabel = "PEAKTABLE";
    static constexpr const char* s_peakTableXyVariableList = "(XY..XY)";
    static constexpr const char* s_peakTableXywVariableList = "(XYW..XYW)";
    //    static const inline std::vector<std::string> expVarLists = {
    //    "(XY..XY)", "(XYW..XYW)" };
};
} // namespace sciformats::jdx

#endif // LIBJDX_PEAKTABLE_HPP
