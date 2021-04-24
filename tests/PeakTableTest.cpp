#include "jdx/Peak.hpp"
#include "jdx/PeakTable.hpp"

#include "catch2/catch.hpp"

#include <sstream>

TEST_CASE("parses well-formed two column PEAK TABLE", "[PeakTable]")
{
    std::string input{"##PEAK TABLE= (XY..XY)\r\n"
                      "450.0, 10.0\r\n"
                      "460.0, 11.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    auto table = sciformats::jdx::PeakTable(stream);
    auto xyData = table.getData();

    REQUIRE(2 == xyData.size());
    REQUIRE(450.0 == Approx(xyData.at(0).x));
    REQUIRE(10.0 == Approx(xyData.at(0).y));
    REQUIRE(!xyData.at(0).w.has_value());
    REQUIRE(460.0 == Approx(xyData.at(1).x));
    REQUIRE(11.0 == Approx(xyData.at(1).y));
    REQUIRE(!xyData.at(1).w.has_value());
}
