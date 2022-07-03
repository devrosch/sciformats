#include "jdx/NTuplesPage.hpp"
#include "util/LdrUtils.hpp"
#include "util/StringUtils.hpp"
#include "jdx/ParseException.hpp"

sciformats::jdx::NTuplesPage::NTuplesPage(std::string& label, std::string pageVar, const std::vector<NTuplesVariables>& nTuplesVars, const std::vector<StringLdr>& blockLdrs, TextReader& reader, std::optional<std::string>& nextLine)
    : m_pageVar{std::move(pageVar)}
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

void sciformats::jdx::NTuplesPage::parse(const std::vector<NTuplesVariables>& nTuplesVars, const std::vector<StringLdr>& blockLdrs, TextReader& reader, std::optional<std::string>& nextLine)
{
    // skip potential comment lines
    nextLine = util::skipToNextLdr(reader, nextLine, true);
    std::vector<StringLdr> pageVars;
    // TODO: similar to parsing logic in Block
    while (nextLine.has_value())
    {
        // "auto [label, value] = util::parseLdrStart(nextLine.value());" cannot
        // be used as lambdas (below) cannot capture these variables
        // see:
        // https://stackoverflow.com/questions/46114214/lambda-implicit-capture-fails-with-variable-declared-from-structured-binding
        std::string label;
        std::string value;
        std::tie(label, value) = util::parseLdrStart(nextLine.value());
        if (label == "PAGE" || label == "ENDNTUPLES" || label == "END")
        {
            // end of page
            break;
        }
        if (label == "DATATABLE")
        {
            // TODO: parse
            // TODO: remove
            nextLine = reader.readLine();
            util::skipToNextLdr(reader, nextLine);
        }
        else {
            // LDR is a regular LDR
            nextLine = parseStringValue(value, reader);
            pageVars.emplace_back(label, value);
        }
    }

    // TODO: merge pageVars with nTupleVars and blockVars

    // TODO: dummy, continue
    util::skipToNextLdr(reader, nextLine);
}
