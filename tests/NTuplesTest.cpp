#include "jdx/NTuples.hpp"

#include "catch2/catch.hpp"

#include <sstream>

TEST_CASE("parses NTUPLES record", "[NTuples]")
{
    // "##NTUPLES= NMR SPECTRUM"
    std::string input{"##VAR_NAME=   FREQUENCY,    SPECTRUM/REAL,    SPECTRUM/IMAG, PAGE NUMBER\n"
                      "##SYMBOL=             X,                R,                I,           N\n"
                      "##VAR_TYPE= INDEPENDENT,        DEPENDENT,        DEPENDENT,        PAGE\n"
                      "##VAR_FORM=        AFFN,             ASDF,             ASDF,        AFFN\n"
                      "##VAR_DIM=            4,                4,                4,           2\n"
                      "##UNITS=             HZ,  ARBITRARY UNITS,  ARBITRARY UNITS,            \n"
                      "##FIRST=            0.1,             50.0,            300.0,           1\n"
                      "##LAST=             0.2,            105.0,            410.0,           2\n"
                      "##MIN=              0.1,             50.0,            300.0,           1\n"
                      "##MAX=             0.25,            105.0,            410.0,           2\n"
                      "##FACTOR=           0.1,              5.0,             10.0,           1\n"
                      "##PAGE= N=1\n"
                      "##DATA TABLE= (X++(R..R)), XYDATA   $$ Real data points\n"
                      "1.0 +10+11\n"
                      "2.0 +20+21\n"
                      "##PAGE= N=2\n"
                      "##DATA TABLE= (X++(I..U)), XYDATA   $$ Imaginary data points\n"
                      "1.0 +30+31\n"
                      "2.0 +40+41\n"
                      "##END NTUPLES= NMR SPECTRUM\n"
                      "##END=\n"
    };
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};

    // TODO: populate
    std::vector<sciformats::jdx::StringLdr> blockLdrs;

    sciformats::jdx::NTuples nTuples{"NTUPLES", "NMR SPECTRUM", reader, blockLdrs};

    REQUIRE(2 == nTuples.getNumPages());
    // TODO: test more
}
