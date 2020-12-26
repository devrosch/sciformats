#include "jdx/JdxLdrParser.hpp"

#include <algorithm>
#include <regex>
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

bool sciformats::jdx::JdxLdrParser::isLdrStart(const std::string& line)
{
    std::regex regex{"^\\s*##.*=.*"};
    return std::regex_match(line, regex);
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

std::string sciformats::jdx::JdxLdrParser::normalizeLdrLabel(
    const std::string& ldr)
{
    std::string output{};
    auto it = ldr.cbegin();
    // skip leading white spaces
    for (; it != ldr.cend(); ++it)
    {
        if (!static_cast<bool>(std::isspace(*it)))
        {
            break;
        }
    }
    // check and skip "##" marking start of LDR
    for (auto i{0}; i < 2; i++)
    {
        if (it == ldr.cend() || *it != '#')
        {
            throw std::runtime_error(
                std::string{"Malformed LDR start, missing double hashes: "}
                + ldr);
        }
        output += *(it++);
    }
    // normalize label
    auto makeUpperCase = [](const unsigned char c) { return std::toupper(c); };
    for (; it != ldr.cend(); ++it)
    {
        const char c = *it;
        if (c == '=')
        {
            // end of label
            break;
        }
        if (c == ' ' || c == '-' || c == '/' || c == '_')
        {
            // discard
            continue;
        }
        output += static_cast<char>(
            makeUpperCase(static_cast<const unsigned char>(c)));
    }
    if (*it != '=')
    {
        throw std::runtime_error(
            std::string{"Malformed LDR start, missing equals: "} + ldr);
    }
    // add remaining string content
    output.append(it, ldr.end());
    return output;
}
