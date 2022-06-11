#ifndef LIBJDX_RADATA_HPP
#define LIBJDX_RADATA_HPP

#include "jdx/Array2DData.hpp"
#include "jdx/RaParameters.hpp"
#include "jdx/StringLdr.hpp"

#include <istream>
#include <vector>

namespace sciformats::jdx
{
/**
 * @brief A JCAMP-DX RADATA record.
 */
class RaData : Array2DData
{
public:
    /**
     * @brief Constructs RaData from istream.
     * @param iStream Input stream with JCAMP-DX data. The stream position is
     * assumed to be at the start of the first line of the record (the line
     * containing "##RADATA="). The istream is expected to exist for the
     * lifetime of this object.
     * @param parameters Parameters from the enclosing block specific to RADATA.
     */
    RaData(std::istream& iStream, const std::vector<StringLdr>& ldrs);
    /**
     * @brief Constructs RaData from first line value and istream.
     * @param label The label of the first line of the record, i.e. "RADATA".
     * @param variableList The value of the first line of the record
     * representing the structure of the data, e.g. "(R++(A..A))".
     * @param iStream Input stream with JCAMP-DX data. The stream position is
     * assumed to be at the start of the second line (the line following the
     * "##RADATA=" line) of the record. The istream is expected to exist for the
     * lifetime of this object.
     * @param parameters Parameters from the enclosing block specific to RADATA.
     */
    RaData(const std::string& label, const std::string& variableList,
        std::istream& iStream, const std::vector<StringLdr>& ldrs);
    /**
     * @brief Provides parameters specific to RADATA.
     * @return The parameters.
     */
    [[nodiscard]] const RaParameters& getParameters() const;
    /**
     * @brief Provides the parsed xy data.
     * @return Pairs of xy data. Invalid values ("?") will be represented by
     * std::numeric_limits<double>::quiet_NaN.
     */
    std::vector<std::pair<double, double>> getData();

private:
    static constexpr const char* s_raDataLabel = "RADATA";
    static constexpr const char* s_raDataVariableList = "(R++(A..A))";

    RaParameters m_parameters;

    static RaParameters parseParameters(const std::vector<StringLdr>& ldrs);
};
} // namespace sciformats::jdx

#endif // LIBJDX_RADATA_HPP
