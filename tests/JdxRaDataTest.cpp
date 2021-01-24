#include "jdx/JdxLdr.hpp"
#include "jdx/RaData.hpp"

#include "catch2/catch.hpp"

#include <sstream>

TEST_CASE("parses AFFN RA data, stream at LDR start", "[RaData]")
{
    std::string input{"##RADATA= (R++(A..A))\r\n"
                      "0, 10.0\r\n"
                      "1, 11.0\r\n"
                      "2, 12.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    std::vector<sciformats::jdx::JdxLdr> ldrs;
    ldrs.emplace_back("RUNITS", "MICROMETERS");
    ldrs.emplace_back("AUNITS", "ARBITRARY UNITS");
    ldrs.emplace_back("FIRSTR", "0");
    ldrs.emplace_back("LASTR", "2");
    ldrs.emplace_back("RFACTOR", "1.0");
    ldrs.emplace_back("AFACTOR", "1.0");
    ldrs.emplace_back("NPOINTS", "3");
    auto raDataRecord = sciformats::jdx::RaData(stream, ldrs);

    auto raData = raDataRecord.getData();

    REQUIRE(3 == raData.size());
    REQUIRE(0 == Approx(raData.at(0).first));
    REQUIRE(10.0 == Approx(raData.at(0).second));
    REQUIRE(1 == Approx(raData.at(1).first));
    REQUIRE(11.0 == Approx(raData.at(1).second));
    REQUIRE(2 == Approx(raData.at(2).first));
    REQUIRE(12.0 == Approx(raData.at(2).second));
    auto params = raDataRecord.getParameters();
    REQUIRE("MICROMETERS" == params.rUnits);
    REQUIRE("ARBITRARY UNITS" == params.aUnits);
    REQUIRE(0.0 == Approx(params.firstR));
    REQUIRE(2.0 == Approx(params.lastR));
    REQUIRE(1.0 == Approx(params.rFactor));
    REQUIRE(1.0 == Approx(params.aFactor));
    REQUIRE(3 == params.nPoints);
    REQUIRE_FALSE(params.alias.has_value());
    REQUIRE_FALSE(params.deltaR.has_value());
    REQUIRE_FALSE(params.resolution.has_value());
    REQUIRE_FALSE(params.zdp.has_value());
}

TEST_CASE("parses AFFN RA data, stream at 2nd line start", "[RaData]")
{
    std::string input{"0, 10.0\r\n"
                      "1, 11.0\r\n"
                      "2, 12.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    std::vector<sciformats::jdx::JdxLdr> ldrs;
    ldrs.emplace_back("RUNITS", "MICROMETERS");
    ldrs.emplace_back("AUNITS", "ARBITRYRY UNITS");
    ldrs.emplace_back("FIRSTR", "0");
    ldrs.emplace_back("LASTR", "2");
    ldrs.emplace_back("RFACTOR", "1.0");
    ldrs.emplace_back("AFACTOR", "1.0");
    ldrs.emplace_back("NPOINTS", "3");
    auto raDataRecord
        = sciformats::jdx::RaData("RADATA", "(R++(A..A))", stream, ldrs);
    auto raData = raDataRecord.getData();

    REQUIRE(3 == raData.size());
    REQUIRE(0 == Approx(raData.at(0).first));
    REQUIRE(10.0 == Approx(raData.at(0).second));
    REQUIRE(1 == Approx(raData.at(1).first));
    REQUIRE(11.0 == Approx(raData.at(1).second));
    REQUIRE(2 == Approx(raData.at(2).first));
    REQUIRE(12.0 == Approx(raData.at(2).second));
}

TEST_CASE("detects mismatching variables list for RADATA", "[RaData]")
{
    std::string input{"##RADATA= (X++(Y..Y))\r\n"
                      "0, 10.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    std::vector<sciformats::jdx::JdxLdr> ldrs;
    ldrs.emplace_back("FIRSTR", "0");
    ldrs.emplace_back("LASTR", "0");
    ldrs.emplace_back("AFACTOR", "1.0");
    ldrs.emplace_back("NPOINTS", "1");
    REQUIRE_THROWS(sciformats::jdx::RaData(stream, ldrs));
}
