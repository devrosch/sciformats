#ifndef LIBJDX_LDRCONTAINER_HPP
#define LIBJDX_LDRCONTAINER_HPP

#include "jdx/TextReader.hpp"

#include <optional>

namespace sciformats::jdx
{
/**
 * @brief Parent class of JCAMP-DX BLOCK and NTUPLES records.
 */
class LdrContainer
{
protected:
    static std::optional<const std::string> parseStringValue(std::string& value, TextReader& reader);
};
} // namespace sciformats::jdx

#endif // LIBJDX_LDRCONTAINER_HPP
