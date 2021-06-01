#include "jdx/DataParser.hpp"

#include "catch2/catch.hpp"

#include <istream>
#include <sstream>

TEST_CASE("parses AFFN data line", "[DataParser]")
{
    std::string input{"1.23 4.5E23 4.5E2 7.89E-14 600"};

    auto [actual, difEncoded] = sciformats::jdx::DataParser::readValues(input);
    auto expect = std::vector<double>{1.23, 4.5E23, 4.5E2, 7.89E-14, 600};

    REQUIRE(false == difEncoded);
    REQUIRE(5 == actual.size());
    for (size_t i{0}; i < expect.size(); i++)
    {
        REQUIRE((expect.at(i) == Approx(actual.at(i))));
    }
}

TEST_CASE("parses FIX (I3) ASCII data line", "[DataParser]")
{
    std::string input{"1  2  3  3  2  1  0 -1 -2 -3"};

    auto [actual, difEncoded] = sciformats::jdx::DataParser::readValues(input);
    auto expect = std::vector<double>{1, 2, 3, 3, 2, 1, 0, -1, -2, -3};

    REQUIRE(false == difEncoded);
    REQUIRE(expect.size() == actual.size());
    for (size_t i{0}; i < expect.size(); i++)
    {
        REQUIRE((expect.at(i) == Approx(actual.at(i))));
    }
}

TEST_CASE("parses PAC data line", "[DataParser]")
{
    std::string input{"1+2+3+3+2+1+0-1-2-3"};

    auto [actual, difEncoded] = sciformats::jdx::DataParser::readValues(input);
    auto expect = std::vector<double>{1, 2, 3, 3, 2, 1, 0, -1, -2, -3};

    REQUIRE(false == difEncoded);
    REQUIRE(expect.size() == actual.size());
    for (size_t i{0}; i < expect.size(); i++)
    {
        REQUIRE((expect.at(i) == Approx(actual.at(i))));
    }
}

TEST_CASE("parses SQZ data line", "[DataParser]")
{
    std::string input{"1BCCBA@abc"};

    auto [actual, difEncoded] = sciformats::jdx::DataParser::readValues(input);
    auto expect = std::vector<double>{1, 2, 3, 3, 2, 1, 0, -1, -2, -3};

    REQUIRE(false == difEncoded);
    REQUIRE(expect.size() == actual.size());
    for (size_t i{0}; i < expect.size(); i++)
    {
        REQUIRE((expect.at(i) == Approx(actual.at(i))));
    }
}

TEST_CASE("parses DIF data line", "[DataParser]")
{
    std::string input{"1JJ%jjjjjj"};

    auto [actual, difEncoded] = sciformats::jdx::DataParser::readValues(input);
    auto expect = std::vector<double>{1, 2, 3, 3, 2, 1, 0, -1, -2, -3};

    REQUIRE(true == difEncoded);
    REQUIRE(expect.size() == actual.size());
    for (size_t i{0}; i < expect.size(); i++)
    {
        REQUIRE((expect.at(i) == Approx(actual.at(i))));
    }
}

TEST_CASE("fails if sequence starts with DIF token", "[DataParser]")
{
    std::string input{"jjj"};

    REQUIRE_THROWS(sciformats::jdx::DataParser::readValues(input));
}

TEST_CASE("parses DIFDUP data line", "[DataParser]")
{
    std::string input{"1JT%jX"};

    auto [actual, difEncoded] = sciformats::jdx::DataParser::readValues(input);
    auto expect = std::vector<double>{1, 2, 3, 3, 2, 1, 0, -1, -2, -3};

    REQUIRE(false == difEncoded); // last ordinate is in DUP format , not DIF
    REQUIRE(expect.size() == actual.size());
    for (size_t i{0}; i < expect.size(); i++)
    {
        REQUIRE((expect.at(i) == Approx(actual.at(i))));
    }
}

TEST_CASE(
    "fails if sequence contains two consecutive DUP tokens", "[DataParser]")
{
    std::string input{"1VZ"};

    REQUIRE_THROWS(sciformats::jdx::DataParser::readValues(input));
}

TEST_CASE("fails for illegal token start character", "[DataParser]")
{
    // "u" is an illegal character
    std::string input{"123 u45"};

    REQUIRE_THROWS(sciformats::jdx::DataParser::readValues(input));
}

TEST_CASE("parses mixed PAC/AFFN stream", "[DataParser]")
{
    std::string input{
        "599.860 0 0 0 0 2 4 4 4 7 5 4 4 5 5 7 10 11 11 6 5 7 6 9 9 7\r\n"
        "648.081 10 10 9 10 11 12 15 16 16 14 17 38 38 35 38 42 47 54\r\n"
        "682.799  59  66  75  78  88  96 104 110 121 128\r\n"
        "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    auto actual = sciformats::jdx::DataParser::readXppYYData(stream);
    auto expect = std::vector<double>{0, 0, 0, 0, 2, 4, 4, 4, 7, 5, 4, 4, 5, 5,
        7, 10, 11, 11, 6, 5, 7, 6, 9, 9, 7, 10, 10, 9, 10, 11, 12, 15, 16, 16,
        14, 17, 38, 38, 35, 38, 42, 47, 54, 59, 66, 75, 78, 88, 96, 104, 110,
        121, 128};
    std::string lastLine;
    getline(stream, lastLine);

    REQUIRE(expect.size() == actual.size());
    REQUIRE(std::string{"##END="} == lastLine);
    for (size_t i{0}; i < expect.size(); i++)
    {
        REQUIRE((expect.at(i) == Approx(actual.at(i))));
    }
}

TEST_CASE("detects failing Y check", "[DataParser]")
{
    // first line ends with y value 3, next line should duplicate it but is 4
    std::string input{"599.000+1jj\r\n"
                      "600.000+4jj\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    REQUIRE_THROWS(sciformats::jdx::DataParser::readXppYYData(stream));
}

TEST_CASE("parses DIFDUP stream", "[DataParser]")
{
    std::string input{
        "599.860@VKT%TLkj%J%KLJ%njKjL%kL%jJULJ%kLK1%lLMNPNPRLJ0QTOJ1P\r\n"
        "700.158A28\r\n"
        "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    auto actual = sciformats::jdx::DataParser::readXppYYData(stream);
    auto expect = std::vector<double>{0, 0, 0, 0, 2, 4, 4, 4, 7, 5, 4, 4, 5, 5,
        7, 10, 11, 11, 6, 5, 7, 6, 9, 9, 7, 10, 10, 9, 10, 11, 12, 15, 16, 16,
        14, 17, 38, 38, 35, 38, 42, 47, 54, 59, 66, 75, 78, 88, 96, 104, 110,
        121, 128};
    std::string lastLine;
    getline(stream, lastLine);

    REQUIRE(expect.size() == actual.size());
    REQUIRE(std::string{"##END="} == lastLine);
    for (size_t i{0}; i < expect.size(); i++)
    {
        REQUIRE((expect.at(i) == Approx(actual.at(i))));
    }
}
