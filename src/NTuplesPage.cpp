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
    : m_pageVariables{std::move(pageVar)}
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

std::optional<sciformats::jdx::DataTable>
sciformats::jdx::NTuplesPage::getDataTable()
{
    return m_dataTable;
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

    auto [dataTableVarList, plotDesc] = parseDataTableVars(value);
    m_dataTable.emplace(DataTable(label, dataTableVarList, plotDesc, blockLdrs,
        nTuplesVars, m_pageVariableLdrs, reader, nextLine));
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
            // end of page or start of DATA TABLE
            break;
        }
        // LDR is a regular LDR
        nextLine = parseStringValue(value, reader);
        pageVars.emplace_back(label, value);
    }
    return pageVars;
}

std::pair<std::string, std::optional<std::string>>
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

    if (segments.size() == 1)
    {
        auto varList = util::stripLineComment(segments.at(0)).first;
        util::trim(varList);
        return {varList, std::nullopt};
    }
    // the regex removed the closing parenthesis
    segments.at(0).append(")");
    // plot descriptor is present
    auto varList = segments.at(0);
    util::trim(varList);
    auto plotDesc = util::stripLineComment(segments.at(1)).first;
    util::trim(plotDesc);
    return {varList, plotDesc};
}
