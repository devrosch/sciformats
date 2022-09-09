#ifndef LIBJDX_XYBASE_HPP
#define LIBJDX_XYBASE_HPP

#include "jdx/Data2D.hpp"
#include "jdx/StringLdr.hpp"
#include "jdx/TextReader.hpp"
#include "jdx/XyParameters.hpp"

#include <vector>

namespace sciformats::jdx
{
/**
 * @brief A JCAMP-DX 2D data record. Base for "##XYPOINTS=" and "##XYDATA="
 * LDRs. LDRs.
 */
class XyBase : public Data2D
{
public:
    /**
     * @brief Provides parameters specific to XYDATA.
     * @return The parameters.
     */
    [[nodiscard]] const XyParameters& getParameters() const;

protected:
    /**
     * @brief Constructs XyBase from first line and reader.
     * @param label The label of the first line of the record, i.e. "XYDATA".
     * @param variableList The value of the first line of the record
     * representing the structure of the data, e.g. "(X++(Y..Y))".
     * @param ldrs Parameters from the enclosing block specific to XYDATA.
     * @param expectedLabel The expected label for this LDR.
     * @param expectedVariableList The expected variable list for this LDR.
     * @param reader Text reader with JCAMP-DX data. The reader position is
     * assumed to be at the start of the second line (the line following the
     * "##XYDATA=" line) of the record. The reader is expected to exist for the
     * lifetime of this object.
     * @param nextLine The first line of the LDR, i.e., the one containing the
     * label. Will contain the line following the record or nullopt if the end
     * of the reader has been reached.
     */
    XyBase(const std::string& label, const std::string& variableList,
        const std::vector<StringLdr>& ldrs, const std::string& expectedLabel,
        std::string expectedVariableList, TextReader& reader,
        std::optional<std::string>& nextLine);

    /**
     * @brief Provides the parsed xy data.
     * @return Pairs of xy data. Invalid values ("?") will be represented by
     * std::numeric_limits<double>::quiet_NaN.
     */
    std::vector<std::pair<double, double>> getData(
        Data2D::VariableList varList);

private:
    XyParameters m_parameters;

    static XyParameters parseParameters(const std::vector<StringLdr>& ldrs);
};
} // namespace sciformats::jdx

#endif // LIBJDX_XYBASE_HPP
