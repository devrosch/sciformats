#include "jdx/DataLdr.hpp"
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

std::streampos& sciformats::jdx::DataLdr::getStreamPos()
{
    return m_streamDataPos;
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

std::pair<std::string, std::string> sciformats::jdx::DataLdr::readFirstLine(
    std::istream& istream)
{
    auto pos = istream.tellg();
    auto line = util::readLine(istream);
    if (!util::isLdrStart(line))
    {
        // reset for consistent state
        istream.seekg(pos);
        throw std::runtime_error(
            "Cannot parse data. Stream position not at LDR start: " + line);
    }
    auto [label, variableList] = util::parseLdrStart(line);
    util::stripLineComment(variableList);
    util::trim(variableList);

    return {label, variableList};
}

void sciformats::jdx::DataLdr::validateInput(const std::string& label,
    const std::string& variableList, const std::string& expectedLabel,
    const std::vector<std::string>& expectedVariableLists)
{
    if (label != expectedLabel)
    {
        throw std::runtime_error("Illegal label at " + expectedLabel
                                 + " start encountered: " + label);
    }
    if (std::none_of(expectedVariableLists.begin(), expectedVariableLists.end(),
            [&variableList](const std::string& expectedVariableList) {
                return variableList == expectedVariableList;
            }))
    {
        throw std::runtime_error("Illegal variable list for " + label
                                 + " encountered: " + variableList);
    }
}
