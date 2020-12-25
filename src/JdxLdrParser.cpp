#include "jdx/JdxLdrParser.hpp"

#include <algorithm>
#include <string>

std::string sciformats::jdx::JdxLdrParser::readLine(std::istream& istream)
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
    throw std::runtime_error("Error reading line from istream.");
}

void sciformats::jdx::JdxLdrParser::trimLeft(std::string& s)
{
    s.erase(s.begin(), std::find_if(s.begin(), s.end(), [](unsigned char ch) {
        return !static_cast<bool>(std::isspace(ch));
    }));
}

void sciformats::jdx::JdxLdrParser::trimRight(std::string& s)
{
    s.erase(std::find_if(s.rbegin(), s.rend(),
                [](unsigned char ch) {
                    return !static_cast<bool>(std::isspace(ch));
                })
                .base(),
        s.end());
}

void sciformats::jdx::JdxLdrParser::trim(std::string& s)
{
    trimRight(s);
    trimLeft(s);
}
