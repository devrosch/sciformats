#ifndef LIBJDX_RADATA_HPP
#define LIBJDX_RADATA_HPP

#include "jdx/Data2D.hpp"
#include "jdx/JdxLdr.hpp"
#include "jdx/RaParameters.hpp"

#include <istream>
#include <vector>

namespace sciformats::jdx
{
/**
 * @brief A JCAMP-DX RADATA record.
 */
class RaData : Data2D
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
    explicit RaData(std::istream& iStream, const std::vector<JdxLdr>& ldrs);
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
        std::istream& iStream, const std::vector<JdxLdr>& ldrs);
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
    static constexpr char const* s_rppAAVariableList = "(R++(A..A))";
    static constexpr char const* s_raVariableList = "(RA..RA)";

    RaParameters m_parameters;

    /**
     * @brief Validates if input is a valid RADATA LDR.
     * @param label LDR label. Must match "RADATA".
     * @param variableList First line LDR value. Must represent a variable list
     * and match "(R++(A..A))" or "(RA..RA)".
     * @throws If label or variable list don't match expectations.
     */
    static void validateInput(
        const std::string& label, const std::string& variableList);
    static RaParameters parseParameters(const std::vector<JdxLdr>& ldrs);
};
} // namespace sciformats::jdx

#endif // LIBJDX_RADATA_HPP
