#include "jdx/PeakAssignments.hpp"
#include "util/LdrUtils.hpp"
#include "util/PeakAssignmentsParser.hpp"

sciformats::jdx::PeakAssignments::PeakAssignments(const std::string& label,
    std::string variableList, TextReader& reader,
    std::optional<std::string>& nextLine)
    : TabularData(label, std::move(variableList), reader)
{
    validateInput(getLabel(), getVariableList(), s_peakAssignentsLabel,
        std::vector<std::string>{
            std::vector<std::string>{std::begin(s_peakAssignentsVariableLists),
                std::end(s_peakAssignentsVariableLists)}});
    util::skipToNextLdr(reader, nextLine, true);
}

std::vector<sciformats::jdx::PeakAssignment>
sciformats::jdx::PeakAssignments::getData() const
{
    util::PeakAssignmentsParser parser{getReader(), getVariableList()};
    return TabularData::getData<util::PeakAssignmentsParser, PeakAssignment>(
        parser);
}
