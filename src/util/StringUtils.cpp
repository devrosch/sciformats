#include "util/StringUtils.hpp"
#include "jdx/ParseException.hpp"

#include <algorithm>
#include <string>

std::string sciformats::jdx::util::readLine(std::istream& istream)
{
    std::string out{};
    if (std::getline(istream, out))
    {
        if (!out.empty() && out.at(out.size() - 1) == '\r')
        {
            // remove trailing \r in case line ending is \r\n and has not been
            // converted to \n by stream already
            out.erase(out.size() - 1);
        }
        return out;
    }
    throw ParseException("Error reading line from istream.");
}

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
