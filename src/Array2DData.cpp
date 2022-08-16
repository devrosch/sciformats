#include "jdx/Array2DData.hpp"
#include "jdx/ParseException.hpp"
#include "util/DataParser.hpp"
#include "util/LdrUtils.hpp"

#include <tuple>

sciformats::jdx::Array2DData::Array2DData(
    std::string label, std::string variableList, TextReader& reader)
    : DataLdr(std::move(label), std::move(variableList), reader)
{
}

std::vector<std::pair<double, double>>
sciformats::jdx::Array2DData::parseXppYYInput(const std::string& label,
    TextReader& reader, double firstX, double lastX, double yFactor,
    size_t nPoints)
{
    // parse
    auto yData = sciformats::jdx::util::DataParser::readXppYYData(reader);
    if (yData.size() != nPoints)
    {
        throw ParseException(
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

std::vector<std::pair<double, double>>
sciformats::jdx::Array2DData::parseXyXyInput(const std::string& label,
    TextReader& reader, double xFactor, double yFactor, size_t nPoints)
{
    // parse
    auto xyData = sciformats::jdx::util::DataParser::readXyXyData(reader);
    if (xyData.size() != nPoints)
    {
        throw ParseException(
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

std::vector<std::pair<double, double>> sciformats::jdx::Array2DData::getData(
    double firstX, double lastX, double xFactor, double yFactor,
    uint64_t nPoints, DataEncoding dataEncoding)
{
    auto func = [&]() {
        auto& reader = getReader();
        std::vector<std::pair<double, double>> data{};
        const auto& label = getLabel();
        if (dataEncoding == DataEncoding::XppYY)
        {
            data = parseXppYYInput(
                label, reader, firstX, lastX, yFactor, nPoints);
        }
        else if (dataEncoding == DataEncoding::XyXy)
        {
            data = parseXyXyInput(label, reader, xFactor, yFactor, nPoints);
            // TODO: check if parsed data matches firstX, lastX
        }
        else
        {
            throw ParseException("Cannot parse xy data. Unsupported encoding.");
        }
        return data;
    };

    return callAndResetStreamPos<std::vector<std::pair<double, double>>>(func);
}
