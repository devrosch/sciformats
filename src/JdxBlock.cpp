#include "jdx/JdxBlock.hpp"

#include <array>
#include <climits>
#include <cstring>
#include <limits>

sciformats::jdx::JdxBlock::JdxBlock(const std::string& filePath)
    : m_ifstream{std::ifstream{}}
    , m_istream{m_ifstream.value()}
{
    m_ifstream.value().exceptions(
        std::ios::eofbit | std::ios::failbit | std::ios::badbit);
    m_ifstream.value().open(filePath, std::ios::in | std::ios::binary);
    // TODO: parse
}

sciformats::jdx::JdxBlock::JdxBlock(std::istream& inputStream)
    : m_ifstream{std::nullopt}
    , m_istream{inputStream}
{
    // TODO: parse
}
