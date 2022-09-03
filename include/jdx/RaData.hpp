#ifndef LIBJDX_RADATA_HPP
#define LIBJDX_RADATA_HPP

#include "jdx/Data2D.hpp"
#include "jdx/RaParameters.hpp"
#include "jdx/StringLdr.hpp"
#include "jdx/TextReader.hpp"

#include <string>
#include <vector>

namespace sciformats::jdx
{
/**
 * @brief A JCAMP-DX RADATA record.
 */
class RaData : public Data2D
{
public:
    /**
     * @brief Constructs RaData from first line value and reader.
     * @param label The label of the first line of the record, i.e. "RADATA".
     * @param variableList The value of the first line of the record
     * representing the structure of the data, e.g. "(R++(A..A))".
     * @param reader Text reader with JCAMP-DX data. The reader position is
     * assumed to be at the start of the second line (the line following the
     * "##RADATA=" line) of the record. The reader is expected to exist for the
     * lifetime of this object.
     * @param parameters Parameters from the enclosing block specific to RADATA.
     */
    RaData(const std::string& label, const std::string& variableList,
        TextReader& reader, const std::vector<StringLdr>& ldrs);
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
