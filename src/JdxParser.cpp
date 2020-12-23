#include "jdxparser/JdxParser.hpp"

#include <array>
#include <climits>
#include <cstring>
#include <limits>

sciformats::jdx::JdxParser::JdxParser(const std::string& filePath)
    : m_ifstream{std::ifstream{}}
    , m_istream{m_ifstream.value()}
{
    m_ifstream.value().exceptions(
        std::ios::eofbit | std::ios::failbit | std::ios::badbit);
    m_ifstream.value().open(filePath, std::ios::in | std::ios::binary);
}

sciformats::jdx::JdxParser::JdxParser(
    std::istream& inputStream, bool activateExceptions)
    : m_ifstream{std::nullopt}
    , m_istream{inputStream}
{
    if (activateExceptions)
    {
        // this also activates exceptions on input_stream, as as m_istream is
        // a reference to input_stream
        m_istream.exceptions(
            std::ios::eofbit | std::ios::failbit | std::ios::badbit);
    }
}
