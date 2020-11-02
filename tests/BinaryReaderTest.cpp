#define CATCH_CONFIG_MAIN
#include "binaryreader/BinaryReader.hpp"
#include "binaryreader/Endianness.hpp"

#include "catch2/catch.hpp"

#include <array>
#include <climits>

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
    REQUIRE(reader.tellg() == bytes.size());
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
    REQUIRE(reader.tellg() == bytes.size());
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
    // TODO: test for all ISO-8859-1 characters

    // "abcABCÄÖÜäöü" ISO-8859-1 encoded
    std::vector<uint8_t> bytes{
        0x41, 0x42, 0x43, 0x61, 0x62, 0x63, 0xc4, 0xd6, 0xdc, 0xe4, 0xf6, 0xfc};
    sciformats::io::BinaryReader reader(bytes);
    size_t size = bytes.size();

    auto output = reader.readString("ISO-8859-1", size);
    // specifying UTF-8 string literals is error prone
    // see:
    // https://stackoverflow.com/questions/23471935/how-are-u8-literals-supposed-to-work?rq=1
    auto expected = std::string{u8"ABCabcÄÖÜäöü"};

    REQUIRE(output.size() == expected.size());
    for (auto i = 0; i < expected.size(); i++)
    {
        REQUIRE(output.at(i) == expected.at(i));
    }
}

// TODO: add tests for additional encodings
