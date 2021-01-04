#include "jdx/RaData.hpp"
#include "jdx/RaParameters.hpp"
#include "jdx/JdxDataParser.hpp"
#include "jdx/JdxLdrParser.hpp"

sciformats::jdx::RaData::RaData(
    std::istream& iStream, const RaParameters& parameters)
    : m_istream{iStream}
    , m_streamDataPos{iStream.tellg()}
    , m_firstX{parameters.firstR}
    , m_lastX{parameters.lastR}
    , m_xFactor{parameters.rFactor}
    , m_yFactor{parameters.aFactor}
    , m_nPoints{parameters.nPoints}
{
    auto line = JdxLdrParser::readLine(iStream);
    m_streamDataPos = iStream.tellg();

    if (!JdxLdrParser::isLdrStart(line))
    {
        throw std::runtime_error(
            "Cannot parse xy data. Stream position not at LDR start: " + line);
    }
    auto [label, variableList] = JdxLdrParser::parseLdrStart(line);
    JdxLdrParser::stripLineComment(variableList);
    JdxLdrParser::trim(variableList);
    m_label = label;
    m_variableList = variableList;

    validateInput(label, variableList);
    skipToNextLdr(iStream);
}

sciformats::jdx::RaData::RaData(const std::string& label,
    const std::string& variableList, std::istream& iStream,
    const RaParameters& parameters)
    : m_istream{iStream}
    , m_streamDataPos{iStream.tellg()}
    , m_label{label}
    , m_variableList{variableList}
    , m_firstX{parameters.firstR}
    , m_lastX{parameters.lastR}
    , m_xFactor{parameters.rFactor}
    , m_yFactor{parameters.aFactor}
    , m_nPoints{parameters.nPoints}
{
    validateInput(label, variableList);
    skipToNextLdr(iStream);
}

std::vector<std::pair<double, double>> sciformats::jdx::RaData::getData()
{
    auto pos = m_istream.tellg();
    auto startPos = m_streamDataPos;
    try
    {
        m_istream.seekg(startPos);
        auto data = parseInput(
            m_label, m_istream, m_firstX, m_lastX, m_yFactor, m_nPoints);
        m_istream.seekg(pos);
        return data;
    }
    catch (...)
    {
        try
        {
            m_istream.seekg(pos);
        }
        catch (...)
        {
        }
        throw;
    }
}

//std::vector<std::pair<double, double>> sciformats::jdx::RaData::parseInput(
//    const std::string& label, std::istream& iStream, double firstR,
//    double lastR, double aFactor, size_t nPoints)
//{
//    // parse
//    auto yData = sciformats::jdx::JdxDataParser::readXppYYData(iStream);
//    if (yData.size() != nPoints)
//    {
//        throw std::runtime_error(
//            "Mismatch betwee NPOINTS and actual number of points in \"" + label
//            + "\". NPOINTS: " + std::to_string(nPoints)
//            + ", actual: " + std::to_string(yData.size()));
//    }
//    // prepare processing
//    std::vector<std::pair<double, double>> xyData{};
//    xyData.reserve(yData.size());
//    // cover special cases nPoints == 0 and nPoints == 1
//    if (nPoints == 0)
//    {
//        return xyData;
//    }
//    auto nominator = nPoints == 1 ? firstR : (lastR - firstR);
//    auto denominator = nPoints == 1 ? 1 : nPoints - 1;
//    // generate and return xy data
//    uint64_t count = 0;
//    for (auto yRaw : yData)
//    {
//        auto x = firstR + nominator / denominator * count++;
//        auto y = aFactor * yRaw;
//        xyData.emplace_back(x, y);
//    }
//    return xyData;
//}

//void sciformats::jdx::RaData::skipToNextLdr(std::istream& iStream)
//{
//    while (!iStream.eof())
//    {
//        std::istream::pos_type pos = iStream.tellg();
//        std::string line = sciformats::jdx::JdxLdrParser::readLine(iStream);
//        if (sciformats::jdx::JdxLdrParser::isLdrStart(line))
//        {
//            // move back to start of LDR
//            iStream.seekg(pos);
//            break;
//        }
//    }
//}

void sciformats::jdx::RaData::validateInput(
    const std::string& label, const std::string& variableList)
{
    if (label != "RADATA")
    {
        throw std::runtime_error(
            "Illegal label at RADATA start encountered: " + label);
    }
    if (variableList != "(R++(A..A))" && variableList != "(RA..RA)")
    {
        throw std::runtime_error(
            "Illegal variable list for RADATA encountered: " + variableList);
    }
}
