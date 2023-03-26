#include "jdx/PeakTable.hpp"

#include "catch2/catch.hpp"

#include <sstream>

TEST_CASE("parses well-formed (XY..XY) PEAK TABLE", "[PeakTable]")
{
    // "##PEAKTABLE= (XY..XY)\r\n"
    const auto* label = "PEAKTABLE";
    const auto* variables = "(XY..XY)";
    std::string input{"$$ peak width kernel line 1\r\n"
                      "$$ peak width kernel line 2\r\n"
                      "450.0,  10.0\r\n"
                      "460.0, 11.0 $$ test comment\r\n"
                      " 470.0, 12.0E2 480.0, 13.0\r\n"
                      "490.0, 14.0;  500.0, 15.0\r\n"
                      "##END="};
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};

    auto nextLine = std::optional<std::string>{};
    auto table = sciformats::jdx::PeakTable(label, variables, reader, nextLine);
    auto kernel = table.getWidthFunction();
    auto xyData = table.getData();

    REQUIRE(kernel.has_value());
    REQUIRE(kernel.value()
            == "peak width kernel line 1"
               "\n"
               "peak width kernel line 2");

    REQUIRE(6 == xyData.size());
    REQUIRE(450.0 == Approx(xyData.at(0).x));
    REQUIRE(10.0 == Approx(xyData.at(0).y));
    REQUIRE(!xyData.at(0).w.has_value());
    REQUIRE(460.0 == Approx(xyData.at(1).x));
    REQUIRE(11.0 == Approx(xyData.at(1).y));
    REQUIRE(!xyData.at(1).w.has_value());
    REQUIRE(470.0 == Approx(xyData.at(2).x));
    REQUIRE(1200.0 == Approx(xyData.at(2).y));
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

TEST_CASE("parses well-formed (XYW..XYW) PEAK TABLE", "[PeakTable]")
{
    // "##PEAKTABLE= (XYW..XYW)\r\n"
    const auto* label = "PEAKTABLE";
    const auto* variables = "(XYW..XYW)";
    std::string input{"450.0, 10.0, 1.0\r\n"
                      "460.0,\t11.0,\t2.0\r\n"
                      "470.0, 12.0, 3.0 480.0, 13.0, 4.0\r\n"
                      "490.0, 14.0, 5.0; 500.0, 15.0, 6.0\r\n"
                      "##END="};
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};

    auto nextLine = std::optional<std::string>{};
    auto table = sciformats::jdx::PeakTable(label, variables, reader, nextLine);
    auto xyData = table.getData();

    REQUIRE(6 == xyData.size());
    REQUIRE(450.0 == Approx(xyData.at(0).x));
    REQUIRE(10.0 == Approx(xyData.at(0).y));
    REQUIRE_FALSE(xyData.at(0).m.has_value());
    REQUIRE(1.0 == Approx(xyData.at(0).w.value()));
    REQUIRE(460.0 == Approx(xyData.at(1).x));
    REQUIRE(11.0 == Approx(xyData.at(1).y));
    REQUIRE_FALSE(xyData.at(1).m.has_value());
    REQUIRE(2.0 == Approx(xyData.at(1).w.value()));
    REQUIRE(470.0 == Approx(xyData.at(2).x));
    REQUIRE(12.0 == Approx(xyData.at(2).y));
    REQUIRE_FALSE(xyData.at(2).m.has_value());
    REQUIRE(3.0 == Approx(xyData.at(2).w.value()));
    REQUIRE(480.0 == Approx(xyData.at(3).x));
    REQUIRE(13.0 == Approx(xyData.at(3).y));
    REQUIRE_FALSE(xyData.at(3).m.has_value());
    REQUIRE(4.0 == Approx(xyData.at(3).w.value()));
    REQUIRE(490.0 == Approx(xyData.at(4).x));
    REQUIRE(14.0 == Approx(xyData.at(4).y));
    REQUIRE_FALSE(xyData.at(4).m.has_value());
    REQUIRE(5.0 == Approx(xyData.at(4).w.value()));
    REQUIRE(500.0 == Approx(xyData.at(5).x));
    REQUIRE(15.0 == Approx(xyData.at(5).y));
    REQUIRE_FALSE(xyData.at(5).m.has_value());
    REQUIRE(6.0 == Approx(xyData.at(5).w.value()));
}

TEST_CASE("parses well-formed (XYM..XYM) PEAK TABLE", "[PeakTable]")
{
    // "##PEAKTABLE= (XYM..XYM)\r\n"
    const auto* label = "PEAKTABLE";
    const auto* variables = "(XYM..XYM)";
    std::string input{"450.0, 10.0, T\r\n"
                      "##END="};
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};

    auto nextLine = std::optional<std::string>{};
    auto table = sciformats::jdx::PeakTable(label, variables, reader, nextLine);
    auto xyData = table.getData();

    REQUIRE(1 == xyData.size());
    REQUIRE(450.0 == Approx(xyData.at(0).x));
    REQUIRE(10.0 == Approx(xyData.at(0).y));
    REQUIRE_FALSE(xyData.at(0).w.has_value());
    REQUIRE("T" == xyData.at(0).m);
}

TEST_CASE("fails when excess component is encountered in two column PEAK TABLE",
    "[PeakTable]")
{
    // "##PEAKTABLE= (XY..XY)\r\n"
    const auto* label = "PEAKTABLE";
    const auto* variables = "(XY..XY)";
    std::string input{"450.0, 10.0, 1.0\r\n"
                      "##END="};
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};

    auto nextLine = std::optional<std::string>{};
    auto table = sciformats::jdx::PeakTable(label, variables, reader, nextLine);
    REQUIRE_THROWS_WITH(table.getData(),
        Catch::Matchers::Contains("illegal", Catch::CaseSensitive::No));
}

TEST_CASE(
    "fails when excess component is encountered in three column PEAK TABLE",
    "[PeakTable]")
{
    // "##PEAKTABLE= (XY..XY)\r\n"
    const auto* label = "PEAKTABLE";
    const auto* variables = "(XYW..XYW)";
    std::string input{"450.0, 10.0, 1.0, -1.0\r\n"
                      "##END="};
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};

    auto nextLine = std::optional<std::string>{};
    auto table = sciformats::jdx::PeakTable(label, variables, reader, nextLine);
    REQUIRE_THROWS_WITH(table.getData(),
        Catch::Matchers::Contains("illegal", Catch::CaseSensitive::No));
}

TEST_CASE(
    "fails when incomplete group is encountered in PEAK TABLE", "[PeakTable]")
{
    // "##PEAKTABLE= (XY..XY)\r\n"
    const auto* label = "PEAKTABLE";
    const auto* variables = "(XY..XY)";
    std::string input{"450.0, 10.0\r\n"
                      "460.0\r\n"
                      "##END="};
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};

    auto nextLine = std::optional<std::string>{};
    auto table = sciformats::jdx::PeakTable(label, variables, reader, nextLine);
    REQUIRE_THROWS_WITH(table.getData(),
        Catch::Matchers::Contains("illegal", Catch::CaseSensitive::No));
}

TEST_CASE("reports blank value as NaN in PEAK TABLE", "[PeakTable]")
{
    // "##PEAKTABLE= (XYW..XYW)\r\n"
    const auto* label = "PEAKTABLE";
    const auto* variables = "(XYW..XYW)";
    std::string input{"450.0,, 10.0\r\n"
                      "##END="};
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};

    auto nextLine = std::optional<std::string>{};
    auto table = sciformats::jdx::PeakTable(label, variables, reader, nextLine);

    auto xyData = table.getData();

    REQUIRE(1 == xyData.size());
    REQUIRE(450.0 == Approx(xyData.at(0).x));
    REQUIRE(std::isnan(xyData.at(0).y));
    REQUIRE(10.0 == Approx(xyData.at(0).w.value()));
}

TEST_CASE("fails when illegal variable list is encountered in PEAK TABLE",
    "[PeakTable]")
{
    // "##PEAKTABLE= (XYWABC..XYWABC)\r\n"
    const auto* label = "PEAKTABLE";
    const auto* variables = "(XYWABC..XYWABC)";
    std::string input{"450.0, 3.0, 10.0\r\n"
                      "##END="};
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};

    auto nextLine = std::optional<std::string>{};

    REQUIRE_THROWS_WITH(
        sciformats::jdx::PeakTable(label, variables, reader, nextLine),
        Catch::Matchers::Contains("illegal", Catch::CaseSensitive::No)
            && Catch::Matchers::Contains("variable list"));
}

TEST_CASE("fails when PEAK TABLE is missing a component", "[PeakTable]")
{
    // "##PEAKTABLE= (XYW..XYW)\r\n"
    const auto* label = "PEAKTABLE";
    const auto* variables = "(XYW..XYW)";
    std::string input{"450.0, 10.0\r\n"
                      "##END="};
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};

    auto nextLine = std::optional<std::string>{};
    auto table = sciformats::jdx::PeakTable(label, variables, reader, nextLine);

    REQUIRE_THROWS(table.getData());
}

TEST_CASE("parses PEAK TABLE peak width function even if zero peaks present",
    "[PeakTable]")
{
    // "##PEAKTABLE= (XY..XY)\r\n"
    const auto* label = "PEAKTABLE";
    const auto* variables = "(XY..XY)";
    std::string input{"$$ peak width kernel line 1\r\n"
                      "$$ peak width kernel line 2\r\n"
                      "##END="};
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};

    auto nextLine = std::optional<std::string>{};
    auto table = sciformats::jdx::PeakTable(label, variables, reader, nextLine);
    auto kernel = table.getWidthFunction();
    auto xyData = table.getData();

    REQUIRE(kernel.has_value());
    REQUIRE(kernel.value()
            == "peak width kernel line 1"
               "\n"
               "peak width kernel line 2");

    REQUIRE(xyData.empty());
}
