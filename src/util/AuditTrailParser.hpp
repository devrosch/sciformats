#ifndef LIBJDX_AUDITTRAILPARSER_HPP
#define LIBJDX_AUDITTRAILPARSER_HPP

#include "jdx/AuditTrailEntry.hpp"
#include "jdx/TextReader.hpp"
#include "util/MultilineTuplesParser.hpp"

#include <array>

namespace sciformats::jdx::util
{
/**
 * @brief A parser for AUDIT TRAIL.
 */
class AuditTrailParser : protected MultilineTuplesParser
{
public:
    explicit AuditTrailParser(TextReader& reader, std::string variableList);

    /**
     * @brief Next audit trail entry.
     * @note Assumes that an audit trail entry tuple always starts on a new
     * line, but may span multiple lines.
     * @return The next audit trail entry, nullopt if there is none.
     * @throws ParseException If next audit trail entry is malformed.
     */
    std::optional<AuditTrailEntry> next();

private:
    static constexpr const char* s_ldrName = "audit trail";

    static constexpr std::array<const char*, 3> s_varLists = {
        "(NUMBER, WHEN, WHO, WHERE, WHAT)",
        "(NUMBER, WHEN, WHO, WHERE, VERSION, WHAT)",
        "(NUMBER, WHEN, WHO, WHERE, PROCESS, VERSION, WHAT)",
    };

    /**
     * matches 5 - 7 audit trail entry segments as groups 1-7, groups 5 nd 6
     * being optional, corresponding to one of (NUMBER, WHEN, WHO, WHERE, WHAT),
     * (NUMBER, WHEN, WHO, WHERE, VERSION, WHAT),
     * (NUMBER, WHEN, WHO, WHERE, PROCESS, VERSION, WHAT)
     */
    const std::regex m_regex{R"(^\s*\(\s*)"
                             R"((\d))"
                             R"((?:\s*,\s*<([^>]*)>))"
                             R"((?:\s*,\s*<([^>]*)>))"
                             R"((?:\s*,\s*<([^>]*)>))"
                             R"((?:\s*,\s*<([^>]*)>)?)"
                             R"((?:\s*,\s*<([^>]*)>)?)"
                             R"((?:\s*,\s*<([^>]*)>))"
                             R"(\s*\)\s*$)"};

    [[nodiscard]] sciformats::jdx::AuditTrailEntry createAuditTrailEntry(
        const std::string& tuple) const;
};
}

#endif /* LIBJDX_AUDITTRAILPARSER_HPP */
