#include "jdx/PeakTable.hpp"

#include "catch2/catch.hpp"

#include <sstream>

TEST_CASE("parses well-formed two column PEAK TABLE", "[PeakTable]")
{
    std::string input{"##PEAK TABLE= (XY..XY)\r\n"
                      "450.0, 10.0\r\n"
                      "460.0, 11.0\r\n"
                      "470.0, 12.0 480.0, 13.0\r\n"
                      "490.0, 14.0; 500.0, 15.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    auto table = sciformats::jdx::PeakTable(stream);
    auto xyData = table.getData();

    REQUIRE(6 == xyData.size());
    REQUIRE(450.0 == Approx(xyData.at(0).x));
    REQUIRE(10.0 == Approx(xyData.at(0).y));
    REQUIRE(!xyData.at(0).w.has_value());
    REQUIRE(460.0 == Approx(xyData.at(1).x));
    REQUIRE(11.0 == Approx(xyData.at(1).y));
    REQUIRE(!xyData.at(1).w.has_value());
    REQUIRE(470.0 == Approx(xyData.at(2).x));
    REQUIRE(12.0 == Approx(xyData.at(2).y));
    REQUIRE(!xyData.at(2).w.has_value());
    REQUIRE(480.0 == Approx(xyData.at(3).x));
    REQUIRE(13.0 == Approx(xyData.at(3).y));
    REQUIRE(!xyData.at(3).w.has_value());
    REQUIRE(490.0 == Approx(xyData.at(4).x));
    REQUIRE(14.0 == Approx(xyData.at(4).y));
    REQUIRE(!xyData.at(4).w.has_value());
    REQUIRE(500.0 == Approx(xyData.at(5).x));
    REQUIRE(15.0 == Approx(xyData.at(5).y));
    REQUIRE(!xyData.at(5).w.has_value());
}

TEST_CASE("parses well-formed three column PEAK TABLE", "[PeakTable]")
{
    std::string input{"##PEAK TABLE= (XYW..XYW)\r\n"
                      "450.0, 10.0, 1.0\r\n"
                      "460.0, 11.0, 2.0\r\n"
                      "470.0, 12.0, 3.0 480.0, 13.0, 4.0\r\n"
                      "490.0, 14.0, 5.0; 500.0, 15.0, 6.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    auto table = sciformats::jdx::PeakTable(stream);
    auto xyData = table.getData();

    REQUIRE(6 == xyData.size());
    REQUIRE(450.0 == Approx(xyData.at(0).x));
    REQUIRE(10.0 == Approx(xyData.at(0).y));
    REQUIRE(1.0 == Approx(xyData.at(0).w.value()));
    REQUIRE(460.0 == Approx(xyData.at(1).x));
    REQUIRE(11.0 == Approx(xyData.at(1).y));
    REQUIRE(2.0 == Approx(xyData.at(1).w.value()));
    REQUIRE(470.0 == Approx(xyData.at(2).x));
    REQUIRE(12.0 == Approx(xyData.at(2).y));
    REQUIRE(3.0 == Approx(xyData.at(2).w.value()));
    REQUIRE(480.0 == Approx(xyData.at(3).x));
    REQUIRE(13.0 == Approx(xyData.at(3).y));
    REQUIRE(4.0 == Approx(xyData.at(3).w.value()));
    REQUIRE(490.0 == Approx(xyData.at(4).x));
    REQUIRE(14.0 == Approx(xyData.at(4).y));
    REQUIRE(5.0 == Approx(xyData.at(4).w.value()));
    REQUIRE(500.0 == Approx(xyData.at(5).x));
    REQUIRE(15.0 == Approx(xyData.at(5).y));
    REQUIRE(6.0 == Approx(xyData.at(5).w.value()));
}

TEST_CASE("fails when excess component is encountered in two column PEAK TABLE",
    "[PeakTable]")
{
    std::string input{"##PEAK TABLE= (XY..XY)\r\n"
                      "450.0, 10.0, 1.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    auto table = sciformats::jdx::PeakTable(stream);
    REQUIRE_THROWS_WITH(
        table.getData(), Catch::Matchers::Contains("excess peak component",
                             Catch::CaseSensitive::No));
}

TEST_CASE(
    "fails when excess component is encountered in three column PEAK TABLE",
    "[PeakTable]")
{
    std::string input{"##PEAK TABLE= (XYW..XYW)\r\n"
                      "450.0, 10.0, 1.0, -1.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    auto table = sciformats::jdx::PeakTable(stream);
    REQUIRE_THROWS_WITH(
        table.getData(), Catch::Matchers::Contains("excess peak component",
                             Catch::CaseSensitive::No));
}

TEST_CASE(
    "fails when incomplete group is encountered in PEAK TABLE", "[PeakTable]")
{
    std::string input{"##PEAK TABLE= (XY..XY)\r\n"
                      "450.0, 10.0\r\n"
                      "460.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    auto table = sciformats::jdx::PeakTable(stream);
    REQUIRE_THROWS_WITH(
        table.getData(), Catch::Matchers::Contains("missing peak component",
                             Catch::CaseSensitive::No));
}

TEST_CASE(
    "fails when non existent value is encountered in PEAK TABLE", "[PeakTable]")
{
    std::string input{"##PEAK TABLE= (XYW..XYW)\r\n"
                      "450.0,, 10.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    auto table = sciformats::jdx::PeakTable(stream);
    REQUIRE_THROWS_WITH(
        table.getData(), Catch::Matchers::Contains("missing peak component",
                             Catch::CaseSensitive::No));
}
