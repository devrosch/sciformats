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
 * @brief The endianness enum indicates if data is expected to be little
 * endian or big endian.
 */
enum class Endianness : uint8_t
{
    LittleEndian,
    BigEndian,
};

} // namespace sciformats::io

#endif // LIBIO_ENDIANNESS_HPP
