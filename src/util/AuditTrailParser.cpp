#include "util/AuditTrailParser.hpp"
#include "jdx/AuditTrailEntry.hpp"
#include "jdx/ParseException.hpp"
#include "util/LdrUtils.hpp"
#include "util/StringUtils.hpp"

#include <algorithm>
#include <regex>

// TODO: duplicate of PeakAssignmentsParser
sciformats::jdx::util::AuditTrailParser::AuditTrailParser(
    TextReader& reader, std::string variableList)
    : m_reader{reader}
    , m_variableList{std::move(variableList)}
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

// TODO: duplicate of PeakAssignmentsParser
std::optional<std::string> sciformats::jdx::util::AuditTrailParser::nextTuple()
{
    std::string tupleString{};
    // find start
    while (!m_reader.eof())
    {
        std::streampos pos = m_reader.tellg();
        auto line = m_reader.readLine();
        auto [lineStart, _] = util::stripLineComment(line, true);
        if (isTupleStart(lineStart))
        {
            tupleString.append(lineStart);
            break;
        }
        if (util::isLdrStart(lineStart))
        {
            // AUDIT TRAIL LDR ended, no peak assignments
            m_reader.seekg(pos);
            return std::nullopt;
        }
        if (!lineStart.empty())
        {
            throw ParseException(
                "Illegal string found in audit trail: " + line);
        }
    }
    if (isTupleEnd(tupleString))
    {
        return tupleString;
    }
    // read to end of current audit trail
    while (!m_reader.eof())
    {
        std::streampos pos = m_reader.tellg();
        auto line = m_reader.readLine();
        auto [lineStart, _] = util::stripLineComment(line, true);

        if (util::isLdrStart(lineStart))
        {
            // AUDIT TRAIL LDR ended before end of last audit trail
            m_reader.seekg(pos);
            throw ParseException(
                "No closing parenthesis found for audit trail: " + tupleString);
        }
        tupleString.append("\n");
        tupleString.append(lineStart);
        if (isTupleEnd(lineStart))
        {
            return tupleString;
        }
        if (m_reader.eof() || util::isLdrStart(lineStart))
        {
            // AUDIT TRAIL LDR ended before end of last audit trail
            m_reader.seekg(pos);
            throw ParseException(
                "No closing parenthesis found for audit trail: " + tupleString);
        }
    }
    throw ParseException(
        "File ended before closing parenthesis was found for audit trail: "
        + tupleString);
}

// TODO: duplicate of PeakAssignmentsParser
bool sciformats::jdx::util::AuditTrailParser::isTupleStart(
    const std::string& stringValue)
{
    std::string value{stringValue};
    util::trimLeft(value);
    return !value.empty() && value.at(0) == '(';
}

// TODO: duplicate of PeakAssignmentsParser
bool sciformats::jdx::util::AuditTrailParser::isTupleEnd(
    const std::string& stringValue)
{
    std::string value{stringValue};
    util::trimRight(value);
    return !value.empty() && value.back() == ')';
}

sciformats::jdx::AuditTrailEntry
sciformats::jdx::util::AuditTrailParser::createAuditTrailEntry(
    const std::string& tuple) const
{
    // up to 7 matches, matches 5 and 6 optional
    // ^\s*\(\s*(\d)(?:\s*,\s*<([^>]*)>)(?:\s*,\s*<([^>]*)>)(?:\s*,\s*<([^>]*)>)(?:\s*,\s*<([^>]*)>)?(?:\s*,\s*<([^>]*)>)?(?:\s*,\s*<([^>]*)>)\s*\)\s*$

    // tokenize
    // matches 5 - 7 audit trail entry segments as groups 1-7, groups 5 nd 6
    // being optional, corresponding to one of (NUMBER, WHEN, WHO, WHERE, WHAT),
    // (NUMBER, WHEN, WHO, WHERE, VERSION, WHAT),
    // (NUMBER, WHEN, WHO, WHERE, PROCESS, VERSION, WHAT)
    const auto* regexString = R"(^\s*\(\s*)"
                              R"((\d))"
                              R"((?:\s*,\s*<([^>]*)>))"
                              R"((?:\s*,\s*<([^>]*)>))"
                              R"((?:\s*,\s*<([^>]*)>))"
                              R"((?:\s*,\s*<([^>]*)>)?)"
                              R"((?:\s*,\s*<([^>]*)>)?)"
                              R"((?:\s*,\s*<([^>]*)>))"
                              R"(\s*\)\s*$)";

    std::regex regex{regexString};
    std::smatch matches;
    auto [lineStart, _] = util::stripLineComment(tuple, true);
    if (!std::regex_match(lineStart, matches, regex))
    {
        throw ParseException("Illegal audit trail entry string: " + tuple);
    }

    auto token1 = matches[1].matched
                      ? std::optional<std::string>{matches[1].str()}
                      : std::nullopt;
    auto token2 = matches[2].matched
                      ? std::optional<std::string>{matches[2].str()}
                      : std::nullopt;
    auto token3 = matches[3].matched
                      ? std::optional<std::string>{matches[3].str()}
                      : std::nullopt;
    auto token4 = matches[4].matched
                      ? std::optional<std::string>{matches[4].str()}
                      : std::nullopt;
    auto token5 = matches[5].matched
                      ? std::optional<std::string>{matches[5].str()}
                      : std::nullopt;
    auto token6 = matches[6].matched
                      ? std::optional<std::string>{matches[6].str()}
                      : std::nullopt;
    auto token7 = matches[7].matched
                      ? std::optional<std::string>{matches[7].str()}
                      : std::nullopt;

    // map to peak assignment
    AuditTrailEntry entry{};
    entry.number = strtol(token1.value().data(), nullptr, 10);
    entry.when = token2.value();
    entry.who = token3.value();
    entry.where = token4.value();
    entry.what = token7.value();
    if ("(NUMBER, WHEN, WHO, WHERE, WHAT)" == m_variableList)
    {
        if (token5 || token6)
        {
            throw ParseException("Illegal audit trail entry components for "
                                 "(NUMBER, WHEN, WHO, WHERE, WHAT): "
                                 + lineStart);
        }
    }
    else if ("(NUMBER, WHEN, WHO, WHERE, VERSION, WHAT)" == m_variableList)
    {
        if (!token5 || token6)
        {
            throw ParseException("Illegal audit trail entry component for "
                                 "(NUMBER, WHEN, WHO, WHERE, VERSION, WHAT): "
                                 + lineStart);
        }
        entry.version = token5.value();
    }
    else if ("(NUMBER, WHEN, WHO, WHERE, PROCESS, VERSION, WHAT)"
             == m_variableList)
    {
        if (!token5 || !token6)
        {
            throw ParseException(
                "Illegal audit trail entry component for (NUMBER, WHEN, WHO, "
                "WHERE, PROCESS, VERSION, WHAT): "
                + lineStart);
        }
        entry.process = token5.value();
        entry.version = token6.value();
    }
    else
    {
        throw ParseException(
            "Unsupported variable list for audit trail: " + m_variableList);
    }
    return entry;
}
