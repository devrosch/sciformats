#include "jdx/NTuplesPage.hpp"
#include "jdx/ParseException.hpp"
#include "util/DataParser.hpp"
#include "util/LdrUtils.hpp"
#include "util/PeakTableParser.hpp"
#include "util/StringUtils.hpp"

sciformats::jdx::NTuplesPage::NTuplesPage(std::string& label,
    std::string pageVar, const std::vector<NTuplesVariables>& nTuplesVars,
    const std::vector<StringLdr>& blockLdrs, TextReader& reader,
    std::optional<std::string>& nextLine)
    : m_reader{reader}
    , m_pageVariables{std::move(pageVar)}
{
    validateInput(label);
    parse(nTuplesVars, blockLdrs, reader, nextLine);
}

void sciformats::jdx::NTuplesPage::validateInput(const std::string& label)
{
    if (label != s_label)
    {
        throw ParseException("Illegal label at " + std::string{s_label}
                             + " start encountered: " + label);
    }
}

std::string sciformats::jdx::NTuplesPage::getPageVariables()
{
    return m_pageVariables;
}

std::vector<sciformats::jdx::StringLdr>
sciformats::jdx::NTuplesPage::getPageVariableLdrs()
{
    return m_pageVariableLdrs;
}

sciformats::jdx::NTuplesPage::VarType
sciformats::jdx::NTuplesPage::getDataTableVariableList()
{
    return m_dataTableVariableList;
}

sciformats::jdx::NTuplesPage::Variables
sciformats::jdx::NTuplesPage::getDataTableVariables()
{
    return m_dataTableVariables;
}

std::optional<sciformats::jdx::NTuplesPage::PlotDescriptor>
sciformats::jdx::NTuplesPage::getDataTablePlotDescriptor()
{
    return m_dataTablePlotDescriptor;
}

std::vector<std::pair<double, double>>
sciformats::jdx::NTuplesPage::getDataTable()
{
    if (m_dataTableVariableList == VarType::XppYY
        || m_dataTableVariableList == VarType::XppRR
        || m_dataTableVariableList == VarType::XppII)
    {
        std::function<std::vector<std::pair<double, double>>()> readData
            = [this]() { return readXppYyData(); };
        // TODO: inline lambda?
        auto xyData = callAndResetStreamPos(readData);
        return xyData;
    }
    if (m_dataTableVariableList == VarType::XYXY)
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

std::vector<std::pair<double, double>>
sciformats::jdx::NTuplesPage::readXppYyData()
{
    auto dataTableParams = getDataTableVariables();
    auto firstX = dataTableParams.xVariables.first.value();
    auto lastX = dataTableParams.xVariables.last.value();
    auto nPoints = dataTableParams.yVariables.varDim.value();
    auto yFactor = dataTableParams.yVariables.factor.value_or(1.0);

    // TODO: duplicate of code in Array2DData
    // parse
    auto yData = util::DataParser::readXppYYData(m_reader);
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
sciformats::jdx::NTuplesPage::readXyXyData()
{
    auto dataTableParams = getDataTableVariables();
    auto xFactor = dataTableParams.xVariables.factor.value_or(1.0);
    auto yFactor = dataTableParams.yVariables.factor.value_or(1.0);

    auto parser = util::PeakTableParser{m_reader, 2};
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

void sciformats::jdx::NTuplesPage::parse(
    const std::vector<NTuplesVariables>& nTuplesVars,
    const std::vector<StringLdr>& blockLdrs, TextReader& reader,
    std::optional<std::string>& nextLine)
{
    // skip potential comment lines
    util::skipToNextLdr(reader, nextLine, true);
    m_pageVariableLdrs = parsePageVarLdrs(reader, nextLine);

    if (!nextLine.has_value() || !util::isLdrStart(nextLine.value()))
    {
        throw ParseException(
            "Unexpected content found while parsing NTUPLES PAGE: "
            + nextLine.value_or("<end of file>"));
    }

    auto [label, value] = util::parseLdrStart(nextLine.value());
    if (label == "PAGE" || label == "ENDNTUPLES" || label == "END")
    {
        // end of page, page is empty
        return;
    }
    if (label != "DATATABLE")
    {
        throw ParseException(
            "Unexpected content found while parsing NTUPLES PAGE: "
            + nextLine.value());
    }

    std::tie(m_dataTableVariableList, m_dataTablePlotDescriptor)
        = parseDataTableVars(value);

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
    if (m_dataTableVariableList == VarType::XppYY
        || m_dataTableVariableList == VarType::XYXY)
    {
        yNTuplesVars = findNTuplesVars("Y");
    }
    else if (m_dataTableVariableList == VarType::XppRR)
    {
        yNTuplesVars = findNTuplesVars("R");
    }
    else if (m_dataTableVariableList == VarType::XppII)
    {
        yNTuplesVars = findNTuplesVars("I");
    }
    else
    {
        // should never happen
        throw ParseException("Unsupported variabe list: " + nextLine.value());
    }
    auto mergedVars
        = mergeVars(blockLdrs, yNTuplesVars.value(), m_pageVariableLdrs);
    m_dataTableVariables = {xNTuplesVars, mergedVars};

    // save stream position for lazy loading of data
    m_dataTablePos = reader.tellg();
    nextLine = reader.readLine();
    util::skipToNextLdr(reader, nextLine);
}

std::vector<sciformats::jdx::StringLdr>
sciformats::jdx::NTuplesPage::parsePageVarLdrs(
    TextReader& reader, std::optional<std::string>& nextLine)
{
    std::vector<StringLdr> pageVars;
    // TODO: similar to parsing logic in Block
    while (nextLine.has_value())
    {
        auto [label, value] = util::parseLdrStart(nextLine.value());
        if (label == "PAGE" || label == "ENDNTUPLES" || label == "END"
            || label == "DATATABLE")
        {
            // end of page
            break;
        }
        // LDR is a regular LDR
        nextLine = parseStringValue(value, reader);
        pageVars.emplace_back(label, value);
    }
    return pageVars;
}

std::pair<sciformats::jdx::NTuplesPage::VarType,
    std::optional<sciformats::jdx::NTuplesPage::PlotDescriptor>>
sciformats::jdx::NTuplesPage::parseDataTableVars(const std::string& rawPageVar)
{
    auto rawPageVarsTrimmed = rawPageVar;
    util::trim(rawPageVarsTrimmed);
    if (rawPageVarsTrimmed.empty())
    {
        // empty
        throw ParseException(
            "Missing variable list in DATA TABLE: " + rawPageVar);
    }
    auto segments = util::split(rawPageVarsTrimmed, R"(\)\s*,\s*)", true);
    if (segments.empty() || segments.size() > 2)
    {
        throw ParseException(
            "Unexpected content found at DATA TABLE start: " + rawPageVar);
    }
    segments.at(0).append(")"); // the regex removed the closing parenthesis

    auto varType = determineVarType(segments.at(0));
    if (segments.size() == 1)
    {
        return std::pair<VarType, std::optional<PlotDescriptor>>{
            varType, std::optional<PlotDescriptor>{std::nullopt}};
    }
    auto plotDesc = determinePlotDescriptor(segments.at(1));
    return std::pair<VarType, std::optional<PlotDescriptor>>{
        varType, std::optional<PlotDescriptor>{plotDesc}};
}

sciformats::jdx::NTuplesPage::VarType
sciformats::jdx::NTuplesPage::determineVarType(const std::string& varList)
{
    const auto* it = std::find_if(s_varTypeMapping.begin(),
        s_varTypeMapping.end(), [&varList](const auto& mappingItem) {
            return mappingItem.first == varList;
        });
    if (it != s_varTypeMapping.end())
    {
        return (*it).second;
    }
    throw ParseException(
        "Unsupported variable type in NTUPLES PAGE: " + varList);
}

sciformats::jdx::NTuplesPage::PlotDescriptor
sciformats::jdx::NTuplesPage::determinePlotDescriptor(
    const std::string& plotDescriptor)
{
    auto noCommentPlotDescriptor = util::stripLineComment(plotDescriptor).first;
    util::trim(noCommentPlotDescriptor);
    // TODO: very similar to determineVarType
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

sciformats::jdx::NTuplesVariables sciformats::jdx::NTuplesPage::mergeVars(
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
