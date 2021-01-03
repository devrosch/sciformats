#include "jdx/JdxXyData.hpp"

#include "catch2/catch.hpp"

#include <sstream>

TEST_CASE("parses AFFN xy data, stream at LDR start", "[JdxXyData]")
{
    std::string input{"##XYDATA= (XY..XY)\r\n"
                      "450.0, 10.0\r\n"
                      "451.0, 11.0\r\n"
                      "452.0, 12.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    auto xyDataRecord
        = sciformats::jdx::JdxXyData(stream, 450.0, 452.0, 1.0, 1.0, 3);
    auto xyData = xyDataRecord.getXyData();

    REQUIRE(3 == xyData.size());
    REQUIRE(450.0 == Approx(xyData.at(0).first));
    REQUIRE(10.0 == Approx(xyData.at(0).second));
    REQUIRE(451.0 == Approx(xyData.at(1).first));
    REQUIRE(11.0 == Approx(xyData.at(1).second));
    REQUIRE(452.0 == Approx(xyData.at(2).first));
    REQUIRE(12.0 == Approx(xyData.at(2).second));
}

TEST_CASE("parses AFFN xy data, stream at 2nd line start", "[JdxXyData]")
{
    std::string input{"450.0, 10.0\r\n"
                      "451.0, 11.0\r\n"
                      "452.0, 12.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    auto xyDataRecord = sciformats::jdx::JdxXyData(
        "XYDATA", "(XY..XY)", stream, 450.0, 452.0, 1.0, 1.0, 3);
    auto xyData = xyDataRecord.getXyData();

    REQUIRE(3 == xyData.size());
    REQUIRE(450.0 == Approx(xyData.at(0).first));
    REQUIRE(10.0 == Approx(xyData.at(0).second));
    REQUIRE(451.0 == Approx(xyData.at(1).first));
    REQUIRE(11.0 == Approx(xyData.at(1).second));
    REQUIRE(452.0 == Approx(xyData.at(2).first));
    REQUIRE(12.0 == Approx(xyData.at(2).second));
}

TEST_CASE("parses single data point record", "[JdxXyData]")
{
    std::string input{"##XYDATA= (XY..XY)\r\n"
                      "450.0, 10.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    auto xyDataRecord
        = sciformats::jdx::JdxXyData(stream, 450.0, 450.0, 1.0, 1.0, 1);
    auto xyData = xyDataRecord.getXyData();

    REQUIRE(1 == xyData.size());
    REQUIRE(450.0 == Approx(xyData.at(0).first));
    REQUIRE(10.0 == Approx(xyData.at(0).second));
}

TEST_CASE("detects mismatching NPOINTS", "[JdxXyData]")
{
    std::string input{"##XYDATA= (XY..XY)\r\n"
                      "450.0, 10.0\r\n"
                      "451.0, 11.0\r\n"
                      "452.0, 12.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    auto xyDataRecord
        = sciformats::jdx::JdxXyData(stream, 450.0, 452.0, 1.0, 1.0, 2);
    REQUIRE_THROWS(xyDataRecord.getXyData());
}

TEST_CASE("detects mismatching variables list for XYDATA", "[JdxXyData]")
{
    std::string input{"##XYDATA= (RA..RA)\r\n"
                      "450.0, 10.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    REQUIRE_THROWS(
        sciformats::jdx::JdxXyData(stream, 450.0, 450.0, 1.0, 1.0, 1));
}

TEST_CASE("detects mismatching variables list for RADATA", "[JdxXyData]")
{
    std::string input{"##RADATA= (XY..XY)\r\n"
                      "450.0, 10.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    REQUIRE_THROWS(
        sciformats::jdx::JdxXyData(stream, 450.0, 450.0, 1.0, 1.0, 1));
}

TEST_CASE("detects illegal stream position (wrong label)", "[JdxXyData]")
{
    std::string input{"##NPOINTS= 1\r\n"
                      "##XYDATA= (XY..XY)\r\n"
                      "450.0, 10.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    REQUIRE_THROWS(
        sciformats::jdx::JdxXyData(stream, 450.0, 450.0, 1.0, 1.0, 1));
}

TEST_CASE("detects illegal stream position (not LDR start)", "[JdxXyData]")
{
    std::string input{"450.0, 10.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    REQUIRE_THROWS(
        sciformats::jdx::JdxXyData(stream, 450.0, 450.0, 1.0, 1.0, 1));
}
