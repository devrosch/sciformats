#ifndef LIBIO_STRINGPREFIXCONFIG_HPP
#define LIBIO_STRINGPREFIXCONFIG_HPP

#include "binaryreader/Endianness.hpp"

#include <cstdint>

namespace sciformats::io
{
/**
 * @brief The configuration for reading a string.
 */
struct StringPrefixConfig
{
    uint8_t prefixSizeBytes = 0;
    Endianness prefixEndianness = Endianness::LittleEndian;
};

} // namespace sciformats::io

#endif // LIBIO_STRINGPREFIXCONFIG_HPP
