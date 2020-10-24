#define CATCH_CONFIG_MAIN
#include <binaryreader/binary_reader.hpp>
#include <catch2/catch.hpp>
#include <climits>

static_assert(CHAR_BIT == 8, "Char size is not 8.");

// see: https://stackoverflow.com/a/47934240
constexpr auto operator"" _c(unsigned long long arg) noexcept
{
    return static_cast<char>(arg);
}

TEST_CASE("correctly reads file", "[binary_reader]")
{
    const auto path = "resources/test.bin";
    sciformats::common::binary_reader reader(path);

    REQUIRE(reader.tellg() == 0);
    REQUIRE(reader.get_length() == 8);
    REQUIRE(reader.read_uint8() == 0x01);
    REQUIRE(reader.read_uint8() == 0x02);
    REQUIRE(reader.read_uint8() == 0x03);
    REQUIRE(reader.read_uint8() == 0x04);
    REQUIRE(reader.read_uint8() == 0xFF);
    REQUIRE(reader.read_uint8() == 0xFE);
    REQUIRE(reader.read_uint8() == 0xFD);
    REQUIRE(reader.read_uint8() == 0xFC);
    reader.seekg(1);
    REQUIRE(reader.tellg() == 1);
    REQUIRE(reader.read_uint8() == 0x02);
}

TEST_CASE("throws exception when reading past end and constructed from file",
    "[binary_reader]")
{
    const auto path = "resources/test.bin";
    sciformats::common::binary_reader reader(path);

    reader.seekg(reader.get_length());
    REQUIRE_THROWS(reader.read_uint8());
}

TEST_CASE("correctly reads istream", "[binary_reader]")
{
    unsigned char bytes[]{0x00, 0xFF, 0x7F}; // NOLINT
    std::istringstream ss(std::string(bytes, bytes + sizeof(bytes)));
    sciformats::common::binary_reader reader(
        ss, sciformats::common::binary_reader::big_endian);

    REQUIRE(reader.tellg() == 0);
    REQUIRE(reader.get_length() == 3);
    REQUIRE(reader.read_uint8() == 0x00);
    REQUIRE(reader.read_uint8() == 0xFF);
    REQUIRE(reader.read_uint8() == 0x7F);
    REQUIRE(reader.tellg() == sizeof(bytes));
    reader.seekg(1);
    REQUIRE(reader.tellg() == 1);
    REQUIRE(reader.read_uint8() == 0xFF);
}

TEST_CASE("throws exception when reading past and constructed from istream "
          "if argument istream is configured to throw exceptions "
          "or constructor argument for activatin exceptions is true",
    "[binary_reader]")
{
    unsigned char bytes[] = {0x00, 0xFF, 0x7F}; // NOLINT
    std::istringstream ss_nothrow(std::string(bytes, bytes + sizeof(bytes)));
    sciformats::common::binary_reader reader_nothrow(
        ss_nothrow, sciformats::common::binary_reader::big_endian, false);

    // unlike for other reader constructors, for constructor the behavior of the
    // argument stream determines if it throws
    reader_nothrow.seekg(3);
    REQUIRE_NOTHROW(reader_nothrow.read_uint8());

    std::istringstream ss_nothrow2(std::string(bytes, bytes + sizeof(bytes)));
    sciformats::common::binary_reader reader_nothrow2(
        ss_nothrow2, sciformats::common::binary_reader::big_endian, true);

    reader_nothrow2.seekg(3);
    REQUIRE_THROWS(reader_nothrow2.read_uint8());

    std::istringstream ss_throw(std::string(bytes, bytes + sizeof(bytes)));
    ss_throw.exceptions(
        std::ios::eofbit | std::ios::failbit | std::ios::badbit);
    sciformats::common::binary_reader reader_throw(
        ss_throw, sciformats::common::binary_reader::big_endian, false);

    reader_throw.seekg(3);
    REQUIRE_THROWS(reader_throw.read_uint8());
}

TEST_CASE("correctly reads char vector", "[binary_reader]")
{
    std::vector<char> bytes{0x00_c, 0xFF_c, 0x7F_c}; // NOLINT
    sciformats::common::binary_reader reader(bytes);

    REQUIRE(reader.tellg() == 0);
    REQUIRE(reader.get_length() == 3);
    REQUIRE(reader.read_uint8() == 0x00);
    REQUIRE(reader.read_uint8() == 0xFF);
    REQUIRE(reader.read_uint8() == 0x7F);
    REQUIRE(reader.tellg() == bytes.size());
    reader.seekg(1);
    REQUIRE(reader.tellg() == 1);
    REQUIRE(reader.read_uint8() == 0xFF);
}

TEST_CASE(
    "throws exception when reading past end and constructed from char vector",
    "[binary_reader]")
{
    std::vector<char> bytes{0x00_c, 0xFF_c, 0x7F_c}; // NOLINT
    sciformats::common::binary_reader reader(bytes);

    reader.seekg(3);
    REQUIRE_THROWS(reader.read_uint8());
}

TEST_CASE("correctly reads uint8_t vector", "[binary_reader]")
{
    std::vector<uint8_t> bytes{0x00, 0xFF, 0x7F}; // NOLINT
    sciformats::common::binary_reader reader(bytes);

    REQUIRE(reader.tellg() == 0);
    REQUIRE(reader.get_length() == 3);
    REQUIRE(reader.read_uint8() == 0x00);
    REQUIRE(reader.read_uint8() == 0xFF);
    REQUIRE(reader.read_uint8() == 0x7F);
    REQUIRE(reader.tellg() == bytes.size());
    reader.seekg(1);
    REQUIRE(reader.tellg() == 1);
    REQUIRE(reader.read_uint8() == 0xFF);
    reader.seekg(3);
    REQUIRE_THROWS(reader.read_uint8());
}

TEST_CASE("throws exception when reading past end and constructed from uint8_t "
          "vector",
    "[binary_reader]")
{
    std::vector<uint8_t> bytes{0x00, 0xFF, 0x7F}; // NOLINT
    sciformats::common::binary_reader reader(bytes);

    reader.seekg(3);
    REQUIRE_THROWS(reader.read_uint8());
}

TEST_CASE("read int8 correctly", "[binary_reader]")
{
    std::vector<uint8_t> bytes{0xFF}; // NOLINT
    sciformats::common::binary_reader reader(bytes);

    REQUIRE(reader.read_int8() == -1);
}

TEST_CASE("read uint8 correctly", "[binary_reader]")
{
    std::vector<uint8_t> bytes{0xFF}; // NOLINT
    sciformats::common::binary_reader reader(bytes);

    REQUIRE(reader.read_uint8() == 255);
}

TEST_CASE("read int16 correctly", "[binary_reader]")
{
    auto expected = -256; // NOLINT

    // little endian
    std::vector<uint8_t> bytes{0x00, 0xFF}; // NOLINT
    sciformats::common::binary_reader reader_le(
        bytes, sciformats::common::binary_reader::little_endian);

    REQUIRE(reader_le.read_int16() == expected);
    reader_le.seekg(0, std::ios::beg);
    REQUIRE(
        reader_le.read_int16(sciformats::common::binary_reader::little_endian)
        == expected);

    // big endian
    std::reverse(bytes.begin(), bytes.end());
    sciformats::common::binary_reader reader_be(
        bytes, sciformats::common::binary_reader::big_endian);

    REQUIRE(reader_be.read_int16() == expected);
    reader_be.seekg(0, std::ios::beg);
    REQUIRE(reader_be.read_int16(sciformats::common::binary_reader::big_endian)
            == expected);
}

TEST_CASE("read uint16 correctly", "[binary_reader]")
{
    auto expected = 65280; // NOLINT

    // little endian
    std::vector<uint8_t> bytes{0x00, 0xFF}; // NOLINT
    sciformats::common::binary_reader reader_le(
        bytes, sciformats::common::binary_reader::little_endian);

    REQUIRE(reader_le.read_uint16() == expected);
    reader_le.seekg(0, std::ios::beg);
    REQUIRE(
        reader_le.read_uint16(sciformats::common::binary_reader::little_endian)
        == expected);

    // big endian
    std::reverse(bytes.begin(), bytes.end());
    sciformats::common::binary_reader reader_be(
        bytes, sciformats::common::binary_reader::big_endian);

    REQUIRE(reader_be.read_uint16() == expected);
    reader_be.seekg(0, std::ios::beg);
    REQUIRE(reader_be.read_uint16(sciformats::common::binary_reader::big_endian)
            == expected);
}

TEST_CASE("read int32 correctly", "[binary_reader]")
{
    auto expected = -16777216L; // NOLINT

    // little endian
    std::vector<uint8_t> bytes{0x00, 0x00, 0x00, 0xFF}; // NOLINT
    sciformats::common::binary_reader reader_le(
        bytes, sciformats::common::binary_reader::little_endian);

    REQUIRE(reader_le.read_int32() == expected);
    reader_le.seekg(0, std::ios::beg);
    REQUIRE(
        reader_le.read_int32(sciformats::common::binary_reader::little_endian)
        == expected);

    // big endian
    std::reverse(bytes.begin(), bytes.end());
    sciformats::common::binary_reader reader_be(
        bytes, sciformats::common::binary_reader::big_endian);

    REQUIRE(reader_be.read_int32() == expected);
    reader_be.seekg(0, std::ios::beg);
    REQUIRE(reader_be.read_int32(sciformats::common::binary_reader::big_endian)
            == expected);
}

TEST_CASE("read uint32 correctly", "[binary_reader]")
{
    auto expected = 4278190080UL; // NOLINT

    // little endian
    std::vector<uint8_t> bytes{0x00, 0x00, 0x00, 0xFF}; // NOLINT
    sciformats::common::binary_reader reader_le(
        bytes, sciformats::common::binary_reader::little_endian);

    REQUIRE(reader_le.read_uint32() == expected);
    reader_le.seekg(0, std::ios::beg);
    REQUIRE(
        reader_le.read_uint32(sciformats::common::binary_reader::little_endian)
        == expected);

    // big endian
    std::reverse(bytes.begin(), bytes.end());
    sciformats::common::binary_reader reader_be(
        bytes, sciformats::common::binary_reader::big_endian);

    REQUIRE(reader_be.read_uint32() == expected);
    reader_be.seekg(0, std::ios::beg);
    REQUIRE(reader_be.read_uint32(sciformats::common::binary_reader::big_endian)
            == expected);
}

TEST_CASE("read int64 correctly", "[binary_reader]")
{
    auto expected = -72057594037927936LL; // NOLINT

    // little endian
    std::vector<uint8_t> bytes{
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF}; // NOLINT
    sciformats::common::binary_reader reader_le(
        bytes, sciformats::common::binary_reader::little_endian);

    REQUIRE(reader_le.read_int64() == expected);
    reader_le.seekg(0, std::ios::beg);
    REQUIRE(
        reader_le.read_int64(sciformats::common::binary_reader::little_endian)
        == expected);

    // big endian
    std::reverse(bytes.begin(), bytes.end());
    sciformats::common::binary_reader reader_be(
        bytes, sciformats::common::binary_reader::big_endian);

    REQUIRE(reader_be.read_int64() == expected);
    reader_be.seekg(0, std::ios::beg);
    REQUIRE(reader_be.read_int64(sciformats::common::binary_reader::big_endian)
            == expected);
}

TEST_CASE("read uint64 correctly", "[binary_reader]")
{
    auto expected = 18374686479671623680ULL; // NOLINT

    // little endian
    std::vector<uint8_t> bytes{
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF}; // NOLINT
    sciformats::common::binary_reader reader_le(
        bytes, sciformats::common::binary_reader::little_endian);

    REQUIRE(reader_le.read_uint64() == expected);
    reader_le.seekg(0, std::ios::beg);
    REQUIRE(
        reader_le.read_uint64(sciformats::common::binary_reader::little_endian)
        == expected);

    // big endian
    std::reverse(bytes.begin(), bytes.end());
    sciformats::common::binary_reader reader_be(
        bytes, sciformats::common::binary_reader::big_endian);

    REQUIRE(reader_be.read_uint64() == expected);
    reader_be.seekg(0, std::ios::beg);
    REQUIRE(reader_be.read_uint64(sciformats::common::binary_reader::big_endian)
            == expected);
}

TEST_CASE("read float32 correctly", "[binary_reader]")
{
    auto expected = 2.5F; // NOLINT

    // little endian
    std::vector<uint8_t> bytes{0x00, 0x00, 0x20, 0x40}; // NOLINT
    sciformats::common::binary_reader reader_le(
        bytes, sciformats::common::binary_reader::little_endian);

    REQUIRE(reader_le.read_float() == Approx(expected));
    reader_le.seekg(0, std::ios::beg);
    REQUIRE(
        reader_le.read_float(sciformats::common::binary_reader::little_endian)
        == Approx(expected));

    // big endian
    std::reverse(bytes.begin(), bytes.end());
    sciformats::common::binary_reader reader_be(
        bytes, sciformats::common::binary_reader::big_endian);

    REQUIRE(reader_be.read_float() == Approx(expected));
    reader_be.seekg(0, std::ios::beg);
    REQUIRE(reader_be.read_float(sciformats::common::binary_reader::big_endian)
            == Approx(expected));
}

TEST_CASE("read float64 correctly", "[binary_reader]")
{
    auto expected = 2.5; // NOLINT

    // little endian
    std::vector<uint8_t> bytes{
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x40}; // NOLINT
    sciformats::common::binary_reader reader_le(
        bytes, sciformats::common::binary_reader::little_endian);

    REQUIRE(reader_le.read_double() == Approx(expected));
    reader_le.seekg(0, std::ios::beg);
    REQUIRE(
        reader_le.read_double(sciformats::common::binary_reader::little_endian)
        == Approx(expected));

    // big endian
    std::reverse(bytes.begin(), bytes.end());
    sciformats::common::binary_reader reader_be(
        bytes, sciformats::common::binary_reader::big_endian);

    REQUIRE(reader_be.read_double() == Approx(expected));
    reader_be.seekg(0, std::ios::beg);
    REQUIRE(reader_be.read_double(sciformats::common::binary_reader::big_endian)
            == Approx(expected));
}

TEST_CASE("read chars into vector correctly", "[binary_reader]")
{
    std::vector<char> bytes{0x00, 0x01, 0x02, 0xFF_c}; // NOLINT
    sciformats::common::binary_reader reader(bytes);
    size_t size = bytes.size();

    auto output = reader.read_chars(size);

    REQUIRE(output.size() == size);
    REQUIRE(output.at(0) == bytes[0]);
    REQUIRE(output.at(1) == bytes[1]);
    REQUIRE(output.at(2) == bytes[2]);
    REQUIRE(output.at(3) == bytes[3]);
}

TEST_CASE("read bytes into vector correctly", "[binary_reader]")
{
    std::vector<uint8_t> bytes{0x00, 0x01, 0x02, 0xFF}; // NOLINT
    sciformats::common::binary_reader reader(bytes);
    size_t size = bytes.size();

    auto output = reader.read_bytes(size);

    REQUIRE(output.size() == size);
    REQUIRE(output.at(0) == bytes[0]);
    REQUIRE(output.at(1) == bytes[1]);
    REQUIRE(output.at(2) == bytes[2]);
    REQUIRE(output.at(3) == bytes[3]);
}
