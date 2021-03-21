#ifndef LIBJDX_XYDATA_HPP
#define LIBJDX_XYDATA_HPP

#include "jdx/Data2D.hpp"
#include "jdx/JdxLdr.hpp"
#include "jdx/XyParameters.hpp"

#include <istream>
#include <vector>

namespace sciformats::jdx
{
/**
 * @brief A JCAMP-DX XYDATA record.
 * LDRs.
 */
class XyData : Data2D
{
public:
    /**
     * @brief Constructs XyData from istream.
     * @param iStream Input stream with JCAMP-DX data. The stream position is
     * assumed to be at the start of the first line of the record (the line
     * containing "##XYDATA="). The istream is expected to exist for the
     * lifetime of this object.
     * @param parameters Parameters from the enclosing block specific to XYDATA.
     */
    XyData(std::istream& istream, const std::vector<JdxLdr>& ldrs);
    /**
     * @brief Constructs XyData from first line and istream.
     * @param label The label of the first line of the record, i.e. "XYDATA".
     * @param variableList The value of the first line of the record
     * representing the structure of the data, e.g. "(X++(Y..Y))".
     * @param iStream Input stream with JCAMP-DX data. The stream position is
     * assumed to be at the start of the second line (the line following the
     * "##XYDATA=" line) of the record. The istream is expected to exist for the
     * lifetime of this object.
     * @param parameters Parameters from the enclosing block specific to XYDATA.
     */
    XyData(const std::string& label, const std::string& variableList,
        std::istream& iStream, const std::vector<JdxLdr>& ldrs);
    /**
     * @brief Provides parameters specific to XYDATA.
     * @return The parameters.
     */
    [[nodiscard]] const XyParameters& getParameters() const;
    /**
     * @brief Provides the parsed xy data.
     * @return Pairs of xy data. Invalid values ("?") will be represented by
     * std::numeric_limits<double>::quiet_NaN.
     */
    std::vector<std::pair<double, double>> getData();

private:
    static constexpr char const* s_label = "XYDATA";
    static constexpr char const* s_xppYYVariableList = "(X++(Y..Y))";
    static constexpr char const* s_xyVariableList = "(XY..XY)";

    XyParameters m_parameters;

    /**
     * @brief Validates if input is a valid XYDATA LDR.
     * @param label LDR label. Must match "XYDATA".
     * @param variableList First line LDR value. Must represent a variable list
     * and match "(X++(Y..Y))" or "(XY..XY)".
     * @throws If label or variable list don't match expectations.
     */
    static void validateInput(
        const std::string& label, const std::string& variableList);
    static XyParameters parseParameters(const std::vector<JdxLdr>& ldrs);
};
} // namespace sciformats::jdx

#endif // LIBJDX_XYDATA_HPP
