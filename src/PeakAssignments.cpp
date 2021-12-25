#include "jdx/PeakAssignments.hpp"
#include "jdx/LdrParser.hpp"

#include <algorithm>
#include <istream>
#include <tuple>

// TODO: duplicate of constructor in PeakTable
sciformats::jdx::PeakAssignments::PeakAssignments(std::istream& istream)
    : m_istream{istream}
    , m_streamDataPos{istream.tellg()}
{
    std::tie(m_label, m_variableList) = readFirstLine(istream);
    m_streamDataPos = istream.tellg();
    validateInput(m_label, m_variableList, s_peakAssignentsLabel,
                  std::vector<std::string>{
                      s_peakAssignentsXyaVariableList, s_peakAssignentsXywaVariableList});
    skipToNextLdr(istream);
}

// TODO: duplicate of constructor in PeakTable
sciformats::jdx::PeakAssignments::PeakAssignments(
        std::string label, std::string variableList, std::istream& istream)
    : m_istream{istream}
    , m_streamDataPos{istream.tellg()}
    , m_label{std::move(label)}
    , m_variableList{std::move(variableList)}
{
    validateInput(m_label, m_variableList, s_peakAssignentsLabel,
                  std::vector<std::string>{
                      s_peakAssignentsXyaVariableList, s_peakAssignentsXywaVariableList});
    skipToNextLdr(istream);
}

// TODO: duplicate of skipToNextLdr() in PeakTable
void sciformats::jdx::PeakAssignments::skipToNextLdr(std::istream& iStream)
{
    while (!iStream.eof())
    {
        std::istream::pos_type pos = iStream.tellg();
        std::string line = sciformats::jdx::LdrParser::readLine(iStream);
        if (sciformats::jdx::LdrParser::isLdrStart(line))
        {
            // move back to start of LDR
            iStream.seekg(pos);
            break;
        }
    }
}

// TODO: duplicate of readFirstLine() in PeakTable
std::pair<std::string, std::string>
sciformats::jdx::PeakAssignments::readFirstLine(std::istream& istream)
{
    auto pos = istream.tellg();
    auto line = LdrParser::readLine(istream);
    if (!LdrParser::isLdrStart(line))
    {
        // reset for consistent state
        istream.seekg(pos);
        throw std::runtime_error(
                    "Cannot parse PEAK TABLE. Stream position not at LDR start: "
                    + line);
    }
    auto [label, variableList] = LdrParser::parseLdrStart(line);
            LdrParser::stripLineComment(variableList);
            LdrParser::trim(variableList);

            return {label, variableList};
}

// TODO: duplicate of validateInput() in PeakTable
void sciformats::jdx::PeakAssignments::validateInput(const std::string& label,
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

// TODO: duplicate of getKernel() in PeakTable
std::optional<std::string> sciformats::jdx::PeakAssignments::getWidthFunction()
{
    // comment $$ in line(s) following LDR start may contain peak function
    auto streamPos = m_istream.eof()
            ? std::nullopt
            : std::optional<std::streampos>(m_istream.tellg());
    try
    {
        m_istream.seekg(m_streamDataPos);
        std::string line;
        std::string kernelFunctionsDescription{};
        while (!m_istream.eof()
               && !sciformats::jdx::LdrParser::isLdrStart(
                   line = sciformats::jdx::LdrParser::readLine(m_istream)))
        {
            auto [content, comment] = LdrParser::stripLineComment(line);
                    LdrParser::trim(content);
            if (content.empty() && comment.has_value())
            {
                if (!kernelFunctionsDescription.empty())
                {
                    kernelFunctionsDescription += '\n';
                }
                LdrParser::trim(comment.value());
                kernelFunctionsDescription.append(comment.value());
            }
            else
            {
                break;
            }
        }
        if (streamPos)
        {
            m_istream.seekg(streamPos.value());
        }
        if (!kernelFunctionsDescription.empty())
        {
            return kernelFunctionsDescription;
        }
        return std::nullopt;
    }
    catch (...)
    {
        // TODO: duplicate code in Data2D
        try
        {
            if (streamPos)
            {
                m_istream.seekg(streamPos.value());
            }
        }
        catch (...)
        {
        }
        throw;
    }
}

std::vector<sciformats::jdx::PeakAssignment> sciformats::jdx::PeakAssignments::getData()
{
    // remember stream position
    auto streamPos = m_istream.eof()
            ? std::nullopt
            : std::optional<std::streampos>(m_istream.tellg());
    try
    {
        std::vector<sciformats::jdx::PeakAssignment> peakAssignments{};
        m_istream.seekg(m_streamDataPos);
        while (true)
        {
            auto assignmentString = readNextAssignmentString(m_istream);
            if (!assignmentString)
            {
                break;
            }
            auto numComponents
                = m_variableList == s_peakAssignentsXyaVariableList ? 3U : 4U;
            PeakAssignment assignment{assignmentString.value(), numComponents};
            peakAssignments.push_back(assignment);
        }
        // reset stream
        if (streamPos)
        {
            m_istream.seekg(streamPos.value());
        }
        return peakAssignments;
    }
    catch (...)
    {
        // TODO: duplicate code in Data2D
        try
        {
            // reset stream
            if (streamPos)
            {
                m_istream.seekg(streamPos.value());
            }
        }
        catch (...)
        {
        }
        throw;
    }
}

std::optional<std::string> sciformats::jdx::PeakAssignments::readNextAssignmentString(std::istream& istream)
{
    std::string peakAssignmentString{};
    // find start
    while (!istream.eof())
    {
        std::streampos pos = istream.tellg();
        auto line = LdrParser::readLine(istream);
        auto [lineStart, comment] = LdrParser::stripLineComment(line);
        LdrParser::trim(lineStart);
        if (PeakAssignment::isPeakAssignmentStart(lineStart))
        {
            peakAssignmentString.append(lineStart);
            break;
        }
        if (LdrParser::isLdrStart(lineStart))
        {
            // PEAKASSIGNMENT LDR ended, no peak assignments
            istream.seekg(pos);
            return std::nullopt;
        }
    }
    if (PeakAssignment::isPeakAssignmentEnd(peakAssignmentString))
    {
        return peakAssignmentString;
    }
    // read to end of current peak assignment
    while (!istream.eof())
    {
        std::streampos pos = istream.tellg();
        auto line = LdrParser::readLine(istream);
        auto [lineStart, comment] = LdrParser::stripLineComment(line);
        LdrParser::trim(lineStart);
        if (PeakAssignment::isPeakAssignmentEnd(lineStart))
        {
            peakAssignmentString.append(lineStart);
            break;
        }
        if (istream.eof() || LdrParser::isLdrStart(lineStart))
        {
            // PEAKASSIGNMENT LDR ended before end of last peak assignment
            istream.seekg(pos);
            throw std::runtime_error(
                        "No closing parenthesis found for peak assignment: "
                        + peakAssignmentString);
        }
    }
    return peakAssignmentString;
}
