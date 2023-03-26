#include "util/PeakTableParser.hpp"
#include "jdx/ParseException.hpp"
#include "jdx/Peak.hpp"
#include "util/LdrUtils.hpp"
#include "util/StringUtils.hpp"

#include <algorithm>
#include <regex>

sciformats::jdx::util::PeakTableParser::PeakTableParser(
    TextReader& reader, std::string variableList)
    : TuplesParser(std::move(variableList), "peak table")
    , m_reader{reader}
{
}

std::optional<sciformats::jdx::Peak>
sciformats::jdx::util::PeakTableParser::next()
{
    return TuplesParser::next<Peak>([this]() { return nextTuple(); },
        [this](const std::string& tuple) { return createPeak(tuple); });
    // alternative without lambdas:
    // return TuplesParser::next<Peak>(
    //    std::bind(&PeakTableParser::nextTuple, this),
    //    std::bind(&PeakTableParser::createPeak, this, std::placeholders::_1));
}

std::optional<std::string> sciformats::jdx::util::PeakTableParser::nextTuple()
{
    while (m_tuples.empty())
    {
        if (m_reader.eof())
        {
            return std::nullopt;
        }

        auto pos = m_reader.tellg();
        auto nextLine = m_reader.readLine();
        if (util::isLdrStart(nextLine))
        {
            // next LDR => end of PEAK TABLE
            m_reader.seekg(pos);
            return std::nullopt;
        }

        auto [value, _] = util::stripLineComment(nextLine, true);
        if (value.empty())
        {
            // skip pure comments
            continue;
        }
        auto tuples
            = util::split(value, R"([^,\s](\s*(?:\s|;)\s*)[^,\s])", true, 1);
        if (tuples.empty())
        {
            throw ParseException(
                "Unexpected content found while parsing PEAK TABLE: "
                + nextLine);
        }
        // add to queue
        for (auto& tuple : tuples)
        {
            m_tuples.push(tuple);
        }
    }

    auto tuple = m_tuples.front();
    m_tuples.pop();
    return tuple;
}

sciformats::jdx::Peak sciformats::jdx::util::PeakTableParser::createPeak(
    const std::string& tuple) const
{
    // tokenize
    // token[0] is the full match so extract 1 + 3 tokens for 3 groups
    const auto tokens = extractTokens(tuple, m_regex, 1 + 3);
    const auto& varList = getVariableList();

    // error conditions {varList, {error condition, error message}}
    const std::multimap<std::string, std::tuple<bool, std::string>> errorMap{
        {s_varLists.at(0),
            {tokens.at(3).has_value(), std::string{"Illegal "} + s_ldrName
                                           + " entry for " + s_varLists.at(0)
                                           + ": " + tuple}},
        {s_varLists.at(1),
            {!tokens.at(3), std::string{"Illegal "} + s_ldrName + " entry for "
                                + s_varLists.at(1) + ": " + tuple}},
        {s_varLists.at(2),
            {!tokens.at(3), std::string{"Illegal "} + s_ldrName + " entry for "
                                + s_varLists.at(2) + ": " + tuple}},
    };

    checkForErrors(varList, errorMap, s_ldrName);

    // map
    Peak peak{};
    peak.x = parseDoubleToken(tokens.at(1));
    peak.y = parseDoubleToken(tokens.at(2));
    // nothing to do for varLists[0]
    if (s_varLists.at(1) == varList)
    {
        peak.w = parseDoubleToken(tokens.at(3));
    }
    else if (s_varLists.at(2) == varList)
    {
        peak.m = tokens.at(3);
    }

    return peak;
}
