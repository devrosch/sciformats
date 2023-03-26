#include "jdx/DataTable.hpp"
#include "jdx/ParseException.hpp"
#include "util/DataParser.hpp"
#include "util/LdrUtils.hpp"
#include "util/PeakTableParser.hpp"
#include "util/StringUtils.hpp"

#include <limits>

sciformats::jdx::DataTable::DataTable(std::string label,
    std::string variableList, std::optional<std::string> plotDescriptor,
    const std::vector<StringLdr>& blockLdrs,
    const std::vector<NTuplesAttributes>& nTuplesAttributes,
    const std::vector<StringLdr>& pageLdrs, TextReader& reader,
    std::optional<std::string>& nextLine)
    : Data2D(std::move(label), std::move(variableList), reader)
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
    parse(blockLdrs, nTuplesAttributes, pageLdrs, nextLine);
}

const std::optional<std::string>&
sciformats::jdx::DataTable::getPlotDescriptor() const
{
    return m_plotDescriptor;
}

const sciformats::jdx::DataTable::Attributes&
sciformats::jdx::DataTable::getAttributes() const
{
    return m_mergedAttributes;
}

std::vector<std::pair<double, double>>
sciformats::jdx::DataTable::getData() const
{
    auto variableList = determineVariableList(getVariableList());
    auto dataTableParams = m_mergedAttributes;

    if (variableList == VariableList::XYXY || variableList == VariableList::XRXR
        || variableList == VariableList::XIXI)
    {
        auto xFactor = dataTableParams.xAttributes.factor.value_or(1.0);
        auto yFactor = dataTableParams.yAttributes.factor.value_or(1.0);
        auto nPoints = dataTableParams.yAttributes.varDim;
        return Data2D::parseXyXyData(
            getLabel(), getReader(), xFactor, yFactor, nPoints);
    }

    auto firstX = dataTableParams.xAttributes.first.value();
    auto lastX = dataTableParams.xAttributes.last.value();
    auto nPoints = dataTableParams.yAttributes.varDim.value();
    auto yFactor = dataTableParams.yAttributes.factor.value_or(1.0);
    return Data2D::parseXppYYData(
        getLabel(), getReader(), firstX, lastX, yFactor, nPoints);
}

void sciformats::jdx::DataTable::parse(const std::vector<StringLdr>& blockLdrs,
    const std::vector<NTuplesAttributes>& nTuplesVars,
    const std::vector<StringLdr>& pageLdrs,
    std::optional<std::string>& nextLine)
{
    auto [variableList, plotDescriptor] = parseDataTableVars();

    auto findNTuplesIndex = [&nTuplesVars](const std::string& symbol) {
        for (size_t i = 0; i < nTuplesVars.size(); ++i)
        {
            if (nTuplesVars.at(i).symbol == symbol)
            {
                return i;
            }
        }
        throw ParseException(
            "Could not find NTUPLES parameters for SYMBOL: " + symbol);
    };

    auto xColumnIndex = std::numeric_limits<size_t>::max();
    auto yColumnIndex = std::numeric_limits<size_t>::max();
    if (variableList == VariableList::XppYY
        || variableList == VariableList::XYXY)
    {
        xColumnIndex = findNTuplesIndex("X");
        yColumnIndex = findNTuplesIndex("Y");
    }
    else if (variableList == VariableList::XppRR)
    {
        xColumnIndex = findNTuplesIndex("X");
        yColumnIndex = findNTuplesIndex("R");
    }
    else if (variableList == VariableList::XppII)
    {
        xColumnIndex = findNTuplesIndex("X");
        yColumnIndex = findNTuplesIndex("I");
    }
    else if (variableList == VariableList::T2ppRR)
    {
        xColumnIndex = findNTuplesIndex("T2");
        yColumnIndex = findNTuplesIndex("R");
    }
    else if (variableList == VariableList::T2ppII)
    {
        xColumnIndex = findNTuplesIndex("T2");
        yColumnIndex = findNTuplesIndex("I");
    }
    else if (variableList == VariableList::F2ppYY)
    {
        xColumnIndex = findNTuplesIndex("F2");
        yColumnIndex = findNTuplesIndex("Y");
    }
    else
    {
        // should never happen
        throw ParseException(
            "Unsupported variabe list in DATA TABLE: " + getVariableList());
    }

    auto xNTuplesAttrs = nTuplesVars.at(xColumnIndex);
    auto yNTuplesAttrs = nTuplesVars.at(yColumnIndex);

    auto mergedXVars = mergeVars(blockLdrs, xNTuplesAttrs, pageLdrs);
    auto mergedYVars = mergeVars(blockLdrs, yNTuplesAttrs, pageLdrs);

    // special treatment for "FIRST" page LDR if present
    // this is described in the README for the JCAMP-DX nD-NMR test file round
    // robin
    mergePageFirstLdr(mergedXVars, pageLdrs, xColumnIndex);
    mergePageFirstLdr(mergedYVars, pageLdrs, yColumnIndex);

    m_mergedAttributes = {mergedXVars, mergedYVars};

    auto& reader = getReader();
    util::skipToNextLdr(reader, nextLine, true);
}

std::pair<sciformats::jdx::Data2D::VariableList,
    std::optional<sciformats::jdx::DataTable::PlotDescriptor>>
sciformats::jdx::DataTable::parseDataTableVars()
{
    auto varType = determineVariableList(getVariableList());
    if (!m_plotDescriptor.has_value() || m_plotDescriptor.value().empty())
    {
        return {varType, std::nullopt};
    }
    auto plotDesc = determinePlotDescriptor(m_plotDescriptor.value());
    return {varType, plotDesc};
}

sciformats::jdx::Data2D::VariableList
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

sciformats::jdx::NTuplesAttributes sciformats::jdx::DataTable::mergeVars(
    const std::vector<StringLdr>& blockLdrs,
    const NTuplesAttributes& nTuplesVars,
    const std::vector<StringLdr>& pageLdrs)
{
    auto outputVars = nTuplesVars;
    outputVars.applicationAttributes.clear();

    if (std::any_of(s_xSymbols.begin(), s_xSymbols.end(),
            [&nTuplesVars](
                const std::string& s) { return s == nTuplesVars.symbol; }))
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

        // fill in block params for missing NTUPLE attributes
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

        // fill in block params for missing NTUPLE attributes
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

void sciformats::jdx::DataTable::mergePageFirstLdr(
    sciformats::jdx::NTuplesAttributes& mergedVars,
    const std::vector<StringLdr>& pageLdrs, size_t columnIndex)
{
    for (const auto& ldr : pageLdrs)
    {
        if ("FIRST" == ldr.getLabel())
        {
            auto segments = util::split(ldr.getValue(), ",", true);
            if (segments.size() > columnIndex)
            {
                auto value = std::stod(segments.at(columnIndex));
                mergedVars.first = value;
            }
            break;
        }
    }
}
