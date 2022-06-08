#ifndef LIBJDX_DATALDR_HPP
#define LIBJDX_DATALDR_HPP

#include <functional>
#include <istream>
#include <optional>
#include <string>

namespace sciformats::jdx
{
/**
 * @brief Base class for JCAMP-DX PEAK TABLE and PEAK ASSIGNMENTS records.
 */
class DataLdr
{
public:
    [[nodiscard]] const std::string& getLabel() const;
    [[nodiscard]] const std::string& getVariableList() const;

protected:
    explicit DataLdr(std::istream& istream);
    DataLdr(std::string label, std::string variableList, std::istream& istream);

    std::istream& getStream();
    std::streampos& getStreamPos();

    /**
     * @brief Moves the stream position to the start of the next LDR or to the
     * EOF if no LDR follows.
     * @param iStream The stream whose position will be changed.
     */
    static void skipToNextLdr(std::istream& iStream);
    static std::pair<std::string, std::string> readFirstLine(
        std::istream& istream);
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
    std::string m_label;
    std::string m_variableList;
    std::istream& m_istream;
    std::streampos m_streamDataPos;
};

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

} // namespace sciformats::jdx

#endif // LIBJDX_DATALDR_HPP
