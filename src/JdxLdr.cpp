#include "jdx/JdxLdr.hpp"

sciformats::jdx::JdxLdr::JdxLdr(const std::string& label)
{
    m_label = label;
}

sciformats::jdx::JdxLdr::JdxLdr(
    const std::string& label, const std::string& value)
    : JdxLdr(label)
{
    m_value = value;
}

void sciformats::jdx::JdxLdr::addValueLine(const std::string& line)
{
    m_value += "\n" + line;
}

const std::string& sciformats::jdx::JdxLdr::getLabel() const
{
    return m_label;
}

const std::string& sciformats::jdx::JdxLdr::getValue() const
{
    return m_value;
}
