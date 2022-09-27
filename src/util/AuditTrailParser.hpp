#ifndef LIBJDX_AUDITTRAILPARSER_HPP
#define LIBJDX_AUDITTRAILPARSER_HPP

#include "jdx/AuditTrailEntry.hpp"
#include "jdx/TextReader.hpp"

namespace sciformats::jdx::util
{
/**
 * @brief A parser for AUDIT TRAIL.
 */
class AuditTrailParser
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
    TextReader& m_reader;
    const std::string m_variableList;

    // tuple
    std::optional<std::string> nextTuple();
    static bool isTupleStart(const std::string& stringValue);
    static bool isTupleEnd(const std::string& stringValue);
    // assignment
    [[nodiscard]] sciformats::jdx::AuditTrailEntry createAuditTrailEntry(
        const std::string& tuple) const;
};
}

#endif /* LIBJDX_AUDITTRAILPARSER_HPP */
