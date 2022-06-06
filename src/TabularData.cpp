#include "jdx/TabularData.hpp"
#include "jdx/util/LdrUtils.hpp"
#include "jdx/Peak.hpp"
#include "jdx/PeakAssignment.hpp"

// TODO: duplicate of constructor in Data2D
sciformats::jdx::TabularData::TabularData(std::istream& istream)
    : DataLdr (istream)
{
}

// TODO: duplicate of constructor in Data2D
sciformats::jdx::TabularData::TabularData(
    std::string label, std::string variableList, std::istream& istream)
    : DataLdr (std::move(label), std::move(variableList), istream)
{
}

// TODO: similar to validateInput() in Data2D
void sciformats::jdx::TabularData::validateInput(const std::string& label,
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
R sciformats::jdx::TabularData::callAndResetStreamPos(
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
sciformats::jdx::TabularData::callAndResetStreamPos<std::optional<std::string>>(
    const std::function<std::optional<std::string>()>& func);

template std::vector<sciformats::jdx::Peak>
sciformats::jdx::TabularData::callAndResetStreamPos<
    std::vector<sciformats::jdx::Peak>>(
    const std::function<std::vector<sciformats::jdx::Peak>()>& func);

template std::vector<sciformats::jdx::PeakAssignment>
sciformats::jdx::TabularData::callAndResetStreamPos<
    std::vector<sciformats::jdx::PeakAssignment>>(
    const std::function<std::vector<sciformats::jdx::PeakAssignment>()>& func);
