#ifndef LIBJDX_DATATABLE_HPP
#define LIBJDX_DATATABLE_HPP

#include "jdx/Data2D.hpp"
#include "jdx/NTuplesAttributes.hpp"
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
class DataTable : public Data2D
{
public:
    struct Attributes
    {
        NTuplesAttributes xAttributes;
        NTuplesAttributes yAttributes;
    };

    /**
     * @brief Constructs the record.
     * @param label he label of the LDR, "DATATABLE".
     * @param variableList The variable list, e.g. "(X++(Y..Y))".
     * @param plotDescriptor The plot descriptor, e.g. "XYDATA".
     * @param blockLdrs The LDRs of the surrounding block.
     * @param nTuplesAttributes The attributes of the surrounding NTUPLES
     * record.
     * @param pageLdrs The LDRs of the surroundling PAGE.
     * @param reader Text reader with position assumed to be on the line
     * following the "DATA TABLE" label.
     * @param nextLine The first line of the LDR, i.e. the one containing the
     * label. Will contain the line following the record or nullopt if the end
     * of the reader has been reached.
     */
    DataTable(std::string label, std::string variableList,
        std::optional<std::string> plotDescriptor,
        const std::vector<StringLdr>& blockLdrs,
        const std::vector<NTuplesAttributes>& nTuplesAttributes,
        const std::vector<StringLdr>& pageLdrs, TextReader& reader,
        std::optional<std::string>& nextLine);

    /**
     * @brief The plot descriptor of the data table, e.g., "XYDATA" for
     * "(X++(R..R)), XYDATA".
     * @return The data table plot descriptor.
     */
    std::optional<std::string> getPlotDescriptor();

    /**
     * @brief The relevant parameters merged from LDRs of BLOCK,
     * NTUPLES, and PAGE for the DATA TABLE.
     * @return The attributes for the DATA TABLE.
     */
    Attributes getAttributes();

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
    static constexpr std::array<std::pair<const char*, VariableList>, 6>
        s_varListMapping = {{
            {"(X++(Y..Y))", VariableList::XppYY},
            {"(X++(R..R))", VariableList::XppRR},
            {"(X++(I..I))", VariableList::XppII},
            {"(XY..XY)", VariableList::XYXY},
            {"(XR..XR)", VariableList::XRXR},
            {"(XI..XI)", VariableList::XIXI},
        }};
    static constexpr std::array<std::pair<const char*, PlotDescriptor>, 4>
        s_plotDescriptorMapping = {{{"PROFILE", PlotDescriptor::Profile},
            {"XYDATA", PlotDescriptor::XyData},
            {"PEAKS", PlotDescriptor::Peaks},
            {"CONTOUR", PlotDescriptor::Contour}}};
    static constexpr const char* s_xSymbol = "X";
    static constexpr std::array<const char*, 3> s_ySymbols = {"Y", "R", "I"};

    const std::optional<std::string> m_plotDescriptor;
    Attributes m_mergedAttributes;

    void parse(const std::vector<StringLdr>& blockLdrs,
        const std::vector<NTuplesAttributes>& nTuplesVars,
        const std::vector<StringLdr>& pageLdrs,
        std::optional<std::string>& nextLine);
    std::pair<VariableList, std::optional<PlotDescriptor>> parseDataTableVars();
    static VariableList determineVariableList(const std::string& varList);
    static PlotDescriptor determinePlotDescriptor(
        const std::string& plotDescriptor);
    static NTuplesAttributes mergeVars(const std::vector<StringLdr>& blockLdrs,
        const NTuplesAttributes& nTuplesVars,
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
