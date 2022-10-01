#ifndef LIBJDX_PEAKTABLEPARSER_HPP
#define LIBJDX_PEAKTABLEPARSER_HPP

#include "jdx/PeakTable.hpp"
#include "jdx/TextReader.hpp"
#include "util/TuplesParser.hpp"

#include <array>
#include <optional>
#include <queue>
#include <regex>

namespace sciformats::jdx::util
{
/**
 * @brief A parser for PEAK TABLE.
 */
class PeakTableParser : protected TuplesParser
{
public:
    explicit PeakTableParser(TextReader& reader, std::string variableList);

    /**
     * @brief Next peak.
     * @note Assumes that a peak tuple does not span multiple lines, but one
     * line may contain multiple tuples.
     * @return The next peak, nullopt if there is none.
     * @throws ParseException If there the next peak is malformed.
     */
    std::optional<Peak> next();

private:
    static constexpr const char* s_ldrName = "peak table";

    static constexpr std::array<const char*, 3> s_varLists = {
        "(XY..XY)",
        "(XYW..XYW)",
        "(XYM..XYM)",
    };

    /**
     * matches 2-3 peak segments as groups 1-3, corresponding to
     * (XY..XY), (XYW..XYW), or (XYM..XYM), with X as matches[1], Y as matche[2]
     * and W or M as matches[3]
     */
    const std::regex m_regex{R"(^\s*)"
                             R"(([^,]*))"
                             R"((?:\s*,\s*([^,]*)))"
                             R"((?:\s*,\s*([^,]*))?)"
                             R"($)"};

    TextReader& m_reader;
    std::queue<std::string> m_tuples;

    // tuple
    std::optional<std::string> nextTuple();
    // peak
    [[nodiscard]] sciformats::jdx::Peak createPeak(
        const std::string& tuple) const;
};
}

#endif /* LIBJDX_PEAKTABLEPARSER_HPP */
