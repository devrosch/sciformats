#include "jdx/NTuples.hpp"
#include "jdx/ParseException.hpp"
#include "util/LdrUtils.hpp"
#include "util/StringUtils.hpp"

sciformats::jdx::NTuples::NTuples(const std::string& label,
    std::string dataForm, TextReader& reader,
    const std::vector<StringLdr>& blockLdrs)
    : m_dataForm{std::move(dataForm)}
{
    validateInput(label);
    parse(blockLdrs, reader);
}

std::string sciformats::jdx::NTuples::getDataForm()
{
    return m_dataForm;
}

std::vector<sciformats::jdx::NTuplesVariables>
sciformats::jdx::NTuples::getVariables()
{
    return m_variables;
}

size_t sciformats::jdx::NTuples::getNumPages()
{
    return m_pages.size();
}

sciformats::jdx::NTuplesPage sciformats::jdx::NTuples::getPage(size_t pageIndex)
{
    return m_pages.at(pageIndex);
}

void sciformats::jdx::NTuples::validateInput(const std::string& label)
{
    if (label != s_label)
    {
        throw ParseException("Illegal label at " + std::string{s_label}
                             + " start encountered: " + label);
    }
}

void sciformats::jdx::NTuples::parse(
    const std::vector<StringLdr>& blockLdrs, TextReader& reader)
{
    std::optional<std::string> nextLine = reader.readLine();
    // TODO: skip $$
    m_variables = parseVariables(reader, nextLine);
    // parse pages
    while (nextLine.has_value() && util::isLdrStart(nextLine.value()))
    {
        auto [label, pageVar] = util::parseLdrStart(nextLine.value());
        pageVar = util::stripLineComment(pageVar).first;
        util::trim(pageVar);
        if (label != "PAGE")
        {
            break;
        }
        nextLine = reader.readLine();
        auto page = NTuplesPage(
            label, pageVar, m_variables, blockLdrs, reader, nextLine);
        m_pages.push_back(std::move(page));
    }
}

std::vector<sciformats::jdx::NTuplesVariables>
sciformats::jdx::NTuples::parseVariables(
    TextReader& reader, std::optional<std::string>& nextLine)
{
    // skip potential pure comment lines, assumes that data form info is in one
    // line only
    nextLine = util::skipToNextLdr(reader, nextLine, true);
    auto varTable = readVariables(nextLine, reader);
    auto varMap = splitValues(varTable);
    auto standardVarMap = extractStandardVariables(varMap);

    auto varNames = findValue("VARNAME", standardVarMap);
    if (!varNames)
    {
        // VARNAMEs are required by the spec
        throw ParseException(
            "No \"VAR_NAME\" LDR found in NTUPLES: " + m_dataForm);
    }
    auto numVars = varNames.value().size();

    std::vector<NTuplesVariables> output;
    for (size_t i = 0; i < numVars; ++i)
    {
        auto ntv = map(standardVarMap, varMap, i);
        output.push_back(ntv);
    }
    return output;
}

std::vector<sciformats::jdx::StringLdr> sciformats::jdx::NTuples::readVariables(
    std::optional<std::string>& firstVarStart,
    sciformats::jdx::TextReader& reader)
{
    std::optional<std::string>& nextLine = firstVarStart;
    std::vector<StringLdr> output;
    while (nextLine)
    {
        auto [title, value] = util::parseLdrStart(nextLine.value());
        if (title == "PAGE" || title == "ENDNTUPLES" || title == "END")
        {
            // all NTUPLES variables read
            break;
        }
        nextLine = parseStringValue(value, reader);
        output.emplace_back(title, value);
    }
    return output;
}

std::map<std::string, std::vector<std::string>>
sciformats::jdx::NTuples::splitValues(const std::vector<StringLdr>& vars)
{
    std::map<std::string, std::vector<std::string>> output;
    for (const auto& var : vars)
    {
        auto values = util::split(var.getValue(), ",", true);
        auto inserted = output.emplace(var.getLabel(), values).second;
        if (!inserted)
        {
            throw ParseException(
                "Duplicate variable found in NTUPLE: " + var.getLabel());
        }
    }
    return output;
}

std::map<std::string, std::vector<std::string>>
sciformats::jdx::NTuples::extractStandardVariables(
    std::map<std::string, std::vector<std::string>>& vars)
{
    // see:
    // https://stackoverflow.com/questions/180516/how-to-filter-items-from-a-stdmap
    auto matches = [](const std::string& var) {
        return std::find(std::begin(s_variables), std::end(s_variables), var)
               != std::end(s_variables);
    };
    std::map<std::string, std::vector<std::string>> standardVars;
    // remove standard vars
    for (auto it = vars.begin(); it != vars.end();)
    {
        if (matches((*it).first))
        {
            standardVars.insert(*it);
            vars.erase(it++);
        }
        else
        {
            ++it;
        }
    }
    return standardVars;
}

sciformats::jdx::NTuplesVariables sciformats::jdx::NTuples::map(
    const std::map<std::string, std::vector<std::string>>& standardVars,
    const std::map<std::string, std::vector<std::string>>& additionalVars,
    size_t valueColumnIndex)
{
    auto findColumnValue = [this, &standardVars = std::as_const(standardVars)](
                               const std::string& key, size_t columnIndex) {
        auto values = findValue(key, standardVars);
        if (!values)
        {
            return std::optional<std::string>{std::nullopt};
        }
        if (values.value().size() <= columnIndex)
        {
            throw ParseException("For variable \"" + key + "\" in NTUPLES \""
                                 + m_dataForm
                                 + "\" value not available at column: "
                                 + std::to_string(columnIndex));
        }
        auto value = values.value().at(columnIndex);
        util::trim(value);
        return value.empty()
                   ? std::optional<std::string>{std::nullopt}
                   : std::optional<std::string>{values.value().at(columnIndex)};
    };

    auto varNameString = findColumnValue("VARNAME", valueColumnIndex);
    if (!varNameString)
    {
        // VARNAMEs are required by the spec
        throw ParseException(
            R"(No "VAR_NAME" LDR found in NTUPLES ")" + m_dataForm
            + "\" column: " + std::to_string(valueColumnIndex));
    }
    auto symbolString = findColumnValue("SYMBOL", valueColumnIndex);
    auto varTypeString = findColumnValue("VARTYPE", valueColumnIndex);
    auto varFormString = findColumnValue("VARFORM", valueColumnIndex);
    auto varDimString = findColumnValue("VARDIM", valueColumnIndex);
    auto unitsString = findColumnValue("UNITS", valueColumnIndex);
    auto firstString = findColumnValue("FIRST", valueColumnIndex);
    auto lastString = findColumnValue("LAST", valueColumnIndex);
    auto minString = findColumnValue("MIN", valueColumnIndex);
    auto maxString = findColumnValue("MAX", valueColumnIndex);
    auto factorString = findColumnValue("FACTOR", valueColumnIndex);

    // TODO: make stol/stod more reliable (new util function) and use across
    // project
    auto varName = varNameString.value();
    auto symbol = symbolString.value();
    auto& varType = varTypeString;
    auto& varForm = varFormString;
    auto varDim = varDimString
                      ? std::optional<uint64_t>(std::stol(varDimString.value()))
                      : std::nullopt;
    auto& units = unitsString;
    auto first = firstString
                     ? std::optional<double>(std::stod(firstString.value()))
                     : std::nullopt;
    auto last = lastString
                    ? std::optional<double>(std::stod(lastString.value()))
                    : std::nullopt;
    auto min = minString ? std::optional<double>(std::stod(minString.value()))
                         : std::nullopt;
    auto max = maxString ? std::optional<double>(std::stod(maxString.value()))
                         : std::nullopt;
    auto factor = factorString
                      ? std::optional<double>(std::stod(factorString.value()))
                      : std::nullopt;
    std::vector<StringLdr> additionalVariables;
    std::transform(additionalVars.begin(), additionalVars.end(),
        std::back_inserter(additionalVariables),
        [valueColumnIndex](
            std::pair<std::string, std::vector<std::string>> var) {
            return StringLdr{var.first, var.second.at(valueColumnIndex)};
        });

    NTuplesVariables nTupleVars{
        varName,
        symbol,
        varType,
        varForm,
        varDim,
        units,
        first,
        last,
        min,
        max,
        factor,
        additionalVariables,
    };

    return nTupleVars;
}

std::optional<std::vector<std::string>> sciformats::jdx::NTuples::findValue(
    const std::string& key,
    const std::map<std::string, std::vector<std::string>>& map)
{
    const auto it = map.find(key);
    if (it != map.end())
    {
        return std::optional<std::vector<std::string>>{(*it).second};
    }
    return std::optional<std::vector<std::string>>{std::nullopt};
}
