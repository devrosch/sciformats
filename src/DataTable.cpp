#include "jdx/DataTable.hpp"
#include "jdx/ParseException.hpp"

sciformats::jdx::DataTable::DataTable(std::string& label, std::string variableList, const std::vector<NTuplesVariables>& nTuplesVars, const std::vector<StringLdr>& blockLdrs, TextReader& reader)
    : m_variableList{std::move(variableList)}
    , m_reader{reader}
{
    validateInput(label);
    parseVariables(variableList, nTuplesVars, blockLdrs);
}

// TODO: duplicate of NTuplesPage
void sciformats::jdx::DataTable::validateInput(const std::string& label)
{
    if (label != s_label)
    {
        throw ParseException("Illegal label at " + std::string{s_label}
                             + " start encountered: " + label);
    }
}
