#include "jdx/PeakAssignments.hpp"
#include "jdx/util/LdrUtils.hpp"
#include "jdx/util/PeakAssignmentsParser.hpp"

#include <algorithm>
#include <istream>
#include <tuple>

sciformats::jdx::PeakAssignments::PeakAssignments(std::istream& istream)
    : TabularData(istream)
{
    validateInput(getLabel(), getVariableList(), s_peakAssignentsLabel,
        std::vector<std::string>{
            s_peakAssignentsXyaVariableList, s_peakAssignentsXywaVariableList});
    skipToNextLdr(istream);
}

sciformats::jdx::PeakAssignments::PeakAssignments(
    std::string label, std::string variableList, std::istream& istream)
    : TabularData(std::move(label), std::move(variableList), istream)
{
    validateInput(getLabel(), getVariableList(), s_peakAssignentsLabel,
        std::vector<std::string>{
            s_peakAssignentsXyaVariableList, s_peakAssignentsXywaVariableList});
    skipToNextLdr(istream);
}

// TODO: duplicate of getKernel() in PeakTable
std::optional<std::string> sciformats::jdx::PeakAssignments::getWidthFunction()
{
    // comment $$ in line(s) following LDR start may contain width function
    auto func = [&]() {
        auto& stream = getStream();
        std::optional<std::string> widthFunction{std::nullopt};
        auto numVariables
            = getVariableList() == s_peakAssignentsXyaVariableList ? 3U : 4U;
        util::PeakAssignmentsParser parser{stream, numVariables};

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

std::vector<sciformats::jdx::PeakAssignment>
sciformats::jdx::PeakAssignments::getData()
{
    auto func = [&]() {
        auto& stream = getStream();
        std::vector<sciformats::jdx::PeakAssignment> peakAssignments{};
        auto numVariables
            = getVariableList() == s_peakAssignentsXyaVariableList ? 3U : 4U;
        util::PeakAssignmentsParser parser{stream, numVariables};

        while (parser.hasNext())
        {
            auto nextVariant = parser.next();
            if (std::holds_alternative<std::string>(nextVariant))
            {
                // skip width function
                continue;
            }
            peakAssignments.push_back(std::get<PeakAssignment>(nextVariant));
        }

        return peakAssignments;
    };

    return callAndResetStreamPos<std::vector<sciformats::jdx::PeakAssignment>>(
        func);
}
