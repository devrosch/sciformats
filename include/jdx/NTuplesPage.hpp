#ifndef LIBJDX_NTUPLESPAGE_HPP
#define LIBJDX_NTUPLESPAGE_HPP

#include "jdx/LdrContainer.hpp"
#include "jdx/TextReader.hpp"
#include "jdx/StringLdr.hpp"
#include "jdx/NTuplesVariables.hpp"

#include <vector>
#include <map>
#include <functional>

namespace sciformats::jdx
{
/**
 * @brief A JCAMP-DX NTUPLES PAGE record.
 */
class NTuplesPage : LdrContainer
{
public:
    enum class VarType
    {
        /** (X++(Y..Y)) */
        XppYY,
        /** (X++(R..R)) */
        XppRR,
        /** (X++(I..I)) */
        XppII,
        /** (XY..XY) */
        XYXY,
    };

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

    struct Variables
    {
        NTuplesVariables xVariables;
        NTuplesVariables yVariables;
    };

    NTuplesPage(std::string& label, std::string pageVar, const std::vector<NTuplesVariables>& nTuplesVars, const std::vector<StringLdr>& blockLdrs, TextReader& reader, std::optional<std::string>& nextLine);
    /**
     * @brief getPageVariables The page variables of the PAGE record (value of the first line of the LDR), e.g., "N=1" or "X=2.2, Y=3.3".
     * @return The page variables.
     */
    std::string getPageVariables();
    /**
     * @brief getPageVariableLdrs The LDRs contained by the PAGE, e.g. "NPOINTS", not including "DATA TABLE".
     * @return The page variable LDRs.
     */
    std::vector<StringLdr> getPageVariableLdrs();
    /**
     * @brief getDataTableVariableList The variable list describing the data table, e.g., (X++(Y..Y)).
     * @return The data table variable list.
     */
    VarType getDataTableVariableList();
    /**
     * @brief getDataTablePlotDescriptor The variable list describing the data table, e.g., "XYDATA" for "(X++(R..R)), XYDATA".
     * @return The data table plot descriptor.
     */
    std::optional<PlotDescriptor> getDataTablePlotDescriptor();
    /**
     * @brief getDataTableVariables The relevant variables merged from LDRs of BLOCK, NTUPLES, and PAGE for the DATA TABLE.
     * @return The variables for the DATA TABLE.
     */
    Variables getDataTableVariables();
    /**
     * @brief getDataTable The data (already scaled if applicable) from the data table.
     * @return The data from the data table.
     */
    std::vector<std::pair<double, double>> getDataTable();

private:
    static constexpr const char* s_label = "PAGE";
    static constexpr std::array<std::pair<const char*, PlotDescriptor>, 4> s_plotDescriptorMapping = {
        {
            {"PROFILE", PlotDescriptor::Profile}, {"XYDATA", PlotDescriptor::XyData},
            {"Peaks", PlotDescriptor::Peaks}, {"CONTOUR", PlotDescriptor::Contour}
        }
    };
    static constexpr std::array<std::pair<const char*, VarType>, 4> s_varTypeMapping = {
        {
            {"(X++(Y..Y))", VarType::XppYY}, {"(X++(R..R))", VarType::XppRR}, {"(X++(I..I))", VarType::XppII},
            {"(XY..XY)", VarType::XYXY},
        }
    };

    TextReader& m_reader;
    std::streampos m_dataPos;
    // page
    const std::string m_pageVariables;
    std::vector<StringLdr> m_pageVariableLdrs;
    // data table
    VarType m_dataTableVariableList;
    std::optional<PlotDescriptor> m_dataTablePlotDescriptor;
    Variables m_dataTableVariables;
    std::streampos m_dataTablePos;

    static void validateInput(const std::string& label);
    void parse(const std::vector<NTuplesVariables>& nTuplesVars, const std::vector<StringLdr>& blockLdrs, TextReader& reader, std::optional<std::string>& nextLine);
    static std::vector<StringLdr> parsePageVarLdrs(TextReader& reader, std::optional<std::string>& nextLine);
    static std::pair<VarType, std::optional<PlotDescriptor>> parseDataTableVars(const std::string& rawPageVar);
    static VarType determineVarType(const std::string& varList);
    static PlotDescriptor determinePlotDescriptor(const std::string& plotDescriptor);
    static NTuplesVariables mergeVars(const std::vector<StringLdr>& blockLdrs, const NTuplesVariables& nTuplesVars, const std::vector<StringLdr>& pageLdrs);
    std::vector<std::pair<double, double>> readXppYyData();
    std::vector<std::pair<double, double>> readXyXyData();
    template<typename R>
    R callAndResetStreamPos(const std::function<R()>& func);
};

// TODO: duplicate of template in DataLdr; also derive from that?
template<typename R>
R sciformats::jdx::NTuplesPage::callAndResetStreamPos(
    const std::function<R()>& func)
{
    auto pos = m_reader.eof() ? std::nullopt
                              : std::optional<std::streampos>(m_reader.tellg());
    try
    {
        m_reader.seekg(m_dataTablePos);
        R returnValue = func();

        // reset reader
        if (pos)
        {
            m_reader.seekg(pos.value());
        }

        return returnValue;
    }
    catch (...)
    {
        try
        {
            if (pos)
            {
                m_reader.seekg(pos.value());
            }
        }
        catch (...)
        {
        }
        throw;
    }
}

} // namespace sciformats::jdx

#endif // LIBJDX_NTUPLESPAGE_HPP
