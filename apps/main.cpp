#include "binaryreader/binary_reader.hpp"
//#include <boost/locale.hpp>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

#include <unicode/ucnv.h>
#include <unicode/unistr.h>

using namespace std;

// see:
// https://stackoverflow.com/questions/8815164/c-wrapping-vectorchar-with-istream/8815308
// template<typename CharT, typename TraitsT = std::char_traits<CharT> >
// class vectorbuf : public std::basic_streambuf<CharT, TraitsT> {
// public:
//    vectorbuf(std::vector<CharT> &vec) {
//        this->setg(vec.data(), vec.data(), vec.data() + vec.size());
//    }
//};

int main()
{
    cout << "Hello World!" << endl;

    // see: https://unicode-org.github.io/icu/userguide/conversion/
    // see:
    // https://github.com/unicode-org/icu/blob/master/icu4c/source/samples/ucnv/convsamp.cpp
    // see:
    // https://stackoverflow.com/questions/6010793/looking-for-simple-practical-c-examples-of-how-to-use-icu
    icu::UnicodeString ucs
        = icu::UnicodeString::fromUTF8(icu::StringPiece("abc"));
    //    icu_62::UnicodeString ucs =
    //    icu_62::UnicodeString::fromUTF8(icu_62::StringPiece("abc"));
    //    icu_60::UnicodeString ucs =
    //    icu_60::UnicodeString::fromUTF8(icu_60::StringPiece("abc"));
    //    UnicodeString ucs = UnicodeString::fromUTF8(StringPiece("abc"));
    std::string converted;
    ucs.toUTF8String(converted);
    cout << converted << endl;

    //    std::vector<char> vec = {static_cast<char>(0xe4)}; // NOLINT
    std::vector<char> vec
        = {0x00, static_cast<char>(0xe4), 0x00, 0x62, 0x00, 0x00}; // NOLINT
    std::string latin1_string(vec.begin(), vec.end());
    UErrorCode status = U_ZERO_ERROR;
    //    UConverter* converter = ucnv_open("ISO-8859-1", &status);
    UConverter* converter = ucnv_open("UTF-16BE", &status);
    // ideomatic ICU
    // NOLINTNEXTLINE(readability-implicit-bool-conversion)
    if (U_SUCCESS(status))
    {
        int8_t maxCharSize = ucnv_getMaxCharSize(converter);
        cout << "ucnv_getMaxCharSize(): " << static_cast<int>(maxCharSize)
             << endl;
        int8_t minCharSize = ucnv_getMinCharSize(converter);
        cout << "ucnv_getMinCharSize(): " << static_cast<int>(minCharSize)
             << endl;

        std::vector<UChar> target{};
        target.resize(vec.size() / minCharSize + 1);
        cout << "target.size(): " << target.size() << endl;
        ucnv_toUChars(converter, target.data(), target.size(), vec.data(),
            vec.size(), &status);
        cout << "error status: " << u_errorName(status) << " (" << status << ")"
             << endl;
        // NOLINTNEXTLINE(readability-implicit-bool-conversion)
        if (U_FAILURE(status))
        {
            cout << "ERROR!" << endl;
        }
        ucnv_close(converter);
        icu::UnicodeString output{target.data()};
        std::string outputString{};
        output.toUTF8String(outputString);
        cout << "first UChar: " << outputString << endl;
    }

    //    const string path = "/home/rob/Desktop/test.txt";
    //    ifstream file(path, ios::in | ios::binary);
    //    file.exceptions(std::ios::eofbit | std::ios::failbit |
    //    std::ios::badbit); sciformats::common::binary_reader reader(
    //        file, sciformats::common::binary_reader::little_endian);
    //    sciformats::common::binary_reader reader2(
    //        path, sciformats::common::binary_reader::little_endian);

    //    cout << reader.read_int64() << endl;
    //    cout << reader.read_uint64() << endl;
    //    cout << reader.read_int32() << endl;
    //    cout << reader.read_uint32() << endl;
    //    cout << reader.read_int16() << endl;
    //    cout << reader.read_uint16() << endl;
    //    cout << static_cast<int>(reader.read_int8()) << endl;
    //    cout << static_cast<int>(reader.read_uint8()) << endl;
    //    cout << reader.read_double() << endl;
    //    cout << reader.read_float() << endl;

    // https://stackoverflow.com/questions/19952174/passing-data-from-byte-arraywith-zeros-to-istringstreamstringstream
    // https://stackoverflow.com/questions/7781898/get-an-istream-from-a-char
    // char bytes[] = { static_cast<char>(0xFF), 0x01, 0x02 };
    // unsigned char bytes[] = { 0xFF, 0x01, 0x02 };
    //    uint8_t bytes[] = {0xFF, 0x01, 0x02};
    //    std::istringstream ss(std::string(bytes, bytes + sizeof(bytes)));
    //    sciformats::common::binary_reader reader3(
    //        ss, sciformats::common::binary_reader::little_endian);
    //    uint8_t a = reader3.read_uint8();
    //    auto b = reader3.read_uint8();
    //    auto c = reader3.read_uint8();

    //    vector<char> data_vec { 0x00, 0x01 };
    //    vector<char>& vec_ref = data_vec;
    //    vectorbuf<char> vb(vec_ref);
    //    std::istream is(&vb);

    //    std::vector<uint8_t> data_vec {1, 2, 3};
    //    vectorbuf<uint8_t> vb(data_vec);
    //    std::istream<uint8_t> is(&vb);

    // boost:locale
    //    std::vector<char> vec = {static_cast<char>(0xe4)}; // NOLINT
    //    std::string latin1_string(vec.begin(), vec.end());
    //    cout << "Latin1 string"
    //         << " (length: " << latin1_string.length() << "): " <<
    //         latin1_string
    //         << endl;
    //    // poosible names for boost locale ICU backend:
    //    // https://icu4c-demos.unicode.org/icu-bin/convexp for lconv backend:
    //    // https://gist.github.com/hakre/4188459 ICU:
    //    //
    //    https://android.developreference.com/article/12744083/Unicode+character+classification+with+boost%3A%3Alocale
    //    //    std::string utf8_string =
    //    //    boost::locale::conv::to_utf<char>(latin1_string, "Latin1");
    //    std::string utf8_string = boost::locale::conv::to_utf<char>(
    //        latin1_string, "ibm-5348_P100-1997");
    //    cout << "utf8 string"
    //         << " (length: " << utf8_string.length() << "): " << utf8_string
    //         << endl;

    //    std::vector<char> vecUtf16be
    //        = {0x00, static_cast<char>(0xe4), 0x00, 0x62}; // NOLINT
    //    std::string utf16be_string(vecUtf16be.begin(), vecUtf16be.end());
    //    cout << "utf16BE string"
    //         << " (length: " << utf16be_string.length() << "): " <<
    //         utf16be_string
    //         << endl;
    //    //    std::string utf8_string2 =
    //    //    boost::locale::conv::to_utf<char>(utf16be_string, "UTF-16BE");
    //    //    std::string utf8_string2 =
    //    //    boost::locale::conv::to_utf<char>(utf16be_string, "cp1201");
    //    std::string utf8_string2
    //        = boost::locale::conv::to_utf<char>(utf16be_string, "UTF-16BE");
    //    cout << "utf8 string 2"
    //         << " (length: " << utf8_string2.length() << "): " << utf8_string2
    //         << endl;

    //    std::vector<char> vecUtf16le = {
    //        0x61, 0x00, 0x62, 0x00, 0x00, 0x00, 0x00, 0x00, 0x63, 0x00}; //
    //        NOLINT
    //    std::string utf16le_string(vecUtf16le.begin(), vecUtf16le.end());
    //    std::string utf8_string3
    //        = boost::locale::conv::to_utf<char>(utf16le_string, "UTF-16LE");
    //    cout << "utf8 string 3"
    //         << " (length: " << utf8_string3.length() << "): " << utf8_string3
    //         << endl;

    return 0;
}
