#include "util/MultilineTuplesParser.hpp"
#include "jdx/ParseException.hpp"
#include "util/LdrUtils.hpp"
#include "util/StringUtils.hpp"

sciformats::jdx::util::MultilineTuplesParser::MultilineTuplesParser(
    TextReader& reader, std::string variableList, std::string ldrName,
    std::string lineBreakChars)
    : TuplesParser(std::move(variableList), std::move(ldrName))
    , m_reader{reader}
    , m_lineBreakChars{std::move(lineBreakChars)}
{
}

std::optional<std::string>
sciformats::jdx::util::MultilineTuplesParser::nextTuple()
{
    const auto& ldrName = getLdrName();
    std::string tupleString{};
    // find start
    while (!m_reader.eof())
    {
        std::streampos pos = m_reader.tellg();
        auto line = m_reader.readLine();
        auto [lineStart, _] = util::stripLineComment(line, true);
        if (isTupleStart(lineStart))
        {
            tupleString.append(lineStart);
            break;
        }
        if (util::isLdrStart(lineStart))
        {
            // LDR ended, no tuple
            m_reader.seekg(pos);
            return std::nullopt;
        }
        if (!lineStart.empty())
        {
            throw ParseException(std::string{"Illegal string found in "}
                                     .append(ldrName)
                                     .append(": ")
                                     .append(line));
        }
    }
    if (isTupleEnd(tupleString))
    {
        return tupleString;
    }
    // read to end of current tuple
    while (!m_reader.eof())
    {
        std::streampos pos = m_reader.tellg();
        auto line = m_reader.readLine();
        auto [lineStart, _] = util::stripLineComment(line, true);

        if (util::isLdrStart(lineStart))
        {
            // LDR ended before end of last tuple
            m_reader.seekg(pos);
            throw ParseException(
                std::string{"No closing parenthesis found for "}
                    .append(ldrName)
                    .append(" entry: ")
                    .append(tupleString));
        }
        tupleString.append(m_lineBreakChars);
        tupleString.append(lineStart);
        if (isTupleEnd(lineStart))
        {
            return tupleString;
        }
        if (m_reader.eof() || util::isLdrStart(lineStart))
        {
            // LDR ended before end of last tuple
            m_reader.seekg(pos);
            throw ParseException(
                std::string{"No closing parenthesis found for "}
                    .append(ldrName)
                    .append(" entry: ")
                    .append(tupleString));
        }
    }
    throw ParseException(
        std::string{"File ended before closing parenthesis was found for "}
            .append(ldrName)
            .append(": ")
            .append(tupleString));
}

bool sciformats::jdx::util::MultilineTuplesParser::isTupleStart(
    const std::string& stringValue)
{
    std::string value{stringValue};
    util::trimLeft(value);
    return !value.empty() && value.at(0) == '(';
}

bool sciformats::jdx::util::MultilineTuplesParser::isTupleEnd(
    const std::string& stringValue)
{
    std::string value{stringValue};
    util::trimRight(value);
    return !value.empty() && value.back() == ')';
}
