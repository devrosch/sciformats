#include "jdx/Data2D.hpp"
#include "jdx/JdxDataParser.hpp"
#include "jdx/JdxLdrParser.hpp"

#include <tuple>

sciformats::jdx::Data2D::Data2D(std::istream& iStream)
    : m_istream{iStream}
    , m_streamDataPos{iStream.tellg()}
{
    std::tie(m_label, m_variableList) = readFirstLine(iStream);
    m_streamDataPos = iStream.tellg();
}

sciformats::jdx::Data2D::Data2D(
    std::string label, std::string variableList, std::istream& iStream)
    : m_istream{iStream}
    , m_streamDataPos{iStream.tellg()}
    , m_label{std::move(label)}
    , m_variableList{std::move(variableList)}
{
}

std::vector<std::pair<double, double>> sciformats::jdx::Data2D::parseInput(
    const std::string& label, std::istream& iStream, double firstX,
    double lastX, double yFactor, size_t nPoints)
{
    // parse
    // TODO: based on the variables list, either parse as (X++(Y..Y)) or
    // (XY..XY)
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

void sciformats::jdx::Data2D::skipToNextLdr(std::istream& iStream)
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

std::pair<std::string, std::string> sciformats::jdx::Data2D::readFirstLine(
    std::istream& iStream)
{
    auto pos = iStream.tellg();
    auto line = JdxLdrParser::readLine(iStream);
    if (!JdxLdrParser::isLdrStart(line))
    {
        // reset for consistent state
        iStream.seekg(pos);
        throw std::runtime_error(
            "Cannot parse xy data. Stream position not at LDR start: " + line);
    }
    auto [label, variableList] = JdxLdrParser::parseLdrStart(line);
    JdxLdrParser::stripLineComment(variableList);
    JdxLdrParser::trim(variableList);

    return {label, variableList};
}

std::vector<std::pair<double, double>> sciformats::jdx::Data2D::getData(
    double firstX, double lastX, double yFactor, uint64_t nPoints)
{
    auto pos = m_istream.tellg();
    auto startPos = m_streamDataPos;
    try
    {
        m_istream.seekg(startPos);
        auto data
            = parseInput(m_label, m_istream, firstX, lastX, yFactor, nPoints);
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

const std::string& sciformats::jdx::Data2D::getLabel()
{
    return m_label;
}

const std::string& sciformats::jdx::Data2D::getVariableList()
{
    return m_variableList;
}
