#ifndef LIBIO_BINARYREADER_HPP
#define LIBIO_BINARYREADER_HPP

#include "binaryreader/Endianness.hpp"
#include "binaryreader/StringPrefixConfig.hpp"

#include <cstdint>
#include <fstream>
#include <istream>
#include <optional>
#include <sstream>
#include <vector>

namespace sciformats::io
{
/**
 * @brief The BinaryReader class provides mechanisms to read binary data from
 * various input sources.
 */
class BinaryReader
{
public:
    /**
     * @brief sciformats::io::binaryreader::BinaryReader Constructs from file.
     * @param filePath Path to the file.
     * @param endian Default endianness of data.
     */
    explicit BinaryReader(const std::string& filePath,
        Endianness endian = Endianness::LittleEndian);

    /**
     * @brief sciformats::io::binaryreader::BinaryReader Constructs from
     * istream. Does not change exceptions flags.
     * @param inputStream Input stream with binary data.
     * @param endian Default endianness of data.
     * @param activateExceptions Activate exceptions for input_stream.
     */
    explicit BinaryReader(std::istream& inputStream,
        Endianness endian = Endianness::LittleEndian,
        bool activateExceptions = true);

    /**
     * @brief sciformats::io::binaryreader::BinaryReader Constructs from vector.
     * @param vec Vector with binary data.
     * @param endian Default endianness of data.
     */
    explicit BinaryReader(
        std::vector<char>& vec, Endianness endian = Endianness::LittleEndian);

    /**
     * @brief sciformats::io::binaryreader::BinaryReader Constructs from vector.
     * @param vec Vector with binary data.
     * @param endian Default endianness of data.
     */
    explicit BinaryReader(std::vector<uint8_t>& vec,
        Endianness endian = Endianness::LittleEndian);

    /**
     * @brief tellg Get the current read position in the data.
     * @return The current read position.
     */
    std::ios::pos_type tellg() const;

    /**
     * @brief seekg Set the read position in the data.
     */
    void seekg(std::ios::pos_type, std::ios_base::seekdir = std::ios_base::beg);

    /**
     * @brief getLength The length (in chars) of the input data.
     * @return Total length (in chars) of the input data.
     */
    std::ios::pos_type getLength();

    /**
     * @brief readInt8 Read byte from current position. Move position by one
     * byte forward.
     * @return Byte value interpreted as int8_t.
     */
    int8_t readInt8();

    /**
     * @brief readUInt8 Read byte from current position. Move position by one
     * byte forward.
     * @return Byte value interpreted as uint8_t.
     */
    uint8_t readUInt8();

    /**
     * @brief readInt16 Read two bytes from current position. Move position by
     * two bytes forward.
     * @return Byte values interpreted as int16_t with default endianness.
     */
    int16_t readInt16();

    /**
     * @brief readInt16 Read two bytes from current position. Move position by
     * two bytes forward.
     * @param endian Endianness of the value in the input source.
     * @return Byte values interpreted as int16_t with specified endianness.
     */
    int16_t readInt16(Endianness endian);

    /**
     * @brief readUInt16 Read two bytes from current position. Move position by
     * two bytes forward.
     * @return Byte values interpreted as uint16_t with default endianness.
     */
    uint16_t readUInt16();

    /**
     * @brief readUInt16 Read two bytes from current position. Move position by
     * two bytes forward.
     * @param endian Endianness of the value in the input source.
     * @return Byte values interpreted as uint16_t with specified endianness.
     */
    uint16_t readUInt16(Endianness endian);

    /**
     * @brief readInt32 Read four bytes from current position. Move position by
     * four bytes forward.
     * @return Byte values interpreted as int32_t with default endianness.
     */
    int32_t readInt32();

    /**
     * @brief readInt32 Read four bytes from current position. Move position by
     * four bytes forward.
     * @param endian Endianness of the value in the input source.
     * @return Byte values interpreted as int32_t with specified endianness.
     */
    int32_t readInt32(Endianness endian);

    /**
     * @brief readUInt32 Read four bytes from current position. Move position by
     * four bytes forward.
     * @return Byte values interpreted as uint32_t with default endianness.
     */
    uint32_t readUInt32();

    /**
     * @brief readUInt32 Read four bytes from current position. Move position by
     * four bytes forward.
     * @param endian Endianness of the value in the input source.
     * @return Byte values interpreted as uint32_t with specified endianness.
     */
    uint32_t readUInt32(Endianness endian);

    /**
     * @brief readInt64 Read eight bytes from current position. Move position by
     * eight bytes forward.
     * @return Byte values interpreted as int64_t with default endianness.
     */
    int64_t readInt64();

    /**
     * @brief readInt64 Read eight bytes from current position. Move position by
     * eight bytes forward.
     * @param endian Endianness of the value in the input source.
     * @return Byte values interpreted as int64_t with specified endianness.
     */
    int64_t readInt64(Endianness endian);

    /**
     * @brief readUInt64 Read eight bytes from current position. Move position
     * by eight bytes forward.
     * @return Byte values interpreted as uint64_t with default endianness.
     */
    uint64_t readUInt64();

    /**
     * @brief readUInt64 Read eight bytes from current position. Move position
     * by eight bytes forward.
     * @param endian Endianness of the value in the input source.
     * @return Byte values interpreted as uint64_t with specified endianness.
     */
    uint64_t readUInt64(Endianness endian);

    /**
     * @brief readFloat Read four bytes from current position. Move position by
     * four bytes forward.
     * @return Byte values interpreted as 32 bit float with default endianness.
     */
    float readFloat();

    /**
     * @brief readFloat Read four bytes from current position. Move position by
     * four bytes forward.
     * @param endian Endianness of the value in the input source.
     * @return Byte values interpreted as 32 bit float with specified
     * endianness.
     */
    float readFloat(Endianness endian);

    /**
     * @brief readDouble Read eight bytes from current position. Move position
     * by eight bytes forward.
     * @return Byte values interpreted as 64 bit double with default endianness.
     */
    double readDouble();

    /**
     * @brief readDouble Read eight bytes from current position. Move position
     * by eight bytes forward.
     * @param endian Endianness of the value in the input source.
     * @return Byte values interpreted as 64 bit double with default endianness.
     */
    double readDouble(Endianness endian);

    /**
     * @brief readChars Read a sequence of characters. Move position by number
     * of characters forward.
     * @param size Number of characters to read.
     * @return The characters read.
     */
    std::vector<char> readChars(size_t size);

    /**
     * @brief readBytes Read a sequence of bytes. Move position by number of
     * bytes forward.
     * @param size Number of bytes to read.
     * @return The bytes read.
     */
    std::vector<uint8_t> readBytes(size_t size);

    std::string readString(const std::string& encoding, int32_t maxSize);
    std::string readPrefixedString(const std::string& encoding, int32_t maxSize,
        StringPrefixConfig prefixConfig);

private:
    std::optional<std::ifstream> m_ifstream;
    std::optional<std::istringstream> m_istringstream;
    std::istream& m_istream;
    Endianness m_endianness;
};
} // namespace sciformats::io

#endif // LIBIO_BINARYREADER_HPP
