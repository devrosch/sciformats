#include "util/StringUtils.hpp"
#include "jdx/ParseException.hpp"

#include <algorithm>
#include <functional>
#include <regex>
#include <string>

void sciformats::jdx::util::trimLeft(std::string& s)
{
    s.erase(s.begin(), std::find_if(s.begin(), s.end(), std::not_fn(isSpace)));
}

void sciformats::jdx::util::trimRight(std::string& s)
{
    s.erase(std::find_if(s.rbegin(), s.rend(), std::not_fn(isSpace)).base(),
        s.end());
}

void sciformats::jdx::util::trim(std::string& s)
{
    trimRight(s);
    trimLeft(s);
}

bool sciformats::jdx::util::isSpace(char c)
{
    return static_cast<bool>(std::isspace(static_cast<unsigned char>(c)));
}

void sciformats::jdx::util::toLower(std::string& s)
{
    std::transform(s.begin(), s.end(), s.begin(),
        [](unsigned char c) { return std::tolower(c); });
}

std::vector<std::string> sciformats::jdx::util::split(const std::string& input,
    const std::string& delimiterRegEx, bool trimSegments, size_t matchGroup)
{
    auto remainder = input;
    std::regex delimiter{delimiterRegEx};
    std::smatch match;
    std::vector<std::string> output;
    while (std::regex_search(remainder, match, delimiter))
    {
        auto matchPos = static_cast<size_t>(match.position(matchGroup));
        std::string segment = remainder.substr(0, matchPos);
        output.push_back(segment);
        auto nextPos
            = matchPos + static_cast<size_t>(match[matchGroup].length());
        remainder = remainder.substr(nextPos);
    }
    output.push_back(remainder);

    if (trimSegments)
    {
        std::for_each(output.begin(), output.end(), trim);
    }

    return output;
}
