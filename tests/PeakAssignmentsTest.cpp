#include "jdx/PeakAssignments.hpp"

#include "catch2/catch.hpp"

#include <cmath>
#include <sstream>

// TODO: add more tests for NMR specific assignments (XYMA), (XYMWA)

TEST_CASE(
    "parses well-formed three column PEAK ASSIGNMENTS", "[PeakAssignments]")
{
    // "##PEAKASSIGNMENTS= (XYA)\r\n"
    const auto* label = "PEAKASSIGNMENTS";
    const auto* variables = "(XYA)";
    std::string input{"$$ peak width function\r\n"
                      "(1.0, 10.0, <peak assignment 1>)\r\n"
                      "( 2.0,20.0,<peak assignment 2> )\r\n"
                      "(3.0, <peak assignment 3>)\r\n"
                      "(4.0, , <peak assignment 4>)\r\n"
                      "(5.0,\r\n"
                      "50.0\r\n"
                      ", <peak\r\n"
                      "assignment 5>)\r\n"
                      "##END="};
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};

    auto nextLine = std::optional<std::string>{};
    auto assignments
        = sciformats::jdx::PeakAssignments(label, variables, reader, nextLine);
    auto widthFunction = assignments.getWidthFunction();
    auto data = assignments.getData();

    REQUIRE(widthFunction.has_value());
    REQUIRE(widthFunction.value() == "peak width function");

    REQUIRE(5 == data.size());

    auto data0 = data.at(0);
    REQUIRE(1.0 == Approx(data0.x));
    REQUIRE(data0.y.has_value());
    REQUIRE(10.0 == Approx(data0.y.value()));
    REQUIRE_FALSE(data0.w.has_value());
    REQUIRE("peak assignment 1" == data0.a);

    auto data1 = data.at(1);
    REQUIRE(2.0 == Approx(data1.x));
    REQUIRE(data1.y.has_value());
    REQUIRE(20.0 == Approx(data1.y.value()));
    REQUIRE_FALSE(data1.w.has_value());
    REQUIRE("peak assignment 2" == data1.a);

    auto data2 = data.at(2);
    REQUIRE(3.0 == Approx(data2.x));
    // alternatively check for y == NaN?
    REQUIRE_FALSE(data2.y.has_value());
    REQUIRE_FALSE(data2.w.has_value());
    REQUIRE("peak assignment 3" == data2.a);

    auto data3 = data.at(3);
    REQUIRE(4.0 == Approx(data3.x));
    // alternatively check for y == NaN?
    REQUIRE(data3.y.has_value());
    REQUIRE(std::isnan(data3.y.value()));
    REQUIRE_FALSE(data3.w.has_value());
    REQUIRE("peak assignment 4" == data3.a);

    auto data4 = data.at(4);
    REQUIRE(5.0 == Approx(data4.x));
    REQUIRE(data4.y.has_value());
    REQUIRE(50.0 == Approx(data4.y.value()));
    REQUIRE_FALSE(data4.w.has_value());
    REQUIRE("peak assignment 5" == data4.a);
}

TEST_CASE(
    "parses well-formed four column PEAK ASSIGNMENTS", "[PeakAssignments]")
{
    // "##PEAKASSIGNMENTS= (XYWA)\r\n"
    const auto* label = "PEAKASSIGNMENTS";
    const auto* variables = "(XYWA)";
    std::string input{"$$ peak width function\r\n"
                      "(1.0, 10.0, 100.0, <peak assignment 1>)\r\n"
                      "( 2.0,20.0,200.0,<peak assignment 2> )\r\n"
                      "(3.0, <peak assignment 3>)\r\n"
                      "(4.0, ,, <peak assignment 4>)\r\n"
                      "(5.0,\r\n"
                      ",\r\n"
                      "500.0,\r\n"
                      "<peak\r\n"
                      "assignment 5>)\r\n"
                      "(6.0, 60.0, , <peak assignment 6>)\r\n"
                      "##END="};
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};

    auto nextLine = std::optional<std::string>{};
    auto assignments
        = sciformats::jdx::PeakAssignments(label, variables, reader, nextLine);
    auto widthFunction = assignments.getWidthFunction();
    auto data = assignments.getData();

    REQUIRE(widthFunction.has_value());
    REQUIRE(widthFunction.value() == "peak width function");

    REQUIRE(6 == data.size());

    auto data0 = data.at(0);
    REQUIRE(1.0 == Approx(data0.x));
    REQUIRE(data0.y.has_value());
    REQUIRE(10.0 == Approx(data0.y.value()));
    REQUIRE(data0.w.has_value());
    REQUIRE(100.0 == Approx(data0.w.value()));
    REQUIRE("peak assignment 1" == data0.a);

    auto data1 = data.at(1);
    REQUIRE(2.0 == Approx(data1.x));
    REQUIRE(data1.y.has_value());
    REQUIRE(20.0 == Approx(data1.y.value()));
    REQUIRE(data1.w.has_value());
    REQUIRE(200.0 == Approx(data1.w.value()));
    REQUIRE("peak assignment 2" == data1.a);

    auto data2 = data.at(2);
    REQUIRE(3.0 == Approx(data2.x));
    // alternatively check for y == NaN?
    REQUIRE_FALSE(data2.y.has_value());
    // alternatively check for w == NaN?
    REQUIRE_FALSE(data2.w.has_value());
    REQUIRE("peak assignment 3" == data2.a);

    auto data3 = data.at(3);
    REQUIRE(4.0 == Approx(data3.x));
    REQUIRE(data3.y.has_value());
    REQUIRE(std::isnan(data3.y.value()));
    REQUIRE(data3.w.has_value());
    REQUIRE(std::isnan(data3.w.value()));
    REQUIRE("peak assignment 4" == data3.a);

    auto data4 = data.at(4);
    REQUIRE(5.0 == Approx(data4.x));
    REQUIRE(data4.y.has_value());
    REQUIRE(std::isnan(data4.y.value()));
    REQUIRE(data4.w.has_value());
    REQUIRE(500.0 == Approx(data4.w.value()));
    REQUIRE("peak assignment 5" == data4.a);

    auto data5 = data.at(5);
    REQUIRE(6.0 == Approx(data5.x));
    REQUIRE(data5.y.has_value());
    REQUIRE(60.0 == data5.y.value());
    REQUIRE(data5.w.has_value());
    REQUIRE(std::isnan(data5.w.value()));
    REQUIRE("peak assignment 6" == data5.a);
}

TEST_CASE("fails when excess component is encountered in three column PEAK "
          "ASSIGNMENTS",
    "[PeakAssignments]")
{
    // "##PEAKASSIGNMENTS= (XYA)\r\n"
    const auto* label = "PEAKASSIGNMENTS";
    const auto* variables = "(XYA)";
    std::string input{"(1.0, 10.0, 100.0, <peak assignment 1>)\r\n"
                      "##END="};
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};

    auto nextLine = std::optional<std::string>{};
    auto assignments
        = sciformats::jdx::PeakAssignments(label, variables, reader, nextLine);

    REQUIRE_THROWS_WITH(assignments.getData(),
        Catch::Matchers::Contains("illegal number", Catch::CaseSensitive::No));
}

TEST_CASE("fails when excess component is encountered in four column PEAK "
          "ASSIGNMENTS",
    "[PeakAssignments]")
{
    // "##PEAKASSIGNMENTS= (XYWA)\r\n"
    const auto* label = "PEAKASSIGNMENTS";
    const auto* variables = "(XYWA)";
    std::string input{"(1.0, 10.0, 100.0, 1000.0, <peak assignment 1>)\r\n"
                      "##END="};
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};

    auto nextLine = std::optional<std::string>{};
    auto assignments
        = sciformats::jdx::PeakAssignments(label, variables, reader, nextLine);

    REQUIRE_THROWS_WITH(assignments.getData(),
        Catch::Matchers::Contains("illegal number", Catch::CaseSensitive::No));
}

TEST_CASE("fails when ambiguous component is encountered in four column PEAK "
          "ASSIGNMENTS",
    "[PeakAssignments]")
{
    // "##PEAKASSIGNMENTS= (XYWA)\r\n"
    const auto* label = "PEAKASSIGNMENTS";
    const auto* variables = "(XYWA)";
    // 10.0 could be Y or W
    std::string input{"(1.0, 10.0, <peak assignment 1>)\r\n"
                      "##END="};
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};

    auto nextLine = std::optional<std::string>{};
    auto assignments
        = sciformats::jdx::PeakAssignments(label, variables, reader, nextLine);

    REQUIRE_THROWS_WITH(assignments.getData(),
        Catch::Matchers::Contains("ambiguous", Catch::CaseSensitive::No));
}

TEST_CASE("fails when opening parenthesis is missing in PEAK ASSIGNMENTS",
    "[PeakAssignments]")
{
    // "##PEAKASSIGNMENTS= (XYWA)\r\n"
    const auto* label = "PEAKASSIGNMENTS";
    const auto* variables = "(XYWA)";
    std::string input{"1.0, 10.0, 100.0, <peak assignment 1>)\r\n"
                      "##END="};
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};

    auto nextLine = std::optional<std::string>{};
    auto assignments
        = sciformats::jdx::PeakAssignments(label, variables, reader, nextLine);

    REQUIRE_THROWS_WITH(assignments.getData(),
        Catch::Matchers::Contains("illegal", Catch::CaseSensitive::No));
}

TEST_CASE("fails when closing parenthesis is missing in PEAK ASSIGNMENTS",
    "[PeakAssignments]")
{
    // "##PEAKASSIGNMENTS= (XYWA)\r\n"
    const auto* label = "PEAKASSIGNMENTS";
    const auto* variables = "(XYWA)";
    std::string input{"(1.0, 10.0, 100.0, <peak assignment 1>\r\n"
                      "##END="};
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};

    auto nextLine = std::optional<std::string>{};
    auto assignments
        = sciformats::jdx::PeakAssignments(label, variables, reader, nextLine);

    REQUIRE_THROWS_WITH(assignments.getData(),
        Catch::Matchers::Contains(
            "no closing parenthesis", Catch::CaseSensitive::No));
}

TEST_CASE("fails when opening angle bracket is missing in assignment string in "
          "PEAK ASSIGNMENTS",
    "[PeakAssignments]")
{
    // "##PEAKASSIGNMENTS= (XYA)\r\n"
    const auto* label = "PEAKASSIGNMENTS";
    const auto* variables = "(XYA)";
    std::string input{"(1.0, 10.0, peak assignment 1>)\r\n"
                      "##END="};
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};

    auto nextLine = std::optional<std::string>{};
    auto assignments
        = sciformats::jdx::PeakAssignments(label, variables, reader, nextLine);

    REQUIRE_THROWS_WITH(assignments.getData(),
        Catch::Matchers::Contains(
            "missing opening angle bracket", Catch::CaseSensitive::No));
}

TEST_CASE("fails when closing angle bracket is missing in assignment string in "
          "PEAK ASSIGNMENTS",
    "[PeakAssignments]")
{
    // "##PEAKASSIGNMENTS= (XYA)\r\n"
    const auto* label = "PEAKASSIGNMENTS";
    const auto* variables = "(XYA)";
    std::string input{"(1.0, 10.0, <peak assignment 1)\r\n"
                      "##END="};
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};

    auto nextLine = std::optional<std::string>{};
    auto assignments
        = sciformats::jdx::PeakAssignments(label, variables, reader, nextLine);

    REQUIRE_THROWS_WITH(assignments.getData(),
        Catch::Matchers::Contains("no delimiter", Catch::CaseSensitive::No));
}

TEST_CASE("fails when illegal separator is used in PEAK ASSIGNMENTS",
    "[PeakAssignments]")
{
    // "##PEAKASSIGNMENTS= (XYA)\r\n"
    const auto* label = "PEAKASSIGNMENTS";
    const auto* variables = "(XYA)";
    std::string input{"(1.0 10.0; <peak assignment 1>)\r\n"
                      "##END="};
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};

    auto nextLine = std::optional<std::string>{};
    auto assignments
        = sciformats::jdx::PeakAssignments(label, variables, reader, nextLine);

    REQUIRE_THROWS_WITH(assignments.getData(),
        Catch::Matchers::Contains("non whitespace", Catch::CaseSensitive::No));
}

TEST_CASE("fails when illegal variable list is encountered in PEAK ASSIGNMENTS",
    "[PeakAssignments]")
{
    // "##PEAKASSIGNMENTS= (XYAUVW)\r\n"
    const auto* label = "PEAKASSIGNMENTS";
    const auto* variables = "(XYAUVW)";
    std::string input{"(1.0, 10.0, <peak assignment 1>)\r\n"
                      "##END="};
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};
    auto nextLine = std::optional<std::string>{};

    REQUIRE_THROWS_WITH(
        sciformats::jdx::PeakAssignments(label, variables, reader, nextLine),
        Catch::Matchers::Contains("illegal", Catch::CaseSensitive::No)
            && Catch::Matchers::Contains("variable list"));
}

TEST_CASE(
    "fails when PEAK ASSIGNMENTS is missing a component", "[PeakAssignments]")
{
    // "##PEAKASSIGNMENTS= (XYA)\r\n"
    const auto* label = "PEAKASSIGNMENTS";
    const auto* variables = "(XYA)";
    std::string input{"(1.0)\r\n"
                      "##END="};
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};

    auto nextLine = std::optional<std::string>{};
    auto assignments
        = sciformats::jdx::PeakAssignments(label, variables, reader, nextLine);

    REQUIRE_THROWS(assignments.getData());
}

TEST_CASE("fails for malformed PEAK ASSIGNMENT in PEAK ASSIGNMENTS",
    "[PeakAssignments]")
{
    // "##PEAKASSIGNMENTS= (XYA)\r\n"
    const auto* label = "PEAKASSIGNMENTS";
    const auto* variables = "(XYA)";
    std::string input{"(1.0, 10.0, <peak assignment 1>)\r\n"
                      "(1.0, 10.0, <peak assignment 1>\r\n"
                      "##END="};
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};

    auto nextLine = std::optional<std::string>{};
    auto assignments
        = sciformats::jdx::PeakAssignments(label, variables, reader, nextLine);

    REQUIRE_THROWS_WITH(assignments.getData(),
        Catch::Matchers::Contains(
            "no closing parenthesis", Catch::CaseSensitive::No));
}

TEST_CASE(
    "parses PEAK ASSIGNMENTS peak width function even if zero peaks present",
    "[PeakAssignments]")
{
    // "##PEAKASSIGNMENTS= (XYA)\r\n"
    const auto* label = "PEAKASSIGNMENTS";
    const auto* variables = "(XYA)";
    std::string input{"$$ peak width function\r\n"
                      "##END="};
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};

    auto nextLine = std::optional<std::string>{};
    auto assignments
        = sciformats::jdx::PeakAssignments(label, variables, reader, nextLine);
    auto widthFunction = assignments.getWidthFunction();
    auto data = assignments.getData();

    REQUIRE(widthFunction.has_value());
    REQUIRE(widthFunction.value() == "peak width function");

    REQUIRE(data.empty());
}
