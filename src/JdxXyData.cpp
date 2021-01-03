#include "jdx/JdxXyData.hpp"
#include "jdx/JdxDataParser.hpp"
#include "jdx/JdxLdrParser.hpp"

sciformats::jdx::JdxXyData::JdxXyData(std::istream& iStream, double firstX,
    double lastX, double xFactor, double yFactor, uint64_t nPoints)
    : m_istream{iStream}
    , m_streamDataPos{iStream.tellg()}
    , m_firstX{firstX}
    , m_lastX{lastX}
    , m_xFactor{xFactor}
    , m_yFactor{yFactor}
    , m_nPoints{nPoints}
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

sciformats::jdx::JdxXyData::JdxXyData(const std::string& label,
    const std::string& variableList, std::istream& iStream, double firstX,
    double lastX, double xFactor, double yFactor, uint64_t nPoints)
    : m_istream{iStream}
    , m_streamDataPos{iStream.tellg()}
    , m_label{label}
    , m_variableList{variableList}
    , m_firstX{firstX}
    , m_lastX{lastX}
    , m_xFactor{xFactor}
    , m_yFactor{yFactor}
    , m_nPoints{nPoints}

{
    validateInput(label, variableList);
    skipToNextLdr(iStream);
}

std::vector<std::pair<double, double>> sciformats::jdx::JdxXyData::getXyData()
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

std::vector<std::pair<double, double>> sciformats::jdx::JdxXyData::parseInput(
    const std::string& label, std::istream& iStream, double firstX,
    double lastX, double yFactor, size_t nPoints)
{
    // parse
    auto yData = sciformats::jdx::JdxDataParser::readXppYYData(iStream);
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

void sciformats::jdx::JdxXyData::skipToNextLdr(std::istream& iStream)
{
    while (!iStream.eof())
    {
        std::istream::pos_type pos = iStream.tellg();
        std::string line = sciformats::jdx::JdxLdrParser::readLine(iStream);
        if (sciformats::jdx::JdxLdrParser::isLdrStart(line))
        {
            // move back to start of LDR
            iStream.seekg(pos);
            break;
        }
    }
}

void sciformats::jdx::JdxXyData::validateInput(
    const std::string& label, const std::string& variableList)
{
    if (label != "XYDATA" && label != "RADATA")
    {
        throw std::runtime_error(
            "Illegal label at xy data start encountered: " + label);
    }
    if ((label == "XYDATA"
            && (variableList != "(X++(Y..Y))" && variableList != "(XY..XY)")))
    {
        throw std::runtime_error(
            "Illegal variable list for XYDATA encountered: " + variableList);
    }
    if ((label == "RADATA"
            && (variableList != "(R++(A..A))" && variableList != "(RA..RA)")))
    {
        throw std::runtime_error(
            "Illegal variable list for RADATA encountered: " + variableList);
    }
}
