#include "jdx/JdxDataParser.hpp"

#include "catch2/catch.hpp"

TEST_CASE("parses AFFN data", "[JdxDataParser]")
{
    // TODO: make this case pass as well
    //    std::string input{"1.23 4.5E2  600"};
    std::string input{"1.23 4.5E23 4.5E2 600"};

    auto output = sciformats::jdx::JdxDataParser::readValues(input);

    REQUIRE(4 == output.size());
    REQUIRE(1.23 == Approx(output.at(0)));
    REQUIRE(4.5E23 == Approx(output.at(1)));
    REQUIRE(4.5E2 == Approx(output.at(2)));
    REQUIRE(600 == Approx(output.at(3)));
}

TEST_CASE("parses FIX (I3) ASCII data", "[JdxDataParser]")
{
    std::string input{"1  2  3  3  2  1  0 -1 -2 -3"};

    auto actual = sciformats::jdx::JdxDataParser::readValues(input);
    auto expect = std::vector<double>{1, 2, 3, 3, 2, 1, 0, -1, -2, -3};

    REQUIRE(expect.size() == actual.size());
    for (size_t i{0}; i < expect.size(); i++)
    {
        REQUIRE((expect.at(i) == Approx(actual.at(i))));
    }
}

TEST_CASE("parses PAC data", "[JdxDataParser]")
{
    std::string input{"1+2+3+3+2+1+0-1-2-3"};

    auto actual = sciformats::jdx::JdxDataParser::readValues(input);
    auto expect = std::vector<double>{1, 2, 3, 3, 2, 1, 0, -1, -2, -3};

    REQUIRE(expect.size() == actual.size());
    for (size_t i{0}; i < expect.size(); i++)
    {
        REQUIRE((expect.at(i) == Approx(actual.at(i))));
    }
}

TEST_CASE("parses SQZ data", "[JdxDataParser]")
{
    std::string input{"1BCCBA@abc"};

    auto actual = sciformats::jdx::JdxDataParser::readValues(input);
    auto expect = std::vector<double>{1, 2, 3, 3, 2, 1, 0, -1, -2, -3};

    REQUIRE(expect.size() == actual.size());
    for (size_t i{0}; i < expect.size(); i++)
    {
        REQUIRE((expect.at(i) == Approx(actual.at(i))));
    }
}

TEST_CASE("parses DIF data", "[JdxDataParser]")
{
    std::string input{"1JJ%jjjjjj"};

    auto actual = sciformats::jdx::JdxDataParser::readValues(input);
    auto expect = std::vector<double>{1, 2, 3, 3, 2, 1, 0, -1, -2, -3};

    REQUIRE(expect.size() == actual.size());
    for (size_t i{0}; i < expect.size(); i++)
    {
        REQUIRE((expect.at(i) == Approx(actual.at(i))));
    }
}

TEST_CASE("parses DIFDUP data", "[JdxDataParser]")
{
    std::string input{"1JT%jX"};

    auto actual = sciformats::jdx::JdxDataParser::readValues(input);
    auto expect = std::vector<double>{1, 2, 3, 3, 2, 1, 0, -1, -2, -3};

    REQUIRE(expect.size() == actual.size());
    for (size_t i{0}; i < expect.size(); i++)
    {
        REQUIRE((expect.at(i) == Approx(actual.at(i))));
    }
}
