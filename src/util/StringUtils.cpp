#include "util/StringUtils.hpp"
#include "jdx/ParseException.hpp"

#include <algorithm>
#include <regex>
#include <string>

void sciformats::jdx::util::trimLeft(std::string& s)
{
    s.erase(s.begin(), std::find_if(s.begin(), s.end(), [](unsigned char ch) {
        return !static_cast<bool>(std::isspace(ch));
    }));
}

void sciformats::jdx::util::trimRight(std::string& s)
{
    s.erase(std::find_if(s.rbegin(), s.rend(),
                [](unsigned char ch) {
                    return !static_cast<bool>(std::isspace(ch));
                })
                .base(),
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
    const std::string& delimiterRegEx, bool trimSegments)
{
    // see:
    // https://en.cppreference.com/w/cpp/regex/regex_token_iterator
    // https://stackoverflow.com/questions/9435385/split-a-string-using-c11
    std::regex delimiter{delimiterRegEx};
    std::sregex_token_iterator first{input.begin(), input.end(), delimiter, -1};
    std::sregex_token_iterator last;
    std::vector<std::string> output{first, last};

    // number of matches of delimiter
    // see: https://stackoverflow.com/a/36320911
    std::ptrdiff_t numMatches = std::distance(
        std::sregex_iterator(input.begin(), input.end(), delimiter),
        std::sregex_iterator());

    if (numMatches >= 0 && output.size() == static_cast<size_t>(numMatches))
    {
        // if input ends on delimiter, include empty trailing segment
        output.emplace_back("");
    }

    if (trimSegments)
    {
        std::for_each(output.begin(), output.end(), trim);
    }
    return output;
}
