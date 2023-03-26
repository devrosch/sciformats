#include "jdx/Data2D.hpp"
#include "jdx/ParseException.hpp"
#include "util/DataParser.hpp"
#include "util/LdrUtils.hpp"

#include <tuple>

sciformats::jdx::Data2D::Data2D(
    std::string label, std::string variableList, TextReader& reader)
    : DataLdr(std::move(label), std::move(variableList), reader)
{
}

std::vector<std::pair<double, double>> sciformats::jdx::Data2D::parseXppYYData(
    const std::string& label, TextReader& reader, double firstX, double lastX,
    double yFactor, size_t nPoints) const
{
    // parse
    auto func = [&]() {
        return sciformats::jdx::util::DataParser::readXppYYData(reader);
    };
    auto yData = callAndResetStreamPos<std::vector<double>>(func);

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
        auto x = firstX
                 + nominator / static_cast<double>(denominator)
                       * static_cast<double>(count++);
        auto y = yFactor * yRaw;
        xyData.emplace_back(x, y);
    }
    // TODO: check if parsed data matches firstX, lastX
    return xyData;
}

std::vector<std::pair<double, double>> sciformats::jdx::Data2D::parseXyXyData(
    const std::string& label, TextReader& reader, double xFactor,
    double yFactor, std::optional<size_t> nPoints) const
{
    // parse
    auto func = [&]() {
        return sciformats::jdx::util::DataParser::readXyXyData(reader);
    };
    auto xyData
        = callAndResetStreamPos<std::vector<std::pair<double, double>>>(func);

    if (nPoints.has_value() && xyData.size() != nPoints.value())
    {
        throw ParseException(
            "Mismatch between NPOINTS and actual number of points in \"" + label
            + "\". NPOINTS: " + std::to_string(nPoints.value())
            + ", actual: " + std::to_string(xyData.size()));
    }
    for (auto& pair : xyData)
    {
        pair.first *= xFactor;
        pair.second *= yFactor;
    }
    return xyData;
}
