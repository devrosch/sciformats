#ifndef LIBIO_BINARYREADER_HPP
#define LIBIO_BINARYREADER_HPP

#include "binaryreader/Endianness.hpp"

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

    std::ios::pos_type tellg() const;
    void seekg(std::ios::pos_type, std::ios_base::seekdir = std::ios_base::beg);
    std::ios::pos_type getLength();

    int8_t readInt8();
    uint8_t readUInt8();
    int16_t readInt16();
    int16_t readInt16(Endianness endian);
    uint16_t readUInt16();
    uint16_t readUInt16(Endianness endian);
    int32_t readInt32();
    int32_t readInt32(Endianness endian);
    uint32_t readUInt32();
    uint32_t readUInt32(Endianness endian);
    int64_t readInt64();
    int64_t readInt64(Endianness endian);
    uint64_t readUInt64();
    uint64_t readUInt64(Endianness endian);
    float readFloat();
    float readFloat(Endianness endian);
    double readDouble();
    double readDouble(Endianness endian);
    std::vector<char> readChars(size_t size);
    std::vector<uint8_t> readBytes(size_t size);

private:
    std::optional<std::ifstream> m_ifstream;
    std::optional<std::istringstream> m_istringstream;
    std::istream& m_istream;
    Endianness m_endianness;
};
} // namespace sciformats::io

#endif // LIBIO_BINARYREADER_HPP
