#ifndef LIBJDX_DATATABLE_HPP
#define LIBJDX_DATATABLE_HPP

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
class DataTable
{
public:
    DataTable(std::string& label, std::string variableList,
        const std::vector<NTuplesVariables>& nTuplesVars,
        const std::vector<StringLdr>& blockLdrs, TextReader& reader);

    std::vector<std::pair<double, double>> getData();

private:
    static constexpr const char* s_label = "DATATABLE";

    const std::string m_variableList;
    const std::optional<std::string> m_plotDescriptor;
    TextReader& m_reader;

    void validateInput(const std::string& label);
    void parseVariables(std::string variableList,
        const std::vector<NTuplesVariables>& nTuplesVars,
        const std::vector<StringLdr>& blockLdrs);
};
} // namespace sciformats::jdx

#endif // LIBJDX_DATATABLE_HPP
