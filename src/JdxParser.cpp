#include "jdx/JdxParser.hpp"

#include <array>
#include <climits>
#include <cstring>
#include <limits>
//#include <filesystem>
#include <algorithm>

bool sciformats::jdx::JdxParser::canParse(
    const std::string& filePath, std::istream& inputStream)
{
    // check extension
    // TODO: in the future use
    // https://en.cppreference.com/w/cpp/filesystem/path/extension
    // TODO: check more extensions
    std::string extension{"jdx"};
    std::string lowerCasePath{filePath};
    // std::tolower has undefined behavior for signed chars
    std::transform(lowerCasePath.begin(), lowerCasePath.end(),
        lowerCasePath.begin(), [](unsigned char c) { return std::tolower(c); });
    bool correctExtension = false;
    if (lowerCasePath.length() >= extension.length())
    {
        correctExtension = (lowerCasePath.compare(
                                lowerCasePath.length() - extension.length(),
                                extension.length(), extension)
                            == 0);
    }
    if (!correctExtension)
    {
        return false;
    }

    // check magic bytes
    std::ios::pos_type position = inputStream.tellg();
    std::string magic{"##TITLE="};
    bool match = true;
    for (auto i{0}; i < magic.size(); i++)
    {
        // TODO: label should be normalized before comparison
        if (inputStream.eof() || magic.at(i) != inputStream.get())
        {
            match = false;
            break;
        }
    }
    inputStream.seekg(position, std::ios_base::beg);
    return match;
}

sciformats::jdx::JdxBlock sciformats::jdx::JdxParser::parse(
    std::istream& inputStream, bool activateExceptions)
{
    if (activateExceptions)
    {
        inputStream.exceptions(
            std::ios::eofbit | std::ios::failbit | std::ios::badbit);
    }
    sciformats::jdx::JdxBlock block{inputStream};
    return block;
}
