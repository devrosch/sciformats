#include "binaryreader/BinaryReader.hpp"
#include "binaryreader/Endianness.hpp"

#include <array>
#include <climits>
#include <cstring>
#include <limits>

sciformats::io::BinaryReader::BinaryReader(
    const std::string& filePath, sciformats::io::Endianness endian)
    : m_ifstream{std::ifstream{}}
    , m_istringstream{std::nullopt}
    , m_istream{m_ifstream.value()}
    , m_endianness{endian}
{
    m_ifstream.value().exceptions(
        std::ios::eofbit | std::ios::failbit | std::ios::badbit);
    m_ifstream.value().open(filePath, std::ios::in | std::ios::binary);
}

sciformats::io::BinaryReader::BinaryReader(std::istream& inputStream,
    sciformats::io::Endianness endian, bool activateExceptions)
    : m_ifstream{std::nullopt}
    , m_istringstream{std::nullopt}
    , m_istream{inputStream}
    , m_endianness{endian}
{
    if (activateExceptions)
    {
        // this also activates exceptions on input_stream, as as m_istream is
        // a reference to input_stream
        m_istream.exceptions(
            std::ios::eofbit | std::ios::failbit | std::ios::badbit);
    }
}

sciformats::io::BinaryReader::BinaryReader(
    std::vector<char>& vec, sciformats::io::Endianness endian)
    : m_ifstream{std::nullopt}
    , m_istringstream{std::istringstream{}}
    , m_istream{m_istringstream.value()}
    , m_endianness{endian}
{
    // see:
    // https://stackoverflow.com/questions/8815164/c-wrapping-vectorchar-with-istream/8815308
    // watch out:
    // https://stackoverflow.com/questions/53199966/tellg-and-seekg-not-working-when-wrap-vectorchar-in-istream
    // =>
    // https://stackoverflow.com/questions/23630386/read-vectorchar-as-stream?rq=1
    // https://stackoverflow.com/questions/45722747/how-can-i-create-a-istream-from-a-uint8-t-vector
    m_istringstream.value().exceptions(
        std::ios::eofbit | std::ios::failbit | std::ios::badbit);
    m_istringstream.value().rdbuf()->pubsetbuf(vec.data(), vec.size());
}

sciformats::io::BinaryReader::BinaryReader(
    std::vector<uint8_t>& vec, sciformats::io::Endianness endian)
    : m_ifstream{std::nullopt}
    , m_istringstream{std::istringstream{}}
    , m_istream{m_istringstream.value()}
    , m_endianness{endian}
{
    m_istringstream.value().exceptions(
        std::ios::eofbit | std::ios::failbit | std::ios::badbit);
    // make sure the reinterpret_cast is legal
    // https://stackoverflow.com/questions/16260033/reinterpret-cast-between-char-and-stduint8-t-safe
    static_assert(std::is_same_v<std::uint8_t,
                      char> || std::is_same_v<std::uint8_t, unsigned char>,
        "uint8_t is not a typedef of char or unsigned char.");
    m_istringstream.value().rdbuf()->pubsetbuf(
        // NOLINTNEXTLINE(cppcoreguidelines-pro-type-reinterpret-cast)
        reinterpret_cast<char*>(vec.data()), vec.size());
}

std::ios::pos_type sciformats::io::BinaryReader::tellg() const
{
    return m_istream.tellg();
}

void sciformats::io::BinaryReader::seekg(
    std::ios::pos_type position, std::ios_base::seekdir seekdir)
{
    m_istream.seekg(position, seekdir);
}

std::ios::pos_type sciformats::io::BinaryReader::getLength()
{
    std::ios::pos_type current = m_istream.tellg();
    m_istream.seekg(0, std::ios::end);
    std::ios::pos_type length = m_istream.tellg();
    m_istream.seekg(current, std::ios::beg);
    return length;
}

int8_t sciformats::io::BinaryReader::readInt8()
{
    static_assert(sizeof(char) == sizeof(int8_t),
        "Char size does not match int8_t size.");
    return m_istream.get();
}

uint8_t sciformats::io::BinaryReader::readUInt8()
{
    static_assert(CHAR_BIT == 8, "Char size is not 8.");
    return m_istream.get();
}

uint16_t sciformats::io::BinaryReader::readUInt16()
{
    return readUInt16(m_endianness);
}

uint16_t sciformats::io::BinaryReader::readUInt16(
    sciformats::io::Endianness endian)
{
    static_assert(CHAR_BIT == 8, "Char size is not 8.");
    // don't initialize array for potentially better performance
    // NOLINTNEXTLINE(cppcoreguidelines-pro-type-member-init,hicpp-member-init)
    std::array<char, 2> bytes;
    m_istream.read(bytes.data(), bytes.size());
    return endian == Endianness::LittleEndian
               ? ((static_cast<uint8_t>(bytes[0]) & 0xFFU) << 0U)
                     | ((static_cast<uint8_t>(bytes[1]) & 0xFFU) << 8U)
               : ((static_cast<uint8_t>(bytes[1]) & 0xFFU) << 0U)
                     | ((static_cast<uint8_t>(bytes[0]) & 0xFFU) << 8U);
}

int16_t sciformats::io::BinaryReader::readInt16()
{
    return readInt16(m_endianness);
}

int16_t sciformats::io::BinaryReader::readInt16(
    sciformats::io::Endianness endian)
{
    static_assert(sizeof(uint16_t) == sizeof(int16_t),
        "Size of uinte16_t does not match size of int16_t.");
    return readUInt16(endian);
}

uint32_t sciformats::io::BinaryReader::readUInt32()
{
    return readUInt32(m_endianness);
}

uint32_t sciformats::io::BinaryReader::readUInt32(
    sciformats::io::Endianness endian)
{
    static_assert(CHAR_BIT == 8, "Char size is not 8.");
    // don't initialize array for potentially better performance
    // NOLINTNEXTLINE(cppcoreguidelines-pro-type-member-init,hicpp-member-init)
    std::array<char, sizeof(uint32_t)> bytes;
    m_istream.read(bytes.data(), bytes.size());
    return endian == sciformats::io::Endianness::LittleEndian
               ? ((static_cast<uint8_t>(bytes[0]) & 0xFFU) << 0U)
                     | ((static_cast<uint8_t>(bytes[1]) & 0xFFU) << 8U)
                     | ((static_cast<uint8_t>(bytes[2]) & 0xFFU) << 16U)
                     | ((static_cast<uint8_t>(bytes[3]) & 0xFFU) << 24U)
               : ((static_cast<uint8_t>(bytes[3]) & 0xFFU) << 0U)
                     | ((static_cast<uint8_t>(bytes[2]) & 0xFFU) << 8U)
                     | ((static_cast<uint8_t>(bytes[1]) & 0xFFU) << 16U)
                     | ((static_cast<uint8_t>(bytes[0]) & 0xFFU) << 24U);
}

int32_t sciformats::io::BinaryReader::readInt32()
{
    return readInt32(m_endianness);
}

int32_t sciformats::io::BinaryReader::readInt32(
    sciformats::io::Endianness endian)
{
    static_assert(sizeof(uint32_t) == sizeof(int32_t),
        "Size of uinte32_t does not match size of int32_t.");
    return readUInt32(endian);
}

uint64_t sciformats::io::BinaryReader::readUInt64()
{
    return readUInt64(m_endianness);
}

uint64_t sciformats::io::BinaryReader::readUInt64(
    sciformats::io::Endianness endian)
{
    static_assert(CHAR_BIT == 8, "Char size is not 8.");
    static_assert(sizeof(uint64_t) == 8, "uint8_t size is not 8 bytes.");
    // don't initialize array for potentially better performance
    // NOLINTNEXTLINE(cppcoreguidelines-pro-type-member-init,hicpp-member-init)
    std::array<char, sizeof(uint64_t)> bytes;
    m_istream.read(bytes.data(), bytes.size());
    return endian == sciformats::io::Endianness::LittleEndian
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

int64_t sciformats::io::BinaryReader::readInt64()
{
    return readInt64(m_endianness);
}

int64_t sciformats::io::BinaryReader::readInt64(
    sciformats::io::Endianness endian)
{
    static_assert(sizeof(uint64_t) == sizeof(int64_t),
        "Size of uinte64_t does not match size of int64_t.");
    return readUInt64(endian);
}

float sciformats::io::BinaryReader::readFloat()
{
    return readFloat(m_endianness);
}

float sciformats::io::BinaryReader::readFloat(sciformats::io::Endianness endian)
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
    const int32_t value = readInt32(endian);
    // don't initialize variable for potentially better performance
    // NOLINTNEXTLINE(cppcoreguidelines-init-variables)
    float output;
    memcpy(&output, &value, sizeof(int32_t));
    return output;
}

double sciformats::io::BinaryReader::readDouble()
{
    return readDouble(m_endianness);
}

double sciformats::io::BinaryReader::readDouble(
    sciformats::io::Endianness endian)
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
    const int64_t value = readInt64(endian);
    // don't initialize variable for potentially better performance
    // NOLINTNEXTLINE(cppcoreguidelines-init-variables)
    double output;
    memcpy(&output, &value, sizeof(int64_t));
    return output;
}

std::vector<char> sciformats::io::BinaryReader::readChars(size_t size)
{
    std::vector<char> dest;
    dest.resize(size);
    m_istream.read(dest.data(), size);
    return dest;
}

std::vector<uint8_t> sciformats::io::BinaryReader::readBytes(size_t size)
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
    m_istream.read(reinterpret_cast<char*>(dest.data()), size);
    // alternative implementation:
    // dest.reserve(size);
    // for (auto i = 0; i<size; i++)
    // {
    //     dest.push_back(m_istream.get());
    // }
    return dest;
}
