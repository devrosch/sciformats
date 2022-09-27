#ifndef LIBJDX_AUDITTRAILENTRY_HPP
#define LIBJDX_AUDITTRAILENTRY_HPP

#include <optional>

namespace sciformats::jdx
{
/**
 * @brief A JCAMP-DX audit trail entry, i.e. one item in an AUDIT TRAIL.
 */
struct AuditTrailEntry
{
public:
    /**
     * @brief Change number.
     */
    std::int64_t number{};

    // TODO: Use std::chrono::time_point
    /**
     * @brief Timestamp.
     */
    std::string when{};

    /**
     * @brief Person who made or authorized the change.
     */
    std::string who{};

    /**
     * @brief Person’s location.
     */
    std::string where{};

    /**
     * @brief Process.
     */
    std::optional<std::string> process{};

    /**
     * @brief Software version.
     */
    std::optional<std::string> version{};

    /**
     * @brief Details of the change made.
     */
    std::string what{};
};
} // namespace sciformats::jdx

#endif // LIBJDX_AUDITTRAILENTRY_HPP
