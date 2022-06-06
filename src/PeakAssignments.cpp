#include "jdx/PeakAssignments.hpp"
#include "jdx/util/LdrUtils.hpp"
#include "jdx/util/PeakAssignmentsParser.hpp"

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
        std::string line = util::readLine(iStream);
        if (util::isLdrStart(line))
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
    auto line = util::readLine(istream);
    if (!util::isLdrStart(line))
    {
        // reset for consistent state
        istream.seekg(pos);
        throw std::runtime_error(
            "Cannot parse PEAK TABLE. Stream position not at LDR start: "
            + line);
    }
    auto [label, variableList] = util::parseLdrStart(line);
    util::stripLineComment(variableList);
    util::trim(variableList);

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
    // comment $$ in line(s) following LDR start may contain width function
    auto func = [&]() {
        std::optional<std::string> widthFunction{std::nullopt};
        m_istream.seekg(m_streamDataPos);
        auto numVariables
            = m_variableList == s_peakAssignentsXyaVariableList ? 3U : 4U;
        util::PeakAssignmentsParser parser{m_istream, numVariables};

        if (parser.hasNext())
        {
            auto nextVariant = parser.next();
            if (std::holds_alternative<std::string>(nextVariant))
            {
                widthFunction = std::get<std::string>(nextVariant);
            }
        }

        return widthFunction;
    };

    return callAndResetStreamPos<std::optional<std::string>>(func);
}

std::vector<sciformats::jdx::PeakAssignment>
sciformats::jdx::PeakAssignments::getData()
{
    auto func = [&]() {
        std::vector<sciformats::jdx::PeakAssignment> peakAssignments{};
        m_istream.seekg(m_streamDataPos);
        auto numVariables
            = m_variableList == s_peakAssignentsXyaVariableList ? 3U : 4U;
        util::PeakAssignmentsParser parser{m_istream, numVariables};

        while (parser.hasNext())
        {
            auto nextVariant = parser.next();
            if (std::holds_alternative<std::string>(nextVariant))
            {
                // skip width function
                continue;
            }
            peakAssignments.push_back(std::get<PeakAssignment>(nextVariant));
        }

        return peakAssignments;
    };

    return callAndResetStreamPos<std::vector<sciformats::jdx::PeakAssignment>>(
        func);
}

template<typename R>
R sciformats::jdx::PeakAssignments::callAndResetStreamPos(
    const std::function<R()>& func)
{
    auto streamPos = m_istream.eof()
                         ? std::nullopt
                         : std::optional<std::streampos>(m_istream.tellg());
    try
    {
        m_istream.seekg(m_streamDataPos);
        R returnValue = func();

        // reset stream
        if (streamPos)
        {
            m_istream.seekg(streamPos.value());
        }

        return returnValue;
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

template std::optional<std::string>
sciformats::jdx::PeakAssignments::callAndResetStreamPos<
    std::optional<std::string>>(
    const std::function<std::optional<std::string>()>& func);

template std::vector<sciformats::jdx::PeakAssignment>
sciformats::jdx::PeakAssignments::callAndResetStreamPos<
    std::vector<sciformats::jdx::PeakAssignment>>(
    const std::function<std::vector<sciformats::jdx::PeakAssignment>()>& func);
