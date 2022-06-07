#include "jdx/Data2D.hpp"
#include "jdx/util/DataParser.hpp"
#include "jdx/util/LdrUtils.hpp"

#include <tuple>

sciformats::jdx::Data2D::Data2D(std::istream& iStream)
    : DataLdr (iStream)
{
}

sciformats::jdx::Data2D::Data2D(
    std::string label, std::string variableList, std::istream& iStream)
    : DataLdr (std::move(label), std::move(variableList), iStream)
{
}

std::vector<std::pair<double, double>> sciformats::jdx::Data2D::parseXppYYInput(
    const std::string& label, std::istream& iStream, double firstX,
    double lastX, double yFactor, size_t nPoints)
{
    // parse
    auto yData = sciformats::jdx::DataParser::readXppYYData(iStream);
    if (yData.size() != nPoints)
    {
        throw std::runtime_error(
            "Mismatch betwee NPOINTS and actual number of points in \"" + label
            + "\". NPOINTS: " + std::to_string(nPoints)
            + ", actual: " + std::to_string(yData.size()));
    }
    // prepare processing
    std::vector<std::pair<double, double>> xyData{};
    xyData.reserve(yData.size());
    // cover special cases nPoints == 0 and nPoints == 1
    if (nPoints == 0)
    {
        return xyData;
    }
    auto nominator = nPoints == 1 ? firstX : (lastX - firstX);
    auto denominator = nPoints == 1 ? 1 : nPoints - 1;
    // generate and return xy data
    uint64_t count = 0;
    for (auto yRaw : yData)
    {
        auto x = firstX + nominator / denominator * count++;
        auto y = yFactor * yRaw;
        xyData.emplace_back(x, y);
    }
    return xyData;
}

std::vector<std::pair<double, double>> sciformats::jdx::Data2D::parseXyXyInput(
    const std::string& label, std::istream& iStream, double xFactor,
    double yFactor, size_t nPoints)
{
    // parse
    auto xyData = sciformats::jdx::DataParser::readXyXyData(iStream);
    if (xyData.size() != nPoints)
    {
        throw std::runtime_error(
            "Mismatch between NPOINTS and actual number of points in \"" + label
            + "\". NPOINTS: " + std::to_string(nPoints)
            + ", actual: " + std::to_string(xyData.size()));
    }
    for (auto& pair : xyData)
    {
        pair.first *= xFactor;
        pair.second *= yFactor;
    }
    return xyData;
}

std::vector<std::pair<double, double>> sciformats::jdx::Data2D::getData(
    double firstX, double lastX, double xFactor, double yFactor,
    uint64_t nPoints, DataEncoding dataEncoding)
{
    auto pos = m_istream.eof()
                   ? std::nullopt
                   : std::optional<std::streampos>(m_istream.tellg());
    auto startPos = m_streamDataPos;
    try
    {
        m_istream.seekg(startPos);
        std::vector<std::pair<double, double>> data{};
        if (dataEncoding == DataEncoding::XppYY)
        {
            data = parseXppYYInput(
                m_label, m_istream, firstX, lastX, yFactor, nPoints);
        }
        else if (dataEncoding == DataEncoding::XyXy)
        {
            data
                = parseXyXyInput(m_label, m_istream, xFactor, yFactor, nPoints);
            // TODO: check if parsed data matches firstX, lastX
        }
        else
        {
            throw std::runtime_error(
                "Cannot parse xy data. Unsupported encoding.");
        }
        if (pos)
        {
            m_istream.seekg(pos.value());
        }
        return data;
    }
    catch (...)
    {
        try
        {
            if (pos)
            {
                m_istream.seekg(pos.value());
            }
        }
        catch (...)
        {
        }
        throw;
    }
}

//void sciformats::jdx::Data2D::validateInput(const std::string& label,
//    const std::string& variableList, const std::string& expectedLabel,
//    const std::string& expectedVariableList)
//{
//    if (label != expectedLabel)
//    {
//        throw std::runtime_error("Illegal label at " + expectedLabel
//                                 + " start encountered: " + label);
//    }
//    if (variableList != expectedVariableList)
//    {
//        throw std::runtime_error("Illegal variable list for " + label
//                                 + " encountered: " + variableList);
//    }
//}

const std::string& sciformats::jdx::Data2D::getLabel()
{
    return m_label;
}

const std::string& sciformats::jdx::Data2D::getVariableList()
{
    return m_variableList;
}
