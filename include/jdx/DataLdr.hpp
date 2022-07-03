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
    [[nodiscard]] const std::string& getVariableList() const;

protected:
    DataLdr(std::string label, std::string variableList, TextReader& reader);
    TextReader& getReader();
    /**
     * @brief Moves the reader position to the start of the next LDR or to the
     * EOF if no LDR follows.
     * @param reader The reader whose position will be changed.
     */
    static void skipToNextLdr(TextReader& reader);
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
    R callAndResetStreamPos(const std::function<R()>& func);

private:
    const std::string m_variableList;
    TextReader& m_reader;
    std::streampos m_dataPos;
};

template<typename R>
R sciformats::jdx::DataLdr::callAndResetStreamPos(
    const std::function<R()>& func)
{
    auto pos = m_reader.eof() ? std::nullopt
                              : std::optional<std::streampos>(m_reader.tellg());
    try
    {
        m_reader.seekg(m_dataPos);
        R returnValue = func();

        // reset reader
        if (pos)
        {
            m_reader.seekg(pos.value());
        }

        return returnValue;
    }
    catch (...)
    {
        try
        {
            if (pos)
            {
                m_reader.seekg(pos.value());
            }
        }
        catch (...)
        {
        }
        throw;
    }
}

} // namespace sciformats::jdx

#endif // LIBJDX_DATALDR_HPP
