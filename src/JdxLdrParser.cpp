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

std::string sciformats::jdx::JdxLdrParser::normalizeLdrStart(
    const std::string& ldr)
{
    std::string output{};
    auto it = ldr.cbegin();
    // skip leading white spaces
    for (; it != ldr.cend(); ++it)
    {
        if (!static_cast<bool>(std::isspace(static_cast<unsigned char>(*it))))
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
    std::string label{};
    while (it != ldr.cend() && *it != '=')
    {
        label += *it++;
    }
    output.append(normalizeLdrLabel(label));
    // add remaining string content
    if (it == ldr.cend() || *it != '=')
    {
        throw std::runtime_error(
            std::string{"Malformed LDR start, missing equals: "} + ldr);
    }
    output.append(it, ldr.end());
    return output;
}

std::string sciformats::jdx::JdxLdrParser::normalizeLdrLabel(
    const std::string& label)
{
    std::string output{};
    // normalize LDR label, i.e. the string between ## and =
    auto makeUpperCase = [](const unsigned char c) { return std::toupper(c); };
    for (const char c : label)
    {
        if (c == ' ' || c == '-' || c == '/' || c == '_')
        {
            // discard
            continue;
        }
        output += static_cast<char>(
            makeUpperCase(static_cast<const unsigned char>(c)));
    }
    return output;
}

std::pair<std::string, std::string>
sciformats::jdx::JdxLdrParser::parseLdrStart(const std::string& ldrStart)
{
    size_t posEquals = ldrStart.find('=');
    if (std::string::npos == posEquals)
    {
        throw std::runtime_error(
            std::string{"Malformed LDR start, missing equals: "} + ldrStart);
    }
    std::string label = ldrStart.substr(0, posEquals + 1);
    std::string normalizedLabel = normalizeLdrStart(label);
    if (normalizedLabel.size() < 3 || normalizedLabel.at(0) != '#'
        || normalizedLabel.at(1) != '#'
        || normalizedLabel.at(normalizedLabel.size() - 1) != '=')
    {
        throw std::runtime_error(
            std::string{
                "Malformed LDR start, normalization yields illegal label: "}
            + normalizedLabel);
    }
    // strip leading and trailing symbols from label
    normalizedLabel.erase(0, 2);
    normalizedLabel.erase(normalizedLabel.size() - 1);

    std::string value = ldrStart.substr(posEquals + 1);
    if (!value.empty() && value.at(0) == ' ')
    {
        // strip leading blank from value if present
        value.erase(0, 1);
    }

    return std::make_pair(normalizedLabel, value);
}

std::pair<std::string, std::optional<std::string>>
sciformats::jdx::JdxLdrParser::stripLineComment(const std::string& line)
{
    const auto pos = line.find("$$");
    if (pos == std::string::npos)
    {
        // no comment
        return make_pair(line, std::nullopt);
    }
    auto content = line.substr(0, pos);
    auto comment = line.substr(pos + 2);
    return make_pair(content, comment);
}

std::optional<const sciformats::jdx::JdxLdr>
sciformats::jdx::JdxLdrParser::findLdr(
    const std::vector<JdxLdr>& ldrs, const std::string& label)
{
    std::string normalizedLabel = normalizeLdrLabel(label);
    auto it = std::find_if(
        ldrs.begin(), ldrs.end(), [&normalizedLabel](const JdxLdr& ldr) {
            return ldr.getLabel() == normalizedLabel;
        });

    if (it != ldrs.end())
    {
        return *it;
    }
    return std::nullopt;
}

std::optional<std::string> sciformats::jdx::JdxLdrParser::findLdrValue(
    const std::vector<JdxLdr>& ldrs, const std::string& label)
{
    auto ldr = JdxLdrParser::findLdr(ldrs, label);
    return ldr.has_value() ? std::optional<std::string>(ldr.value().getValue())
                           : std::optional<std::string>(std::nullopt);
}
