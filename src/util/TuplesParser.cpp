#include "util/TuplesParser.hpp"
#include "jdx/ParseException.hpp"
#include "util/LdrUtils.hpp"
#include "util/StringUtils.hpp"

sciformats::jdx::util::TuplesParser::TuplesParser(
    std::string variableList, std::string ldrName)
    : m_variableList{std::move(variableList)}
    , m_ldrName{std::move(ldrName)}
{
}

const std::string& sciformats::jdx::util::TuplesParser::getVariableList() const
{
    return m_variableList;
}

const std::string& sciformats::jdx::util::TuplesParser::getLdrName() const
{
    return m_ldrName;
}

std::vector<std::optional<std::string>>
sciformats::jdx::util::TuplesParser::extractTokens(
    const std::string& tuple, const std::regex& regex, size_t numTokens) const
{
    auto [noCommentTuple, _] = util::stripLineComment(tuple, true);
    std::smatch matches = match(noCommentTuple, regex);

    std::vector<std::optional<std::string>> tokens;
    for (size_t i = 0; i < numTokens; ++i)
    {
        auto token = extractToken(matches, i);
        tokens.push_back(token);
    }
    return tokens;
}

void sciformats::jdx::util::TuplesParser::checkForErrors(
    const std::string& varList,
    const std::multimap<std::string, std::tuple<bool, std::string>>& errorMap,
    const std::string& ldrName)
{
    if (errorMap.count(varList) == 0)
    {
        throw ParseException(std::string{"Unsupported variable list for "}
                             + ldrName + ": " + varList);
    }
    auto errorConditions = errorMap.equal_range(varList);
    for (auto it = errorConditions.first; it != errorConditions.second; ++it)
    {
        bool errorCondition = std::get<0>(it->second);
        if (errorCondition)
        {
            const auto& errorMessage = std::get<1>(it->second);
            throw ParseException(errorMessage);
        }
    }
}

double sciformats::jdx::util::TuplesParser::parseDoubleToken(
    const std::optional<std::string>& token)
{
    return token.value().empty() ? std::numeric_limits<double>::quiet_NaN()
                                 : strtod(token.value().data(), nullptr);
}

std::optional<std::string> sciformats::jdx::util::TuplesParser::extractToken(
    const std::smatch& matches, size_t index)
{
    return matches[index].matched
               ? std::optional<std::string>{matches[index].str()}
               : std::nullopt;
}

std::smatch sciformats::jdx::util::TuplesParser::match(
    const std::string& tuple, const std::regex& regex) const
{
    std::smatch matches;
    if (!std::regex_match(tuple, matches, regex))
    {
        throw ParseException("Illegal " + m_ldrName + " tuple: " + tuple);
    }
    return matches;
}
