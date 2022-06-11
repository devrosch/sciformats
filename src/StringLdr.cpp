#include "jdx/StringLdr.hpp"

sciformats::jdx::StringLdr::StringLdr(
    const std::string& label, const std::string& value)
{
    m_label = label;
    m_value = value;
}

const std::string& sciformats::jdx::StringLdr::getLabel() const
{
    return m_label;
}

const std::string& sciformats::jdx::StringLdr::getValue() const
{
    return m_value;
}

bool sciformats::jdx::StringLdr::isUserDefined() const
{
    return !m_label.empty() && m_label.at(0) == '$';
}

bool sciformats::jdx::StringLdr::isTechniqueSpecific() const
{
    return !m_label.empty() && m_label.at(0) == '.';
}
