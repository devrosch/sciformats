#ifndef LIBJDX_PEAKTABLE_HPP
#define LIBJDX_PEAKTABLE_HPP

#include "jdx/DataLdr.hpp"
#include "jdx/Peak.hpp"
#include "jdx/TabularData.hpp"
#include "jdx/TextReader.hpp"

#include <optional>
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
    PeakTable(const std::string& label, std::string variableList,
        TextReader& reader, std::optional<std::string>& nextLine);
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
