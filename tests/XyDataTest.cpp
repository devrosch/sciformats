#include "jdx/XyData.hpp"
#include "jdx/StringLdr.hpp"

#include "catch2/catch.hpp"

#include <sstream>

TEST_CASE("parses AFFN xy data with required parameters only", "[XyData]")
{
    // "##XYDATA= (X++(Y..Y))\r\n"
    const auto* label = "XYDATA";
    const auto* variables = "(X++(Y..Y))";
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
    auto xyDataRecord = sciformats::jdx::XyData(label, variables, stream, ldrs);

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
    REQUIRE_FALSE(params.maxX.has_value());
    REQUIRE_FALSE(params.minX.has_value());
    REQUIRE_FALSE(params.maxY.has_value());
    REQUIRE_FALSE(params.minY.has_value());
    REQUIRE_FALSE(params.deltaX.has_value());
    REQUIRE_FALSE(params.resolution.has_value());
}

TEST_CASE("parses AFFN xy data with all optional parameters", "[XyData]")
{
    // "##XYDATA= (X++(Y..Y))\r\n"
    const auto* label = "XYDATA";
    const auto* variables = "(X++(Y..Y))";
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
    ldrs.emplace_back("MAXX", "452.0");
    ldrs.emplace_back("MINX", "450.0");
    ldrs.emplace_back("MAXY", "12.0");
    ldrs.emplace_back("MINY", "10.0");
    ldrs.emplace_back("DELTAX", "1.0");
    ldrs.emplace_back("RESOLUTION", "2.0");
    auto xyDataRecord = sciformats::jdx::XyData(label, variables, stream, ldrs);

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
    REQUIRE(452.0 == params.maxX.value());
    REQUIRE(450.0 == params.minX.value());
    REQUIRE(12.0 == params.maxY.value());
    REQUIRE(10.0 == params.minY.value());
    REQUIRE(1.0 == params.deltaX.value());
    REQUIRE(2.0 == params.resolution.value());
}

TEST_CASE("parses single data point record", "[XyData]")
{
    // "##XYDATA= (X++(Y..Y))\r\n"
    const auto* label = "XYDATA";
    const auto* variables = "(X++(Y..Y))";
    std::string input{"450.0, 10.0\r\n"
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

    auto xyDataRecord = sciformats::jdx::XyData(label, variables, stream, ldrs);
    auto xyData = xyDataRecord.getData();

    REQUIRE(1 == xyData.size());
    REQUIRE(450.0 == Approx(xyData.at(0).first));
    REQUIRE(10.0 == Approx(xyData.at(0).second));
}

TEST_CASE("detects mismatching NPOINTS", "[XyData]")
{
    // "##XYDATA= (X++(Y..Y))\r\n"
    const auto* label = "XYDATA";
    const auto* variables = "(X++(Y..Y))";
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
    ldrs.emplace_back("NPOINTS", "1");
    auto xyDataRecord = sciformats::jdx::XyData(label, variables, stream, ldrs);
    REQUIRE_THROWS(xyDataRecord.getData());
}

TEST_CASE("detects mismatching variables list for XYDATA", "[XyData]")
{
    // "##XYDATA= (R++(A..A))\r\n"
    const auto* label = "XYDATA";
    const auto* variables = "(R++(A..A))";
    std::string input{"450.0, 10.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    std::vector<sciformats::jdx::StringLdr> ldrs;
    ldrs.emplace_back("FIRSTX", "450.0");
    ldrs.emplace_back("LASTX", "450.0");
    ldrs.emplace_back("YFACTOR", "1.0");
    ldrs.emplace_back("NPOINTS", "1");
    REQUIRE_THROWS(sciformats::jdx::XyData(label, variables, stream, ldrs));
}

TEST_CASE("detects illegal stream position (wrong label)", "[XyData]")
{
    // "##NPOINTS= 1r\n"
    const auto* label = "NPOINTS";
    const auto* variables = "1";
    std::string input{"##XYDATA= (XY..XY)\r\n"
                      "450.0, 10.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    std::vector<sciformats::jdx::StringLdr> ldrs;
    ldrs.emplace_back("FIRSTX", "450.0");
    ldrs.emplace_back("LASTX", "450.0");
    ldrs.emplace_back("YFACTOR", "1.0");
    ldrs.emplace_back("NPOINTS", "1");
    REQUIRE_THROWS(sciformats::jdx::XyData(label, variables, stream, ldrs));
}

TEST_CASE(
    "omit Y value check if last digit in previous line is not DIF encoded",
    "[XyData]")
{
    // "##XYDATA= (X++(Y..Y))\r\n"
    const auto* label = "XYDATA";
    const auto* variables = "(X++(Y..Y))";
    // y values: 10 11 12 13  20 21 22 23
    std::string input{"1 A0JJA3\r\n"
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

    auto xyData = sciformats::jdx::XyData(label, variables, stream, ldrs);
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

TEST_CASE("parses zero data point record", "[XyData]")
{
    // "##XYDATA= (X++(Y..Y))\r\n"
    const auto* label = "XYDATA";
    const auto* variables = "(X++(Y..Y))";
    std::string input{"##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    std::vector<sciformats::jdx::StringLdr> ldrs;
    ldrs.emplace_back("XUNITS", "1/CM");
    ldrs.emplace_back("YUNITS", "ABSORBANCE");
    ldrs.emplace_back("FIRSTX", "450.0");
    ldrs.emplace_back("LASTX", "450.0");
    ldrs.emplace_back("XFACTOR", "1.0");
    ldrs.emplace_back("YFACTOR", "1.0");
    ldrs.emplace_back("NPOINTS", "0");

    auto xyDataRecord = sciformats::jdx::XyData(label, variables, stream, ldrs);
    auto xyData = xyDataRecord.getData();

    REQUIRE(xyData.empty());
}
