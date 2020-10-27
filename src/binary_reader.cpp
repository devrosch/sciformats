#include "binaryreader/binary_reader.hpp"

#include <array>
#include <climits>
#include <cstring>
#include <limits>

sciformats::common::binary_reader::binary_reader(
    const std::string& file_path, endianness endian)
    : _file_stream{std::ifstream{}}
    , _istringstream{std::nullopt}
    , _input_stream{_file_stream.value()}
    , _endianness{endian}
{
    _file_stream.value().exceptions(
        std::ios::eofbit | std::ios::failbit | std::ios::badbit);
    _file_stream.value().open(file_path, std::ios::in | std::ios::binary);
}

sciformats::common::binary_reader::binary_reader(std::istream& input_stream,
    endianness endian, bool activateExceptions)
    : _file_stream{std::nullopt}
    , _istringstream{std::nullopt}
    , _input_stream{input_stream}
    , _endianness{endian}
{
    if (activateExceptions)
    {
        // this also activate exceptions on input_stream, as as _input_stream is
        // a reference to input_stream
        _input_stream.exceptions(
            std::ios::eofbit | std::ios::failbit | std::ios::badbit);
    }
}

sciformats::common::binary_reader::binary_reader(
    std::vector<char>& vec, endianness endian)
    : _file_stream{std::nullopt}
    , _istringstream{std::istringstream{}}
    , _input_stream{_istringstream.value()}
    , _endianness{endian}
{
    // see:
    // https://stackoverflow.com/questions/8815164/c-wrapping-vectorchar-with-istream/8815308
    // watch out:
    // https://stackoverflow.com/questions/53199966/tellg-and-seekg-not-working-when-wrap-vectorchar-in-istream
    // =>
    // https://stackoverflow.com/questions/23630386/read-vectorchar-as-stream?rq=1
    // https://stackoverflow.com/questions/45722747/how-can-i-create-a-istream-from-a-uint8-t-vector
    _istringstream.value().exceptions(
        std::ios::eofbit | std::ios::failbit | std::ios::badbit);
    _istringstream.value().rdbuf()->pubsetbuf(vec.data(), vec.size());
}

sciformats::common::binary_reader::binary_reader(
    std::vector<uint8_t>& vec, endianness endian)
    : _file_stream{std::nullopt}
    , _istringstream{std::istringstream{}}
    , _input_stream{_istringstream.value()}
    , _endianness{endian}
{
    _istringstream.value().exceptions(
        std::ios::eofbit | std::ios::failbit | std::ios::badbit);
    // make sure the reinterpret_cast is legal
    // https://stackoverflow.com/questions/16260033/reinterpret-cast-between-char-and-stduint8-t-safe
    static_assert(std::is_same_v<std::uint8_t,
                      char> || std::is_same_v<std::uint8_t, unsigned char>,
        "uint8_t is not a typedef of char or unsigned char.");
    _istringstream.value().rdbuf()->pubsetbuf(
        // NOLINTNEXTLINE(cppcoreguidelines-pro-type-reinterpret-cast)
        reinterpret_cast<char*>(vec.data()), vec.size());
}

std::ios::pos_type sciformats::common::binary_reader::tellg() const
{
    return _input_stream.tellg();
}

void sciformats::common::binary_reader::seekg(
    std::ios::pos_type position, std::ios_base::seekdir seekdir)
{
    _input_stream.seekg(position, seekdir);
}

std::ios::pos_type sciformats::common::binary_reader::get_length()
{
    std::ios::pos_type current = _input_stream.tellg();
    _input_stream.seekg(0, std::ios::end);
    std::ios::pos_type length = _input_stream.tellg();
    _input_stream.seekg(current, std::ios::beg);
    return length;
}

int8_t sciformats::common::binary_reader::read_int8()
{
    static_assert(sizeof(char) == sizeof(int8_t),
        "Char size does not match int8_t size.");
    return _input_stream.get();
}

uint8_t sciformats::common::binary_reader::read_uint8()
{
    static_assert(CHAR_BIT == 8, "Char size is not 8.");
    return _input_stream.get();
}

uint16_t sciformats::common::binary_reader::read_uint16()
{
    return read_uint16(_endianness);
}

uint16_t sciformats::common::binary_reader::read_uint16(endianness endian)
{
    static_assert(CHAR_BIT == 8, "Char size is not 8.");
    // don't initialize array for potentially better performance
    // NOLINTNEXTLINE(cppcoreguidelines-pro-type-member-init,hicpp-member-init)
    std::array<char, 2> bytes;
    _input_stream.read(bytes.data(), bytes.size());
    return endian == little_endian
               ? ((static_cast<uint8_t>(bytes[0]) & 0xFFU) << 0U)
                     | ((static_cast<uint8_t>(bytes[1]) & 0xFFU) << 8U)
               : ((static_cast<uint8_t>(bytes[1]) & 0xFFU) << 0U)
                     | ((static_cast<uint8_t>(bytes[0]) & 0xFFU) << 8U);
}

int16_t sciformats::common::binary_reader::read_int16()
{
    return read_int16(_endianness);
}

int16_t sciformats::common::binary_reader::read_int16(endianness endian)
{
    static_assert(sizeof(uint16_t) == sizeof(int16_t),
        "Size of uinte16_t does not match size of int16_t.");
    return read_uint16(endian);
}

uint32_t sciformats::common::binary_reader::read_uint32()
{
    return read_uint32(_endianness);
}

uint32_t sciformats::common::binary_reader::read_uint32(endianness endian)
{
    static_assert(CHAR_BIT == 8, "Char size is not 8.");
    // don't initialize array for potentially better performance
    // NOLINTNEXTLINE(cppcoreguidelines-pro-type-member-init,hicpp-member-init)
    std::array<char, sizeof(uint32_t)> bytes;
    _input_stream.read(bytes.data(), bytes.size());
    return endian == little_endian
               ? ((static_cast<uint8_t>(bytes[0]) & 0xFFU) << 0U)
                     | ((static_cast<uint8_t>(bytes[1]) & 0xFFU) << 8U)
                     | ((static_cast<uint8_t>(bytes[2]) & 0xFFU) << 16U)
                     | ((static_cast<uint8_t>(bytes[3]) & 0xFFU) << 24U)
               : ((static_cast<uint8_t>(bytes[3]) & 0xFFU) << 0U)
                     | ((static_cast<uint8_t>(bytes[2]) & 0xFFU) << 8U)
                     | ((static_cast<uint8_t>(bytes[1]) & 0xFFU) << 16U)
                     | ((static_cast<uint8_t>(bytes[0]) & 0xFFU) << 24U);
}

int32_t sciformats::common::binary_reader::read_int32()
{
    return read_int32(_endianness);
}

int32_t sciformats::common::binary_reader::read_int32(endianness endian)
{
    static_assert(sizeof(uint32_t) == sizeof(int32_t),
        "Size of uinte32_t does not match size of int32_t.");
    return read_uint32(endian);
}

uint64_t sciformats::common::binary_reader::read_uint64()
{
    return read_uint64(_endianness);
}

uint64_t sciformats::common::binary_reader::read_uint64(endianness endian)
{
    static_assert(CHAR_BIT == 8, "Char size is not 8.");
    static_assert(sizeof(uint64_t) == 8, "uint8_t size is not 8 bytes.");
    // don't initialize array for potentially better performance
    // NOLINTNEXTLINE(cppcoreguidelines-pro-type-member-init,hicpp-member-init)
    std::array<char, sizeof(uint64_t)> bytes;
    _input_stream.read(bytes.data(), bytes.size());
    return endian == little_endian
               ? (static_cast<uint64_t>(static_cast<uint8_t>(bytes[0]) & 0xFFU)
                     << 0U)
                     | (static_cast<uint64_t>(
                            static_cast<uint8_t>(bytes[1]) & 0xFFU)
                         << 8U)
                     | (static_cast<uint64_t>(
                            static_cast<uint8_t>(bytes[2]) & 0xFFU)
                         << 16U)
                     | (static_cast<uint64_t>(
                            static_cast<uint8_t>(bytes[3]) & 0xFFU)
                         << 24U)
                     | (static_cast<uint64_t>(
                            static_cast<uint8_t>(bytes[4]) & 0xFFU)
                         << 32U)
                     | (static_cast<uint64_t>(
                            static_cast<uint8_t>(bytes[5]) & 0xFFU)
                         << 40U)
                     | (static_cast<uint64_t>(
                            static_cast<uint8_t>(bytes[6]) & 0xFFU)
                         << 48U)
                     | (static_cast<uint64_t>(
                            static_cast<uint8_t>(bytes[7]) & 0xFFU)
                         << 56U)
               : (static_cast<uint64_t>(static_cast<uint8_t>(bytes[7]) & 0xFFU)
                     << 0U)
                     | (static_cast<uint64_t>(
                            static_cast<uint8_t>(bytes[6]) & 0xFFU)
                         << 8U)
                     | (static_cast<uint64_t>(
                            static_cast<uint8_t>(bytes[5]) & 0xFFU)
                         << 16U)
                     | (static_cast<uint64_t>(
                            static_cast<uint8_t>(bytes[4]) & 0xFFU)
                         << 24U)
                     | (static_cast<uint64_t>(
                            static_cast<uint8_t>(bytes[3]) & 0xFFU)
                         << 32U)
                     | (static_cast<uint64_t>(
                            static_cast<uint8_t>(bytes[2]) & 0xFFU)
                         << 40U)
                     | (static_cast<uint64_t>(
                            static_cast<uint8_t>(bytes[1]) & 0xFFU)
                         << 48U)
                     | (static_cast<uint64_t>(
                            static_cast<uint8_t>(bytes[0]) & 0xFFU)
                         << 56U);
}

int64_t sciformats::common::binary_reader::read_int64()
{
    return read_int64(_endianness);
}

int64_t sciformats::common::binary_reader::read_int64(endianness endian)
{
    static_assert(sizeof(uint64_t) == sizeof(int64_t),
        "Size of uinte64_t does not match size of int64_t.");
    return read_uint64(endian);
}

float sciformats::common::binary_reader::read_float()
{
    return read_float(_endianness);
}

float sciformats::common::binary_reader::read_float(endianness endian)
{
    static_assert(CHAR_BIT == 8, "Char size is not 8.");
    static_assert(sizeof(float) == sizeof(int32_t),
        "Size of float does not match size of int32_t.");
    static_assert(std::numeric_limits<float>::is_iec559,
        "Float does not conform to IEC 559");

    // reinterpret_cast<float&>(value) is not guaranteed to work, see:
    // https://stackoverflow.com/questions/13982340/is-it-safe-to-reinterpret-cast-an-integer-to-float
    // https://stackoverflow.com/questions/20762952/most-efficient-standard-compliant-way-of-reinterpreting-int-as-float
    // https://stackoverflow.com/questions/15531232/reinterpreting-an-unsigned-int-to-a-float-in-c
    const int32_t value = read_int32(endian);
    // don't initialize variable for potentially better performance
    // NOLINTNEXTLINE(cppcoreguidelines-init-variables)
    float output;
    memcpy(&output, &value, sizeof(int32_t));
    return output;
}

double sciformats::common::binary_reader::read_double()
{
    return read_double(_endianness);
}

double sciformats::common::binary_reader::read_double(endianness endian)
{
    static_assert(CHAR_BIT == 8, "Char size is not 8.");
    static_assert(sizeof(double) == sizeof(int64_t),
        "Size of double does not match size of int64_t.");
    static_assert(std::numeric_limits<double>::is_iec559,
        "Double does not conform to IEC 559");

    // reinterpret_cast<double&>(value) is not guaranteed to work, see:
    // https://stackoverflow.com/questions/13982340/is-it-safe-to-reinterpret-cast-an-integer-to-float
    // https://stackoverflow.com/questions/20762952/most-efficient-standard-compliant-way-of-reinterpreting-int-as-float
    // https://stackoverflow.com/questions/15531232/reinterpreting-an-unsigned-int-to-a-float-in-c
    const int64_t value = read_int64(endian);
    // don't initialize variable for potentially better performance
    // NOLINTNEXTLINE(cppcoreguidelines-init-variables)
    double output;
    memcpy(&output, &value, sizeof(int64_t));
    return output;
}

std::vector<char> sciformats::common::binary_reader::read_chars(
    size_t size)
{
    std::vector<char> dest;
    dest.resize(size);
    _input_stream.read(dest.data(), size);
    return dest;
}

std::vector<uint8_t> sciformats::common::binary_reader::read_bytes(
    size_t size)
{
    static_assert(CHAR_BIT == 8, "Char size is not 8.");
    std::vector<uint8_t> dest;
    // reinterpret cast is safe as signed and unsigned char have same
    // representation and alignment
    // see:
    // https://en.cppreference.com/w/cpp/language/types
    // also:
    // https://stackoverflow.com/questions/15078638/can-i-turn-unsigned-char-into-char-and-vice-versa/15172304
    dest.resize(size);
    // NOLINTNEXTLINE(cppcoreguidelines-pro-type-reinterpret-cast)
    _input_stream.read(reinterpret_cast<char*>(dest.data()), size);
    // alternative implementation:
    // dest.reserve(size);
    // for (auto i = 0; i<size; i++)
    // {
    //     dest.push_back(_input_stream.get());
    // }
    return dest;
}
