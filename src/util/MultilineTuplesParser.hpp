#ifndef LIBJDX_MULTILINETUPLESPARSER_HPP
#define LIBJDX_MULTILINETUPLESPARSER_HPP

#include "jdx/TextReader.hpp"
#include "util/TuplesParser.hpp"

#include <regex>

namespace sciformats::jdx::util
{
/**
 * @brief A parser base class for multiline tuple parsers such as AUDIT TRAIL
 * and PEAK ASSIGNMENTS.
 */
class MultilineTuplesParser : protected TuplesParser
{
protected:
    /**
     * @brief Base for parsing tuples that may span multiples lines, delimited
     * by a ")" at the tuples last line ends.
     * @param reader A reader at the start of the first tuple.
     * @param variableList A variable list describing the tuples.
     * @param ldrName The name of the LDR.
     * @param lineBreakChars A char array to replace line breaks with.
     */
    explicit MultilineTuplesParser(TextReader& reader, std::string variableList,
        std::string ldrName, std::string lineBreakChars);

    /**
     * @brief Retrieves the next tuple, delimited by ")" at the end of a line
     * from the TextReader.
     * @return The next tuple.
     */
    std::optional<std::string> nextTuple();

private:
    TextReader& m_reader;
    const std::string m_lineBreakChars;

    static bool isTupleStart(const std::string& stringValue);
    static bool isTupleEnd(const std::string& stringValue);
};
}

#endif /* LIBJDX_MULTILINETUPLESPARSER_HPP */
