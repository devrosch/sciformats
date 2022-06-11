#include "jdx/TabularData.hpp"
#include "util/PeakAssignmentsParser.hpp"

#include <variant>

sciformats::jdx::TabularData::TabularData(std::istream& istream)
    : DataLdr(istream)
{
}

sciformats::jdx::TabularData::TabularData(
    std::string label, std::string variableList, std::istream& istream)
    : DataLdr(std::move(label), std::move(variableList), istream)
{
}
