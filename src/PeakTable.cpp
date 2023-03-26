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

std::vector<sciformats::jdx::Peak> sciformats::jdx::PeakTable::getData() const
{
    util::PeakTableParser parser{getReader(), getVariableList()};
    return TabularData::getData<util::PeakTableParser, Peak>(parser);
}
