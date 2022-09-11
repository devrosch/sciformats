#include "util/PeakAssignmentsParser.hpp"
#include "jdx/ParseException.hpp"
#include "jdx/PeakAssignment.hpp"
#include "util/LdrUtils.hpp"
#include "util/StringUtils.hpp"

#include <algorithm>
#include <regex>

sciformats::jdx::util::PeakAssignmentsParser::PeakAssignmentsParser(
    TextReader& reader, size_t numVariables)
    : m_reader{reader}
    , m_numVariables{numVariables}
{
}

sciformats::jdx::PeakAssignment
sciformats::jdx::util::PeakAssignmentsParser::next()
{
    auto nextAssignmentString = nextTuple();
    if (!nextAssignmentString)
    {
        throw ParseException("No next peak assignment found at: "
                             + std::to_string(m_reader.tellg()));
    }
    auto nextAssignment = createPeakAssignment(nextAssignmentString.value());
    return nextAssignment;
}

bool sciformats::jdx::util::PeakAssignmentsParser::hasNext()
{
    if (m_reader.eof())
    {
        return false;
    }
    auto readerPos = m_reader.tellg();
    auto nextAssignmentString = nextTuple();
    // TODO: optimize
    m_reader.seekg(readerPos);
    return nextAssignmentString.has_value();
}

std::optional<std::string>
sciformats::jdx::util::PeakAssignmentsParser::nextTuple()
{
    std::string peakAssignmentString{};
    // find start
    while (!m_reader.eof())
    {
        std::streampos pos = m_reader.tellg();
        auto line = m_reader.readLine();
        auto [lineStart, comment] = util::stripLineComment(line);
        util::trim(lineStart);
        if (isTupleStart(lineStart))
        {
            peakAssignmentString.append(lineStart);
            break;
        }
        if (util::isLdrStart(lineStart))
        {
            // PEAKASSIGNMENT LDR ended, no peak assignments
            m_reader.seekg(pos);
            return std::nullopt;
        }
        if (!lineStart.empty())
        {
            throw ParseException(
                "Illegal string found in peak assignment: " + line);
        }
    }
    if (isTupleEnd(peakAssignmentString))
    {
        return peakAssignmentString;
    }
    // read to end of current peak assignment
    while (!m_reader.eof())
    {
        std::streampos pos = m_reader.tellg();
        auto line = m_reader.readLine();
        auto [lineStart, comment] = util::stripLineComment(line);
        util::trim(lineStart);

        if (util::isLdrStart(lineStart))
        {
            // PEAKASSIGNMENT LDR ended before end of last peak assignment
            m_reader.seekg(pos);
            throw ParseException(
                "No closing parenthesis found for peak assignment: "
                + peakAssignmentString);
        }
        peakAssignmentString.append(" ");
        peakAssignmentString.append(lineStart);
        if (isTupleEnd(lineStart))
        {
            return peakAssignmentString;
        }
        if (m_reader.eof() || util::isLdrStart(lineStart))
        {
            // PEAKASSIGNMENT LDR ended before end of last peak assignment
            m_reader.seekg(pos);
            throw ParseException(
                "No closing parenthesis found for peak assignment: "
                + peakAssignmentString);
        }
    }
    throw ParseException(
        "File ended before closing parenthesis was found for peak assignment: "
        + peakAssignmentString);
}

bool sciformats::jdx::util::PeakAssignmentsParser::isTupleStart(
    const std::string& stringValue)
{
    std::string value{stringValue};
    util::trimLeft(value);
    return !value.empty() && value.at(0) == '(';
}

bool sciformats::jdx::util::PeakAssignmentsParser::isTupleEnd(
    const std::string& stringValue)
{
    std::string value{stringValue};
    util::trimRight(value);
    return !value.empty() && value.back() == ')';
}

sciformats::jdx::PeakAssignment
sciformats::jdx::util::PeakAssignmentsParser::createPeakAssignment(
    const std::string& stringValue) const
{
    // matches 2 - 5 peak assignments segments  as groups 1-5, corresponding to
    // one of (X[, Y][, W], A), (X[, Y][, M], A), (X[, Y][, M][, W], A), with X
    // as matches[1] and A as matches[5]
    const auto* regexString = R"(^\s*\(\s*)"
                              R"(([^,]*))"
                              R"((?:\s*,\s*([^,]*))?)"
                              R"((?:\s*,\s*([^,]*))?)"
                              R"((?:\s*,\s*([^,]*))?)"
                              R"(\s*,\s*(<.*>)\s*\))"
                              R"(\s*$)";
    std::regex regex{regexString};
    std::smatch matches;
    auto [lineStart, comment] = util::stripLineComment(stringValue);
    util::trim(lineStart);
    if (!std::regex_match(lineStart, matches, regex)
        || (m_numVariables <= 3 && (matches[3].matched || matches[4].matched))
        || (m_numVariables <= 4 && matches[4].matched) || !matches[5].matched)
    {
        throw ParseException("Illegal peak assignment string: " + stringValue);
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
    auto token4 = matches[4].matched
                      ? std::optional<std::string>{matches[4].str()}
                      : std::nullopt;
    auto token5 = matches[5].matched
                      ? std::optional<std::string>{matches[5].str()}
                      : std::nullopt;
    // remove enclosing < and >
    token5 = token5.value().substr(1, token5.value().size() - 2);

    auto parseDoubleToken = [](const std::optional<std::string>& token) {
        return token.value().empty() ? std::numeric_limits<double>::quiet_NaN()
                                     : strtod(token.value().data(), nullptr);
    };

    // map to peak assignment
    PeakAssignment peakAssignment{};
    peakAssignment.x = parseDoubleToken(token1);
    peakAssignment.a = token5.value();
    if (m_numVariables == 3)
    {
        if (token2)
        {
            // 3 tokens
            peakAssignment.y = parseDoubleToken(token2);
        }
    }
    else if (m_numVariables == 4)
    {
        // 2, 3 or 4 tokens
        if (token2 && token3)
        {
            peakAssignment.y = parseDoubleToken(token2);
            peakAssignment.w = parseDoubleToken(token3);
        }
        else if (token2)
        {
            // 3 tokens
            throw ParseException("Ambiguous peak assignment (second "
                                 "variable Y or W) for four variables: "
                                 + lineStart);
        }
    }
    else
    {
        throw ParseException("Unsupported number of variables: "
                             + std::to_string(m_numVariables));
    }
    return peakAssignment;
}
