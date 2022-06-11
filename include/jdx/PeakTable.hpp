#ifndef LIBJDX_PEAKTABLE_HPP
#define LIBJDX_PEAKTABLE_HPP

#include "jdx/DataLdr.hpp"
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
     * @brief Definition of peak width (and other kernel) functions
     *
     * Comment $$ in line(s) following LDR start may contain peak width and
     * other peak kernel functions
     *
     * @return Textual description of kernel functions.
     */
    std::optional<std::string> getWidthFunction();

private:
    static constexpr const char* s_peakTableLabel = "PEAKTABLE";
    static constexpr std::array<const char*, 2> s_peakTableVariableLists
        = {"(XY..XY)", "(XYW..XYW)"};

    size_t getNumVariables();
};
} // namespace sciformats::jdx

#endif // LIBJDX_PEAKTABLE_HPP
