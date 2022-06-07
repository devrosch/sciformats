#include "jdx/DataLdr.hpp"
#include "jdx/Peak.hpp"
#include "jdx/PeakAssignment.hpp"
#include "jdx/util/LdrUtils.hpp"

sciformats::jdx::DataLdr::DataLdr(std::istream& istream)
    : m_istream{istream}
    , m_streamDataPos{istream.tellg()}
{
    std::tie(m_label, m_variableList) = readFirstLine(istream);
    m_streamDataPos = istream.tellg();
}

sciformats::jdx::DataLdr::DataLdr(
    std::string label, std::string variableList, std::istream& istream)
    : m_label{std::move(label)}
    , m_variableList{std::move(variableList)}
    , m_istream{istream}
    , m_streamDataPos{istream.tellg()}
{
}

const std::string& sciformats::jdx::DataLdr::getLabel() const
{
    return m_label;
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

template<typename R>
R sciformats::jdx::DataLdr::callAndResetStreamPos(
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
sciformats::jdx::DataLdr::callAndResetStreamPos<std::optional<std::string>>(
    const std::function<std::optional<std::string>()>& func);

template std::vector<sciformats::jdx::Peak>
sciformats::jdx::DataLdr::callAndResetStreamPos<
    std::vector<sciformats::jdx::Peak>>(
    const std::function<std::vector<sciformats::jdx::Peak>()>& func);

template std::vector<sciformats::jdx::PeakAssignment>
sciformats::jdx::DataLdr::callAndResetStreamPos<
    std::vector<sciformats::jdx::PeakAssignment>>(
    const std::function<std::vector<sciformats::jdx::PeakAssignment>()>& func);

template std::vector<std::pair<double, double>>
sciformats::jdx::DataLdr::callAndResetStreamPos<
    std::vector<std::pair<double, double>>>(
    const std::function<std::vector<std::pair<double, double>>()>& func);
