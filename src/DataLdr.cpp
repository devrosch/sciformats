#include "jdx/DataLdr.hpp"
#include "jdx/ParseException.hpp"
#include "util/LdrUtils.hpp"
#include "util/StringUtils.hpp"

sciformats::jdx::DataLdr::DataLdr(
    std::string label, std::string variableList, TextReader& reader)
    : Ldr{std::move(label)}
    , m_variableList{std::move(variableList)}
    , m_reader{reader}
    , m_dataPos{reader.tellg()}
{
}

const std::string& sciformats::jdx::DataLdr::getVariableList() const
{
    return m_variableList;
}

sciformats::jdx::TextReader& sciformats::jdx::DataLdr::getReader()
{
    return m_reader;
}

void sciformats::jdx::DataLdr::validateInput(const std::string& label,
    const std::string& variableList, const std::string& expectedLabel,
    const std::vector<std::string>& expectedVariableLists)
{
    if (label != expectedLabel)
    {
        throw ParseException("Illegal label at " + expectedLabel
                             + " start encountered: " + label);
    }
    if (std::none_of(expectedVariableLists.begin(), expectedVariableLists.end(),
            [&variableList](const std::string& expectedVariableList) {
                return variableList == expectedVariableList;
            }))
    {
        throw ParseException("Illegal variable list for " + label
                             + " encountered: " + variableList);
    }
}
