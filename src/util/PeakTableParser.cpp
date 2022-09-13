#include "util/PeakTableParser.hpp"
#include "jdx/ParseException.hpp"
#include "jdx/Peak.hpp"
#include "util/LdrUtils.hpp"
#include "util/StringUtils.hpp"

#include <algorithm>
#include <regex>

sciformats::jdx::util::PeakTableParser::PeakTableParser(
    TextReader& reader, std::string variableList)
    : m_reader{reader}
    , m_variableList{std::move(variableList)}
{
}

std::optional<sciformats::jdx::Peak>
sciformats::jdx::util::PeakTableParser::next()
{
    auto nextString = nextTuple();
    if (!nextString)
    {
        return std::nullopt;
    }
    auto nextPeak = createPeak(nextString.value());
    return nextPeak;
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

        auto [value, comment] = util::stripLineComment(nextLine);
        util::trim(value);
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
    // matches 2-3 peak segments as groups 1-3, corresponding to
    // (XY..XY), (XYW..XYW), or (XYM..XYM), with X as matches[1], Y as matche[2]
    // and W or M as matches[3]
    const auto* regexString = R"(^\s*)"
                              R"(([^,]*))"
                              R"((?:\s*,\s*([^,]*)))"
                              R"((?:\s*,\s*([^,]*))?)"
                              R"($)";
    std::regex regex{regexString};
    std::smatch matches;
    auto [lineStart, comment] = util::stripLineComment(tuple);
    util::trim(lineStart);
    if (!std::regex_match(lineStart, matches, regex))
    {
        throw ParseException("Illegal peak string: " + tuple);
    }

    auto token1 = matches[1].matched
                      ? std::optional<std::string>{matches[1].str()}
                      : std::nullopt;
    auto token2 = matches[2].matched
                      ? std::optional<std::string>{matches[2].str()}
                      : std::nullopt;
    auto token3 = matches[3].matched
                      ? std::optional<std::string>{matches[3].str()}
                      : std::nullopt;

    auto parseDoubleToken = [](const std::optional<std::string>& token) {
        return token.value().empty() ? std::numeric_limits<double>::quiet_NaN()
                                     : strtod(token.value().data(), nullptr);
    };

    // map to peak
    Peak peak{};
    peak.x = parseDoubleToken(token1);
    peak.y = parseDoubleToken(token2);
    if ("(XY..XY)" == m_variableList)
    {
        if (token3)
        {
            throw ParseException(
                "Illegal peak component for (XY..XY): " + lineStart);
        }
    }
    else if ("(XYW..XYW)" == m_variableList)
    {
        peak.w = parseDoubleToken(token3);
    }
    else if ("(XYM..XYM)" == m_variableList)
    {
        peak.m = token3;
    }
    else
    {
        throw ParseException(
            "Unsupported variable list for peak table: " + m_variableList);
    }

    return peak;
}
