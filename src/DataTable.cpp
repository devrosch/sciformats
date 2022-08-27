#include "jdx/DataTable.hpp"
#include "jdx/ParseException.hpp"
#include "util/DataParser.hpp"
#include "util/LdrUtils.hpp"
#include "util/PeakTableParser.hpp"
#include "util/StringUtils.hpp"

sciformats::jdx::DataTable::DataTable(std::string& label,
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
    if (variableList == VariableList::XppYY
        || variableList == VariableList::XppRR
        || variableList == VariableList::XppII)
    {
        std::function<std::vector<std::pair<double, double>>()> readData
            = [this]() { return readXppYyData(); };
        // TODO: inline lambda?
        auto xyData = callAndResetStreamPos(readData);
        return xyData;
    }
    if (variableList == VariableList::XyXy)
    {
        std::function<std::vector<std::pair<double, double>>()> readData
            = [this]() { return readXyXyData(); };
        // TODO: inline lambda?
        auto xyData = callAndResetStreamPos(readData);
        return xyData;
    }
    // should never happen
    throw ParseException("Unsupported variable list in PAGE's DATA TABLE.");
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
    auto mergedVars = mergeVars(blockLdrs, yNTuplesVars.value(), pageLdrs);
    m_mergedVariables = {xNTuplesVars, mergedVars};

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
    const auto* it = std::find_if(s_varListMapping.begin(),
        s_varListMapping.end(), [&varList](const auto& mappingItem) {
            return mappingItem.first == varList;
        });
    if (it != s_varListMapping.end())
    {
        return (*it).second;
    }
    throw ParseException(
        "Unsupported variable type in NTUPLES PAGE: " + varList);
}

sciformats::jdx::DataTable::PlotDescriptor
sciformats::jdx::DataTable::determinePlotDescriptor(
    const std::string& plotDescriptor)
{
    auto noCommentPlotDescriptor = util::stripLineComment(plotDescriptor).first;
    util::trim(noCommentPlotDescriptor);
    // TODO: very similar to determineVariableList
    const auto* it = std::find_if(s_plotDescriptorMapping.begin(),
        s_plotDescriptorMapping.end(),
        [&noCommentPlotDescriptor](const auto& mappingItem) {
            return mappingItem.first == noCommentPlotDescriptor;
        });
    if (it != s_plotDescriptorMapping.end())
    {
        return (*it).second;
    }
    throw ParseException(
        "Illegal plot descriptor in NTUPLES PAGE: " + noCommentPlotDescriptor);
}

sciformats::jdx::NTuplesVariables sciformats::jdx::DataTable::mergeVars(
    const std::vector<StringLdr>& blockLdrs,
    const NTuplesVariables& nTuplesVars, const std::vector<StringLdr>& pageLdrs)
{
    auto ySymbols = {"Y", "R", "I"};
    auto outputVars = nTuplesVars;

    // fill in block vars for missing NTUPLE vars
    if (nTuplesVars.symbol == "X")
    {
        // use values from block relevant for abscissa
        // UNITS <-> XUNITS
        // FIRST <-> FIRSTX
        // LAST <-> LASTX
        // MIN <-> MINX
        // MAX <-> MAXX
        // FACTOR <-> XFACTOR
        for (const auto& blockLdr : blockLdrs)
        {
            if ("XUNITS" == blockLdr.getLabel()
                && (!outputVars.units || outputVars.units.value().empty()))
            {
                outputVars.units = blockLdr.getValue();
            }
            else if ("FIRSTX" == blockLdr.getLabel() && !outputVars.first)
            {
                outputVars.first = std::stod(blockLdr.getValue());
            }
            else if ("LASTX" == blockLdr.getLabel() && !outputVars.last)
            {
                outputVars.last = std::stod(blockLdr.getValue());
            }
            else if ("MINX" == blockLdr.getLabel() && !outputVars.min)
            {
                outputVars.min = std::stod(blockLdr.getValue());
            }
            else if ("MAXX" == blockLdr.getLabel() && !outputVars.max)
            {
                outputVars.max = std::stod(blockLdr.getValue());
            }
            else if ("XFACTOR" == blockLdr.getLabel() && !outputVars.factor)
            {
                outputVars.factor = std::stod(blockLdr.getValue());
            }
        }

        // replace with page LDRs if applicable
        for (const auto& pageLdr : pageLdrs)
        {
            if ("XUNITS" == pageLdr.getLabel())
            {
                outputVars.units = pageLdr.getValue();
            }
            else if ("FIRSTX" == pageLdr.getLabel())
            {
                outputVars.first = std::stod(pageLdr.getValue());
            }
            else if ("LASTX" == pageLdr.getLabel())
            {
                outputVars.last = std::stod(pageLdr.getValue());
            }
            else if ("MINX" == pageLdr.getLabel())
            {
                outputVars.min = std::stod(pageLdr.getValue());
            }
            else if ("MAXX" == pageLdr.getLabel())
            {
                outputVars.max = std::stod(pageLdr.getValue());
            }
            else if ("XFACTOR" == pageLdr.getLabel())
            {
                outputVars.factor = std::stod(pageLdr.getValue());
            }
            else
            {
                outputVars.applicationAttributes.push_back(pageLdr);
            }
        }
    }
    else if (std::any_of(ySymbols.begin(), ySymbols.end(),
                 [&nTuplesVars](
                     const std::string& s) { return s == nTuplesVars.symbol; }))
    {
        // use values from block relevant for abscissa
        // UNITS <-> YUNITS or <symbol>UNITS
        // FIRST <-> FIRSTY or FIRST<symbol>
        // LAST <-> LASTY or LAST<symbol>
        // MIN <-> MINY or MIN<symbol>
        // MAX <-> MAXY or MAX<symbol>
        // FACTOR <-> YFACTOR
        // TODO: also check for other symbols but Y? Does not seem relevant for
        // NMR and MS.
        for (const auto& blockLdr : blockLdrs)
        {
            if ("YUNITS" == blockLdr.getLabel()
                && (!outputVars.units || outputVars.units.value().empty()))
            {
                outputVars.units = blockLdr.getValue();
            }
            else if ("FIRSTY" == blockLdr.getLabel() && !outputVars.first)
            {
                outputVars.first = std::stod(blockLdr.getValue());
            }
            else if ("LASTY" == blockLdr.getLabel() && !outputVars.last)
            {
                outputVars.last = std::stod(blockLdr.getValue());
            }
            else if ("MINY" == blockLdr.getLabel() && !outputVars.min)
            {
                outputVars.min = std::stod(blockLdr.getValue());
            }
            else if ("MAXY" == blockLdr.getLabel() && !outputVars.max)
            {
                outputVars.max = std::stod(blockLdr.getValue());
            }
            else if ("YFACTOR" == blockLdr.getLabel() && !outputVars.factor)
            {
                outputVars.factor = std::stod(blockLdr.getValue());
            }
        }

        // replace with page LDRs if applicable
        for (const auto& pageLdr : pageLdrs)
        {
            if ("YUNITS" == pageLdr.getLabel())
            {
                outputVars.units = pageLdr.getValue();
            }
            else if ("FIRSTY" == pageLdr.getLabel())
            {
                outputVars.first = std::stod(pageLdr.getValue());
            }
            else if ("LASTY" == pageLdr.getLabel())
            {
                outputVars.last = std::stod(pageLdr.getValue());
            }
            else if ("MINY" == pageLdr.getLabel())
            {
                outputVars.min = std::stod(pageLdr.getValue());
            }
            else if ("MAXY" == pageLdr.getLabel())
            {
                outputVars.max = std::stod(pageLdr.getValue());
            }
            else if ("YFACTOR" == pageLdr.getLabel())
            {
                outputVars.factor = std::stod(pageLdr.getValue());
            }
            else
            {
                outputVars.applicationAttributes.push_back(pageLdr);
            }
        }
    }
    else
    {
        throw ParseException("Unexpected symbol found during parsing of PAGE: "
                             + nTuplesVars.symbol);
    }
    // replacements independent of abscissa/ordinate
    // VAR_DIM <-> NPOINTS
    if (!nTuplesVars.varDim)
    {
        for (const auto& blockLdr : blockLdrs)
        {
            if ("NPOINTS" == blockLdr.getLabel())
            {
                outputVars.varDim = std::stod(blockLdr.getValue());
                break;
            }
        }
    }

    return outputVars;
}

std::vector<std::pair<double, double>>
sciformats::jdx::DataTable::readXppYyData()
{
    auto dataTableParams = m_mergedVariables;
    auto firstX = dataTableParams.xVariables.first.value();
    auto lastX = dataTableParams.xVariables.last.value();
    auto nPoints = dataTableParams.yVariables.varDim.value();
    auto yFactor = dataTableParams.yVariables.factor.value_or(1.0);

    // TODO: duplicate of code in Array2DData
    // parse
    auto yData = util::DataParser::readXppYYData(getReader());
    if (yData.size() != nPoints)
    {
        throw ParseException("Mismatch betwee NPOINTS and actual number of "
                             "points in PAGE. VAR_DIM: "
                             + std::to_string(nPoints)
                             + ", actual: " + std::to_string(yData.size()));
    }
    // prepare processing
    std::vector<std::pair<double, double>> xyData{};
    xyData.reserve(yData.size());
    // cover special cases nPoints == 0 and nPoints == 1
    if (nPoints == 0)
    {
        return xyData;
    }
    auto nominator = nPoints == 1 ? firstX : (lastX - firstX);
    auto denominator = nPoints == 1 ? 1 : nPoints - 1;
    // generate and return xy data
    uint64_t count = 0;
    for (auto yRaw : yData)
    {
        auto x = firstX + nominator / denominator * count++;
        auto y = yFactor * yRaw;
        xyData.emplace_back(x, y);
    }
    return xyData;
}

std::vector<std::pair<double, double>>
sciformats::jdx::DataTable::readXyXyData()
{
    auto dataTableParams = m_mergedVariables;
    auto xFactor = dataTableParams.xVariables.factor.value_or(1.0);
    auto yFactor = dataTableParams.yVariables.factor.value_or(1.0);

    auto parser = util::PeakTableParser{getReader(), 2};
    std::vector<std::pair<double, double>> xyData{};
    while (parser.hasNext())
    {
        auto next = parser.next();
        if (!std::holds_alternative<Peak>(next))
        {
            auto peak = std::get<Peak>(next);
            xyData.emplace_back(peak.x * xFactor, peak.y * yFactor);
            continue;
        }
        if (!xyData.empty())
        {
            throw ParseException("Unexpected content in DATA TABLE: "
                                 + std::get<std::string>(next));
        }
        // if neither previous condition is true, this is a pure comment at the
        // start of DATA TABLE => continue
    }
    return xyData;
}
