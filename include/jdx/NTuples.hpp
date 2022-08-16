#ifndef LIBJDX_NTUPLES_HPP
#define LIBJDX_NTUPLES_HPP

#include "jdx/LdrContainer.hpp"
#include "jdx/TextReader.hpp"
#include "jdx/StringLdr.hpp"
#include "jdx/NTuplesVariables.hpp"
#include "jdx/NTuplesPage.hpp"

#include <vector>
#include <map>
#include <functional>

namespace sciformats::jdx
{
/**
 * @brief A JCAMP-DX NTUPLES record.
 */
class NTuples : public LdrContainer
{
public:
    /**
     * @brief Constructs the record.
     * @param label The label of the LDR, "NTUPLES".
     * @param dataForm The value of the first line of the record.
     * @param reader Text reader with position assumed to be on the line following the NTUPLES label.
     * @param ldrs String LDRs of the surrounding block.
     * representing the data form, e.g. "NMR FID" or "MASS SPECTRUM".
     */
    NTuples(const std::string& label, std::string dataForm, TextReader& reader, const std::vector<StringLdr>& blockLdrs);
    /**
     * @brief getDataForm The data form of the NTUPLES record (value of the first line of the LDR), e.g., "NMR FID" or "MASS SPECTRUM".
     * @return The data form.
     */
    std::string getDataForm();
    /**
     * @brief getNumPages Returns the number of pages in this record.
     * @return The number of pages.
     */
    size_t getNumPages();
    /**
     * @brief getPage Retrieves a page from the record.
     * @param pageIndex The page index starting at zero.
     * @return The page.
     */
    NTuplesPage getPage(size_t pageIndex);

private:
    static constexpr const char* s_label = "NTUPLES";
    static constexpr std::array<const char*, 11> s_variables
        = {"VARNAME", "SYMBOL", "VARTYPE", "VARFORM", "VARDIM", "UNITS", "FIRST", "LAST", "MIN", "MAX", "FACTOR"};

    const std::string m_dataForm;
    std::vector<NTuplesPage> m_pages;

    static void validateInput(const std::string& label);
    void parse(const std::vector<StringLdr>& blockLdrs, TextReader& reader);
    std::vector<NTuplesVariables> parseVariables(TextReader& reader, std::optional<std::string>& nextLine);
    static std::vector<StringLdr> readVariables(std::optional<std::string>& firstVarStart, TextReader& reader);
    static std::map<std::string, std::vector<std::string>> splitValues(const std::vector<StringLdr>& vars);
    static std::map<std::string, std::vector<std::string>> extractStandardVariables(std::map<std::string, std::vector<std::string>>& vars);
    NTuplesVariables map(const std::map<std::string, std::vector<std::string>>& standardVars, const std::map<std::string, std::vector<std::string>>& additionalVars, size_t valueColumnIndex);
    static std::optional<std::vector<std::string>> findValue(const std::string& key, const std::map<std::string, std::vector<std::string>>& map);
};
} // namespace sciformats::jdx

#endif // LIBJDX_NTUPLES_HPP
