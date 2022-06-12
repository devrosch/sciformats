#include "jdx/TabularData.hpp"
#include "util/PeakAssignmentsParser.hpp"

sciformats::jdx::TabularData::TabularData(
    std::string label, std::string variableList, std::istream& istream)
    : DataLdr(std::move(label), std::move(variableList), istream)
{
}
