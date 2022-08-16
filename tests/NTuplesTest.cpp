#include "jdx/NTuples.hpp"

#include "catch2/catch.hpp"

#include <sstream>

TEST_CASE("parses NTUPLES NMR record", "[NTuples]")
{
    // "##NTUPLES= NMR SPECTRUM"
    std::string input{"##VAR_NAME=   FREQUENCY,    SPECTRUM/REAL,    SPECTRUM/IMAG, PAGE NUMBER\n"
                      "##SYMBOL=             X,                R,                I,           N\n"
                      "##VAR_TYPE= INDEPENDENT,        DEPENDENT,        DEPENDENT,        PAGE\n"
                      "##VAR_FORM=        AFFN,             ASDF,             ASDF,        AFFN\n"
                      "##VAR_DIM=            4,                4,                4,           2\n"
                      "##UNITS=             HZ,  ARBITRARY UNITS,  ARBITRARY UNITS,            \n"
                      "##FIRST=            0.1,             50.0,            300.0,           1\n"
                      "##LAST=            0.25,            105.0,            410.0,           2\n"
                      "##MIN=              0.1,             50.0,            300.0,           1\n"
                      "##MAX=             0.25,            105.0,            410.0,           2\n"
                      "##FACTOR=           0.1,              5.0,             10.0,           1\n"
                      "##PAGE= N=1\n"
                      "##DATA TABLE= (X++(R..R)), XYDATA   $$ Real data points\n"
                      "1.0 +10+11\n"
                      "2.0 +20+21\n"
                      "##PAGE= N=2\n"
                      "##DATA TABLE= (X++(I..I)), XYDATA   $$ Imaginary data points\n"
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
    REQUIRE("NMR SPECTRUM" == nTuples.getDataForm());

    auto pageN1 = nTuples.getPage(0);

    REQUIRE("N=1" == pageN1.getPageVariables());
    REQUIRE(pageN1.getPageVariableLdrs().empty());
    REQUIRE(sciformats::jdx::NTuplesPage::VarType::XppRR == pageN1.getDataTableVariableList());
    REQUIRE(sciformats::jdx::NTuplesPage::PlotDescriptor::XyData == pageN1.getDataTablePlotDescriptor().value());

    auto pageN1XVariables = pageN1.getDataTableVariables().xVariables;
    REQUIRE("FREQUENCY" == pageN1XVariables.varName);
    REQUIRE("X" == pageN1XVariables.symbol);
    REQUIRE("INDEPENDENT" == pageN1XVariables.varType);
    REQUIRE("AFFN" == pageN1XVariables.varForm);
    REQUIRE(4 == pageN1XVariables.varDim);
    REQUIRE("HZ" == pageN1XVariables.units);
    REQUIRE(Approx(0.1) == pageN1XVariables.first);
    REQUIRE(Approx(0.25) == pageN1XVariables.last);
    REQUIRE(Approx(0.1) == pageN1XVariables.min);
    REQUIRE(Approx(0.25) == pageN1XVariables.max);
    REQUIRE(Approx(0.1) == pageN1XVariables.factor);

    auto pageN1DataTable = pageN1.getDataTable();
    REQUIRE(pageN1DataTable.size() == 4);
    REQUIRE(Approx(0.1) == pageN1DataTable.at(0).first);
    REQUIRE(Approx(50.0) == pageN1DataTable.at(0).second);
    REQUIRE(Approx(0.25) == pageN1DataTable.at(3).first);
    REQUIRE(Approx(105.0) == pageN1DataTable.at(3).second);

    // TODO: test more
}
