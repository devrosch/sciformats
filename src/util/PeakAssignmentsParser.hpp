#ifndef LIBJDX_PEAKASSIGNMENTSPARSER_HPP
#define LIBJDX_PEAKASSIGNMENTSPARSER_HPP

#include "jdx/PeakAssignment.hpp"
#include "jdx/TextReader.hpp"
#include "util/MultilineTuplesParser.hpp"

#include <array>

namespace sciformats::jdx::util
{
/**
 * @brief A parser for PEAK ASSIGNMENTS.
 */
class PeakAssignmentsParser : protected MultilineTuplesParser
{
public:
    explicit PeakAssignmentsParser(
        TextReader& reader, std::string variableList);

    /**
     * @brief Next peak assignment.
     * @note Assumes that a peak assignment tuple always starts on a new line,
     * but may span multiple lines.
     * @return The next peak assignment, nullopt if there is none.
     * @throws ParseException If next peak assignment is malformed.
     */
    std::optional<PeakAssignment> next();

private:
    static constexpr const char* s_ldrName = "peak assignments";

    static constexpr std::array<const char*, 4> s_varLists = {
        "(XYA)",
        "(XYWA)",
        "(XYMA)",
        "(XYMWA)",
    };

    /**
     * matches 2 - 5 peak assignments segments  as groups 1-5, corresponding to
     * one of (X[, Y][, W], A), (X[, Y][, M], A), (X[, Y][, M][, W], A), with X
     * as matches[1] and A as matches[5]
     */
    const std::regex m_regex{R"(^\s*\(\s*)"
                             R"(([^,]*))"
                             R"((?:\s*,\s*([^,]*))?)"
                             R"((?:\s*,\s*([^,]*))?)"
                             R"((?:\s*,\s*([^,]*))?)"
                             R"(\s*,\s*<(.*)>\s*\))"
                             R"(\s*$)"};

    [[nodiscard]] sciformats::jdx::PeakAssignment createPeakAssignment(
        const std::string& tuple) const;
};
}

#endif /* LIBJDX_PEAKASSIGNMENTSPARSER_HPP */
