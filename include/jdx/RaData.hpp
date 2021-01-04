#ifndef LIBJDX_RADATA_HPP
#define LIBJDX_RADATA_HPP

#include "jdx/Data2D.hpp"
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
     * containing "##RADATA="). The istream is expected to exist for the lifetime of this object.
     * @param parameters Parameters from the enclosing block specific to RADATA.
     */
    explicit RaData(std::istream& iStream, const RaParameters& parameters);
    /**
     * @brief Constructs RaData from first line value and istream.
     * @param label The label of the first line of the record, i.e. "RADATA".
     * @param variableList The value of the first line of the record
     * representing the structure of the data, e.g. "(R++(A..A))".
     * @param iStream Input stream with JCAMP-DX data. The stream position is
     * assumed to be at the start of the second line (the line following the "##RADATA=" line) of the record. The istream is expected to exist for the lifetime of this object.
     * @param parameters Parameters from the enclosing block specific to RADATA.
     */
    RaData(const std::string& label, const std::string& variableList,
        std::istream& iStream, const RaParameters& parameters);
    /**
     * @brief Provides the parsed xy data.
     * @return Pairs of xy data. Invalid values ("?") will be represented by
     * std::numeric_limits<double>::quiet_NaN.
     */
    std::vector<std::pair<double, double>> getData();

private:
    std::istream& m_istream;
    std::streampos m_streamDataPos;
    std::string m_label;
    std::string m_variableList;
    const double m_firstX;
    const double m_lastX;
    const double m_xFactor;
    const double m_yFactor;
    const uint64_t m_nPoints;

//    /**
//     * @brief Parses the xy data from first line value and istream.
//     * @param label The label of the first line of the record, i.e. "RADATA".
//     * @param iStream Input stream with JCAMP-DX data. The stream position is
//     * assumed to be at the start of the second line (the line following the
//     * "##RADATA=" line) of the record. The istream is expected to exist for the lifetime of this object.
//     * @param firstR The first R value.
//     * @param lastR The last R value.
//     * @param aFactor The factor by which to multiply raw A values to arrive at
//     * the actual value.
//     * @param nPoints The number of data points in this record.
//     * @return Pairs of RA data. Invalid values ("?") will be represented by
//     * std::numeric_limits<T>::quiet_NaN.
//     *
//     * Note: RFACTOR is not required for parsing as the R values are determined
//     * by FIRSTR, LASTR and NPOINTS.
//     */
//    static std::vector<std::pair<double, double>> parseInput(
//        const std::string& label, std::istream& iStream, double firstR,
//        double lastR, double aFactor, size_t nPoints);
//    /**
//     * @brief Moves the stream position to the start of the next LDR or to the
//     * EOF if no LDR follows.
//     * @param iStream The stream whose position will be changed.
//     */
//    static void skipToNextLdr(std::istream& iStream);
    /**
     * @brief Validates if input is a vlaid xy data LDR.
     * @param label LDR label. Must match "RADATA".
     * @param variableList First line LDR value. Must represent a variable list and match "(R++(A..A))" or "(RA..RA)".
     * @throws If label or variable list don't match expectations.
     */
    static void validateInput(
        const std::string& label, const std::string& variableList);
};
} // namespace sciformats::jdx

#endif // LIBJDX_RADATA_HPP
