#include "jdx/PeakTable.hpp"
#include "jdx/util/LdrUtils.hpp"
#include "jdx/util/PeakTableParser.hpp"

#include <algorithm>
#include <istream>
#include <tuple>

sciformats::jdx::PeakTable::PeakTable(std::istream& istream)
    : TabularData(istream)
{
    validateInput(getLabel(), getVariableList(), s_peakTableLabel,
        std::vector<std::string>{std::begin(s_peakTableVariableLists),
            std::end(s_peakTableVariableLists)});
    skipToNextLdr(istream);
}

sciformats::jdx::PeakTable::PeakTable(
    std::string label, std::string variableList, std::istream& istream)
    : TabularData(std::move(label), std::move(variableList), istream)
{
    validateInput(getLabel(), getVariableList(), s_peakTableLabel,
        std::vector<std::string>{std::begin(s_peakTableVariableLists),
            std::end(s_peakTableVariableLists)});
    skipToNextLdr(istream);
}

std::optional<std::string> sciformats::jdx::PeakTable::getWidthFunction()
{
    util::PeakTableParser parser{getStream(), getNumVariables()};
    return TabularData::getWidthFunction(parser);
}

std::vector<sciformats::jdx::Peak> sciformats::jdx::PeakTable::getData()
{
    util::PeakTableParser parser{getStream(), getNumVariables()};
    return TabularData::getData<util::PeakTableParser, Peak>(parser);
}

size_t sciformats::jdx::PeakTable::getNumVariables()
{
    return getVariableList() == s_peakTableVariableLists.at(0) ? 2U : 3U;
}
