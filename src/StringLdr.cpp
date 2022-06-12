#include "jdx/StringLdr.hpp"

sciformats::jdx::StringLdr::StringLdr(std::string label, std::string value)
    : Ldr{std::move(label)}
    , m_value{std::move(value)}
{
}

const std::string& sciformats::jdx::StringLdr::getValue() const
{
    return m_value;
}
