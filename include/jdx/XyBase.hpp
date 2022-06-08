#ifndef LIBJDX_XYBASE_HPP
#define LIBJDX_XYBASE_HPP

#include "jdx/Data2D.hpp"
#include "jdx/StringLdr.hpp"
#include "jdx/XyParameters.hpp"

#include <istream>
#include <vector>

namespace sciformats::jdx
{
/**
 * @brief A JCAMP-DX 2D data record. Base for "##XYPOINTS=" and "##XYDATA="
 * LDRs. LDRs.
 */
class XyBase : protected Data2D
{
public:
    /**
     * @brief Provides parameters specific to XYDATA.
     * @return The parameters.
     */
    [[nodiscard]] const XyParameters& getParameters() const;

protected:
    /**
     * @brief Constructs XyBase from istream.
     * @param iStream Input stream with JCAMP-DX data. The stream position is
     * assumed to be at the start of the first line of the record (the line
     * containing "##XYDATA="). The istream is expected to exist for the
     * lifetime of this object.
     * @param ldrs Parameters from the enclosing block specific to XYDATA.
     * @param expectedLabel The expected label for this LDR.
     * @param expectedVariableList The expected variable list for this LDR.
     */
    XyBase(std::istream& istream, const std::vector<StringLdr>& ldrs,
        std::string expectedLabel, std::string expectedVariableList);
    /**
     * @brief Constructs XyBase from first line and istream.
     * @param label The label of the first line of the record, i.e. "XYDATA".
     * @param variableList The value of the first line of the record
     * representing the structure of the data, e.g. "(X++(Y..Y))".
     * @param iStream Input stream with JCAMP-DX data. The stream position is
     * assumed to be at the start of the second line (the line following the
     * "##XYDATA=" line) of the record. The istream is expected to exist for the
     * lifetime of this object.
     * @param ldrs Parameters from the enclosing block specific to XYDATA.
     * @param expectedLabel The expected label for this LDR.
     * @param expectedVariableList The expected variable list for this LDR.
     */
    XyBase(const std::string& label, const std::string& variableList,
        std::istream& iStream, const std::vector<StringLdr>& ldrs,
        std::string expectedLabel, std::string expectedVariableList);
    /**
     * @brief Provides the parsed xy data.
     * @return Pairs of xy data. Invalid values ("?") will be represented by
     * std::numeric_limits<double>::quiet_NaN.
     */
    std::vector<std::pair<double, double>> getData(Data2D::DataEncoding);

private:
    std::string m_expectedLabel;
    std::string m_expectedVariableList;
    XyParameters m_parameters;

    static XyParameters parseParameters(const std::vector<StringLdr>& ldrs);
};
} // namespace sciformats::jdx

#endif // LIBJDX_XYBASE_HPP
