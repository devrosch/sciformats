#include <iostream>
#include <string>
#include <sstream>
#include <vector>
#include <boost/locale.hpp>
#include "binaryreader/binary_reader.h"

using namespace std;

// see: https://stackoverflow.com/questions/8815164/c-wrapping-vectorchar-with-istream/8815308
//template<typename CharT, typename TraitsT = std::char_traits<CharT> >
//class vectorbuf : public std::basic_streambuf<CharT, TraitsT> {
//public:
//    vectorbuf(std::vector<CharT> &vec) {
//        this->setg(vec.data(), vec.data(), vec.data() + vec.size());
//    }
//};

int main()
{
    cout << "Hello World!" << endl;

    const string path = "/home/rob/Desktop/test.txt";
    ifstream file (path, ios::in | ios::binary);
    file.exceptions(std::ios::eofbit | std::ios::failbit | std::ios::badbit);
    sciformats::common::binary_reader reader(file, sciformats::common::binary_reader::little_endian);
    sciformats::common::binary_reader reader2(path, sciformats::common::binary_reader::little_endian);

    cout << reader.read_int64() << endl;
    cout << reader.read_uint64() << endl;
    cout << reader.read_int32() << endl;
    cout << reader.read_uint32() << endl;
    cout << reader.read_int16() << endl;
    cout << reader.read_uint16() << endl;
    cout << static_cast<int>(reader.read_int8()) << endl;
    cout << static_cast<int>(reader.read_uint8()) << endl;
//    cout << reader.read_double() << endl;
//    cout << reader.read_float() << endl;

    // https://stackoverflow.com/questions/19952174/passing-data-from-byte-arraywith-zeros-to-istringstreamstringstream
    // https://stackoverflow.com/questions/7781898/get-an-istream-from-a-char
    //char bytes[] = { static_cast<char>(0xFF), 0x01, 0x02 };
    //unsigned char bytes[] = { 0xFF, 0x01, 0x02 };
    uint8_t bytes[] = { 0xFF, 0x01, 0x02 };
    std::istringstream ss(std::string(bytes, bytes + sizeof(bytes)));
    sciformats::common::binary_reader reader3(ss, sciformats::common::binary_reader::little_endian);
    uint8_t a = reader3.read_uint8();
    auto b = reader3.read_uint8();
    auto c = reader3.read_uint8();

//    vector<char> data_vec { 0x00, 0x01 };
//    vector<char>& vec_ref = data_vec;
//    vectorbuf<char> vb(vec_ref);
//    std::istream is(&vb);

//    std::vector<uint8_t> data_vec {1, 2, 3};
//    vectorbuf<uint8_t> vb(data_vec);
//    std::istream<uint8_t> is(&vb);

    std::vector<char> vec = { static_cast<char>(0xe4) };
    std::string latin1_string(vec.begin(), vec.end());
    cout << "Latin1 string" << " (length: " << latin1_string.length() << "): " << latin1_string << endl;
    std::string utf8_string = boost::locale::conv::to_utf<char>(latin1_string, "Latin1");
    cout << "utf8 string" << " (length: " << utf8_string.length() << "): " << utf8_string << endl;

    std::vector<char> vecUtf16be = { 0x00, static_cast<char>(0xe4), 0x00, 0x62 };
    std::string utf16be_string(vecUtf16be.begin(), vecUtf16be.end());
    cout << "utf16BE string" << " (length: " << utf16be_string.length() << "): " << utf16be_string << endl;
    std::string utf8_string2 = boost::locale::conv::to_utf<char>(utf16be_string, "UTF-16BE");
    cout << "utf8 string 2" << " (length: " << utf8_string2.length() << "): " << utf8_string2 << endl;

    return 0;
}
