#ifndef LIBJDX_TABULARDATA_HPP
#define LIBJDX_TABULARDATA_HPP

#include "jdx/DataLdr.hpp"
#include "jdx/TextReader.hpp"

#include <functional>
#include <string>
#include <variant>

namespace sciformats::jdx
{
/**
 * @brief Base class for JCAMP-DX PEAK TABLE and PEAK ASSIGNMENTS records.
 */
class TabularData : public DataLdr
{
public:
protected:
    TabularData(
        std::string label, std::string variableList, TextReader& reader);

    /**
     * @brief Provides the parsed peak assignments.
     * @param Parser for the data.
     * @return The list of peak assignments.
     */
    template<typename Parser, typename R> std::vector<R> getData(Parser parser);
    /**
     * @brief Definition of peak width (and other kernel) functions.
     *
     * Comment $$ in line(s) following LDR start may contain peak width and
     * other peak kernel functions
     *
     * @param Parser for the width function.
     * @return Textual description of peak width function.
     */
    template<typename Parser>
    std::optional<std::string> getWidthFunction(Parser parser);
};

template<typename Parser, typename R>
std::vector<R> sciformats::jdx::TabularData::getData(Parser parser)
{
    auto func = [&]() {
        std::vector<R> data{};
        while (parser.hasNext())
        {
            auto nextVariant = parser.next();
            if (std::holds_alternative<std::string>(nextVariant))
            {
                // skip width function
                continue;
            }
            data.push_back(std::get<R>(nextVariant));
        }
        return data;
    };
    return callAndResetStreamPos<std::vector<R>>(func);
}

template<typename Parser>
std::optional<std::string> sciformats::jdx::TabularData::getWidthFunction(
    Parser parser)
{
    auto func = [&]() {
        std::optional<std::string> widthFunction{std::nullopt};
        if (parser.hasNext())
        {
            auto nextVariant = parser.next();
            if (std::holds_alternative<std::string>(nextVariant))
            {
                widthFunction = std::get<std::string>(nextVariant);
            }
        }
        return widthFunction;
    };
    return callAndResetStreamPos<std::optional<std::string>>(func);
}

} // namespace sciformats::jdx

#endif // LIBJDX_TABULARDATA_HPP
