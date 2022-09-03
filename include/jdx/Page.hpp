#ifndef LIBJDX_PAGE_HPP
#define LIBJDX_PAGE_HPP

#include "jdx/DataTable.hpp"
#include "jdx/LdrContainer.hpp"
#include "jdx/NTuplesAttributes.hpp"
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
class Page : LdrContainer
{
public:
    /**
     * @brief Constructs the record.
     * @param label he label of the LDR, "PAGE".
     * @param pageVar The PAGE variables, e.g., "N=1".
     * @param nTuplesAttributes The attributes of the surrounding NTUPLES
     * record.
     * @param blockLdrs The LDRs of the surrounding block.
     * @param reader Text reader with position assumed to be on the line
     * following the "PAGE" label.
     * @param nextLine The line after the first line of the PAGE. Will return
     * the line following the PAGE record.
     */
    Page(std::string& label, std::string pageVar,
        const std::vector<NTuplesAttributes>& nTuplesAttributes,
        const std::vector<StringLdr>& blockLdrs, TextReader& reader,
        std::optional<std::string>& nextLine);

    /**
     * @brief The page variables of the PAGE record (value of
     * the first line of the LDR), e.g., "N=1" or "X=2.2, Y=3.3".
     * @return The page variables.
     */
    std::string getPageVariables();

    /**
     * @brief The LDRs contained by the PAGE, e.g.
     * "NPOINTS", not including "DATA TABLE".
     * @return The page LDRs.
     */
    std::vector<StringLdr> getPageLdrs();

    /**
     * @brief The DATA TABLE.
     * @return The DATA TABLE.
     */
    std::optional<DataTable> getDataTable();

private:
    static constexpr const char* s_label = "PAGE";

    const std::string m_pageVariables;
    std::vector<StringLdr> m_pageLdrs;
    std::optional<DataTable> m_dataTable;

    static void validateInput(const std::string& label);
    void parse(const std::vector<NTuplesAttributes>& nTuplesAttributes,
        const std::vector<StringLdr>& blockLdrs, TextReader& reader,
        std::optional<std::string>& nextLine);
    static std::vector<StringLdr> parsePageLdrs(
        TextReader& reader, std::optional<std::string>& nextLine);
    static std::pair<std::string, std::optional<std::string>>
    parseDataTableVars(const std::string& rawPageVars);
};
} // namespace sciformats::jdx

#endif // LIBJDX_PAGE_HPP
