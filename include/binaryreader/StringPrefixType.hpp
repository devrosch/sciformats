#ifndef LIBIO_STRINGPREFIXTYPE_HPP
#define LIBIO_STRINGPREFIXTYPE_HPP

#include "binaryreader/Endianness.hpp"
#include "binaryreader/StringPrefixNumericType.hpp"

namespace sciformats::io
{
/**
 * @brief The configuration for reading a string.
 */
struct StringPrefixType
{
    /**
     * @brief numericType The numeric type of the prefix, including the number
     * of bytes. Default: Int16Chars16.
     */
    StringPrefixNumericType numericType = StringPrefixNumericType::Int16Chars16;
    /**
     * @brief prefixEndianness The endianness of the prefix chars. Only relevant
     * if prefix consist of more than one char. Default: LittleEndian.
     */
    Endianness prefixEndianness = Endianness::LittleEndian;
};

} // namespace sciformats::io

#endif // LIBIO_STRINGPREFIXTYPE_HPP
