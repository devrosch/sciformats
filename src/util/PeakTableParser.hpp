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
    explicit PeakTableParser(TextReader& reader, size_t numVariables);

    /**
     * @brief Next peak.
     * @note Assumes that a peak tuple does not span multiple lines, but one
     * line may contain multiple tuples.
     * @return The next peak.
     * @throws ParseException If there is no next peak.
     */
    Peak next();

    /**
     * @brief Whether there is another peak.
     * @return True if there is another peak, false otherwise.
     */
    bool hasNext();

private:
    TextReader& m_reader;
    size_t m_numVariables;
    std::queue<std::string> m_tuples;

    // tuple
    std::optional<std::string> nextTuple();
    // peak
    [[nodiscard]] sciformats::jdx::Peak createPeak(
        const std::string& tuple) const;
};
}

#endif /* LIBJDX_PEAKTABLEPARSER_HPP */
