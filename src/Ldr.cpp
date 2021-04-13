#include "jdx/Ldr.hpp"

sciformats::jdx::Ldr::Ldr(const std::string& label, const std::string& value)
{
    m_label = label;
    m_value = value;
}

const std::string& sciformats::jdx::Ldr::getLabel() const
{
    return m_label;
}

const std::string& sciformats::jdx::Ldr::getValue() const
{
    return m_value;
}

bool sciformats::jdx::Ldr::isUserDefined() const
{
    return !m_label.empty() && m_label.at(0) == '$';
}

bool sciformats::jdx::Ldr::isTechniqueSpecific() const
{
    return !m_label.empty() && m_label.at(0) == '.';
}
