#include "jdx/PeakAssignments.hpp"
#include "jdx/util/LdrUtils.hpp"
#include "jdx/util/PeakAssignmentsParser.hpp"

#include <istream>

sciformats::jdx::PeakAssignments::PeakAssignments(std::istream& istream)
    : TabularData(istream)
{
    validateInput(getLabel(), getVariableList(), s_peakAssignentsLabel,
        std::vector<std::string>{
            std::vector<std::string>{std::begin(s_peakAssignentsVariableLists),
                std::end(s_peakAssignentsVariableLists)}});
    skipToNextLdr(istream);
}

sciformats::jdx::PeakAssignments::PeakAssignments(
    std::string label, std::string variableList, std::istream& istream)
    : TabularData(std::move(label), std::move(variableList), istream)
{
    validateInput(getLabel(), getVariableList(), s_peakAssignentsLabel,
        std::vector<std::string>{
            std::vector<std::string>{std::begin(s_peakAssignentsVariableLists),
                std::end(s_peakAssignentsVariableLists)}});
    skipToNextLdr(istream);
}

std::optional<std::string> sciformats::jdx::PeakAssignments::getWidthFunction()
{
    util::PeakAssignmentsParser parser{getStream(), getNumVariables()};
    return TabularData::getWidthFunction<util::PeakAssignmentsParser>(parser);
}

std::vector<sciformats::jdx::PeakAssignment>
sciformats::jdx::PeakAssignments::getData()
{
    util::PeakAssignmentsParser parser{getStream(), getNumVariables()};
    return TabularData::getData<util::PeakAssignmentsParser, PeakAssignment>(
        parser);
}

size_t sciformats::jdx::PeakAssignments::getNumVariables()
{
    return getVariableList() == s_peakAssignentsVariableLists.at(0) ? 3U : 4U;
}
