#ifndef LIBJDX_NTUPLESPAGE_HPP
#define LIBJDX_NTUPLESPAGE_HPP

#include "jdx/DataTable.hpp"
#include "jdx/LdrContainer.hpp"
#include "jdx/NTuplesVariables.hpp"
#include "jdx/StringLdr.hpp"
#include "jdx/TextReader.hpp"

#include <array>
#include <functional>
#include <map>
#include <vector>

namespace sciformats::jdx
{
/**
 * @brief A JCAMP-DX NTUPLES PAGE record.
 */
class NTuplesPage : LdrContainer
{
public:
    NTuplesPage(std::string& label, std::string pageVar,
        const std::vector<NTuplesVariables>& nTuplesVars,
        const std::vector<StringLdr>& blockLdrs, TextReader& reader,
        std::optional<std::string>& nextLine);

    /**
     * @brief getPageVariables The page variables of the PAGE record (value of
     * the first line of the LDR), e.g., "N=1" or "X=2.2, Y=3.3".
     * @return The page variables.
     */
    std::string getPageVariables();

    /**
     * @brief getPageVariableLdrs The LDRs contained by the PAGE, e.g.
     * "NPOINTS", not including "DATA TABLE".
     * @return The page variable LDRs.
     */
    std::vector<StringLdr> getPageVariableLdrs();

    /**
     * @brief getDataTable The DATA TABLE.
     * @return The DATA TABLE.
     */
    std::optional<DataTable> getDataTable();

private:
    static constexpr const char* s_label = "PAGE";

    const std::string m_pageVariables;
    std::vector<StringLdr> m_pageVariableLdrs;
    std::optional<DataTable> m_dataTable;

    static void validateInput(const std::string& label);
    void parse(const std::vector<NTuplesVariables>& nTuplesVars,
        const std::vector<StringLdr>& blockLdrs, TextReader& reader,
        std::optional<std::string>& nextLine);
    static std::vector<StringLdr> parsePageVarLdrs(
        TextReader& reader, std::optional<std::string>& nextLine);
    static std::pair<std::string, std::optional<std::string>>
    parseDataTableVars(const std::string& rawPageVar);
};
} // namespace sciformats::jdx

#endif // LIBJDX_NTUPLESPAGE_HPP
