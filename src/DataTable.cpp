#include "jdx/DataTable.hpp"
#include "jdx/ParseException.hpp"
#include "util/DataParser.hpp"
#include "util/LdrUtils.hpp"
#include "util/PeakTableParser.hpp"
#include "util/StringUtils.hpp"

sciformats::jdx::DataTable::DataTable(std::string label,
    std::string variableList, std::optional<std::string> plotDescriptor,
    const std::vector<StringLdr>& blockLdrs,
    const std::vector<NTuplesVariables>& nTuplesVars,
    const std::vector<StringLdr>& pageLdrs, TextReader& reader,
    std::optional<std::string>& nextLine)
    : Array2DData(std::move(label), std::move(variableList), reader)
    , m_plotDescriptor{std::move(plotDescriptor)}
{
    // extract permitted variable lists from mapping keys
    std::vector<std::string> permittedVarLists;
    permittedVarLists.reserve(s_varListMapping.size());
    for (const auto& [key, _] : s_varListMapping)
    {
        permittedVarLists.emplace_back(key);
    }
    // validate label and variable list
    validateInput(getLabel(), getVariableList(), s_label, permittedVarLists);
    // validate plot descriptor if present
    if (m_plotDescriptor)
    {
        determinePlotDescriptor(m_plotDescriptor.value());
    }
    // parse
    parse(blockLdrs, nTuplesVars, pageLdrs, nextLine);
}

std::optional<std::string> sciformats::jdx::DataTable::getPlotDescriptor()
{
    return m_plotDescriptor;
}

sciformats::jdx::DataTable::Variables sciformats::jdx::DataTable::getVariables()
{
    return m_mergedVariables;
}

std::vector<std::pair<double, double>> sciformats::jdx::DataTable::getData()
{
    auto variableList = determineVariableList(getVariableList());
    auto dataTableParams = m_mergedVariables;

    if (variableList == VariableList::XyXy)
    {
        auto xFactor = dataTableParams.xVariables.factor.value_or(1.0);
        auto yFactor = dataTableParams.yVariables.factor.value_or(1.0);
        auto nPoints = dataTableParams.yVariables.varDim;
        return Array2DData::parseXyXyData(
            getLabel(), getReader(), xFactor, yFactor, nPoints, variableList);
    }

    auto firstX = dataTableParams.xVariables.first.value();
    auto lastX = dataTableParams.xVariables.last.value();
    auto nPoints = dataTableParams.yVariables.varDim.value();
    auto yFactor = dataTableParams.yVariables.factor.value_or(1.0);
    return Array2DData::parseXppYYData(
        getLabel(), getReader(), firstX, lastX, yFactor, nPoints, variableList);
}

void sciformats::jdx::DataTable::parse(const std::vector<StringLdr>& blockLdrs,
    const std::vector<NTuplesVariables>& nTuplesVars,
    const std::vector<StringLdr>& pageLdrs,
    std::optional<std::string>& nextLine)
{
    auto [variableList, plotDescriptor] = parseDataTableVars();

    auto findNTuplesVars = [&nTuplesVars](const std::string& symbol) {
        auto it = std::find_if(std::begin(nTuplesVars), std::end(nTuplesVars),
            [&symbol](const NTuplesVariables& vars) {
                return vars.symbol == symbol;
            });
        if (it != std::end(nTuplesVars))
        {
            return *it;
        }
        throw ParseException(
            "Could not find NTUPLES parameters for SYMBOL: " + symbol);
    };

    auto xNTuplesVars = findNTuplesVars("X");
    std::optional<NTuplesVariables> yNTuplesVars;
    if (variableList == VariableList::XppYY
        || variableList == VariableList::XyXy)
    {
        yNTuplesVars = findNTuplesVars("Y");
    }
    else if (variableList == VariableList::XppRR)
    {
        yNTuplesVars = findNTuplesVars("R");
    }
    else if (variableList == VariableList::XppII)
    {
        yNTuplesVars = findNTuplesVars("I");
    }
    else
    {
        // should never happen
        throw ParseException(
            "Unsupported variabe list in DATA TABLE: " + getVariableList());
    }
    auto mergedXVars = mergeVars(blockLdrs, xNTuplesVars, pageLdrs);
    auto mergedYVars = mergeVars(blockLdrs, yNTuplesVars.value(), pageLdrs);
    m_mergedVariables = {mergedXVars, mergedYVars};

    auto& reader = getReader();
    nextLine = reader.readLine();
    util::skipToNextLdr(reader, nextLine);
}

std::pair<sciformats::jdx::Array2DData::VariableList,
    std::optional<sciformats::jdx::DataTable::PlotDescriptor>>
sciformats::jdx::DataTable::parseDataTableVars()
{
    auto varType = determineVariableList(getVariableList());
    if (!m_plotDescriptor.has_value() || m_plotDescriptor.value().empty())
    {
        return std::pair<VariableList, std::optional<PlotDescriptor>>{
            varType, std::optional<PlotDescriptor>{std::nullopt}};
    }
    auto plotDesc = determinePlotDescriptor(m_plotDescriptor.value());
    return std::pair<VariableList, std::optional<PlotDescriptor>>{
        varType, std::optional<PlotDescriptor>{plotDesc}};
}

sciformats::jdx::Array2DData::VariableList
sciformats::jdx::DataTable::determineVariableList(const std::string& varList)
{
    return findValue(s_varListMapping, varList, "variable list");
}

sciformats::jdx::DataTable::PlotDescriptor
sciformats::jdx::DataTable::determinePlotDescriptor(
    const std::string& plotDescriptor)
{
    return findValue(
        s_plotDescriptorMapping, plotDescriptor, "plot descriptor");
}

sciformats::jdx::NTuplesVariables sciformats::jdx::DataTable::mergeVars(
    const std::vector<StringLdr>& blockLdrs,
    const NTuplesVariables& nTuplesVars, const std::vector<StringLdr>& pageLdrs)
{
    auto outputVars = nTuplesVars;
    outputVars.applicationAttributes.clear();

    if (nTuplesVars.symbol == s_xSymbol)
    {
        // use values from block relevant for abscissa
        std::map<std::string, std::optional<std::string>&> stringMapping{
            {"XUNITS", outputVars.units},
        };
        std::map<std::string, std::optional<double>&> doubleMapping{
            {"FIRSTX", outputVars.first},
            {"LASTX", outputVars.last},
            {"MINX", outputVars.min},
            {"MAXX", outputVars.max},
            {"XFACTOR", outputVars.factor},
        };
        std::map<std::string, std::optional<uint64_t>&> uint64Mapping{
            {"NPOINTS", outputVars.varDim},
        };

        // fill in block vars for missing NTUPLE vars
        mergeLdrs(
            blockLdrs, stringMapping, doubleMapping, uint64Mapping, false);

        // replace with page LDR values if available
        mergeLdrs(pageLdrs, stringMapping, doubleMapping, uint64Mapping, true);
    }
    else if (std::any_of(s_ySymbols.begin(), s_ySymbols.end(),
                 [&nTuplesVars](
                     const std::string& s) { return s == nTuplesVars.symbol; }))
    {
        // use values from block relevant for ordinate
        std::map<std::string, std::optional<std::string>&> stringMapping{
            {"YUNITS", outputVars.units},
        };
        std::map<std::string, std::optional<double>&> doubleMapping{
            {"FIRSTY", outputVars.first},
            {"LASTY", outputVars.last},
            {"MINY", outputVars.min},
            {"MAXY", outputVars.max},
            {"YFACTOR", outputVars.factor},
        };
        std::map<std::string, std::optional<uint64_t>&> uint64Mapping{
            {"NPOINTS", outputVars.varDim},
        };
        // Also check for other symbols but Y? Does not seem relevant for NMR
        // and MS.

        // fill in block vars for missing NTUPLE vars
        mergeLdrs(
            blockLdrs, stringMapping, doubleMapping, uint64Mapping, false);

        // replace with page LDR values if available
        mergeLdrs(pageLdrs, stringMapping, doubleMapping, uint64Mapping, true);
    }
    else
    {
        throw ParseException("Unexpected symbol found during parsing of PAGE: "
                             + nTuplesVars.symbol);
    }

    return outputVars;
}

void sciformats::jdx::DataTable::mergeLdrs(const std::vector<StringLdr>& ldrs,
    std::map<std::string, std::optional<std::string>&> stringMapping,
    std::map<std::string, std::optional<double>&> doubleMapping,
    std::map<std::string, std::optional<uint64_t>&> uint64Mapping, bool replace)
{
    for (const auto& ldr : ldrs)
    {
        if (stringMapping.count(ldr.getLabel()) > 0)
        {
            auto& field = stringMapping.at(ldr.getLabel());
            if (replace || !field || field.value().empty())
            {
                field = ldr.getValue();
            }
        }
        else if (doubleMapping.count(ldr.getLabel()) > 0)
        {
            auto& field = doubleMapping.at(ldr.getLabel());
            if (replace || !field)
            {
                field = std::stod(ldr.getValue());
            }
        }
        else if (uint64Mapping.count(ldr.getLabel()) > 0)
        {
            auto& field = uint64Mapping.at(ldr.getLabel());
            if (replace || !field)
            {
                field = std::stol(ldr.getValue());
            }
        }
    }
}
