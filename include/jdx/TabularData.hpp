#ifndef LIBJDX_TABULARDATA_HPP
#define LIBJDX_TABULARDATA_HPP

#include "jdx/DataLdr.hpp"

#include <functional>
#include <istream>
#include <string>

namespace sciformats::jdx
{
/**
 * @brief Base class for JCAMP-DX PEAK TABLE and PEAK ASSIGNMENTS records.
 */
class TabularData : public DataLdr
{
public:
protected:
    explicit TabularData(std::istream& istream);
    TabularData(
        std::string label, std::string variableList, std::istream& istream);

    static void validateInput(const std::string& label,
        const std::string& variableList, const std::string& expectedLabel,
        const std::vector<std::string>& expectedVariableLists);
    template<typename R>
    R callAndResetStreamPos(const std::function<R()>& func);
};
} // namespace sciformats::jdx

#endif // LIBJDX_TABULARDATA_HPP
