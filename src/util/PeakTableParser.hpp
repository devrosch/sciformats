#ifndef LIBJDX_PEAKTABLEPARSER_HPP
#define LIBJDX_PEAKTABLEPARSER_HPP

#include "jdx/PeakTable.hpp"
#include "jdx/TextReader.hpp"

#include <queue>

namespace sciformats::jdx::util
{
/**
 * @brief A parser for PEAK TABLE.
 */
class PeakTableParser
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
    TextReader& m_reader;
    std::string m_variableList;
    std::queue<std::string> m_tuples;

    // tuple
    std::optional<std::string> nextTuple();
    // peak
    [[nodiscard]] sciformats::jdx::Peak createPeak(
        const std::string& tuple) const;
};
}

#endif /* LIBJDX_PEAKTABLEPARSER_HPP */
