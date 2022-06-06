#ifndef LIBJDX_DATALDR_HPP
#define LIBJDX_DATALDR_HPP

#include <functional>
#include <istream>
#include <string>

namespace sciformats::jdx
{
/**
 * @brief Base class for JCAMP-DX PEAK TABLE and PEAK ASSIGNMENTS records.
 */
class DataLdr
{
public:
protected:
    std::istream& m_istream;
    std::streampos m_streamDataPos;
    std::string m_label;
    std::string m_variableList;

    explicit DataLdr(std::istream& istream);
    DataLdr(
        std::string label, std::string variableList, std::istream& istream);

    /**
     * @brief Moves the stream position to the start of the next LDR or to the
     * EOF if no LDR follows.
     * @param iStream The stream whose position will be changed.
     */
    static void skipToNextLdr(std::istream& iStream);
    static std::pair<std::string, std::string> readFirstLine(
        std::istream& istream);
};
} // namespace sciformats::jdx

#endif // LIBJDX_DATALDR_HPP
