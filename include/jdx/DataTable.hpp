#ifndef LIBJDX_DATATABLE_HPP
#define LIBJDX_DATATABLE_HPP

#include "jdx/Array2DData.hpp"
#include "jdx/NTuplesVariables.hpp"
#include "jdx/ParseException.hpp"
#include "jdx/TextReader.hpp"

#include <array>
#include <map>
#include <optional>
#include <string>
#include <vector>

namespace sciformats::jdx
{
/**
 * @brief A JCAMP-DX NTUPLES DATA TABLE record.
 */
class DataTable : public Array2DData
{
public:
    struct Variables
    {
        NTuplesVariables xVariables;
        NTuplesVariables yVariables;
    };

    DataTable(std::string& label, std::string variableList,
        std::optional<std::string> plotDescriptor,
        const std::vector<StringLdr>& blockLdrs,
        const std::vector<NTuplesVariables>& nTuplesVars,
        const std::vector<StringLdr>& pageLdrs, TextReader& reader,
        std::optional<std::string>& nextLine);

    /**
     * @brief The descriptor of the data table, e.g., "XYDATA" for "(X++(R..R)),
     * XYDATA".
     * @return The data table plot descriptor.
     */
    std::optional<std::string> getPlotDescriptor();

    /**
     * @brief The relevant variables merged from LDRs of BLOCK,
     * NTUPLES, and PAGE for the DATA TABLE.
     * @return The variables for the DATA TABLE.
     */
    Variables getVariables();

    /**
     * @brief The (already scaled if applicable) data from the DATA TABLE.
     * @return The data from the data table.
     */
    std::vector<std::pair<double, double>> getData();

private:
    enum class PlotDescriptor
    {
        /** PROFILE */
        Profile,
        /** XYDATA */
        XyData,
        /** PEAKS */
        Peaks,
        /** CONTOUR */
        Contour,
    };

    static constexpr const char* s_label = "DATATABLE";
    static constexpr std::array<std::pair<const char*, VariableList>, 4>
        s_varListMapping = {{
            {"(X++(Y..Y))", VariableList::XppYY},
            {"(X++(R..R))", VariableList::XppRR},
            {"(X++(I..I))", VariableList::XppII},
            {"(XY..XY)", VariableList::XyXy},
        }};
    static constexpr std::array<std::pair<const char*, PlotDescriptor>, 4>
        s_plotDescriptorMapping = {{{"PROFILE", PlotDescriptor::Profile},
            {"XYDATA", PlotDescriptor::XyData},
            {"PEAKS", PlotDescriptor::Peaks},
            {"CONTOUR", PlotDescriptor::Contour}}};
    static constexpr const char* s_xSymbol = "X";
    static constexpr std::array<const char*, 3> s_ySymbols = {"Y", "R", "I"};

    const std::optional<std::string> m_plotDescriptor;
    Variables m_mergedVariables;

    void parse(const std::vector<StringLdr>& blockLdrs,
        const std::vector<NTuplesVariables>& nTuplesVars,
        const std::vector<StringLdr>& pageLdrs,
        std::optional<std::string>& nextLine);
    std::pair<VariableList, std::optional<PlotDescriptor>> parseDataTableVars();
    static VariableList determineVariableList(const std::string& varList);
    static PlotDescriptor determinePlotDescriptor(
        const std::string& plotDescriptor);
    static NTuplesVariables mergeVars(const std::vector<StringLdr>& blockLdrs,
        const NTuplesVariables& nTuplesVars,
        const std::vector<StringLdr>& pageLdrs);
    static void mergeLdrs(const std::vector<StringLdr>& ldrs,
        std::map<std::string, std::optional<std::string>&> stringMapping,
        std::map<std::string, std::optional<double>&> doubleMapping,
        std::map<std::string, std::optional<uint64_t>&> uint64Mapping,
        bool replace);
    template<typename R, size_t SIZE>
    static R findValue(
        std::array<std::pair<const char*, R>, SIZE> keyValuePairs,
        const std::string& key, const std::string& type);
};

template<typename R, size_t SIZE>
R sciformats::jdx::DataTable::findValue(
    std::array<std::pair<const char*, R>, SIZE> keyValuePairs,
    const std::string& key, const std::string& type)
{
    const auto* it = std::find_if(keyValuePairs.begin(), keyValuePairs.end(),
        [&key](const auto& mappingItem) { return mappingItem.first == key; });
    if (it != keyValuePairs.end())
    {
        return (*it).second;
    }
    throw ParseException("Illegal " + type + "in NTUPLES PAGE: " + key);
}

} // namespace sciformats::jdx

#endif // LIBJDX_DATATABLE_HPP
