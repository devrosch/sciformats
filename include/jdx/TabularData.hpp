#ifndef LIBJDX_TABULARDATA_HPP
#define LIBJDX_TABULARDATA_HPP

#include "jdx/DataLdr.hpp"
#include "jdx/TextReader.hpp"
#include "util/LdrUtils.hpp"

#include <functional>
#include <string>

namespace sciformats::jdx
{
/**
 * @brief Base class for JCAMP-DX PEAK TABLE and PEAK ASSIGNMENTS records.
 */
class TabularData : public DataLdr
{
public:
    /**
     * @brief Definition of peak width (and other kernel) functions.
     *
     * Comment $$ in line(s) following LDR start may contain peak width and
     * other peak kernel functions
     *
     * @return Textual description of peak width function.
     */
    [[nodiscard]] std::optional<std::string> getWidthFunction();

protected:
    TabularData(
        std::string label, std::string variableList, TextReader& reader);

    /**
     * @brief Provides the parsed peak assignments or peaks.
     * @param Parser for the data.
     * @return The list of peak assignments or peaks.
     */
    template<typename Parser, typename R>
    std::vector<R> getData(Parser parser) const;
};

template<typename Parser, typename R>
std::vector<R> sciformats::jdx::TabularData::getData(Parser parser) const
{
    auto func = [&]() {
        std::vector<R> data{};
        auto& reader = getReader();

        // TODO: use util::skipPureComments()
        // skip possible initial comment lines
        std::optional<std::streampos> pos;
        while (!reader.eof())
        {
            pos = reader.tellg();
            auto line = reader.readLine();
            if (!util::isPureComment(line))
            {
                break;
            }
        }
        if (pos)
        {
            reader.seekg(pos.value());
        }

        // read peaks
        while (auto next = parser.next())
        {
            data.push_back(std::move(next.value()));
        }
        return data;
    };

    return callAndResetStreamPos<std::vector<R>>(func);
}

} // namespace sciformats::jdx

#endif // LIBJDX_TABULARDATA_HPP
