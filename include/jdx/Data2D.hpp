#ifndef LIBJDX_DATA2D_HPP
#define LIBJDX_DATA2D_HPP

#include "jdx/XyParameters.hpp"

#include <istream>
#include <vector>

namespace sciformats::jdx
{
/**
 * @brief A JCAMP-DX 2D data record. Base for "##XYDATA=" and "##RADATA=" LDRs.
 */
class Data2D
{
public:
protected:
    enum class DataEncoding
    {
        XppYY,
        XyXy
    };
    /**
     * @brief Constructs Data2D from istream.
     * @param iStream Input stream with JCAMP-DX data. The stream position is
     * assumed to be at the start of the first line of the record (the line
     * containing "##XYDATA=" or "##RADATA="). The istream is expected to exist
     * for the lifetime of this object.
     */
    explicit Data2D(std::istream& iStream);
    /**
     * @brief Constructs Data2D from first line value and istream.
     * @param label The label of the first line of the record, i.e. "XYDATA" or
     * "RADATA".
     * @param variableList The value of the first line of the record
     * representing the structure of the data, e.g. "(X++(Y..Y))".
     * @param iStream Input stream with JCAMP-DX data. The stream position is
     * assumed to be at the start of the first line (the line containing
     * "##XYDATA=" or "##RADATA=") of the record. The istream is expected to
     * exist for the lifetime of this object.
     */
    Data2D(std::string label, std::string variableList, std::istream& iStream);
    /**
     * @brief Moves the stream position to the start of the next LDR or to the
     * EOF if no LDR follows.
     * @param iStream The stream whose position will be changed.
     */
    static void skipToNextLdr(std::istream& iStream);
    std::vector<std::pair<double, double>> getData(double firstX, double lastX,
        double xFactor, double yFactor, uint64_t nPoints,
        DataEncoding dataEncoding);
    const std::string& getLabel();
    const std::string& getVariableList();

private:
    std::istream& m_istream;
    std::streampos m_streamDataPos;
    std::string m_label;
    std::string m_variableList;

    /**
     * @brief Parses the equally x spaced xy data (i.e. "X++(Y..Y)" or
     * "R++(A..A)") from a "##XYDATA=" or "##RADATA=" block.
     * @param label The label of the first line of the record, i.e. "XYDATA" or
     * "RADATA".
     * @param iStream Input stream with JCAMP-DX data. The stream position is
     * assumed to be at the start of the second line (the line following the
     * "##XYDATA=" or "##RADATA=" line) of the record. The istream is expected
     * to exist for the lifetime of this object.
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
    static std::vector<std::pair<double, double>> parseXppYYInput(
        const std::string& label, std::istream& iStream, double firstX,
        double lastX, double yFactor, size_t nPoints);
    /**
     * @brief Parses the xy data pairs (i.e. "(XY..XY)" or "(RA..RA)") from a
     * "##XYDATA=" or "##RADATA=" block.
     * @param label The label of the first line of the record, i.e. "XYDATA" or
     * "RADATA".
     * @param iStream Input stream with JCAMP-DX data. The stream position is
     * assumed to be at the start of the second line (the line following the
     * "##XYDATA=" or "##RADATA=" line) of the record. The istream is expected
     * to exist for the lifetime of this object.
     * @param xFactor The factor by which to multiply raw x values to arrive at
     * the actual value.
     * @param yFactor The factor by which to multiply raw y values to arrive at
     * the actual value.
     * @param nPoints The number of xy pairs in this record.
     * @return Pairs of xy data. Invalid y values ("?") will be represented by
     * std::numeric_limits<T>::quiet_NaN.
     */
    static std::vector<std::pair<double, double>> parseXyXyInput(
        const std::string& label, std::istream& iStream, double xFactor,
        double yFactor, size_t nPoints);
    static std::pair<std::string, std::string> readFirstLine(
        std::istream& istream);
    static std::optional<std::string> findLdrValue();
};
} // namespace sciformats::jdx

#endif // LIBJDX_DATA2D_HPP
