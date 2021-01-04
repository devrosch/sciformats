#include "jdx/XyData.hpp"
#include "jdx/XyParameters.hpp"

#include "catch2/catch.hpp"

#include <sstream>

TEST_CASE("parses AFFN xy data, stream at LDR start", "[XyData]")
{
    std::string input{"##XYDATA= (XY..XY)\r\n"
                      "450.0, 10.0\r\n"
                      "451.0, 11.0\r\n"
                      "452.0, 12.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    sciformats::jdx::XyParameters params = {"", "", 450.0, 452.0, 452.0, 450.0,
        12.0, 10.0, 1.0, 1.0, 3, 10.0, 1.0, 1.0};
    auto xyDataRecord = sciformats::jdx::XyData(stream, params);
    auto xyData = xyDataRecord.getData();

    REQUIRE(3 == xyData.size());
    REQUIRE(450.0 == Approx(xyData.at(0).first));
    REQUIRE(10.0 == Approx(xyData.at(0).second));
    REQUIRE(451.0 == Approx(xyData.at(1).first));
    REQUIRE(11.0 == Approx(xyData.at(1).second));
    REQUIRE(452.0 == Approx(xyData.at(2).first));
    REQUIRE(12.0 == Approx(xyData.at(2).second));
}

TEST_CASE("parses AFFN xy data, stream at 2nd line start", "[XyData]")
{
    std::string input{"450.0, 10.0\r\n"
                      "451.0, 11.0\r\n"
                      "452.0, 12.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    sciformats::jdx::XyParameters params = {"", "", 450.0, 452.0, 452.0, 450.0,
        12.0, 10.0, 1.0, 1.0, 3, 10.0, 1.0, 1.0};
    auto xyDataRecord
        = sciformats::jdx::XyData("XYDATA", "(XY..XY)", stream, params);
    auto xyData = xyDataRecord.getData();

    REQUIRE(3 == xyData.size());
    REQUIRE(450.0 == Approx(xyData.at(0).first));
    REQUIRE(10.0 == Approx(xyData.at(0).second));
    REQUIRE(451.0 == Approx(xyData.at(1).first));
    REQUIRE(11.0 == Approx(xyData.at(1).second));
    REQUIRE(452.0 == Approx(xyData.at(2).first));
    REQUIRE(12.0 == Approx(xyData.at(2).second));
}

TEST_CASE("parses single data point record", "[XyData]")
{
    std::string input{"##XYDATA= (XY..XY)\r\n"
                      "450.0, 10.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    sciformats::jdx::XyParameters params = {"", "", 450.0, 450.0, 450.0, 450.0,
        10.0, 10.0, 1.0, 1.0, 1, 10.0, 1.0, 1.0};
    auto xyDataRecord = sciformats::jdx::XyData(stream, params);
    auto xyData = xyDataRecord.getData();

    REQUIRE(1 == xyData.size());
    REQUIRE(450.0 == Approx(xyData.at(0).first));
    REQUIRE(10.0 == Approx(xyData.at(0).second));
}

TEST_CASE("detects mismatching NPOINTS", "[XyData]")
{
    std::string input{"##XYDATA= (XY..XY)\r\n"
                      "450.0, 10.0\r\n"
                      "451.0, 11.0\r\n"
                      "452.0, 12.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    sciformats::jdx::XyParameters params = {"", "", 450.0, 452.0, 452.0, 450.0,
        12.0, 10.0, 1.0, 1.0, 1, 10.0, 1.0, 1.0};
    auto xyDataRecord = sciformats::jdx::XyData(stream, params);
    REQUIRE_THROWS(xyDataRecord.getData());
}

TEST_CASE("detects mismatching variables list for XYDATA", "[XyData]")
{
    std::string input{"##XYDATA= (RA..RA)\r\n"
                      "450.0, 10.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    sciformats::jdx::XyParameters params = {"", "", 450.0, 450.0, 450.0, 450.0,
        10.0, 10.0, 1.0, 1.0, 1, 10.0, 1.0, 1.0};
    REQUIRE_THROWS(sciformats::jdx::XyData(stream, params));
}

TEST_CASE("detects mismatching variables list for RADATA", "[XyData]")
{
    std::string input{"##RADATA= (XY..XY)\r\n"
                      "450.0, 10.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    sciformats::jdx::XyParameters params = {"", "", 450.0, 450.0, 450.0, 450.0,
        10.0, 10.0, 1.0, 1.0, 1, 10.0, 1.0, 1.0};
    REQUIRE_THROWS(sciformats::jdx::XyData(stream, params));
}

TEST_CASE("detects illegal stream position (wrong label)", "[XyData]")
{
    std::string input{"##NPOINTS= 1\r\n"
                      "##XYDATA= (XY..XY)\r\n"
                      "450.0, 10.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    sciformats::jdx::XyParameters params = {"", "", 450.0, 450.0, 450.0, 450.0,
        10.0, 10.0, 1.0, 1.0, 1, 10.0, 1.0, 1.0};
    REQUIRE_THROWS(sciformats::jdx::XyData(stream, params));
}

TEST_CASE("detects illegal stream position (not LDR start)", "[XyData]")
{
    std::string input{"450.0, 10.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    sciformats::jdx::XyParameters params = {"", "", 450.0, 450.0, 450.0, 450.0,
        10.0, 10.0, 1.0, 1.0, 1, 10.0, 1.0, 1.0};
    REQUIRE_THROWS(sciformats::jdx::XyData(stream, params));
}
