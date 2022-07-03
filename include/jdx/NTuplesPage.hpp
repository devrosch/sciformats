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
    NTuplesPage(std::string& label, std::string pageVar, const std::vector<NTuplesVariables>& nTuplesVars, const std::vector<StringLdr>& blockLdrs, TextReader& reader, std::optional<std::string>& nextLine);

private:
    static constexpr const char* s_label = "PAGE";

    const std::string m_pageVar;

    static void validateInput(const std::string& label);
    void parse(const std::vector<NTuplesVariables>& nTuplesVars, const std::vector<StringLdr>& blockLdrs, TextReader& reader, std::optional<std::string>& nextLine);
};
} // namespace sciformats::jdx

#endif // LIBJDX_NTUPLESPAGE_HPP
