#ifndef LIBJDX_DATALDR_HPP
#define LIBJDX_DATALDR_HPP

#include "jdx/Ldr.hpp"
#include "jdx/TextReader.hpp"

#include <functional>
#include <optional>
#include <string>
#include <vector>

namespace sciformats::jdx
{
/**
 * @brief Base class for JCAMP-DX data records.
 */
class DataLdr : public Ldr
{
public:
    /**
     * @brief The record's variable list.
     * @return The record's variable list.
     */
    [[nodiscard]] const std::string& getVariableList() const;

protected:
    DataLdr(std::string label, std::string variableList, TextReader& reader);

    TextReader& getReader() const;
    /**
     * @brief Validates if input is a valid data LDR.
     * @param label LDR label.
     * @param variableList First line LDR value. Must represent a variable list.
     * @param expectedLabel The expected LDR label.
     * @param expectedVariableList The expected variable list.
     * @throws If label or variable list don't match expectations.
     */
    static void validateInput(const std::string& label,
        const std::string& variableList, const std::string& expectedLabel,
        const std::vector<std::string>& expectedVariableLists);

    template<typename R>
    R callAndResetStreamPos(const std::function<R()>& func) const;

private:
    const std::string m_variableList;
    TextReader& m_reader;
    std::streampos m_dataPos;
};

template<typename R>
R sciformats::jdx::DataLdr::callAndResetStreamPos(
    const std::function<R()>& func) const
{
    auto pos = m_reader.eof() ? std::nullopt
                              : std::optional<std::streampos>(m_reader.tellg());
    auto resetPosition = [pos, this] {
        if (pos)
        {
            m_reader.seekg(pos.value());
        }
        else
        {
            m_reader.seekg(0, std::ios_base::end);
        }
    };

    try
    {
        m_reader.seekg(m_dataPos);
        R returnValue = func();
        resetPosition();
        return returnValue;
    }
    catch (...)
    {
        try
        {
            resetPosition();
        }
        catch (...)
        {
        }
        throw;
    }
}

} // namespace sciformats::jdx

#endif // LIBJDX_DATALDR_HPP
