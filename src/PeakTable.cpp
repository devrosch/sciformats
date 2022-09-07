#include "jdx/PeakTable.hpp"
#include "util/LdrUtils.hpp"
#include "util/PeakTableParser.hpp"

#include <algorithm>
#include <tuple>

sciformats::jdx::PeakTable::PeakTable(const std::string& label,
    std::string variableList, TextReader& reader,
    std::optional<std::string>& nextLine)
    : TabularData(label, std::move(variableList), reader)
{
    validateInput(getLabel(), getVariableList(), s_peakTableLabel,
        std::vector<std::string>{std::begin(s_peakTableVariableLists),
            std::end(s_peakTableVariableLists)});
    util::skipToNextLdr(reader, nextLine, true);
}

std::optional<std::string> sciformats::jdx::PeakTable::getWidthFunction()
{
    util::PeakTableParser parser{getReader(), getNumVariables()};
    return TabularData::getWidthFunction(parser);
}

std::vector<sciformats::jdx::Peak> sciformats::jdx::PeakTable::getData()
{
    util::PeakTableParser parser{getReader(), getNumVariables()};
    return TabularData::getData<util::PeakTableParser, Peak>(parser);
}

size_t sciformats::jdx::PeakTable::getNumVariables()
{
    return getVariableList() == s_peakTableVariableLists.at(0) ? 2U : 3U;
}
