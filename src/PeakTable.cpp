#include "jdx/PeakTable.hpp"
#include "jdx/util/LdrUtils.hpp"
#include "jdx/util/PeakTableParser.hpp"

#include <algorithm>
#include <istream>
#include <tuple>

// TODO: duplicate of constructor in Data2D
sciformats::jdx::PeakTable::PeakTable(std::istream& istream)
    : TabularData(istream)
{
    validateInput(getLabel(), getVariableList(), s_peakTableLabel,
        std::vector<std::string>{
            s_peakTableXyVariableList, s_peakTableXywVariableList});
    skipToNextLdr(istream);
}

// TODO: duplicate of constructor in Data2D
sciformats::jdx::PeakTable::PeakTable(
    std::string label, std::string variableList, std::istream& istream)
    : TabularData(std::move(label), std::move(variableList), istream)
{
    validateInput(getLabel(), getVariableList(), s_peakTableLabel,
        std::vector<std::string>{
            s_peakTableXyVariableList, s_peakTableXywVariableList});
    skipToNextLdr(istream);
}

std::optional<std::string> sciformats::jdx::PeakTable::getKernel()
{
    // comment $$ in line(s) following LDR start may contain peak width and
    // other peak kernel functions
    auto func = [&]() {
        auto& stream = getStream();
        auto& streamPos = getStreamPos();
        std::optional<std::string> kernelFunction{std::nullopt};
        stream.seekg(streamPos);
        auto numVariables
            = getVariableList() == s_peakTableXyVariableList ? 2U : 3U;
        util::PeakTableParser parser{stream, numVariables};

        if (parser.hasNext())
        {
            auto nextVariant = parser.next();
            if (std::holds_alternative<std::string>(nextVariant))
            {
                kernelFunction = std::get<std::string>(nextVariant);
            }
        }

        return kernelFunction;
    };

    return callAndResetStreamPos<std::optional<std::string>>(func);
}

std::vector<sciformats::jdx::Peak> sciformats::jdx::PeakTable::getData()
{
    auto func = [&]() {
        auto& stream = getStream();
        auto& streamPos = getStreamPos();
        std::vector<sciformats::jdx::Peak> peaks{};
        stream.seekg(streamPos);
        auto numVariables
            = getVariableList() == s_peakTableXyVariableList ? 2U : 3U;
        util::PeakTableParser parser{stream, numVariables};

        while (parser.hasNext())
        {
            auto nextVariant = parser.next();
            if (std::holds_alternative<std::string>(nextVariant))
            {
                // skip kernel function
                continue;
            }
            peaks.push_back(std::get<Peak>(nextVariant));
        }

        return peaks;
    };

    return callAndResetStreamPos<std::vector<sciformats::jdx::Peak>>(func);
}
