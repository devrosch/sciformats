#include "jdx/PeakAssignments.hpp"
#include "util/LdrUtils.hpp"
#include "util/PeakAssignmentsParser.hpp"

sciformats::jdx::PeakAssignments::PeakAssignments(
    const std::string& label, std::string variableList, TextReader& reader)
    : TabularData(label, std::move(variableList), reader)
{
    validateInput(getLabel(), getVariableList(), s_peakAssignentsLabel,
        std::vector<std::string>{
            std::vector<std::string>{std::begin(s_peakAssignentsVariableLists),
                std::end(s_peakAssignentsVariableLists)}});
    skipToNextLdr(reader);
}

std::optional<std::string> sciformats::jdx::PeakAssignments::getWidthFunction()
{
    util::PeakAssignmentsParser parser{getReader(), getNumVariables()};
    return TabularData::getWidthFunction<util::PeakAssignmentsParser>(parser);
}

std::vector<sciformats::jdx::PeakAssignment>
sciformats::jdx::PeakAssignments::getData()
{
    util::PeakAssignmentsParser parser{getReader(), getNumVariables()};
    return TabularData::getData<util::PeakAssignmentsParser, PeakAssignment>(
        parser);
}

size_t sciformats::jdx::PeakAssignments::getNumVariables()
{
    return getVariableList() == s_peakAssignentsVariableLists.at(0) ? 3U : 4U;
}
