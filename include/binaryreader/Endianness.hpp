#ifndef LIBIO_ENDIANNESS_HPP
#define LIBIO_ENDIANNESS_HPP

#include <fstream>
#include <istream>
#include <optional>
#include <sstream>
#include <vector>

namespace sciformats::io
{
/**
 * @brief The endianness indicates the byte order of data.
 */
enum class Endianness : uint8_t
{
    /**
      The least significant byte comes first.
    */
    LittleEndian,
    /**
      The most significant byte comes first.
    */
    BigEndian,
};

} // namespace sciformats::io

#endif // LIBIO_ENDIANNESS_HPP
