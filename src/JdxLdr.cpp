#include "jdx/JdxLdr.hpp"

sciformats::jdx::JdxLdr::JdxLdr(
    const std::string& label, const std::string& value)
{
    m_label = label;
    m_value = value;
}

const std::string& sciformats::jdx::JdxLdr::getLabel() const
{
    return m_label;
}

const std::string& sciformats::jdx::JdxLdr::getValue() const
{
    return m_value;
}

bool sciformats::jdx::JdxLdr::isUserDefined() const
{
    return !m_label.empty() && m_label.at(0) == '$';
}

bool sciformats::jdx::JdxLdr::isTechniqueSpecific() const
{
    return !m_label.empty() && m_label.at(0) == '.';
}
