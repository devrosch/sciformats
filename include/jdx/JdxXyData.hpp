#ifndef LIBJDX_JDXXYDATA_HPP
#define LIBJDX_JDXXYDATA_HPP

#include <istream>
#include <vector>

namespace sciformats::jdx
{
/**
 * @brief A JCAMP-DX xy data record. Can represent "##XYDATA=" and "##RADATA="
 * LDRs.
 */
class JdxXyData
{
public:
    /**
     * @brief Constructs JdxXyData from istream.
     * @param iStream Input stream with JCAMP-DX data. The stream position is
     * assumed to be at the start of the first line of the record (the line
     * containing "##XYDATA=" or "##RADATA="). The inputStream is expected to
     * exist for the lifetime of this object.
     * @param firstX The first X value.
     * @param lastX The last X value.
     * @param xFactor The factor by which to multiply raw x values to arrive at
     * the actual value.
     * @param yFactor The factor by which to multiply raw y values to arrive at
     * the actual value.
     * @param nPoints The number of xy pairs in this record.
     */
    explicit JdxXyData(std::istream& iStream, double firstX, double lastX,
        double xFactor, double yFactor, uint64_t nPoints);
    /**
     * @brief Constructs JdxXyData from first line value and istream.
     * @param label The label of the first line of the record, i.e. either
     * "XYDATA" or "RADATA".
     * @param variableList The value of the first line of the record
     * representing the structure of the data, e.g. "(X++(Y..Y))".
     * @param iStream Input stream with JCAMP-DX data. The stream position is
     * assumed to be at the start of the second line (the line following the
     * "##XYDATA=" or "##RADATA=" line) of the record. The inputStream is
     * expected to exist for the lifetime of this object.
     * @param firstX The first X value.
     * @param lastX The last X value.
     * @param xFactor The factor by which to multiply raw x values to arrive at
     * the actual value.
     * @param yFactor The factor by which to multiply raw y values to arrive at
     * the actual value.
     * @param nPoints The number of xy pairs in this record.
     */
    JdxXyData(const std::string& label, const std::string& variableList,
        std::istream& iStream, double firstX, double lastX, double xFactor,
        double yFactor, uint64_t nPoints);
    /**
     * @brief Provides the parsed xy data.
     * @return Pairs of xy data. Invalid values ("?") will be represented by
     * std::numeric_limits<double>::quiet_NaN.
     */
    std::vector<std::pair<double, double>> getXyData();

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

    /**
     * @brief Parses the xy data from first line value and istream.
     * @param label The label of the first line of the record, i.e. either
     * "XYDATA" or "RADATA".
     * @param iStream Input stream with JCAMP-DX data. The stream position is
     * assumed to be at the start of the second line (the line following the
     * "##XYDATA=" or "##RADATA=" line) of the record. The inputStream is
     * expected to exist for the lifetime of this object.
     * @param firstX The first X value.
     * @param lastX The last X value.
     * @param yFactor The factor by which to multiply raw y values to arrive at
     * the actual value.
     * @param nPoints The number of xy pairs in this record.
     * @return Pairs of xy data. Invalid values ("?") will be represented by
     * std::numeric_limits<T>::quiet_NaN.
     *
     * Note: XFACTOR is not required for parsing as the x values are determined
     * by FIRSTX, LASTX and NPOINTS.
     */
    static std::vector<std::pair<double, double>> parseInput(
        const std::string& label, std::istream& iStream, double firstX,
        double lastX, double yFactor, size_t nPoints);
    /**
     * @brief Moves the stream position to the start of the next LDR or to the
     * EOF if no LDR follows.
     * @param iStream The stream whose position will be changed.
     */
    static void skipToNextLdr(std::istream& iStream);
    /**
     * @brief Validates if input is a vlaid xy data LDR.
     * @param label LDR label. Must match "XYDATA" or "RADATA".
     * @param variableList LDR value. Must represent a variable list and match
     * "(X++(Y..Y))" or "(XY..XY)" for XYDATA and "(R++(A..A))" or "(RA..RA)"
     * for RADATA.
     * @throws If label or variable list don't match expectations.
     */
    static void validateInput(
        const std::string& label, const std::string& variableList);
};
} // namespace sciformats::jdx

#endif // LIBJDX_JDXXYDATA_HPP
