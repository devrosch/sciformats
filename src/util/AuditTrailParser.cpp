#include "util/AuditTrailParser.hpp"
#include "jdx/AuditTrailEntry.hpp"
#include "jdx/ParseException.hpp"
#include "util/LdrUtils.hpp"
#include "util/StringUtils.hpp"

#include <algorithm>
#include <map>
#include <regex>

sciformats::jdx::util::AuditTrailParser::AuditTrailParser(
    TextReader& reader, std::string variableList)
    : MultilineTuplesParser(reader, std::move(variableList), s_ldrName, "\n")
{
}

// TODO: duplicate of PeakAssignmentsParser
std::optional<sciformats::jdx::AuditTrailEntry>
sciformats::jdx::util::AuditTrailParser::next()
{
    auto nextString = nextTuple();
    if (!nextString)
    {
        return std::nullopt;
    }
    auto nextAssignment = createAuditTrailEntry(nextString.value());
    return nextAssignment;
}

sciformats::jdx::AuditTrailEntry
sciformats::jdx::util::AuditTrailParser::createAuditTrailEntry(
    const std::string& tuple) const
{
    // tokenize
    // token[0] is the full match so extract 1 + 7 tokens for 7 groups
    const auto tokens = extractTokens(tuple, m_regex, 1 + 7);
    const auto& varList = getVariableList();

    // error conditions {varList, {error condition, error message}}
    const std::multimap<std::string, std::tuple<bool, std::string>> errorMap{
        {s_varLists.at(0),
            {tokens.at(5) || tokens.at(6),
                std::string{"Illegal "} + s_ldrName + " entry for "
                    + s_varLists.at(0) + ": " + tuple}},
        {s_varLists.at(1),
            {!tokens.at(5) || tokens.at(6),
                std::string{"Illegal "} + s_ldrName + " entry for "
                    + s_varLists.at(1) + ": " + tuple}},
        {s_varLists.at(2),
            {!tokens.at(5) || !tokens.at(6),
                std::string{"Illegal "} + s_ldrName + " entry for "
                    + s_varLists.at(2) + ": " + tuple}},
    };

    checkForErrors(varList, errorMap, s_ldrName);

    // map
    AuditTrailEntry entry{};
    entry.number = strtol(tokens.at(1).value().data(), nullptr, 10);
    entry.when = tokens.at(2).value();
    entry.who = tokens.at(3).value();
    entry.where = tokens.at(4).value();
    entry.what = tokens.at(7).value();
    // nothing to do for varLists[0]
    if (s_varLists.at(1) == varList)
    {
        entry.version = tokens.at(5).value();
    }
    else if (s_varLists.at(2) == varList)
    {
        entry.process = tokens.at(5).value();
        entry.version = tokens.at(6).value();
    }

    return entry;
}
