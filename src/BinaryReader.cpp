#include "binaryreader/BinaryReader.hpp"
#include "binaryreader/Endianness.hpp"

#include <unicode/ucnv.h>
#include <unicode/unistr.h>

#include <array>
#include <climits>
#include <cstring>
#include <limits>

sciformats::io::BinaryReader::BinaryReader(
    const std::string& filePath, sciformats::io::Endianness endian)
    : m_ifstream{std::ifstream{}}
    , m_stringstream{std::nullopt}
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
    , m_stringstream{std::nullopt}
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
    , m_stringstream{std::stringstream{}}
    , m_istream{m_stringstream.value()}
    , m_endianness{endian}
{
    // see:
    // https://stackoverflow.com/questions/8815164/c-wrapping-vectorchar-with-istream/8815308
    // watch out:
    // https://stackoverflow.com/questions/53199966/tellg-and-seekg-not-working-when-wrap-vectorchar-in-istream
    // =>
    // https://stackoverflow.com/questions/23630386/read-vectorchar-as-stream?rq=1
    // https://stackoverflow.com/questions/45722747/how-can-i-create-a-istream-from-a-uint8-t-vector
    m_stringstream.value().exceptions(
        std::ios::eofbit | std::ios::failbit | std::ios::badbit);
#ifdef __EMSCRIPTEN__
    // pubsetbuf() does not work for Emscripten
    // probable reason:
    // https://stackoverflow.com/questions/12481463/stringstream-rdbuf-pubsetbuf-is-not-setting-the-buffer
    m_stringstream.value().write(
        vec.data(), static_cast<std::streamsize>(vec.size()));
#else
    m_stringstream.value().rdbuf()->pubsetbuf(vec.data(), vec.size());
#endif
}

sciformats::io::BinaryReader::BinaryReader(
    std::vector<uint8_t>& vec, sciformats::io::Endianness endian)
    : m_ifstream{std::nullopt}
    , m_stringstream{std::stringstream{}}
    , m_istream{m_stringstream.value()}
    , m_endianness{endian}
{
    m_stringstream.value().exceptions(
        std::ios::eofbit | std::ios::failbit | std::ios::badbit);
    // make sure the reinterpret_cast is legal
    // https://stackoverflow.com/questions/16260033/reinterpret-cast-between-char-and-stduint8-t-safe
    static_assert(std::is_same_v<std::uint8_t,
                      char> || std::is_same_v<std::uint8_t, unsigned char>,
        "uint8_t is not a typedef of char or unsigned char.");
#ifdef __EMSCRIPTEN__
    // pubsetbuf() does not work for Emscripten
    // probable reason:
    // https://stackoverflow.com/questions/12481463/stringstream-rdbuf-pubsetbuf-is-not-setting-the-buffer
    m_stringstream.value().write(reinterpret_cast<char*>(vec.data()),
        static_cast<std::streamsize>(vec.size()));
#else
    m_stringstream.value().rdbuf()->pubsetbuf(
        // NOLINTNEXTLINE(cppcoreguidelines-pro-type-reinterpret-cast)
        reinterpret_cast<char*>(vec.data()), vec.size());
#endif
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

std::string sciformats::io::BinaryReader::readString(
    const std::string& encoding, int32_t size)
{
    if (size < 0)
    {
        return std::string{};
    }
    if (size > std::numeric_limits<int32_t>::max() / 2)
    {
        std::string message
            = std::to_string(size)
              + std::string{" exceeds maximum permitted string char size: "}
              + std::to_string(std::numeric_limits<int32_t>::max() / 2);
        throw std::runtime_error(message.c_str());
    }
    // see: https://unicode-org.github.io/icu/userguide/conversion/
    // see:
    // https://github.com/unicode-org/icu/blob/master/icu4c/source/samples/ucnv/convsamp.cpp
    // see:
    // https://stackoverflow.com/questions/6010793/looking-for-simple-practical-c-examples-of-how-to-use-icu
    // TODO: possibly use UnicodeString as buffer already for efficiency
    std::vector<char> input = readChars(size);
    UErrorCode status = U_ZERO_ERROR;
    UConverter* converter = ucnv_open(encoding.c_str(), &status);
    try
    {
        // ideomatic ICU
        // NOLINTNEXTLINE(readability-implicit-bool-conversion)
        if (U_FAILURE(status))
        {
            std::string message = std::string{std::to_string(status)} + ": "
                                  + u_errorName(status);
            throw std::runtime_error(message.c_str());
        }

        // U_SUCCESS(status) must be truthy
        // reserve buffer
        // 2 * size UChars is the upper required limit
        // see:
        // https://unicode-org.github.io/icu-docs/apidoc/released/icu4c/ucnv_8h.html#aa3d7e4ae84f8a95b9735ed3491cdb77e
        int32_t maxUCharBufferSize = 2 * size;
        icu::UnicodeString target(maxUCharBufferSize, UChar32{0}, size);
        UChar* buffer = target.getBuffer(maxUCharBufferSize);
        // convert input and store in buffer
        // ignore returned length as it will always give the number of UChars
        // for the whole input sequence, possibly including NULL UChars
        ucnv_toUChars(converter, buffer, maxUCharBufferSize, input.data(),
            input.size(), &status);
        // do not use length returned from ucnv_toUChars as the string then may
        // include intermediate nulls
        target.releaseBuffer();
        // ideomatic ICU
        // NOLINTNEXTLINE(readability-implicit-bool-conversion)
        if (U_FAILURE(status))
        {
            std::string message = std::string{std::to_string(status)} + ": "
                                  + u_errorName(status);
            throw std::runtime_error(message.c_str());
        }
        // convert to UTF-8 string
        std::string outputString{};
        target.toUTF8String(outputString);
        // clean up and return
        ucnv_close(converter);
        return outputString;
    }
    catch (...)
    {
        ucnv_close(converter);
        throw;
    }
}

std::string sciformats::io::BinaryReader::readPrefixedString(
    StringPrefixType prefixType, const std::string& encoding, int32_t maxSize)
{
    static_assert(CHAR_BIT == 8, "Char size is not 8.");

    if (maxSize > std::numeric_limits<uint16_t>::max())
    {
        std::string message = std::string{"maxSize exceeds permitted maximum "
                                          "size of 32767: "}
                              + std::to_string(maxSize);
        throw std::runtime_error(message.c_str());
    }

    int32_t numChars = -1;
    int32_t multiplicationFactor = 1;

    switch (prefixType.numericType)
    {
    case StringPrefixNumericType::Int8Chars8:
        // signed char should be interpreted as a signed number here
        // NOLINTNEXTLINE(bugprone-signed-char-misuse)
        numChars = readInt8();
        multiplicationFactor = 1;
        break;
    case StringPrefixNumericType::UInt8Chars8:
        numChars = readUInt8();
        multiplicationFactor = 1;
        break;
    case StringPrefixNumericType::Int8Chars16:
        // signed char should be interpreted as a signed number here
        // NOLINTNEXTLINE(bugprone-signed-char-misuse)
        numChars = static_cast<int32_t>(readInt8());
        multiplicationFactor = 2;
        break;
    case StringPrefixNumericType::UInt8Chars16:
        numChars = readUInt8();
        multiplicationFactor = 2;
        break;
    case StringPrefixNumericType::Int16Chars8:
        numChars = readInt16(prefixType.endianness);
        multiplicationFactor = 1;
        break;
    case StringPrefixNumericType::UInt16Chars8:
        numChars = readUInt16(prefixType.endianness);
        multiplicationFactor = 1;
        break;
    case StringPrefixNumericType::Int16Chars16:
        numChars = readInt16(prefixType.endianness);
        multiplicationFactor = 2;
        break;
    case StringPrefixNumericType::UInt16Chars16:
        numChars = readUInt16(prefixType.endianness);
        multiplicationFactor = 2;
        break;
    default:
        std::string message
            = std::string{"Unsupported string prefix type: "}
              + std::to_string(static_cast<int>(prefixType.numericType));
        throw std::runtime_error(message.c_str());
        break;
    }

    int32_t numBytes = numChars * multiplicationFactor;

    if (numBytes > maxSize)
    {
        std::string message = std::string{"Number of bytes \""}
                              + std::to_string(numBytes)
                              + std::string{"\" from string prefix "
                                            "exceeds specified maximum "
                                            "size of: "}
                              + std::to_string(maxSize);
        throw std::runtime_error(message.c_str());
    }

    return readString(encoding, numBytes);
}
