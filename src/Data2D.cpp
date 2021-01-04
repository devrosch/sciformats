#include "jdx/Data2D.hpp"
#include "jdx/JdxDataParser.hpp"
#include "jdx/JdxLdrParser.hpp"

std::vector<std::pair<double, double>> sciformats::jdx::Data2D::parseInput(
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
