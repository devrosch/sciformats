#ifndef LIBJDX_DATATABLE_HPP
#define LIBJDX_DATATABLE_HPP

#include "jdx/Array2DData.hpp"
#include "jdx/NTuplesVariables.hpp"
#include "jdx/TextReader.hpp"

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
     * @brief getPlotDescriptor The descriptor of the data table, e.g., "XYDATA"
     * for "(X++(R..R)), XYDATA".
     * @return The data table plot descriptor.
     */
    std::optional<std::string> getPlotDescriptor();

    /**
     * @brief getVariables The relevant variables merged from LDRs of BLOCK,
     * NTUPLES, and PAGE for the DATA TABLE.
     * @return The variables for the DATA TABLE.
     */
    Variables getVariables();

    /**
     * @brief getData The (already scaled if applicable) data from the DATA
     * TABLE.
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
};
} // namespace sciformats::jdx

#endif // LIBJDX_DATATABLE_HPP
