#include "binaryreader/binary_reader.hpp"

#include <limits>
#include <climits>
#include <cstring>

sciformats::common::binary_reader::binary_reader(const std::string& file_path, const endianness endian)
    :_istringstream(), _file_stream(), _input_stream(_file_stream), _endianness(endian)
{
    _file_stream.exceptions(std::ios::eofbit | std::ios::failbit | std::ios::badbit);
    _file_stream.open(file_path, std::ios::in | std::ios::binary);
}

sciformats::common::binary_reader::binary_reader(std::istream& input_stream, const endianness endian, const bool activateExceptions)
    :_istringstream(), _input_stream(input_stream), _endianness(endian)
{
    if (activateExceptions)
    {
        // this also activate exceptions on input_stream, as as _input_stream is a reference to input_stream
        _input_stream.exceptions(std::ios::eofbit | std::ios::failbit | std::ios::badbit);
    }
}

sciformats::common::binary_reader::binary_reader(std::vector<char>& vec, const endianness endian)
    :_istringstream(), _input_stream(_istringstream), _endianness(endian)
{
    // see: https://stackoverflow.com/questions/8815164/c-wrapping-vectorchar-with-istream/8815308
    // watch out: https://stackoverflow.com/questions/53199966/tellg-and-seekg-not-working-when-wrap-vectorchar-in-istream
    // =>
    // https://stackoverflow.com/questions/23630386/read-vectorchar-as-stream?rq=1
    // https://stackoverflow.com/questions/45722747/how-can-i-create-a-istream-from-a-uint8-t-vector
    _istringstream.exceptions(std::ios::eofbit | std::ios::failbit | std::ios::badbit);
    _istringstream.rdbuf()->pubsetbuf(vec.data(), vec.size());
}

sciformats::common::binary_reader::binary_reader(std::vector<uint8_t>& vec, const endianness endian)
    :_istringstream(), _input_stream(_istringstream), _endianness(endian)
{
    _istringstream.exceptions(std::ios::eofbit | std::ios::failbit | std::ios::badbit);
    _istringstream.rdbuf()->pubsetbuf(reinterpret_cast<char*>(vec.data()), vec.size());
}

std::ios::pos_type sciformats::common::binary_reader::tellg() const
{
    return _input_stream.tellg();
}

void sciformats::common::binary_reader::seekg(const std::ios::pos_type position, const std::ios_base::seekdir seekdir)
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
    static_assert(CHAR_BIT == 8, "Char size is not 8.");

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

uint16_t sciformats::common::binary_reader::read_uint16(const endianness endian)
{
    static_assert(CHAR_BIT == 8, "Char size is not 8.");

    char bytes[2];
    _input_stream.read(bytes, 2);
    return endian == little_endian
            ? ((bytes[0] & 0xFF)<<0) | ((bytes[1] & 0xFF)<<8)
            : ((bytes[1] & 0xFF)<<0) | ((bytes[0] & 0xFF)<<8);
}

int16_t sciformats::common::binary_reader::read_int16()
{
    return read_int16(_endianness);
}

int16_t sciformats::common::binary_reader::read_int16(const endianness endian)
{
    static_assert(CHAR_BIT == 8, "Char size is not 8.");

    char bytes[2];
    _input_stream.read(bytes, 2);
    return endian == little_endian
            ? ((bytes[0] & 0xFF)<<0) | ((bytes[1] & 0xFF)<<8)
            : ((bytes[1] & 0xFF)<<0) | ((bytes[0] & 0xFF)<<8);
}

uint32_t sciformats::common::binary_reader::read_uint32()
{
    return read_uint32(_endianness);
}

uint32_t sciformats::common::binary_reader::read_uint32(const endianness endian)
{
    static_assert(CHAR_BIT == 8, "Char size is not 8.");

    char bytes[4];
    _input_stream.read(bytes, 4);
    return endian == little_endian
            ? ((bytes[0] & 0xFF)<<0)
            | ((bytes[1] & 0xFF)<<8)
            | ((bytes[2] & 0xFF)<<16)
            | ((bytes[3] & 0xFF)<<24)
            : ((bytes[3] & 0xFF)<<0)
            | ((bytes[2] & 0xFF)<<8)
            | ((bytes[1] & 0xFF)<<16)
            | ((bytes[0] & 0xFF)<<24);
}

int32_t sciformats::common::binary_reader::read_int32()
{
    return read_int32(_endianness);
}

int32_t sciformats::common::binary_reader::read_int32(const endianness endian)
{
    static_assert(CHAR_BIT == 8, "Char size is not 8.");

    char bytes[4];
    _input_stream.read(bytes, 4);
    return endian == little_endian
            ? ((bytes[0] & 0xFF)<<0)
            | ((bytes[1] & 0xFF)<<8)
            | ((bytes[2] & 0xFF)<<16)
            | ((bytes[3] & 0xFF)<<24)
            : ((bytes[3] & 0xFF)<<0)
            | ((bytes[2] & 0xFF)<<8)
            | ((bytes[1] & 0xFF)<<16)
            | ((bytes[0] & 0xFF)<<24);
}

uint64_t sciformats::common::binary_reader::read_uint64()
{
    return read_uint64(_endianness);
}

uint64_t sciformats::common::binary_reader::read_uint64(const endianness endian)
{
    static_assert(CHAR_BIT == 8, "Char size is not 8.");

    char bytes[8];
    _input_stream.read(bytes, 8);

    return endian == little_endian ?
        (static_cast<uint64_t>(bytes[0] & 0xFF)<<0) |
        (static_cast<uint64_t>(bytes[1] & 0xFF)<<8) |
        (static_cast<uint64_t>(bytes[2] & 0xFF)<<16) |
        (static_cast<uint64_t>(bytes[3] & 0xFF)<<24) |
        (static_cast<uint64_t>(bytes[4] & 0xFF)<<32) |
        (static_cast<uint64_t>(bytes[5] & 0xFF)<<40) |
        (static_cast<uint64_t>(bytes[6] & 0xFF)<<48) |
        (static_cast<uint64_t>(bytes[7] & 0xFF)<<56)
    : (static_cast<uint64_t>(bytes[7] & 0xFF)<<0) |
        (static_cast<uint64_t>(bytes[6] & 0xFF)<<8) |
        (static_cast<uint64_t>(bytes[5] & 0xFF)<<16) |
        (static_cast<uint64_t>(bytes[4] & 0xFF)<<24) |
        (static_cast<uint64_t>(bytes[3] & 0xFF)<<32) |
        (static_cast<uint64_t>(bytes[2] & 0xFF)<<40) |
        (static_cast<uint64_t>(bytes[1] & 0xFF)<<48) |
        (static_cast<uint64_t>(bytes[0] & 0xFF)<<56);
}

int64_t sciformats::common::binary_reader::read_int64()
{
    return read_int64(_endianness);
}

int64_t sciformats::common::binary_reader::read_int64(const endianness endian)
{
    static_assert(CHAR_BIT == 8, "Char size is not 8.");

    char bytes[8];
    _input_stream.read(bytes, 8);

    return endian == little_endian ?
        (static_cast<int64_t>(bytes[0] & 0xFF)<<0) |
        (static_cast<int64_t>(bytes[1] & 0xFF)<<8) |
        (static_cast<int64_t>(bytes[2] & 0xFF)<<16) |
        (static_cast<int64_t>(bytes[3] & 0xFF)<<24) |
        (static_cast<int64_t>(bytes[4] & 0xFF)<<32) |
        (static_cast<int64_t>(bytes[5] & 0xFF)<<40) |
        (static_cast<int64_t>(bytes[6] & 0xFF)<<48) |
        (static_cast<int64_t>(bytes[7] & 0xFF)<<56)
    : (static_cast<int64_t>(bytes[7] & 0xFF)<<0) |
        (static_cast<int64_t>(bytes[6] & 0xFF)<<8) |
        (static_cast<int64_t>(bytes[5] & 0xFF)<<16) |
        (static_cast<int64_t>(bytes[4] & 0xFF)<<24) |
        (static_cast<int64_t>(bytes[3] & 0xFF)<<32) |
        (static_cast<int64_t>(bytes[2] & 0xFF)<<40) |
        (static_cast<int64_t>(bytes[1] & 0xFF)<<48) |
        (static_cast<int64_t>(bytes[0] & 0xFF)<<56);
}

float sciformats::common::binary_reader::read_float()
{
    return read_float(_endianness);
}

float sciformats::common::binary_reader::read_float(const endianness endian)
{
    static_assert(CHAR_BIT == 8, "Char size is not 8.");
    static_assert(sizeof(float) == sizeof(int32_t), "Size of float does not match size of int32_t.");
    static_assert(std::numeric_limits<float>::is_iec559, "Float does not conform to IEC 559");

    // reinterpret_cast<float&>(value) is not guaranteed to work, see:
    // https://stackoverflow.com/questions/13982340/is-it-safe-to-reinterpret-cast-an-integer-to-float
    // https://stackoverflow.com/questions/20762952/most-efficient-standard-compliant-way-of-reinterpreting-int-as-float
    // https://stackoverflow.com/questions/15531232/reinterpreting-an-unsigned-int-to-a-float-in-c
    const int32_t value = read_int32(endian);
    float output;
    memcpy(&output, &value, sizeof(int32_t));
    return output;
}

double sciformats::common::binary_reader::read_double()
{
    return read_double(_endianness);
}

double sciformats::common::binary_reader::read_double(const endianness endian)
{
    static_assert(CHAR_BIT == 8, "Char size is not 8.");
    static_assert(sizeof(double) == sizeof(int64_t), "Size of double does not match size of int64_t.");
    static_assert(std::numeric_limits<double>::is_iec559, "Double does not conform to IEC 559");

    // reinterpret_cast<double&>(value) is not guaranteed to work, see:
    // https://stackoverflow.com/questions/13982340/is-it-safe-to-reinterpret-cast-an-integer-to-float
    // https://stackoverflow.com/questions/20762952/most-efficient-standard-compliant-way-of-reinterpreting-int-as-float
    // https://stackoverflow.com/questions/15531232/reinterpreting-an-unsigned-int-to-a-float-in-c
    const int64_t value = read_int64(endian);
    double output;
    memcpy(&output, &value, sizeof(int64_t));
    return output;
}

std::vector<char> sciformats::common::binary_reader::read_chars(const size_t size)
{
    // for alternative implementation:
    // see: https://stackoverflow.com/questions/10823264/is-there-a-more-efficient-way-to-set-a-stdvector-from-a-stream
    // https://stackoverflow.com/questions/16727125/how-does-stdcopy-work-with-stream-iterators
    std::vector<char> dest;
    dest.resize(size);
    _input_stream.read(dest.data(), size);
    return dest;
}

std::vector<uint8_t> sciformats::common::binary_reader::read_bytes(const size_t size)
{
    static_assert(CHAR_BIT == 8, "Char size is not 8.");

    // for alternative implementation:
    // see: https://stackoverflow.com/questions/10823264/is-there-a-more-efficient-way-to-set-a-stdvector-from-a-stream
    // https://stackoverflow.com/questions/16727125/how-does-stdcopy-work-with-stream-iterators
    std::vector<uint8_t> dest;
    dest.resize(size);
    // reinterpret cast is safe as signed and unsigned char have same representation and alignment
    // see: https://en.cppreference.com/w/cpp/language/types
    // also: https://stackoverflow.com/questions/15078638/can-i-turn-unsigned-char-into-char-and-vice-versa/15172304
    _input_stream.read(reinterpret_cast<char*>(dest.data()), size);
    return dest;
}
