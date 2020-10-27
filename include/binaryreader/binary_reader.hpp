#ifndef LIBIO_BINARY_READER_HPP
#define LIBIO_BINARY_READER_HPP

#include <fstream>
#include <istream>
#include <optional>
#include <sstream>
#include <vector>

namespace sciformats::common
{
/**
 * @brief The binary_reader class provides mechanisms to read binary data from
 * various input sources.
 */
class binary_reader
{
public:
    /**
     * @brief The endianness enum indicates if data is expected to be little
     * endian or big endian.
     */
    enum endianness : uint8_t
    {
        little_endian,
        big_endian,
    };

    /**
     * @brief sciformats::common::binary_reader::binary_reader Constructs from
     * file.
     * @param file_path Path to the file.
     * @param endian Default endianness of data.
     */
    explicit binary_reader(
        const std::string& file_path, endianness endian = little_endian);
    /**
     * @brief sciformats::common::binary_reader::binary_reader Constructs from
     * istream. Does not change exceptions flags.
     * @param input_stream Input stream with binary data.
     * @param endian Default endianness of data.
     * @param activateExceptions Activate exceptions for input_stream.
     */
    explicit binary_reader(std::istream& input_stream,
        endianness endian = little_endian,
        bool activateExceptions = true);
    /**
     * @brief sciformats::common::binary_reader::binary_reader Constructs from
     * vector.
     * @param vec Vector with binary data.
     * @param endian Default endianness of data.
     */
    explicit binary_reader(
        std::vector<char>& vec, endianness endian = little_endian);
    /**
     * @brief sciformats::common::binary_reader::binary_reader Constructs from
     * vector.
     * @param vec Vector with binary data.
     * @param endian Default endianness of data.
     */
    explicit binary_reader(
        std::vector<uint8_t>& vec, endianness endian = little_endian);

    std::ios::pos_type tellg() const;
    void seekg(std::ios::pos_type,
        std::ios_base::seekdir = std::ios_base::beg);
    std::ios::pos_type get_length();

    int8_t read_int8();
    uint8_t read_uint8();
    int16_t read_int16();
    int16_t read_int16(endianness endian);
    uint16_t read_uint16();
    uint16_t read_uint16(endianness endian);
    int32_t read_int32();
    int32_t read_int32(endianness endian);
    uint32_t read_uint32();
    uint32_t read_uint32(endianness endian);
    int64_t read_int64();
    int64_t read_int64(endianness endian);
    uint64_t read_uint64();
    uint64_t read_uint64(endianness endian);
    float read_float();
    float read_float(endianness endian);
    double read_double();
    double read_double(endianness endian);
    std::vector<char> read_chars(size_t size);
    std::vector<uint8_t> read_bytes(size_t size);

private:
    std::optional<std::ifstream> _file_stream;
    std::optional<std::istringstream> _istringstream;
    std::istream& _input_stream;
    endianness _endianness;
};
} // namespace sciformats::common

#endif // LIBIO_BINARY_READER_HPP
