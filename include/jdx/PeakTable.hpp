#ifndef LIBJDX_PEAKTABLE_HPP
#define LIBJDX_PEAKTABLE_HPP

#include "jdx/Peak.hpp"

#include <functional>
#include <istream>
#include <string>
#include <vector>

namespace sciformats::jdx
{
/**
 * @brief A JCAMP-DX PEAK TABLE record.
 */
class PeakTable
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

    std::istream& m_istream;
    std::streampos m_streamDataPos;
    std::string m_label;
    std::string m_variableList;

    static void skipToNextLdr(std::istream& iStream);
    static void validateInput(const std::string& label,
        const std::string& variableList, const std::string& expectedLabel,
        const std::vector<std::string>& expectedVariableLists);
    static std::pair<std::string, std::string> readFirstLine(
        std::istream& istream);
    template<typename R>
    R callAndResetStreamPos(const std::function<R()>& func);
};
} // namespace sciformats::jdx

#endif // LIBJDX_PEAKTABLE_HPP
