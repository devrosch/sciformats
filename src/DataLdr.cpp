#include "jdx/DataLdr.hpp"
#include "jdx/ParseException.hpp"
#include "util/LdrUtils.hpp"

sciformats::jdx::DataLdr::DataLdr(
    std::string label, std::string variableList, std::istream& istream)
    : Ldr{std::move(label)}
    , m_variableList{std::move(variableList)}
    , m_istream{istream}
    , m_streamDataPos{istream.tellg()}
{
}

const std::string& sciformats::jdx::DataLdr::getVariableList() const
{
    return m_variableList;
}

std::istream& sciformats::jdx::DataLdr::getStream()
{
    return m_istream;
}

void sciformats::jdx::DataLdr::skipToNextLdr(std::istream& iStream)
{
    while (!iStream.eof())
    {
        std::istream::pos_type pos = iStream.tellg();
        std::string line = util::readLine(iStream);
        if (util::isLdrStart(line))
        {
            // move back to start of LDR
            iStream.seekg(pos);
            break;
        }
    }
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
