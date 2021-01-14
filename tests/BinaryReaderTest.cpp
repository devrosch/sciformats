#define CATCH_CONFIG_MAIN
#include "binaryreader/BinaryReader.hpp"
#include "binaryreader/Endianness.hpp"

#include "catch2/catch.hpp"

#include <array>
#include <climits>
#include <string>

static_assert(CHAR_BIT == 8, "Char size is not 8.");

// see: https://stackoverflow.com/a/47934240
constexpr auto operator"" _c(unsigned long long arg) noexcept
{
    return static_cast<char>(arg);
}

TEST_CASE("correctly reads file", "[BinaryReader]")
{
    const std::string path{"resources/test.bin"};
    sciformats::io::BinaryReader reader(path);

    REQUIRE(reader.tellg() == 0);
    REQUIRE(reader.getLength() == 8);
    REQUIRE(reader.readUInt8() == 0x01);
    REQUIRE(reader.readUInt8() == 0x02);
    REQUIRE(reader.readUInt8() == 0x03);
    REQUIRE(reader.readUInt8() == 0x04);
    REQUIRE(reader.readUInt8() == 0xFF);
    REQUIRE(reader.readUInt8() == 0xFE);
    REQUIRE(reader.readUInt8() == 0xFD);
    REQUIRE(reader.readUInt8() == 0xFC);
    reader.seekg(1);
    REQUIRE(reader.tellg() == 1);
    REQUIRE(reader.readUInt8() == 0x02);
}

TEST_CASE("throws exception when reading past end and constructed from file",
    "[BinaryReader]")
{
    const std::string path{"resources/test.bin"};
    sciformats::io::BinaryReader reader(path);

    reader.seekg(reader.getLength());
    REQUIRE_THROWS(reader.readUInt8());
}

TEST_CASE("correctly reads istream", "[BinaryReader]")
{
    std::array<unsigned char, 3> bytes{0x00, 0xFF, 0x7F};
    std::istringstream ss(std::string(std::begin(bytes), std::end(bytes)));
    sciformats::io::BinaryReader reader(
        ss, sciformats::io::Endianness::BigEndian);

    REQUIRE(reader.tellg() == 0);
    REQUIRE(reader.getLength() == 3);
    REQUIRE(reader.readUInt8() == 0x00);
    REQUIRE(reader.readUInt8() == 0xFF);
    REQUIRE(reader.readUInt8() == 0x7F);
    REQUIRE(reader.tellg() == sizeof(bytes));
    reader.seekg(1);
    REQUIRE(reader.tellg() == 1);
    REQUIRE(reader.readUInt8() == 0xFF);
}

TEST_CASE("throws exception when reading past and constructed from istream "
          "if argument istream is configured to throw exceptions "
          "or constructor argument for activating exceptions is true",
    "[BinaryReader]")
{
    std::array<unsigned char, 3> bytes{0x00, 0xFF, 0x7F};
    std::istringstream ss_nothrow(
        std::string(std::begin(bytes), std::end(bytes)));
    sciformats::io::BinaryReader reader_nothrow(
        ss_nothrow, sciformats::io::Endianness::BigEndian, false);

    // unlike for other reader constructors, for constructor the behavior of the
    // argument stream determines if it throws
    reader_nothrow.seekg(3);
    REQUIRE_NOTHROW(reader_nothrow.readUInt8());

    std::istringstream ss_nothrow2(
        std::string(std::begin(bytes), std::end(bytes)));
    sciformats::io::BinaryReader reader_nothrow2(
        ss_nothrow2, sciformats::io::Endianness::BigEndian, true);

    reader_nothrow2.seekg(3);
    REQUIRE_THROWS(reader_nothrow2.readUInt8());

    std::istringstream ss_throw(
        std::string(std::begin(bytes), std::end(bytes)));
    ss_throw.exceptions(
        std::ios::eofbit | std::ios::failbit | std::ios::badbit);
    sciformats::io::BinaryReader reader_throw(
        ss_throw, sciformats::io::Endianness::BigEndian, false);

    reader_throw.seekg(3);
    REQUIRE_THROWS(reader_throw.readUInt8());
}

TEST_CASE("correctly reads char vector", "[BinaryReader]")
{
    std::vector<char> bytes{0x00_c, 0xFF_c, 0x7F_c};
    sciformats::io::BinaryReader reader(bytes);

    REQUIRE(reader.tellg() == 0);
    REQUIRE(reader.getLength() == 3);
    REQUIRE(reader.readUInt8() == 0x00);
    REQUIRE(reader.readUInt8() == 0xFF);
    REQUIRE(reader.readUInt8() == 0x7F);
    REQUIRE(reader.tellg() == static_cast<std::streamoff>(bytes.size()));
    reader.seekg(1);
    REQUIRE(reader.tellg() == 1);
    REQUIRE(reader.readUInt8() == 0xFF);
}

TEST_CASE(
    "throws exception when reading past end and constructed from char vector",
    "[BinaryReader]")
{
    std::vector<char> bytes{0x00_c, 0xFF_c, 0x7F_c};
    sciformats::io::BinaryReader reader(bytes);

    reader.seekg(3);
    REQUIRE_THROWS(reader.readUInt8());
}

TEST_CASE("correctly reads uint8_t vector", "[BinaryReader]")
{
    std::vector<uint8_t> bytes{0x00, 0xFF, 0x7F};
    sciformats::io::BinaryReader reader(bytes);

    REQUIRE(reader.tellg() == 0);
    REQUIRE(reader.getLength() == 3);
    REQUIRE(reader.readUInt8() == 0x00);
    REQUIRE(reader.readUInt8() == 0xFF);
    REQUIRE(reader.readUInt8() == 0x7F);
    REQUIRE(reader.tellg() == static_cast<std::streamoff>(bytes.size()));
    reader.seekg(1);
    REQUIRE(reader.tellg() == 1);
    REQUIRE(reader.readUInt8() == 0xFF);
    reader.seekg(3);
    REQUIRE_THROWS(reader.readUInt8());
}

TEST_CASE("throws exception when reading past end and constructed from uint8_t "
          "vector",
    "[BinaryReader]")
{
    std::vector<uint8_t> bytes{0x00, 0xFF, 0x7F};
    sciformats::io::BinaryReader reader(bytes);

    reader.seekg(3);
    REQUIRE_THROWS(reader.readUInt8());
}

TEST_CASE("read int8 correctly", "[BinaryReader]")
{
    std::vector<uint8_t> bytes{0xFF};
    sciformats::io::BinaryReader reader(bytes);

    REQUIRE(reader.readInt8() == -1);
}

TEST_CASE("read uint8 correctly", "[BinaryReader]")
{
    std::vector<uint8_t> bytes{0xFF};
    sciformats::io::BinaryReader reader(bytes);

    REQUIRE(reader.readUInt8() == 255);
}

TEST_CASE("read int16 correctly", "[BinaryReader]")
{
    auto expected = -256;

    // little endian
    std::vector<uint8_t> bytes{0x00, 0xFF};
    sciformats::io::BinaryReader reader_le(
        bytes, sciformats::io::Endianness::LittleEndian);

    REQUIRE(reader_le.readInt16() == expected);
    reader_le.seekg(0, std::ios::beg);
    REQUIRE(reader_le.readInt16(sciformats::io::Endianness::LittleEndian)
            == expected);

    // big endian
    std::reverse(bytes.begin(), bytes.end());
    sciformats::io::BinaryReader reader_be(
        bytes, sciformats::io::Endianness::BigEndian);

    REQUIRE(reader_be.readInt16() == expected);
    reader_be.seekg(0, std::ios::beg);
    REQUIRE(
        reader_be.readInt16(sciformats::io::Endianness::BigEndian) == expected);
}

TEST_CASE("read uint16 correctly", "[BinaryReader]")
{
    auto expected = 65280;

    // little endian
    std::vector<uint8_t> bytes{0x00, 0xFF};
    sciformats::io::BinaryReader reader_le(
        bytes, sciformats::io::Endianness::LittleEndian);

    REQUIRE(reader_le.readUInt16() == expected);
    reader_le.seekg(0, std::ios::beg);
    REQUIRE(reader_le.readUInt16(sciformats::io::Endianness::LittleEndian)
            == expected);

    // big endian
    std::reverse(bytes.begin(), bytes.end());
    sciformats::io::BinaryReader reader_be(
        bytes, sciformats::io::Endianness::BigEndian);

    REQUIRE(reader_be.readUInt16() == expected);
    reader_be.seekg(0, std::ios::beg);
    REQUIRE(reader_be.readUInt16(sciformats::io::Endianness::BigEndian)
            == expected);
}

TEST_CASE("read int32 correctly", "[BinaryReader]")
{
    auto expected = -16777216L;

    // little endian
    std::vector<uint8_t> bytes{0x00, 0x00, 0x00, 0xFF};
    sciformats::io::BinaryReader reader_le(
        bytes, sciformats::io::Endianness::LittleEndian);

    REQUIRE(reader_le.readInt32() == expected);
    reader_le.seekg(0, std::ios::beg);
    REQUIRE(reader_le.readInt32(sciformats::io::Endianness::LittleEndian)
            == expected);

    // big endian
    std::reverse(bytes.begin(), bytes.end());
    sciformats::io::BinaryReader reader_be(
        bytes, sciformats::io::Endianness::BigEndian);

    REQUIRE(reader_be.readInt32() == expected);
    reader_be.seekg(0, std::ios::beg);
    REQUIRE(
        reader_be.readInt32(sciformats::io::Endianness::BigEndian) == expected);
}

TEST_CASE("read uint32 correctly", "[BinaryReader]")
{
    auto expected = 4278190080UL;

    // little endian
    std::vector<uint8_t> bytes{0x00, 0x00, 0x00, 0xFF};
    sciformats::io::BinaryReader reader_le(
        bytes, sciformats::io::Endianness::LittleEndian);

    REQUIRE(reader_le.readUInt32() == expected);
    reader_le.seekg(0, std::ios::beg);
    REQUIRE(reader_le.readUInt32(sciformats::io::Endianness::LittleEndian)
            == expected);

    // big endian
    std::reverse(bytes.begin(), bytes.end());
    sciformats::io::BinaryReader reader_be(
        bytes, sciformats::io::Endianness::BigEndian);

    REQUIRE(reader_be.readUInt32() == expected);
    reader_be.seekg(0, std::ios::beg);
    REQUIRE(reader_be.readUInt32(sciformats::io::Endianness::BigEndian)
            == expected);
}

TEST_CASE("read int64 correctly", "[BinaryReader]")
{
    auto expected = -72057594037927936LL;

    // little endian
    std::vector<uint8_t> bytes{0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF};
    sciformats::io::BinaryReader reader_le(
        bytes, sciformats::io::Endianness::LittleEndian);

    REQUIRE(reader_le.readInt64() == expected);
    reader_le.seekg(0, std::ios::beg);
    REQUIRE(reader_le.readInt64(sciformats::io::Endianness::LittleEndian)
            == expected);

    // big endian
    std::reverse(bytes.begin(), bytes.end());
    sciformats::io::BinaryReader reader_be(
        bytes, sciformats::io::Endianness::BigEndian);

    REQUIRE(reader_be.readInt64() == expected);
    reader_be.seekg(0, std::ios::beg);
    REQUIRE(
        reader_be.readInt64(sciformats::io::Endianness::BigEndian) == expected);
}

TEST_CASE("read uint64 correctly", "[BinaryReader]")
{
    auto expected = 18374686479671623680ULL;

    // little endian
    std::vector<uint8_t> bytes{0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF};
    sciformats::io::BinaryReader reader_le(
        bytes, sciformats::io::Endianness::LittleEndian);

    REQUIRE(reader_le.readUInt64() == expected);
    reader_le.seekg(0, std::ios::beg);
    REQUIRE(reader_le.readUInt64(sciformats::io::Endianness::LittleEndian)
            == expected);

    // big endian
    std::reverse(bytes.begin(), bytes.end());
    sciformats::io::BinaryReader reader_be(
        bytes, sciformats::io::Endianness::BigEndian);

    REQUIRE(reader_be.readUInt64() == expected);
    reader_be.seekg(0, std::ios::beg);
    REQUIRE(reader_be.readUInt64(sciformats::io::Endianness::BigEndian)
            == expected);
}

TEST_CASE("read float32 correctly", "[BinaryReader]")
{
    auto expected = 2.5F;

    // little endian
    std::vector<uint8_t> bytes{0x00, 0x00, 0x20, 0x40};
    sciformats::io::BinaryReader reader_le(
        bytes, sciformats::io::Endianness::LittleEndian);

    REQUIRE(reader_le.readFloat() == Approx(expected));
    reader_le.seekg(0, std::ios::beg);
    REQUIRE(reader_le.readFloat(sciformats::io::Endianness::LittleEndian)
            == Approx(expected));

    // big endian
    std::reverse(bytes.begin(), bytes.end());
    sciformats::io::BinaryReader reader_be(
        bytes, sciformats::io::Endianness::BigEndian);

    REQUIRE(reader_be.readFloat() == Approx(expected));
    reader_be.seekg(0, std::ios::beg);
    REQUIRE(reader_be.readFloat(sciformats::io::Endianness::BigEndian)
            == Approx(expected));
}

TEST_CASE("read float64 correctly", "[BinaryReader]")
{
    auto expected = 2.5;

    // little endian
    std::vector<uint8_t> bytes{0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x40};
    sciformats::io::BinaryReader reader_le(
        bytes, sciformats::io::Endianness::LittleEndian);

    REQUIRE(reader_le.readDouble() == Approx(expected));
    reader_le.seekg(0, std::ios::beg);
    REQUIRE(reader_le.readDouble(sciformats::io::Endianness::LittleEndian)
            == Approx(expected));

    // big endian
    std::reverse(bytes.begin(), bytes.end());
    sciformats::io::BinaryReader reader_be(
        bytes, sciformats::io::Endianness::BigEndian);

    REQUIRE(reader_be.readDouble() == Approx(expected));
    reader_be.seekg(0, std::ios::beg);
    REQUIRE(reader_be.readDouble(sciformats::io::Endianness::BigEndian)
            == Approx(expected));
}

TEST_CASE("read chars into vector correctly", "[BinaryReader]")
{
    std::vector<char> bytes{0x00, 0x01, 0x02, 0xFF_c};
    sciformats::io::BinaryReader reader(bytes);
    size_t size = bytes.size();

    auto output = reader.readChars(size);

    REQUIRE(output.size() == size);
    REQUIRE(output.at(0) == bytes[0]);
    REQUIRE(output.at(1) == bytes[1]);
    REQUIRE(output.at(2) == bytes[2]);
    REQUIRE(output.at(3) == bytes[3]);
}

TEST_CASE("read bytes into vector correctly", "[BinaryReader]")
{
    std::vector<uint8_t> bytes{0x00, 0x01, 0x02, 0xFF};
    sciformats::io::BinaryReader reader(bytes);
    size_t size = bytes.size();

    auto output = reader.readBytes(size);

    REQUIRE(output.size() == size);
    REQUIRE(output.at(0) == bytes[0]);
    REQUIRE(output.at(1) == bytes[1]);
    REQUIRE(output.at(2) == bytes[2]);
    REQUIRE(output.at(3) == bytes[3]);
}

TEST_CASE("read ISO-8859-1 encoded string correctly", "[BinaryReader]")
{
    // all ISO-8859-1 characters ISO-8859-1 encoded
    std::vector<uint8_t> bytes{// printable ascii characters
        0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b,
        0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37,
        0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f, 0x40, 0x41, 0x42, 0x43,
        0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f,
        0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b,
        0x5c, 0x5d, 0x5e, 0x5f, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67,
        0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73,
        0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e,
        // ISO-8859-1 additional characters
        0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab,
        0xac, 0xad, 0xae, 0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7,
        0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 0xc0, 0xc1, 0xc2, 0xc3,
        0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf,
        0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb,
        0xdc, 0xdd, 0xde, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7,
        0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3,
        0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff};
    // specifying UTF-8 string literals is error prone
    // see:
    // https://stackoverflow.com/questions/23471935/how-are-u8-literals-supposed-to-work?rq=1
    // all ISO-8859-1 characters UTF-8 encoded
    auto expected = std::string{// printable ascii characters
        u8" !\"#$%&'()*+"
        ",-./01234567"
        "89:;<=>?@ABC"
        "DEFGHIJKLMNO"
        "PQRSTUVWXYZ["
        "\\]^_`abcdefg"
        "hijklmnopqrs"
        "tuvwxyz{|}~"
        // ISO-8859-1 additional characters
        "\u00a0¡¢£¤¥¦§¨©ª«"
        "¬\u00ad®¯°±²³´µ¶·"
        "¸¹º»¼½¾¿ÀÁÂÃ"
        "ÄÅÆÇÈÉÊËÌÍÎÏ"
        "ÐÑÒÓÔÕÖ×ØÙÚÛ"
        "ÜÝÞßàáâãäåæç"
        "èéêëìíîïðñòó"
        "ôõö÷øùúûüýþÿ"};

    sciformats::io::BinaryReader reader(bytes);
    auto output
        = reader.readString("ISO-8859-1", static_cast<int32_t>(bytes.size()));

    REQUIRE(output.size() == expected.size());
    for (size_t i = 0; i < expected.size(); i++)
    {
        REQUIRE(output.at(i) == expected.at(i));
    }
}

TEST_CASE("all single byte values can be converted from ISO-8859-1 to UTF-8",
    "[BinaryReader]")
{
    // all 255 non \0 ISO-8859-1 characters from 0x01 to 0xff, see:
    // https://icu4c-demos.unicode.org/icu-bin/convexp?conv=ISO-8859-1&s=ALL
    std::vector<uint8_t> bytes{};
    bytes.resize(255);
    std::iota(std::begin(bytes), std::end(bytes), 1);

    // U+FFFD REPLACEMENT CHARACTER for unmappable sequence, see:
    // http://userguide.icu-project.org/conversion/converters
    auto replacementChar = std::string{u8"�"};

    sciformats::io::BinaryReader reader(bytes);
    auto output
        = reader.readString("ISO-8859-1", static_cast<int32_t>(bytes.size()));

    // REPLACEMENT CHARACTER not in generated UTF-8 string
    REQUIRE(output.find(replacementChar) == std::string::npos);
    // first 127 chars (after initial \0) as single byte UTF-8 code points,
    // 2nd 128 chars as 2 byte UTF-8 code points
    REQUIRE(output.size() == 127 + 128 * 2);
}

TEST_CASE(
    "show escape character for byte values illegal in ASCII", "[BinaryReader]")
{
    // characters not defined in ASCII
    std::vector<uint8_t> bytes{
        0x41, 0x42, 0x43, 0x80, 0x90, 0xa0, 0x61, 0x62, 0x63};
    // U+FFFD REPLACEMENT CHARACTER for unmappable sequence, see:
    // http://userguide.icu-project.org/conversion/converters
    auto expected = std::string{u8"ABC���abc"};

    sciformats::io::BinaryReader reader(bytes);
    // does not accept "ASCII" in Emscripten build, but in Linux build
    auto output
        = reader.readString("US-ASCII", static_cast<int32_t>(bytes.size()));

    REQUIRE(output.size() == expected.size());
    for (size_t i = 0; i < expected.size(); i++)
    {
        REQUIRE(output.at(i) == expected.at(i));
    }
}

TEST_CASE("read UTF-8 encoded string correctly", "[BinaryReader]")
{
    // "!\"#123ABCabcä®€𝄞ह한" UTF-8 encoded
    std::vector<uint8_t> bytes{0x21, 0x22, 0x23, 0x31, 0x32, 0x33, 0x41, 0x42,
        0x43, 0x61, 0x62, 0x63, 0xc3, 0xa4, 0xc2, 0xae, 0xe2, 0x82, 0xac, 0xf0,
        0x9d, 0x84, 0x9e, 0xe0, 0xa4, 0xb9, 0xed, 0x95, 0x9c};
    auto expected = std::string{u8"!\"#123ABCabcä®€𝄞ह한"};

    sciformats::io::BinaryReader reader(bytes);
    auto output
        = reader.readString("UTF-8", static_cast<int32_t>(bytes.size()));

    REQUIRE(output.size() == expected.size());
    for (size_t i = 0; i < expected.size(); i++)
    {
        REQUIRE(output.at(i) == expected.at(i));
    }
}

TEST_CASE("show escape character for byte sequences illegal in UTF-8",
    "[BinaryReader]")
{
    // characters not defined in UTF-8
    std::vector<uint8_t> bytes{0x41, 0x80, 0x61};
    // U+FFFD REPLACEMENT CHARACTER for unmappable sequence, see:
    // http://userguide.icu-project.org/conversion/converters
    auto expected = std::string{u8"A�a"};

    sciformats::io::BinaryReader reader(bytes);
    auto output
        = reader.readString("UTF-8", static_cast<int32_t>(bytes.size()));

    REQUIRE(output.size() == expected.size());
    for (size_t i = 0; i < expected.size(); i++)
    {
        REQUIRE(output.at(i) == expected.at(i));
    }
}

TEST_CASE("read UTF-16BE encoded string correctly", "[BinaryReader]")
{
    // "!\"#123ABCabcä®€𝄞ह한" UTF-16BE encoded
    std::vector<uint8_t> bytes{0x00, 0x21, 0x00, 0x22, 0x00, 0x23, 0x00, 0x31,
        0x00, 0x32, 0x00, 0x33, 0x00, 0x41, 0x00, 0x42, 0x00, 0x43, 0x00, 0x61,
        0x00, 0x62, 0x00, 0x63, 0x00, 0xe4, 0x00, 0xae, 0x20, 0xac, 0xd8, 0x34,
        0xdd, 0x1e, 0x09, 0x39, 0xd5, 0x5c};
    auto expected = std::string{u8"!\"#123ABCabcä®€𝄞ह한"};

    sciformats::io::BinaryReader reader(bytes);
    auto output
        = reader.readString("UTF-16BE", static_cast<int32_t>(bytes.size()));

    REQUIRE(output.size() == expected.size());
    for (size_t i = 0; i < expected.size(); i++)
    {
        REQUIRE(output.at(i) == expected.at(i));
    }
}

TEST_CASE("show escape character for byte sequences illegal in UTF-16BE",
    "[BinaryReader]")
{
    // characters not defined in UTF-16BE
    std::vector<uint8_t> bytes{0x00, 0x41, 0xd8, 0x34, 0x00, 0x61};
    // U+FFFD REPLACEMENT CHARACTER for unmappable sequence, see:
    // http://userguide.icu-project.org/conversion/converters
    auto expected = std::string{u8"A�a"};

    sciformats::io::BinaryReader reader(bytes);
    auto output
        = reader.readString("UTF-16BE", static_cast<int32_t>(bytes.size()));

    REQUIRE(output.size() == expected.size());
    for (size_t i = 0; i < expected.size(); i++)
    {
        REQUIRE(output.at(i) == expected.at(i));
    }
}

TEST_CASE("read UTF-16LE encoded string correctly", "[BinaryReader]")
{
    // "!\"#123ABCabcä®€𝄞ह한" UTF-16LE encoded
    std::vector<uint8_t> bytes{0x21, 0x00, 0x22, 0x00, 0x23, 0x00, 0x31, 0x00,
        0x32, 0x00, 0x33, 0x00, 0x41, 0x00, 0x42, 0x00, 0x43, 0x00, 0x61, 0x00,
        0x62, 0x00, 0x63, 0x00, 0xe4, 0x00, 0xae, 0x00, 0xac, 0x20, 0x34, 0xd8,
        0x1e, 0xdd, 0x39, 0x09, 0x5c, 0xd5};
    auto expected = std::string{u8"!\"#123ABCabcä®€𝄞ह한"};

    sciformats::io::BinaryReader reader(bytes);
    auto output
        = reader.readString("UTF-16LE", static_cast<int32_t>(bytes.size()));

    REQUIRE(output.size() == expected.size());
    for (size_t i = 0; i < expected.size(); i++)
    {
        REQUIRE(output.at(i) == expected.at(i));
    }
    std::cout << std::endl;
}

TEST_CASE("read zero terminated ISO-8859-1 encoded string correctly",
    "[BinaryReader]")
{
    // "ab\0cd" ISO-8859-1 encoded
    std::vector<uint8_t> bytes{0x61, 0x62, 0x00, 0x63, 0x64};
    auto expected = std::string{u8"ab"};

    sciformats::io::BinaryReader reader(bytes);
    auto output
        = reader.readString("ISO-8859-1", static_cast<int32_t>(bytes.size()));

    REQUIRE(output.size() == expected.size());
    for (size_t i = 0; i < expected.size(); i++)
    {
        REQUIRE(output.at(i) == expected.at(i));
    }
}

TEST_CASE(
    "read zero terminated UTF-8 encoded string correctly", "[BinaryReader]")
{
    // "aäb\0cd" UTF-8 encoded
    std::vector<uint8_t> bytes{0x61, 0xc3, 0xa4, 0x62, 0x00, 0x63, 0x64};
    auto expected = std::string{u8"aäb"};

    sciformats::io::BinaryReader reader(bytes);
    auto output
        = reader.readString("UTF-8", static_cast<int32_t>(bytes.size()));

    REQUIRE(output.size() == expected.size());
    for (size_t i = 0; i < expected.size(); i++)
    {
        REQUIRE(output.at(i) == expected.at(i));
    }
}

TEST_CASE(
    "read zero terminated UTF-16BE encoded string correctly", "[BinaryReader]")
{
    // "aä\0bc" UTF-16BE encoded
    std::vector<uint8_t> bytes{
        0x00, 0x61, 0x00, 0xe4, 0x00, 0x00, 0x00, 0x62, 0x00, 0x63};
    auto expected = std::string{u8"aä"};

    sciformats::io::BinaryReader reader(bytes);
    auto output
        = reader.readString("UTF-16BE", static_cast<int32_t>(bytes.size()));

    REQUIRE(output.size() == expected.size());
    for (size_t i = 0; i < expected.size(); i++)
    {
        REQUIRE(output.at(i) == expected.at(i));
    }
}

TEST_CASE("show escape character for byte sequences illegal in UTF-16LE",
    "[BinaryReader]")
{
    // characters not defined in UTF-16LE
    std::vector<uint8_t> bytes{0x41, 0x00, 0x34, 0xd8, 0x61, 0x00};
    // U+FFFD REPLACEMENT CHARACTER for unmappable sequence, see:
    // http://userguide.icu-project.org/conversion/converters
    auto expected = std::string{u8"A�a"};

    sciformats::io::BinaryReader reader(bytes);
    auto output
        = reader.readString("UTF-16LE", static_cast<int32_t>(bytes.size()));

    REQUIRE(output.size() == expected.size());
    for (size_t i = 0; i < expected.size(); i++)
    {
        REQUIRE(output.at(i) == expected.at(i));
    }
}

TEST_CASE(
    "read zero terminated UTF-16LE encoded string correctly", "[BinaryReader]")
{
    // "aä\0bc" UTF-16LE encoded
    std::vector<uint8_t> bytes{
        0x61, 0x00, 0xe4, 0x00, 0x00, 0x00, 0x62, 0x00, 0x63, 0x00};
    auto expected = std::string{u8"aä"};

    sciformats::io::BinaryReader reader(bytes);
    auto output
        = reader.readString("UTF-16LE", static_cast<int32_t>(bytes.size()));

    REQUIRE(output.size() == expected.size());
    for (size_t i = 0; i < expected.size(); i++)
    {
        REQUIRE(output.at(i) == expected.at(i));
    }
}

TEST_CASE(
    "reading negative length string results in empty string", "[BinaryReader]")
{
    // "a" UTF-16LE encoded
    std::vector<uint8_t> bytes{0x61, 0x00};

    sciformats::io::BinaryReader reader(bytes);
    auto output = reader.readString("UTF-16LE", -1);

    REQUIRE(output.empty());
}

TEST_CASE(
    "reading string with length exceeding half maximum for int32_t results in "
    "runtime_error",
    "[BinaryReader]")
{
    // "a" UTF-16LE encoded
    std::vector<uint8_t> bytes{0x61, 0x00};

    sciformats::io::BinaryReader reader(bytes);
    REQUIRE_THROWS(reader.readString(
        "UTF-16LE", std::numeric_limits<int32_t>::max() / 2 + 1));
}

TEST_CASE(
    "read Int8Chars8 prefixed UTF-8 encoded string correctly", "[BinaryReader]")
{
    // "abc" UTF-8 encoded, 3 as length prefix (8 bit)
    std::vector<uint8_t> bytes{0x03, 0x61, 0x62, 0x63};
    auto expected = std::string{u8"abc"};

    sciformats::io::StringPrefixType prefixType{
        sciformats::io::StringPrefixNumericType::Int8Chars8,
        sciformats::io::Endianness::LittleEndian};
    sciformats::io::BinaryReader reader(bytes);
    auto output = reader.readPrefixedString(prefixType, "UTF-8");

    REQUIRE(output.size() == expected.size());
    for (size_t i = 0; i < expected.size(); i++)
    {
        REQUIRE(output.at(i) == expected.at(i));
    }
}

TEST_CASE("read UInt8Chars8 prefixed UTF-8 encoded string correctly",
    "[BinaryReader]")
{
    // "abc" UTF-8 encoded, 3 as length prefix (8 bit)
    std::vector<uint8_t> bytes{0x03, 0x61, 0x62, 0x63};
    auto expected = std::string{u8"abc"};

    sciformats::io::StringPrefixType prefixType{
        sciformats::io::StringPrefixNumericType::UInt8Chars8,
        sciformats::io::Endianness::LittleEndian};
    sciformats::io::BinaryReader reader(bytes);
    auto output = reader.readPrefixedString(prefixType, "UTF-8");

    REQUIRE(output.size() == expected.size());
    for (size_t i = 0; i < expected.size(); i++)
    {
        REQUIRE(output.at(i) == expected.at(i));
    }
}

TEST_CASE("read Int8Chars16 prefixed UTF-8 encoded string correctly",
    "[BinaryReader]")
{
    // "abc" UTF-8 encoded, 3 as length prefix (8 bit)
    std::vector<uint8_t> bytes{0x03, 0x61, 0x00, 0x62, 0x00, 0x63, 0x00};
    auto expected = std::string{u8"abc"};

    sciformats::io::StringPrefixType prefixType{
        sciformats::io::StringPrefixNumericType::Int8Chars16,
        sciformats::io::Endianness::LittleEndian};
    sciformats::io::BinaryReader reader(bytes);
    auto output = reader.readPrefixedString(prefixType, "UTF-16LE");

    REQUIRE(output.size() == expected.size());
    for (size_t i = 0; i < expected.size(); i++)
    {
        REQUIRE(output.at(i) == expected.at(i));
    }
}

TEST_CASE("read UInt8Chars16 prefixed UTF-8 encoded string correctly",
    "[BinaryReader]")
{
    // "abc" UTF-16LE encoded, 3 as length prefix (8 bit)
    std::vector<uint8_t> bytes{0x03, 0x61, 0x00, 0x62, 0x00, 0x63, 0x00};
    auto expected = std::string{u8"abc"};

    sciformats::io::StringPrefixType prefixType{
        sciformats::io::StringPrefixNumericType::UInt8Chars16,
        sciformats::io::Endianness::LittleEndian};
    sciformats::io::BinaryReader reader(bytes);
    auto output = reader.readPrefixedString(prefixType, "UTF-16LE");

    REQUIRE(output.size() == expected.size());
    for (size_t i = 0; i < expected.size(); i++)
    {
        REQUIRE(output.at(i) == expected.at(i));
    }
}

TEST_CASE("read Int16LEChars8 prefixed UTF-8 encoded string correctly",
    "[BinaryReader]")
{
    // "abc" UTF-8 encoded, 3 as length prefix (16 bit LE)
    std::vector<uint8_t> bytes{0x03, 0x00, 0x61, 0x62, 0x63};
    auto expected = std::string{u8"abc"};

    sciformats::io::StringPrefixType prefixType{
        sciformats::io::StringPrefixNumericType::Int16Chars8,
        sciformats::io::Endianness::LittleEndian};
    sciformats::io::BinaryReader reader(bytes);
    auto output = reader.readPrefixedString(prefixType, "UTF-8");

    REQUIRE(output.size() == expected.size());
    for (size_t i = 0; i < expected.size(); i++)
    {
        REQUIRE(output.at(i) == expected.at(i));
    }
}

TEST_CASE("read Int16BEChars8 prefixed UTF-8 encoded string correctly",
    "[BinaryReader]")
{
    // "abc" UTF-8 encoded, 3 as length prefix (16 bit LE)
    std::vector<uint8_t> bytes{0x00, 0x03, 0x61, 0x62, 0x63};
    auto expected = std::string{u8"abc"};

    sciformats::io::StringPrefixType prefixType{
        sciformats::io::StringPrefixNumericType::Int16Chars8,
        sciformats::io::Endianness::BigEndian};
    sciformats::io::BinaryReader reader(bytes);
    auto output = reader.readPrefixedString(prefixType, "UTF-8");

    REQUIRE(output.size() == expected.size());
    for (size_t i = 0; i < expected.size(); i++)
    {
        REQUIRE(output.at(i) == expected.at(i));
    }
}

TEST_CASE("read UInt16LEChars8 prefixed UTF-8 encoded string correctly",
    "[BinaryReader]")
{
    // "abc" UTF-8 encoded, 3 as length prefix (16 bit BE)
    std::vector<uint8_t> bytes{0x03, 0x00, 0x61, 0x62, 0x63};
    auto expected = std::string{u8"abc"};

    sciformats::io::StringPrefixType prefixType{
        sciformats::io::StringPrefixNumericType::UInt16Chars8,
        sciformats::io::Endianness::LittleEndian};
    sciformats::io::BinaryReader reader(bytes);
    auto output = reader.readPrefixedString(prefixType, "UTF-8");

    REQUIRE(output.size() == expected.size());
    for (size_t i = 0; i < expected.size(); i++)
    {
        REQUIRE(output.at(i) == expected.at(i));
    }
}

TEST_CASE("read Int16LEChars16 prefixed UTF-16LE encoded string correctly",
    "[BinaryReader]")
{
    // "abc" UTF-16LE encoded, 3 as length prefix (16 bit LE)
    std::vector<uint8_t> bytes{0x03, 0x00, 0x61, 0x00, 0x62, 0x00, 0x63, 0x00};
    auto expected = std::string{u8"abc"};

    sciformats::io::StringPrefixType prefixType{
        sciformats::io::StringPrefixNumericType::Int16Chars16,
        sciformats::io::Endianness::LittleEndian};
    sciformats::io::BinaryReader reader(bytes);
    auto output = reader.readPrefixedString(prefixType, "UTF-16LE");

    REQUIRE(output.size() == expected.size());
    for (size_t i = 0; i < expected.size(); i++)
    {
        REQUIRE(output.at(i) == expected.at(i));
    }
}

TEST_CASE("read Int16LEChars16 prefixed UTF-16LE encoded "
          "zero terminated string correctly",
    "[BinaryReader]")
{
    // "abc" with zero terminator and trailing char UTF-16LE encoded
    std::vector<uint8_t> bytes{
        0x05, 0x00, 0x61, 0x00, 0x62, 0x00, 0x63, 0x00, 0x00, 0x00, 0x64, 0x00};
    auto expected = std::string{u8"abc"};

    sciformats::io::StringPrefixType prefixType{
        sciformats::io::StringPrefixNumericType::Int16Chars16,
        sciformats::io::Endianness::LittleEndian};
    sciformats::io::BinaryReader reader(bytes);
    auto output = reader.readPrefixedString(prefixType, "UTF-16LE");

    REQUIRE(output.size() == expected.size());
    for (size_t i = 0; i < expected.size(); i++)
    {
        REQUIRE(output.at(i) == expected.at(i));
    }
}

TEST_CASE(
    "read UInt16LEChars16 prefixed UTF-16LE string correctly", "[BinaryReader]")
{
    // "abc" UTF-16LE encoded, 3 as length prefix (16 bit LE)
    std::vector<uint8_t> bytes{0x03, 0x00, 0x61, 0x00, 0x62, 0x00, 0x63, 0x00};
    auto expected = std::string{u8"abc"};

    sciformats::io::StringPrefixType prefixType{
        sciformats::io::StringPrefixNumericType::UInt16Chars16,
        sciformats::io::Endianness::LittleEndian};
    sciformats::io::BinaryReader reader(bytes);
    auto output = reader.readPrefixedString(prefixType, "UTF-16LE");

    REQUIRE(output.size() == expected.size());
    for (size_t i = 0; i < expected.size(); i++)
    {
        REQUIRE(output.at(i) == expected.at(i));
    }
}

TEST_CASE("read UInt16LEChars16 prefixed UTF-16LE encoded"
          "zero terminated string correctly",
    "[BinaryReader]")
{
    // "abc" with zero terminator and trailing char UTF-16LE encoded, 5 as
    // length prefix (16 bit LE)
    std::vector<uint8_t> bytes{
        0x05, 0x00, 0x61, 0x00, 0x62, 0x00, 0x63, 0x00, 0x00, 0x00, 0x64, 0x00};
    auto expected = std::string{u8"abc"};

    sciformats::io::StringPrefixType prefixType{
        sciformats::io::StringPrefixNumericType::UInt16Chars16,
        sciformats::io::Endianness::LittleEndian};
    sciformats::io::BinaryReader reader(bytes);
    auto output = reader.readPrefixedString(prefixType, "UTF-16LE");

    REQUIRE(output.size() == expected.size());
    for (size_t i = 0; i < expected.size(); i++)
    {
        REQUIRE(output.at(i) == expected.at(i));
    }
}

TEST_CASE("when reading zero terminated string reader is moved forward by "
          "prefix specified length",
    "[BinaryReader]")
{
    // "abc" with zero terminator and trailing char UTF-16LE encoded, 5 as
    // length prefix (16 bit LE)
    std::vector<uint8_t> bytes{0x05, 0x00, 0x61, 0x00, 0x62, 0x00, 0x63, 0x00,
        0x00, 0x00, 0x64, 0x00, 0x00, 0x00};
    auto expected = std::string{u8"abc"};

    sciformats::io::StringPrefixType prefixType{
        sciformats::io::StringPrefixNumericType::UInt16Chars16,
        sciformats::io::Endianness::LittleEndian};
    sciformats::io::BinaryReader reader(bytes);

    REQUIRE(reader.tellg() == 0);

    reader.readPrefixedString(prefixType, "UTF-16LE");

    REQUIRE(reader.tellg() == 12);
}

TEST_CASE("throws exception when Int16LEChars16 prefix value "
          "exceeds specified maxSize",
    "[BinaryReader]")
{
    // "ab" UTF-16LE encoded
    std::vector<uint8_t> bytes{0x02, 0x00, 0x61, 0x00, 0x62};

    sciformats::io::StringPrefixType prefixType{
        sciformats::io::StringPrefixNumericType::Int16Chars16,
        sciformats::io::Endianness::LittleEndian};
    sciformats::io::BinaryReader reader(bytes);
    REQUIRE_THROWS(reader.readPrefixedString(prefixType, "UTF-16LE", 3));
}

TEST_CASE("throws exception when maxSize exceeds "
          "std::numeric_limits<uint16_t>::max()",
    "[BinaryReader]")
{
    // "abc" with zero terminator and trailing char UTF-16LE encoded
    std::vector<uint8_t> bytes{0x03, 0x00, 0x61, 0x00, 0x62, 0x00, 0x63, 0x00};

    sciformats::io::StringPrefixType prefixType{
        sciformats::io::StringPrefixNumericType::Int16Chars16,
        sciformats::io::Endianness::LittleEndian};
    sciformats::io::BinaryReader reader(bytes);
    REQUIRE_THROWS(reader.readPrefixedString(
        prefixType, "UTF-16LE", std::numeric_limits<uint16_t>::max() + 1));
}

TEST_CASE("negative prefix results in empty string", "[BinaryReader]")
{
    // "abc" UTF-16LE encoded, -1 as length prefix
    std::vector<uint8_t> bytes{0xff, 0xff, 0x61, 0x00, 0x62, 0x00, 0x63, 0x00};

    sciformats::io::StringPrefixType prefixType{
        sciformats::io::StringPrefixNumericType::Int16Chars16,
        sciformats::io::Endianness::LittleEndian};
    sciformats::io::BinaryReader reader(bytes);
    auto output = reader.readPrefixedString(prefixType, "UTF-16LE");

    REQUIRE(output.empty());
}

TEST_CASE(
    "throws exception when trying to reading string with non-existent encoding",
    "[BinaryReader]")
{
    // "abc" in ASCII
    std::vector<uint8_t> bytes{0x61, 0x62, 0x63};
    sciformats::io::BinaryReader reader(bytes);

    REQUIRE_THROWS(reader.readString("non-existent encoding name", 1));
}
