#include "jdx/NTuples.hpp"

#include "catch2/catch.hpp"

#include <sstream>

TEST_CASE("parses NTUPLES NMR record", "[NTuples]")
{
    // clang-format off
    // "##NTUPLES= NMR SPECTRUM"
    std::string input{
        "##VAR_NAME=   FREQUENCY,    SPECTRUM/REAL,    SPECTRUM/IMAG, PAGE NUMBER\n"
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
        "##$CUSTOM_LDR=     VAL1,             VAL2,             VAL3,       VAL4,\n"
        "##PAGE= N=1\n"
        "##DATA TABLE= (X++(R..R)), XYDATA   $$ Real data points\n"
        "1.0 +10+11\n"
        "2.0 +20+21\n"
        "##PAGE= N=2\n"
        "##DATA TABLE= (X++(I..I)), XYDATA   $$ Imaginary data points\n"
        "1.0 +30+31\n"
        "2.0 +40+41\n"
        "##END NTUPLES= NMR SPECTRUM\n"
        "##END=\n"};
    // clang-format on
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};
    std::vector<sciformats::jdx::StringLdr> blockLdrs;

    sciformats::jdx::NTuples nTuples{
        "NTUPLES", "NMR SPECTRUM", reader, blockLdrs};

    REQUIRE(2 == nTuples.getNumPages());
    REQUIRE("NMR SPECTRUM" == nTuples.getDataForm());

    auto pageN1 = nTuples.getPage(0);
    REQUIRE("N=1" == pageN1.getPageVariables());
    REQUIRE(pageN1.getPageVariableLdrs().empty());
    REQUIRE(4 == nTuples.getVariables().size());
    auto pageVars0 = nTuples.getVariables().at(0);
    REQUIRE(1 == pageVars0.applicationAttributes.size());
    REQUIRE("$CUSTOMLDR" == pageVars0.applicationAttributes.at(0).getLabel());
    REQUIRE("VAL1" == pageVars0.applicationAttributes.at(0).getValue());

    REQUIRE(pageN1.getDataTable().has_value());
    auto pageN1DataTable = pageN1.getDataTable().value();
    REQUIRE("(X++(R..R))" == pageN1DataTable.getVariableList());
    REQUIRE("XYDATA" == pageN1DataTable.getPlotDescriptor().value());

    auto pageN1XVariables = pageN1DataTable.getVariables().xVariables;
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

    auto pageN1YVariables = pageN1DataTable.getVariables().yVariables;
    REQUIRE("SPECTRUM/REAL" == pageN1YVariables.varName);
    REQUIRE("R" == pageN1YVariables.symbol);
    REQUIRE("DEPENDENT" == pageN1YVariables.varType);
    REQUIRE("ASDF" == pageN1YVariables.varForm);
    REQUIRE(4 == pageN1YVariables.varDim);
    REQUIRE("ARBITRARY UNITS" == pageN1YVariables.units);
    REQUIRE(Approx(50.0) == pageN1YVariables.first);
    REQUIRE(Approx(105.0) == pageN1YVariables.last);
    REQUIRE(Approx(50.0) == pageN1YVariables.min);
    REQUIRE(Approx(105.0) == pageN1YVariables.max);
    REQUIRE(Approx(5.0) == pageN1YVariables.factor);

    auto pageN1Data = pageN1DataTable.getData();
    REQUIRE(4 == pageN1Data.size());
    REQUIRE(Approx(0.1) == pageN1Data.at(0).first);
    REQUIRE(Approx(50.0) == pageN1Data.at(0).second);
    REQUIRE(Approx(0.25) == pageN1Data.at(3).first);
    REQUIRE(Approx(105.0) == pageN1Data.at(3).second);

    auto pageN2 = nTuples.getPage(1);
    REQUIRE("N=2" == pageN2.getPageVariables());
    REQUIRE(pageN2.getPageVariableLdrs().empty());

    REQUIRE(pageN2.getDataTable().has_value());
    auto pageN2DataTable = pageN2.getDataTable().value();
    REQUIRE("(X++(I..I))" == pageN2DataTable.getVariableList());
    REQUIRE("XYDATA" == pageN2DataTable.getPlotDescriptor().value());

    auto pageN2Data = pageN2DataTable.getData();
    REQUIRE(4 == pageN2Data.size());
    REQUIRE(Approx(0.1) == pageN2Data.at(0).first);
    REQUIRE(Approx(300.0) == pageN2Data.at(0).second);
    REQUIRE(Approx(0.25) == pageN2Data.at(3).first);
    REQUIRE(Approx(410.0) == pageN2Data.at(3).second);
}

TEST_CASE("parses NTUPLES MS record", "[NTuples]")
{
    // clang-format off
    // "##NTUPLES=          MASS SPECTRUM"
    std::string input{
        "##VAR_NAME=        MASS,          INTENSITY,          RETENTION TIME\n"
        "##SYMBOL=          X,             Y,                  T\n"
        "##VAR_TYPE=        INDEPENDENT,   DEPENDENT,          INDEPENDENT\n"
        "##VAR_FORM=        AFFN,          AFFN,               AFFN\n"
        "##VAR_DIM=         ,              ,                   3\n"
        "##UNITS=           M/Z,           RELATIVE ABUNDANCE, SECONDS\n"
        "##FIRST=           ,              ,                   5\n"
        "##LAST=            ,              ,                   15\n"
        "##PAGE=            T = 5\n"
        "##DATA TABLE=      (XY..XY),      PEAKS\n"
        "100,  50.0;  110,  60.0;  120,  70.0   \n"
        "130,  80.0;  140,  90.0                \n"
        "##PAGE=            T = 10              \n"
        "##NPOINTS=         4                   \n"
        "##DATA TABLE= (XY..XY), PEAKS          \n"
        "200,  55.0;  220,  77.0                \n"
        "230,  88.0;  240,  99.0                \n"
        "##PAGE=            T = 15              \n"
        "##DATA TABLE= (XY..XY), PEAKS          \n"
        "300,  55.5;  310,  66.6;  320,  77.7   \n"
        "330,  88.8;  340,  99.9                \n"
        "##END NTUPLES= MASS SPECTRUM\n"
        "##END=\n"};
    // clang-format on
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};
    std::vector<sciformats::jdx::StringLdr> blockLdrs;

    sciformats::jdx::NTuples nTuples{
        "NTUPLES", "MASS SPECTRUM", reader, blockLdrs};

    REQUIRE(3 == nTuples.getNumPages());
    REQUIRE("MASS SPECTRUM" == nTuples.getDataForm());

    auto pageT5 = nTuples.getPage(0);
    REQUIRE("T = 5" == pageT5.getPageVariables());
    REQUIRE(pageT5.getPageVariableLdrs().empty());

    REQUIRE(pageT5.getDataTable().has_value());
    auto pageT5DataTable = pageT5.getDataTable().value();
    REQUIRE("(XY..XY)" == pageT5DataTable.getVariableList());
    REQUIRE("PEAKS" == pageT5DataTable.getPlotDescriptor().value());

    auto pageT5XVariables = pageT5DataTable.getVariables().xVariables;
    REQUIRE("MASS" == pageT5XVariables.varName);
    REQUIRE("X" == pageT5XVariables.symbol);
    REQUIRE("INDEPENDENT" == pageT5XVariables.varType);
    REQUIRE("AFFN" == pageT5XVariables.varForm);
    REQUIRE_FALSE(pageT5XVariables.varDim.has_value());
    REQUIRE("M/Z" == pageT5XVariables.units);
    REQUIRE_FALSE(pageT5XVariables.first.has_value());
    REQUIRE_FALSE(pageT5XVariables.last.has_value());
    REQUIRE_FALSE(pageT5XVariables.min.has_value());
    REQUIRE_FALSE(pageT5XVariables.max.has_value());
    REQUIRE_FALSE(pageT5XVariables.factor.has_value());

    auto pageT5YVariables = pageT5DataTable.getVariables().yVariables;
    REQUIRE("INTENSITY" == pageT5YVariables.varName);
    REQUIRE("Y" == pageT5YVariables.symbol);
    REQUIRE("DEPENDENT" == pageT5YVariables.varType);
    REQUIRE("AFFN" == pageT5YVariables.varForm);
    REQUIRE_FALSE(pageT5YVariables.varDim.has_value());
    REQUIRE("RELATIVE ABUNDANCE" == pageT5YVariables.units);
    REQUIRE_FALSE(pageT5YVariables.first.has_value());
    REQUIRE_FALSE(pageT5YVariables.last.has_value());
    REQUIRE_FALSE(pageT5YVariables.min.has_value());
    REQUIRE_FALSE(pageT5YVariables.max.has_value());
    REQUIRE_FALSE(pageT5YVariables.factor.has_value());

    auto pageT5Data = pageT5DataTable.getData();
    REQUIRE(5 == pageT5Data.size());
    REQUIRE(Approx(100) == pageT5Data.at(0).first);
    REQUIRE(Approx(50.0) == pageT5Data.at(0).second);
    REQUIRE(Approx(140) == pageT5Data.at(4).first);
    REQUIRE(Approx(90.0) == pageT5Data.at(4).second);

    auto pageT10 = nTuples.getPage(1);
    REQUIRE("T = 10" == pageT10.getPageVariables());
    REQUIRE(1 == pageT10.getPageVariableLdrs().size());

    auto pageT10Data = pageT10.getDataTable().value().getData();
    REQUIRE(4 == pageT10Data.size());
    REQUIRE(Approx(200) == pageT10Data.at(0).first);
    REQUIRE(Approx(55.0) == pageT10Data.at(0).second);
    REQUIRE(Approx(240) == pageT10Data.at(3).first);
    REQUIRE(Approx(99.0) == pageT10Data.at(3).second);
}

TEST_CASE("uses block LDRs to fill missing NTUPLES variables", "[NTuples]")
{
    // clang-format off
    // "##NTUPLES=          MASS SPECTRUM"
    std::string input{
        "##VAR_NAME=        MASS,          INTENSITY,          RETENTION TIME\n"
        "##SYMBOL=          X,             Y,                  T\n"
        "##VAR_TYPE=        INDEPENDENT,   DEPENDENT,          INDEPENDENT\n"
        "##VAR_FORM=        AFFN,          AFFN,               AFFN\n"
        "##PAGE=            T = 5\n"
        "##DATA TABLE=      (XY..XY)            \n"
        "100,  50.0;  110,  60.0;  120,  70.0   \n"
        "130,  80.0;  140,  90.0                \n"
        "##END NTUPLES= MASS SPECTRUM\n"
        "##END=\n"};
    // clang-format on
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};

    std::vector<sciformats::jdx::StringLdr> blockLdrs;
    blockLdrs.emplace_back("XUNITS", "XUNITS-TEST");
    blockLdrs.emplace_back("FIRSTX", "200.0");
    blockLdrs.emplace_back("LASTX", "280.0");
    blockLdrs.emplace_back("MINX", "200.0");
    blockLdrs.emplace_back("MAXX", "280.0");
    blockLdrs.emplace_back("XFACTOR", "2.0");
    blockLdrs.emplace_back("YUNITS", "YUNITS-TEST");
    blockLdrs.emplace_back("FIRSTY", "150.0");
    blockLdrs.emplace_back("LASTY", "270.0");
    blockLdrs.emplace_back("MINY", "150.0");
    blockLdrs.emplace_back("MAXY", "270.0");
    blockLdrs.emplace_back("YFACTOR", "3.0");
    blockLdrs.emplace_back("NPOINTS", "5");

    sciformats::jdx::NTuples nTuples{
        "NTUPLES", "MASS SPECTRUM", reader, blockLdrs};

    REQUIRE(1 == nTuples.getNumPages());
    REQUIRE("MASS SPECTRUM" == nTuples.getDataForm());

    auto pageT5 = nTuples.getPage(0);
    REQUIRE(pageT5.getDataTable().has_value());
    auto pageT5DataTable = pageT5.getDataTable().value();
    REQUIRE("(XY..XY)" == pageT5DataTable.getVariableList());
    REQUIRE_FALSE(pageT5DataTable.getPlotDescriptor().has_value());

    auto pageT5XVariables = pageT5DataTable.getVariables().xVariables;
    REQUIRE("MASS" == pageT5XVariables.varName);
    REQUIRE("X" == pageT5XVariables.symbol);
    REQUIRE("INDEPENDENT" == pageT5XVariables.varType.value());
    REQUIRE("AFFN" == pageT5XVariables.varForm);
    REQUIRE(5 == pageT5XVariables.varDim.value());
    REQUIRE("XUNITS-TEST" == pageT5XVariables.units);
    REQUIRE(Approx(200.0) == pageT5XVariables.first.value());
    REQUIRE(Approx(280.0) == pageT5XVariables.last.value());
    REQUIRE(Approx(200.0) == pageT5XVariables.min.value());
    REQUIRE(Approx(280.0) == pageT5XVariables.max.value());
    REQUIRE(Approx(2.0) == pageT5XVariables.factor.value());

    auto pageT5YVariables = pageT5DataTable.getVariables().yVariables;
    REQUIRE("INTENSITY" == pageT5YVariables.varName);
    REQUIRE("Y" == pageT5YVariables.symbol);
    REQUIRE("DEPENDENT" == pageT5YVariables.varType.value());
    REQUIRE("AFFN" == pageT5YVariables.varForm);
    REQUIRE(5 == pageT5YVariables.varDim.value());
    REQUIRE("YUNITS-TEST" == pageT5YVariables.units);
    REQUIRE(Approx(150.0) == pageT5YVariables.first.value());
    REQUIRE(Approx(270.0) == pageT5YVariables.last.value());
    REQUIRE(Approx(150.0) == pageT5YVariables.min.value());
    REQUIRE(Approx(270.0) == pageT5YVariables.max.value());
    REQUIRE(Approx(3.0) == pageT5YVariables.factor.value());
}

TEST_CASE(
    "uses oage LDRs to fill missing or override NTUPLES variables", "[NTuples]")
{
    // clang-format off
    // "##NTUPLES=          MASS SPECTRUM"
    std::string input{
        "##VAR_NAME=        MASS,          INTENSITY,          RETENTION TIME\n"
        "##SYMBOL=          X,             Y,                  T\n"
        "##VAR_TYPE=        INDEPENDENT,   DEPENDENT,          INDEPENDENT\n"
        "##VAR_FORM=        AFFN,          AFFN,               AFFN\n"
        "##PAGE=            T = 5\n"
        "##XUNITS=          XUNITS-TEST\n"
        "##FIRSTX=          200.0\n"
        "##LASTX=           280.0\n"
        "##MINX=            200.0\n"
        "##MAXX=            280.0\n"
        "##XFACTOR=         2.0\n"
        "##YUNITS=          YUNITS-TEST\n"
        "##FIRSTY=          150.0\n"
        "##LASTY=           270.0\n"
        "##MINY=            150.0\n"
        "##MAXY=            270.0\n"
        "##YFACTOR=         3.0\n"
        "##NPOINTS=         5\n"
        "##DATA TABLE=      (XY..XY)            \n"
        "100,  50.0;  110,  60.0;  120,  70.0   \n"
        "130,  80.0;  140,  90.0                \n"
        "##END NTUPLES= MASS SPECTRUM\n"
        "##END=\n"};
    // clang-format on
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};

    std::vector<sciformats::jdx::StringLdr> blockLdrs;
    blockLdrs.emplace_back("NPOINTS", "10"); // to be overridden by PAGE LDR

    sciformats::jdx::NTuples nTuples{
        "NTUPLES", "MASS SPECTRUM", reader, blockLdrs};

    auto pageT5 = nTuples.getPage(0);
    auto pageT5DataTable = pageT5.getDataTable().value();
    auto pageT5XVariables = pageT5DataTable.getVariables().xVariables;
    REQUIRE("MASS" == pageT5XVariables.varName);
    REQUIRE("X" == pageT5XVariables.symbol);
    REQUIRE("INDEPENDENT" == pageT5XVariables.varType.value());
    REQUIRE("AFFN" == pageT5XVariables.varForm);
    REQUIRE(5 == pageT5XVariables.varDim.value());
    REQUIRE("XUNITS-TEST" == pageT5XVariables.units.value());
    REQUIRE(Approx(200.0) == pageT5XVariables.first.value());
    REQUIRE(Approx(280.0) == pageT5XVariables.last.value());
    REQUIRE(Approx(200.0) == pageT5XVariables.min.value());
    REQUIRE(Approx(280.0) == pageT5XVariables.max.value());
    REQUIRE(Approx(2.0) == pageT5XVariables.factor.value());

    auto pageT5YVariables = pageT5DataTable.getVariables().yVariables;
    REQUIRE("INTENSITY" == pageT5YVariables.varName);
    REQUIRE("Y" == pageT5YVariables.symbol);
    REQUIRE("DEPENDENT" == pageT5YVariables.varType.value());
    REQUIRE("AFFN" == pageT5YVariables.varForm);
    REQUIRE(5 == pageT5YVariables.varDim.value());
    REQUIRE("YUNITS-TEST" == pageT5YVariables.units.value());
    REQUIRE(Approx(150.0) == pageT5YVariables.first.value());
    REQUIRE(Approx(270.0) == pageT5YVariables.last.value());
    REQUIRE(Approx(150.0) == pageT5YVariables.min.value());
    REQUIRE(Approx(270.0) == pageT5YVariables.max.value());
    REQUIRE(Approx(3.0) == pageT5YVariables.factor.value());
}

TEST_CASE("fails when NTUPLES record is missing VAR_NAME LDR", "[NTuples]")
{
    // clang-format off
    // "##NTUPLES= NMR SPECTRUM"
    // missing:
    // "##VAR_NAME=   FREQUENCY,    SPECTRUM/REAL,    PAGE NUMBER\n"
    std::string input{
        "##SYMBOL=             X,                Y,             N\n"
        "##VAR_TYPE= INDEPENDENT,        DEPENDENT,          PAGE\n"
        "##VAR_FORM=        AFFN,             ASDF,          AFFN\n"
        "##VAR_DIM=            4,                4,             1\n"
        "##UNITS=             HZ,  ARBITRARY UNITS,              \n"
        "##PAGE= N=1\n"
        "##DATA TABLE= (X++(Y..Y)), XYDATA   $$ Real data points\n"
        "1.0 +10+11\n"
        "2.0 +20+21\n"
        "##PAGE= N=2\n"
        "##END NTUPLES= NMR SPECTRUM\n"
        "##END=\n"};
    // clang-format on
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};
    std::vector<sciformats::jdx::StringLdr> blockLdrs;

    REQUIRE_THROWS_WITH(
        sciformats::jdx::NTuples("NTUPLES", "NMR SPECTRUM", reader, blockLdrs),
        Catch::Matchers::Contains("VAR_NAME", Catch::CaseSensitive::Yes));
}

TEST_CASE("fails when NTUPLES record is contains duplicate LDRs", "[NTuples]")
{
    // clang-format off
    // "##NTUPLES= NMR SPECTRUM"
    // missing:
    std::string input{
        "##VAR_NAME=   FREQUENCY,    SPECTRUM/REAL,    PAGE NUMBER\n"
        "##SYMBOL=             X,                Y,             N\n"
        "##SYMBOL=             X,                Y,             N\n"
        "##VAR_TYPE= INDEPENDENT,        DEPENDENT,          PAGE\n"
        "##VAR_FORM=        AFFN,             ASDF,          AFFN\n"
        "##VAR_DIM=            4,                4,             1\n"
        "##UNITS=             HZ,  ARBITRARY UNITS,              \n"
        "##PAGE= N=1\n"
        "##DATA TABLE= (X++(Y..Y)), XYDATA   $$ Real data points\n"
        "1.0 +10+11\n"
        "2.0 +20+21\n"
        "##PAGE= N=2\n"
        "##END NTUPLES= NMR SPECTRUM\n"
        "##END=\n"};
    // clang-format on
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};
    std::vector<sciformats::jdx::StringLdr> blockLdrs;

    REQUIRE_THROWS_WITH(
        sciformats::jdx::NTuples("NTUPLES", "NMR SPECTRUM", reader, blockLdrs),
        Catch::Matchers::Contains("Duplicate", Catch::CaseSensitive::No)
            || Catch::Matchers::Contains("Multipe", Catch::CaseSensitive::No));
}
