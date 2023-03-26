#ifndef LIBIO_STRINGPREFIXNUMERICTYPE_HPP
#define LIBIO_STRINGPREFIXNUMERICTYPE_HPP

#include <cstdint> // for uint8_t

namespace sciformats::io
{
/**
 * @brief The endianness indicates the byte order of data.
 */
enum class StringPrefixNumericType : uint8_t
{
    Int8Chars8, ///< One signed byte with length of uint8_t characters
    UInt8Chars8, ///< One unsigned byte, with length of uint8_t characters
    Int8Chars16, ///< One signed byte, with length of uint16_t characters
    UInt8Chars16, ///< One unsigned byte, with length of uint16_t characters
    Int16Chars8, ///< Two signed bytes, with length of uint8_t characters
    UInt16Chars8, ///< Two unsigned bytes, with length of uint8_t characters
    Int16Chars16, ///< Two signed bytes, with length of uint16_t characters
    UInt16Chars16, ///< Two unsigned bytes, with length of uint16_t characters

    // this could lead to extremely large strings,
    // would require additional checks during parsing
    // Int32Chars8, ///< Four signed bytes, with length of uint8_t characters
    // Int32Chars16, ///< Four signed bytes, with length of uint16_t characters

    // this excedds possible ICU string length
    // UInt32Chars8, ///< Four unsigned bytes, with length of uint8_t characters
    // UInt32Chars16, ///< Four unsigned bytes, with length of uint16_t
    // characters
};

} // namespace sciformats::io

#endif // LIBIO_STRINGPREFIXNUMERICTYPE_HPP
