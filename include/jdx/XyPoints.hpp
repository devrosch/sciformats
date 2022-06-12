#ifndef LIBJDX_XYPOINTS_HPP
#define LIBJDX_XYPOINTS_HPP

#include "jdx/StringLdr.hpp"
#include "jdx/XyBase.hpp"
#include "jdx/XyParameters.hpp"

#include <istream>
#include <vector>

namespace sciformats::jdx
{
/**
 * @brief A JCAMP-DX XYPOINTS record.
 */
class XyPoints : public XyBase
{
public:
    /**
     * @brief Constructs XyPoints from first line and istream.
     * @param label The label of the first line of the record, i.e. "XYDATA".
     * @param variableList The value of the first line of the record
     * representing the structure of the data, e.g. "(X++(Y..Y))".
     * @param iStream Input stream with JCAMP-DX data. The stream position is
     * assumed to be at the start of the second line (the line following the
     * "##XYDATA=" line) of the record. The istream is expected to exist for the
     * lifetime of this object.
     * @param parameters Parameters from the enclosing block specific to XYDATA.
     */
    XyPoints(const std::string& label, const std::string& variableList,
        std::istream& iStream, const std::vector<StringLdr>& ldrs);
    /**
     * @brief Provides the parsed xy data.
     * @return Pairs of xy data. Invalid values ("?") will be represented by
     * std::numeric_limits<double>::quiet_NaN.
     */
    std::vector<std::pair<double, double>> getData();

private:
    static constexpr const char* s_xyPointsLabel = "XYPOINTS";
    static constexpr const char* s_xyPointsVariableList = "(XY..XY)";
};
} // namespace sciformats::jdx

#endif // LIBJDX_XYPOINTS_HPP
