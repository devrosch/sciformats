#include "jdx/DataLdr.hpp"
#include "jdx/util/LdrUtils.hpp"
#include "jdx/Peak.hpp"
#include "jdx/PeakAssignment.hpp"

// TODO: duplicate of constructor in Data2D
sciformats::jdx::DataLdr::DataLdr(std::istream& istream)
    : m_istream{istream}
    , m_streamDataPos{istream.tellg()}
{
    std::tie(m_label, m_variableList) = readFirstLine(istream);
    m_streamDataPos = istream.tellg();
}

// TODO: duplicate of constructor in Data2D
sciformats::jdx::DataLdr::DataLdr(
    std::string label, std::string variableList, std::istream& istream)
    : m_istream{istream}
    , m_streamDataPos{istream.tellg()}
    , m_label{std::move(label)}
    , m_variableList{std::move(variableList)}
{
}

// TODO: duplicate of skipToNextLdr() in Data2D
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

// TODO: duplicate of readFirstLine() in Data2D
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
            "Cannot parse data. Stream position not at LDR start: "
            + line);
    }
    auto [label, variableList] = util::parseLdrStart(line);
    util::stripLineComment(variableList);
    util::trim(variableList);

    return {label, variableList};
}
