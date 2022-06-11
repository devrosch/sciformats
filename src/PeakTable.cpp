#include "jdx/PeakTable.hpp"
#include "jdx/util/LdrUtils.hpp"
#include "jdx/util/PeakTableParser.hpp"

#include <algorithm>
#include <istream>
#include <tuple>

sciformats::jdx::PeakTable::PeakTable(std::istream& istream)
    : DataLdr(istream)
{
    validateInput(getLabel(), getVariableList(), s_peakTableLabel,
        std::vector<std::string>{std::begin(s_peakTableVariableLists),
            std::end(s_peakTableVariableLists)});
    skipToNextLdr(istream);
}

sciformats::jdx::PeakTable::PeakTable(
    std::string label, std::string variableList, std::istream& istream)
    : DataLdr(std::move(label), std::move(variableList), istream)
{
    validateInput(getLabel(), getVariableList(), s_peakTableLabel,
        std::vector<std::string>{std::begin(s_peakTableVariableLists),
            std::end(s_peakTableVariableLists)});
    skipToNextLdr(istream);
}

std::optional<std::string> sciformats::jdx::PeakTable::getWidthFunction()
{
    auto func = [&]() {
        auto& stream = getStream();
        std::optional<std::string> widthFunction{std::nullopt};
        auto numVariables
            = getVariableList() == s_peakTableVariableLists.at(0) ? 2U : 3U;
        util::PeakTableParser parser{stream, numVariables};

        if (parser.hasNext())
        {
            auto nextVariant = parser.next();
            if (std::holds_alternative<std::string>(nextVariant))
            {
                widthFunction = std::get<std::string>(nextVariant);
            }
        }

        return widthFunction;
    };

    return callAndResetStreamPos<std::optional<std::string>>(func);
}

std::vector<sciformats::jdx::Peak> sciformats::jdx::PeakTable::getData()
{
    auto func = [&]() {
        auto& stream = getStream();
        std::vector<sciformats::jdx::Peak> peaks{};
        auto numVariables
            = getVariableList() == s_peakTableVariableLists.at(0) ? 2U : 3U;
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
