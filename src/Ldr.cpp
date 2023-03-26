#include "jdx/Ldr.hpp"

sciformats::jdx::Ldr::Ldr(std::string label)
    : m_label{std::move(label)}
{
}

const std::string& sciformats::jdx::Ldr::getLabel() const
{
    return m_label;
}

bool sciformats::jdx::Ldr::isUserDefined() const
{
    return !m_label.empty() && m_label.at(0) == '$';
}

bool sciformats::jdx::Ldr::isTechniqueSpecific() const
{
    return !m_label.empty() && m_label.at(0) == '.';
}
