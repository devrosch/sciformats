#ifndef LIBJDX_PEAKTABLEPARSER_HPP
#define LIBJDX_PEAKTABLEPARSER_HPP

#include "jdx/PeakTable.hpp"
#include "jdx/TextReader.hpp"

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
    std::string m_currentLine;
    size_t m_currentPos;

    // peak
    std::optional<Peak> nextPeak();
    static std::optional<Peak> nextPeak(
        const std::string& line, size_t& pos, size_t numComponents);
    static bool skipToNextToken(const std::string& line, size_t& pos);
    static std::optional<std::string> nextToken(
        const std::string& line, size_t& pos);
    static bool isTokenDelimiter(const std::string& line, size_t& pos);
};
}

#endif /* LIBJDX_PEAKTABLEPARSER_HPP */
