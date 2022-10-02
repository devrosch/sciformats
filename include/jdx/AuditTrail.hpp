#ifndef LIBJDX_AUDITTRAIL_HPP
#define LIBJDX_AUDITTRAIL_HPP

#include "jdx/AuditTrailEntry.hpp"
#include "jdx/TabularData.hpp"
#include "jdx/TextReader.hpp"

#include <array>
#include <optional>
#include <string>
#include <vector>

namespace sciformats::jdx
{
/**
 * @brief A JCAMP-DX AUDIT TRAIL record.
 */
class AuditTrail : public TabularData
{
public:
    /**
     * @brief AuditTrail Constructs AuditTrail.
     * @param label The label of the first line of the record, i.e.
     * "AUDITTRAIL".
     * @param variableList The value of the first line of the record
     * representing the structure of the data, e.g., "(NUMBER, WHEN, WHO, WHERE,
     * WHAT)".
     * @param reader Text reader with JCAMP-DX data. The reader position is
     * assumed to be at the start of the second line (the line following the
     * line containing the label) of the record. The reader is expected to exist
     * for the lifetime of this object.
     * @param nextLine The first line of the LDR, i.e., the one containing the
     * label. Will contain the line following the record or nullopt if the end
     * of the reader has been reached.
     */
    AuditTrail(const std::string& label, std::string variableList,
        TextReader& reader, std::optional<std::string>& nextLine);

    /**
     * @brief Provides the parsed audit trail data.
     * @return The list of audit trail entries.
     */
    std::vector<AuditTrailEntry> getData();

private:
    static constexpr const char* s_label = "AUDITTRAIL";
    static constexpr std::array<const char*, 6> s_variableLists
        = {"(NUMBER, WHEN, WHO, WHERE, WHAT)",
            "$$ (NUMBER, WHEN, WHO, WHERE, WHAT)",
            "(NUMBER, WHEN, WHO, WHERE, VERSION, WHAT)",
            "$$ (NUMBER, WHEN, WHO, WHERE, VERSION, WHAT)",
            "(NUMBER, WHEN, WHO, WHERE, PROCESS, VERSION, WHAT)",
            "$$ (NUMBER, WHEN, WHO, WHERE, PROCESS, VERSION, WHAT)"};

    std::optional<std::string> m_brukerVarList;

    std::optional<std::string> scanForBrukerVarList(
        std::optional<std::string>& nextLine);
};
} // namespace sciformats::jdx

#endif // LIBJDX_AUDITTRAIL_HPP
