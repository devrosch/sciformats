#include "jdx/AuditTrail.hpp"
#include "util/AuditTrailParser.hpp"
#include "util/LdrUtils.hpp"
#include "util/StringUtils.hpp"

#include <algorithm>

sciformats::jdx::AuditTrail::AuditTrail(const std::string& label,
    std::string variableList, TextReader& reader,
    std::optional<std::string>& nextLine)
    : TabularData(label, std::move(variableList), reader)
{
    auto varList = getVariableList();
    util::trim(varList);
    validateInput(getLabel(), varList, s_label,
        std::vector<std::string>{
            std::begin(s_variableLists), std::end(s_variableLists)});

    // check if overruling Bruker var list is present
    m_brukerVarList = scanForBrukerVarList(nextLine);
    if (m_brukerVarList)
    {
        validateInput(getLabel(), m_brukerVarList.value(), s_label,
            std::vector<std::string>{
                std::begin(s_variableLists), std::end(s_variableLists)});
    }

    util::skipToNextLdr(reader, nextLine, false);
}

std::vector<sciformats::jdx::AuditTrailEntry>
sciformats::jdx::AuditTrail::getData()
{
    auto variableList = m_brukerVarList.value_or(getVariableList());
    if (util::isPureComment(variableList))
    {
        // deal with variable lists that sit behind "$$"
        variableList
            = util::stripLineComment(variableList, false, true).second.value();
    }
    util::AuditTrailParser parser{getReader(), variableList};
    return TabularData::getData<util::AuditTrailParser, AuditTrailEntry>(
        parser);
}

std::optional<std::string> sciformats::jdx::AuditTrail::scanForBrukerVarList(
    std::optional<std::string>& nextLine)
{
    auto& reader = getReader();
    if (!reader.eof())
    {
        nextLine = reader.readLine();
        if (!nextLine
            || nextLine.value().rfind("$$ ##TITLE= Audit trail,", 0) != 0)
        {
            return std::nullopt;
        }
    }
    // Bruker audit trail
    while (!reader.eof())
    {
        nextLine = reader.readLine();
        if (!nextLine || !util::isPureComment(nextLine.value()))
        {
            break;
        }
        if (nextLine.value().rfind("$$ ##AUDIT TRAIL=", 0) == 0)
        {
            auto brukerAuditTrail
                = util::stripLineComment(nextLine.value(), false, true)
                      .second.value();
            auto brukerVarList = util::parseLdrStart(brukerAuditTrail).second;
            util::trim(brukerVarList);
            return brukerVarList;
        }
    }
    return std::nullopt;
}
