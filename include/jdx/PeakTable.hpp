#ifndef LIBJDX_PEAKTABLE_HPP
#define LIBJDX_PEAKTABLE_HPP

#include "jdx/Peak.hpp"

#include <vector>

namespace sciformats::jdx
{
/**
 * @brief A JCAMP-DX PEAK TABLE record.
 * LDRs.
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
    static std::optional<Peak> nextPeak(
        const std::string& line, size_t& pos, size_t numComponents);
    static bool skipToNextToken(const std::string& line, size_t& pos);
    static std::optional<std::string> nextToken(
        const std::string& line, size_t& pos);
    static bool isTokenDelimiter(const std::string& line, size_t& pos);
};
} // namespace sciformats::jdx

#endif // LIBJDX_PEAKTABLE_HPP
