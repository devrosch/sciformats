#include "jdx/XyPoints.hpp"
#include "jdx/StringLdr.hpp"

#include "catch2/catch.hpp"

#include <sstream>

TEST_CASE("parses unevenly spaced xy data", "[XyPoints]")
{
    // "##XYPOINTS= (XY..XY)\r\n"
    const auto* label = "XYPOINTS";
    const auto* variables = "(XY..XY)";
    std::string input{"450.0, 10.0; 451.0, 11.0\r\n"
                      "460.0, ?; 461.0, 21.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    std::vector<sciformats::jdx::StringLdr> ldrs;
    ldrs.emplace_back("XUNITS", "1/CM");
    ldrs.emplace_back("YUNITS", "ABSORBANCE");
    ldrs.emplace_back("FIRSTX", "900.0");
    ldrs.emplace_back("LASTX", "922.0");
    ldrs.emplace_back("XFACTOR", "2.0");
    ldrs.emplace_back("YFACTOR", "10.0");
    ldrs.emplace_back("NPOINTS", "4");
    auto xyPointsRecord
        = sciformats::jdx::XyPoints(label, variables, stream, ldrs);

    auto xyData = xyPointsRecord.getData();

    REQUIRE(4 == xyData.size());
    REQUIRE(900.0 == Approx(xyData.at(0).first));
    REQUIRE(100.0 == Approx(xyData.at(0).second));
    REQUIRE(902.0 == Approx(xyData.at(1).first));
    REQUIRE(110.0 == Approx(xyData.at(1).second));
    REQUIRE(920.0 == Approx(xyData.at(2).first));
    REQUIRE(std::isnan(xyData.at(2).second));
    REQUIRE(922.0 == Approx(xyData.at(3).first));
    REQUIRE(210.0 == Approx(xyData.at(3).second));
    auto params = xyPointsRecord.getParameters();
    REQUIRE("1/CM" == params.xUnits);
    REQUIRE("ABSORBANCE" == params.yUnits);
    REQUIRE(900.0 == Approx(params.firstX));
    REQUIRE(922.0 == Approx(params.lastX));
    REQUIRE(2.0 == Approx(params.xFactor));
    REQUIRE(10.0 == Approx(params.yFactor));
    REQUIRE(4 == params.nPoints);
    REQUIRE_FALSE(params.deltaX.has_value());
    REQUIRE_FALSE(params.resolution.has_value());
}

TEST_CASE("fails when x value undefined while parsing unevenly spaced xy data",
    "[XyPoints]")
{
    // "##XYPOINTS= (XY..XY)\r\n"
    const auto* label = "XYPOINTS";
    const auto* variables = "(XY..XY)";
    std::string input{"450.0, 10.0; 451.0, 11.0\r\n"
                      "?, 20.0; 461.0, 21.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    std::vector<sciformats::jdx::StringLdr> ldrs;
    ldrs.emplace_back("XUNITS", "1/CM");
    ldrs.emplace_back("YUNITS", "ABSORBANCE");
    ldrs.emplace_back("FIRSTX", "900.0");
    ldrs.emplace_back("LASTX", "922.0");
    ldrs.emplace_back("XFACTOR", "2.0");
    ldrs.emplace_back("YFACTOR", "10.0");
    ldrs.emplace_back("NPOINTS", "4");
    auto xyPointsRecord
        = sciformats::jdx::XyPoints(label, variables, stream, ldrs);

    REQUIRE_THROWS_WITH(
        xyPointsRecord.getData(), Catch::Matchers::Contains("NaN")
                                      && Catch::Matchers::Contains("x value"));
}

TEST_CASE(
    "fails when NPOINTS does not match number of xy data points", "[XyPoints]")
{
    // "##XYPOINTS= (XY..XY)\r\n"
    const auto* label = "XYPOINTS";
    const auto* variables = "(XY..XY)";
    std::string input{"450.0, 10.0; 451.0, 11.0\r\n"
                      "460.0, 20.0; 461.0, 21.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    std::vector<sciformats::jdx::StringLdr> ldrs;
    ldrs.emplace_back("XUNITS", "1/CM");
    ldrs.emplace_back("YUNITS", "ABSORBANCE");
    ldrs.emplace_back("FIRSTX", "900.0");
    ldrs.emplace_back("LASTX", "922.0");
    ldrs.emplace_back("XFACTOR", "2.0");
    ldrs.emplace_back("YFACTOR", "10.0");
    ldrs.emplace_back("NPOINTS", "3");
    auto xyPointsRecord
        = sciformats::jdx::XyPoints(label, variables, stream, ldrs);

    REQUIRE_THROWS_WITH(xyPointsRecord.getData(),
        Catch::Matchers::Contains("NPOINTS")
            && Catch::Matchers::Contains("mismatch", Catch::CaseSensitive::No));
}

TEST_CASE("fails for incomplete xy pair", "[XyPoints]")
{
    // "##XYPOINTS= (XY..XY)\r\n"
    const auto* label = "XYPOINTS";
    const auto* variables = "(XY..XY)";
    std::string input{"450.0, 10.0; 451.0, 11.0\r\n"
                      "460.0, 20.0; 461.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    std::vector<sciformats::jdx::StringLdr> ldrs;
    ldrs.emplace_back("XUNITS", "1/CM");
    ldrs.emplace_back("YUNITS", "ABSORBANCE");
    ldrs.emplace_back("FIRSTX", "900.0");
    ldrs.emplace_back("LASTX", "922.0");
    ldrs.emplace_back("XFACTOR", "2.0");
    ldrs.emplace_back("YFACTOR", "10.0");
    ldrs.emplace_back("NPOINTS", "4");
    // use other constructor for better coverage
    auto xyPointsRecord
        = sciformats::jdx::XyPoints(label, variables, stream, ldrs);

    REQUIRE_THROWS_WITH(xyPointsRecord.getData(),
        Catch::Matchers::Contains("uneven", Catch::CaseSensitive::No));
}

TEST_CASE("fails parsing ? as X value", "[XyPoints]")
{
    // "##XYPOINTS= (XY..XY)\r\n"
    const auto* label = "XYPOINTS";
    const auto* variables = "(XY..XY)";
    std::string input{"450.0, 10.0; ?, 11.0\r\n"
                      "460.0, 20.0; 461.0, 21.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    std::vector<sciformats::jdx::StringLdr> ldrs;
    ldrs.emplace_back("XUNITS", "1/CM");
    ldrs.emplace_back("YUNITS", "ABSORBANCE");
    ldrs.emplace_back("FIRSTX", "450.0");
    ldrs.emplace_back("LASTX", "461.0");
    ldrs.emplace_back("XFACTOR", "1.0");
    ldrs.emplace_back("YFACTOR", "1.0");
    ldrs.emplace_back("NPOINTS", "4");
    auto xyPointsRecord
        = sciformats::jdx::XyPoints(label, variables, stream, ldrs);

    REQUIRE_THROWS_WITH(xyPointsRecord.getData(),
        Catch::Matchers::Contains("NaN", Catch::CaseSensitive::No)
            && Catch::Matchers::Contains("x value", Catch::CaseSensitive::No));
}
