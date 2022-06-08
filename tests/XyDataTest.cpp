#include "jdx/XyData.hpp"
#include "jdx/StringLdr.hpp"

#include "catch2/catch.hpp"

#include <sstream>

TEST_CASE("parses AFFN xy data, stream at LDR start", "[XyData]")
{
    std::string input{"##XYDATA= (X++(Y..Y))\r\n"
                      "450.0, 10.0\r\n"
                      "451.0, 11.0\r\n"
                      "452.0, 12.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    std::vector<sciformats::jdx::StringLdr> ldrs;
    ldrs.emplace_back("XUNITS", "1/CM");
    ldrs.emplace_back("YUNITS", "ABSORBANCE");
    ldrs.emplace_back("FIRSTX", "450.0");
    ldrs.emplace_back("LASTX", "452.0");
    ldrs.emplace_back("XFACTOR", "1.0");
    ldrs.emplace_back("YFACTOR", "1.0");
    ldrs.emplace_back("NPOINTS", "3");
    auto xyDataRecord = sciformats::jdx::XyData(stream, ldrs);

    auto xyData = xyDataRecord.getData();

    REQUIRE(3 == xyData.size());
    REQUIRE(450.0 == Approx(xyData.at(0).first));
    REQUIRE(10.0 == Approx(xyData.at(0).second));
    REQUIRE(451.0 == Approx(xyData.at(1).first));
    REQUIRE(11.0 == Approx(xyData.at(1).second));
    REQUIRE(452.0 == Approx(xyData.at(2).first));
    REQUIRE(12.0 == Approx(xyData.at(2).second));
    auto params = xyDataRecord.getParameters();
    REQUIRE("1/CM" == params.xUnits);
    REQUIRE("ABSORBANCE" == params.yUnits);
    REQUIRE(450.0 == Approx(params.firstX));
    REQUIRE(452.0 == Approx(params.lastX));
    REQUIRE(1.0 == Approx(params.xFactor));
    REQUIRE(1.0 == Approx(params.yFactor));
    REQUIRE(3 == params.nPoints);
    REQUIRE_FALSE(params.deltaX.has_value());
    REQUIRE_FALSE(params.resolution.has_value());
}

TEST_CASE("parses AFFN xy data, stream at 2nd line start", "[XyData]")
{
    std::string input{"450.0, 10.0\r\n"
                      "451.0, 11.0\r\n"
                      "452.0, 12.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    std::vector<sciformats::jdx::StringLdr> ldrs;
    ldrs.emplace_back("XUNITS", "1/CM");
    ldrs.emplace_back("YUNITS", "ABSORBANCE");
    ldrs.emplace_back("FIRSTX", "450.0");
    ldrs.emplace_back("LASTX", "452.0");
    ldrs.emplace_back("XFACTOR", "1.0");
    ldrs.emplace_back("YFACTOR", "1.0");
    ldrs.emplace_back("NPOINTS", "3");
    // optional
    ldrs.emplace_back("MAXX", "452.0");
    ldrs.emplace_back("MINX", "450.0");
    ldrs.emplace_back("MAXY", "12.0");
    ldrs.emplace_back("MINY", "10.0");

    auto xyDataRecord
        = sciformats::jdx::XyData("XYDATA", "(X++(Y..Y))", stream, ldrs);

    auto xyData = xyDataRecord.getData();
    REQUIRE(3 == xyData.size());
    REQUIRE(450.0 == Approx(xyData.at(0).first));
    REQUIRE(10.0 == Approx(xyData.at(0).second));
    REQUIRE(451.0 == Approx(xyData.at(1).first));
    REQUIRE(11.0 == Approx(xyData.at(1).second));
    REQUIRE(452.0 == Approx(xyData.at(2).first));
    REQUIRE(12.0 == Approx(xyData.at(2).second));

    auto params = xyDataRecord.getParameters();
    REQUIRE(452.0 == Approx(params.maxX.value()));
    REQUIRE(450.0 == Approx(params.minX.value()));
    REQUIRE(12.0 == Approx(params.maxY.value()));
    REQUIRE(10.0 == Approx(params.minY.value()));
}

TEST_CASE("parses single data point record", "[XyData]")
{
    std::string input{"##XYDATA= (X++(Y..Y))\r\n"
                      "450.0, 10.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    std::vector<sciformats::jdx::StringLdr> ldrs;
    ldrs.emplace_back("XUNITS", "1/CM");
    ldrs.emplace_back("YUNITS", "ABSORBANCE");
    ldrs.emplace_back("FIRSTX", "450.0");
    ldrs.emplace_back("LASTX", "450.0");
    ldrs.emplace_back("XFACTOR", "1.0");
    ldrs.emplace_back("YFACTOR", "1.0");
    ldrs.emplace_back("NPOINTS", "1");

    auto xyDataRecord = sciformats::jdx::XyData(stream, ldrs);
    auto xyData = xyDataRecord.getData();

    REQUIRE(1 == xyData.size());
    REQUIRE(450.0 == Approx(xyData.at(0).first));
    REQUIRE(10.0 == Approx(xyData.at(0).second));
}

TEST_CASE("detects mismatching NPOINTS", "[XyData]")
{
    std::string input{"##XYDATA= (X++(Y..Y))\r\n"
                      "450.0, 10.0\r\n"
                      "451.0, 11.0\r\n"
                      "452.0, 12.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    std::vector<sciformats::jdx::StringLdr> ldrs;
    ldrs.emplace_back("XUNITS", "1/CM");
    ldrs.emplace_back("YUNITS", "ABSORBANCE");
    ldrs.emplace_back("FIRSTX", "450.0");
    ldrs.emplace_back("LASTX", "452.0");
    ldrs.emplace_back("XFACTOR", "1.0");
    ldrs.emplace_back("YFACTOR", "1.0");
    ldrs.emplace_back("NPOINTS", "1");
    auto xyDataRecord = sciformats::jdx::XyData(stream, ldrs);
    REQUIRE_THROWS(xyDataRecord.getData());
}

TEST_CASE("detects mismatching variables list for XYDATA", "[XyData]")
{
    std::string input{"##XYDATA= (R++(A..A))\r\n"
                      "450.0, 10.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    std::vector<sciformats::jdx::StringLdr> ldrs;
    ldrs.emplace_back("FIRSTX", "450.0");
    ldrs.emplace_back("LASTX", "450.0");
    ldrs.emplace_back("YFACTOR", "1.0");
    ldrs.emplace_back("NPOINTS", "1");
    REQUIRE_THROWS(sciformats::jdx::XyData(stream, ldrs));
}

TEST_CASE("detects illegal stream position (wrong label)", "[XyData]")
{
    std::string input{"##NPOINTS= 1\r\n"
                      "##XYDATA= (XY..XY)\r\n"
                      "450.0, 10.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    std::vector<sciformats::jdx::StringLdr> ldrs;
    ldrs.emplace_back("FIRSTX", "450.0");
    ldrs.emplace_back("LASTX", "450.0");
    ldrs.emplace_back("YFACTOR", "1.0");
    ldrs.emplace_back("NPOINTS", "1");
    REQUIRE_THROWS(sciformats::jdx::XyData(stream, ldrs));
}

TEST_CASE("detects illegal stream position (not LDR start)", "[XyData]")
{
    std::string input{"450.0, 10.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    std::vector<sciformats::jdx::StringLdr> ldrs;
    ldrs.emplace_back("FIRSTX", "450.0");
    ldrs.emplace_back("LASTX", "450.0");
    ldrs.emplace_back("YFACTOR", "1.0");
    ldrs.emplace_back("NPOINTS", "1");
    REQUIRE_THROWS(sciformats::jdx::XyData(stream, ldrs));
}

TEST_CASE(
    "omit Y value check if last digit in previous line is not DIF encoded",
    "[XyData]")
{
    // y values: 10 11 12 13  20 21 22 23
    std::string input{"##XYDATA= (X++(Y..Y))\r\n"
                      "1 A0JJA3\r\n"
                      "5 B0JJB3\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    std::vector<sciformats::jdx::StringLdr> ldrs;
    ldrs.emplace_back("XUNITS", "1/CM");
    ldrs.emplace_back("YUNITS", "ABSORBANCE");
    ldrs.emplace_back("FIRSTX", "1.0");
    ldrs.emplace_back("LASTX", "8.0");
    ldrs.emplace_back("XFACTOR", "1.0");
    ldrs.emplace_back("YFACTOR", "1.0");
    ldrs.emplace_back("NPOINTS", "8");

    auto xyData = sciformats::jdx::XyData(stream, ldrs);
    auto data = xyData.getData();

    REQUIRE(data.size() == 8);
    REQUIRE(data.at(0).second == Approx(10.0));
    REQUIRE(data.at(1).second == Approx(11.0));
    REQUIRE(data.at(2).second == Approx(12.0));
    REQUIRE(data.at(3).second == Approx(13.0));
    REQUIRE(data.at(4).second == Approx(20.0));
    REQUIRE(data.at(5).second == Approx(21.0));
    REQUIRE(data.at(6).second == Approx(22.0));
    REQUIRE(data.at(7).second == Approx(23.0));
}
