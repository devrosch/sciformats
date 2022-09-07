#include "jdx/NTuples.hpp"
#include "jdx/DataTable.hpp"

#include "catch2/catch.hpp"

#include <sstream>

TEST_CASE("parses NTUPLES NMR record", "[NTuples]")
{
    // "##NTUPLES= NMR SPECTRUM"
    auto nextLine = std::optional<std::string>{"##NTUPLES= NMR SPECTRUM"};
    // clang-format off
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

    //   auto nextLine = std::optional<std::string>{};
    sciformats::jdx::NTuples nTuples{
        "NTUPLES", "NMR SPECTRUM", reader, blockLdrs, nextLine};

    REQUIRE(2 == nTuples.getNumPages());
    REQUIRE("NMR SPECTRUM" == nTuples.getDataForm());

    auto pageN1 = nTuples.getPage(0);
    REQUIRE("N=1" == pageN1.getPageVariables());
    REQUIRE(pageN1.getPageLdrs().empty());
    REQUIRE(4 == nTuples.getAttributes().size());
    auto pageAttrs0 = nTuples.getAttributes().at(0);
    REQUIRE(1 == pageAttrs0.applicationAttributes.size());
    REQUIRE("$CUSTOMLDR" == pageAttrs0.applicationAttributes.at(0).getLabel());
    REQUIRE("VAL1" == pageAttrs0.applicationAttributes.at(0).getValue());

    REQUIRE(pageN1.getDataTable().has_value());
    auto pageN1DataTable = pageN1.getDataTable().value();
    REQUIRE("(X++(R..R))" == pageN1DataTable.getVariableList());
    REQUIRE("XYDATA" == pageN1DataTable.getPlotDescriptor().value());

    auto pageN1XAttributes = pageN1DataTable.getAttributes().xAttributes;
    REQUIRE("FREQUENCY" == pageN1XAttributes.varName);
    REQUIRE("X" == pageN1XAttributes.symbol);
    REQUIRE("INDEPENDENT" == pageN1XAttributes.varType);
    REQUIRE("AFFN" == pageN1XAttributes.varForm);
    REQUIRE(4 == pageN1XAttributes.varDim);
    REQUIRE("HZ" == pageN1XAttributes.units);
    REQUIRE(Approx(0.1) == pageN1XAttributes.first);
    REQUIRE(Approx(0.25) == pageN1XAttributes.last);
    REQUIRE(Approx(0.1) == pageN1XAttributes.min);
    REQUIRE(Approx(0.25) == pageN1XAttributes.max);
    REQUIRE(Approx(0.1) == pageN1XAttributes.factor);

    auto pageN1YAttributes = pageN1DataTable.getAttributes().yAttributes;
    REQUIRE("SPECTRUM/REAL" == pageN1YAttributes.varName);
    REQUIRE("R" == pageN1YAttributes.symbol);
    REQUIRE("DEPENDENT" == pageN1YAttributes.varType);
    REQUIRE("ASDF" == pageN1YAttributes.varForm);
    REQUIRE(4 == pageN1YAttributes.varDim);
    REQUIRE("ARBITRARY UNITS" == pageN1YAttributes.units);
    REQUIRE(Approx(50.0) == pageN1YAttributes.first);
    REQUIRE(Approx(105.0) == pageN1YAttributes.last);
    REQUIRE(Approx(50.0) == pageN1YAttributes.min);
    REQUIRE(Approx(105.0) == pageN1YAttributes.max);
    REQUIRE(Approx(5.0) == pageN1YAttributes.factor);

    auto pageN1Data = pageN1DataTable.getData();
    REQUIRE(4 == pageN1Data.size());
    REQUIRE(Approx(0.1) == pageN1Data.at(0).first);
    REQUIRE(Approx(50.0) == pageN1Data.at(0).second);
    REQUIRE(Approx(0.25) == pageN1Data.at(3).first);
    REQUIRE(Approx(105.0) == pageN1Data.at(3).second);

    auto pageN2 = nTuples.getPage(1);
    REQUIRE("N=2" == pageN2.getPageVariables());
    REQUIRE(pageN2.getPageLdrs().empty());

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

    auto nextLine = std::optional<std::string>{};
    sciformats::jdx::NTuples nTuples{
        "NTUPLES", "MASS SPECTRUM", reader, blockLdrs, nextLine};

    REQUIRE(3 == nTuples.getNumPages());
    REQUIRE("MASS SPECTRUM" == nTuples.getDataForm());

    auto pageT5 = nTuples.getPage(0);
    REQUIRE("T = 5" == pageT5.getPageVariables());
    REQUIRE(pageT5.getPageLdrs().empty());

    REQUIRE(pageT5.getDataTable().has_value());
    auto pageT5DataTable = pageT5.getDataTable().value();
    REQUIRE("(XY..XY)" == pageT5DataTable.getVariableList());
    REQUIRE("PEAKS" == pageT5DataTable.getPlotDescriptor().value());

    auto pageT5XAttributes = pageT5DataTable.getAttributes().xAttributes;
    REQUIRE("MASS" == pageT5XAttributes.varName);
    REQUIRE("X" == pageT5XAttributes.symbol);
    REQUIRE("INDEPENDENT" == pageT5XAttributes.varType);
    REQUIRE("AFFN" == pageT5XAttributes.varForm);
    REQUIRE_FALSE(pageT5XAttributes.varDim.has_value());
    REQUIRE("M/Z" == pageT5XAttributes.units);
    REQUIRE_FALSE(pageT5XAttributes.first.has_value());
    REQUIRE_FALSE(pageT5XAttributes.last.has_value());
    REQUIRE_FALSE(pageT5XAttributes.min.has_value());
    REQUIRE_FALSE(pageT5XAttributes.max.has_value());
    REQUIRE_FALSE(pageT5XAttributes.factor.has_value());

    auto pageT5YAttributes = pageT5DataTable.getAttributes().yAttributes;
    REQUIRE("INTENSITY" == pageT5YAttributes.varName);
    REQUIRE("Y" == pageT5YAttributes.symbol);
    REQUIRE("DEPENDENT" == pageT5YAttributes.varType);
    REQUIRE("AFFN" == pageT5YAttributes.varForm);
    REQUIRE_FALSE(pageT5YAttributes.varDim.has_value());
    REQUIRE("RELATIVE ABUNDANCE" == pageT5YAttributes.units);
    REQUIRE_FALSE(pageT5YAttributes.first.has_value());
    REQUIRE_FALSE(pageT5YAttributes.last.has_value());
    REQUIRE_FALSE(pageT5YAttributes.min.has_value());
    REQUIRE_FALSE(pageT5YAttributes.max.has_value());
    REQUIRE_FALSE(pageT5YAttributes.factor.has_value());

    auto pageT5Data = pageT5DataTable.getData();
    REQUIRE(5 == pageT5Data.size());
    REQUIRE(Approx(100) == pageT5Data.at(0).first);
    REQUIRE(Approx(50.0) == pageT5Data.at(0).second);
    REQUIRE(Approx(140) == pageT5Data.at(4).first);
    REQUIRE(Approx(90.0) == pageT5Data.at(4).second);

    auto pageT10 = nTuples.getPage(1);
    REQUIRE("T = 10" == pageT10.getPageVariables());
    REQUIRE(1 == pageT10.getPageLdrs().size());

    auto pageT10Data = pageT10.getDataTable().value().getData();
    REQUIRE(4 == pageT10Data.size());
    REQUIRE(Approx(200) == pageT10Data.at(0).first);
    REQUIRE(Approx(55.0) == pageT10Data.at(0).second);
    REQUIRE(Approx(240) == pageT10Data.at(3).first);
    REQUIRE(Approx(99.0) == pageT10Data.at(3).second);
}

TEST_CASE("uses block LDRs to fill missing NTUPLES attributes", "[NTuples]")
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

    auto nextLine = std::optional<std::string>{};
    sciformats::jdx::NTuples nTuples{
        "NTUPLES", "MASS SPECTRUM", reader, blockLdrs, nextLine};

    REQUIRE(1 == nTuples.getNumPages());
    REQUIRE("MASS SPECTRUM" == nTuples.getDataForm());

    auto pageT5 = nTuples.getPage(0);
    REQUIRE(pageT5.getDataTable().has_value());
    auto pageT5DataTable = pageT5.getDataTable().value();
    REQUIRE("(XY..XY)" == pageT5DataTable.getVariableList());
    REQUIRE_FALSE(pageT5DataTable.getPlotDescriptor().has_value());

    auto pageT5XAttributes = pageT5DataTable.getAttributes().xAttributes;
    REQUIRE("MASS" == pageT5XAttributes.varName);
    REQUIRE("X" == pageT5XAttributes.symbol);
    REQUIRE("INDEPENDENT" == pageT5XAttributes.varType.value());
    REQUIRE("AFFN" == pageT5XAttributes.varForm);
    REQUIRE(5 == pageT5XAttributes.varDim.value());
    REQUIRE("XUNITS-TEST" == pageT5XAttributes.units);
    REQUIRE(Approx(200.0) == pageT5XAttributes.first.value());
    REQUIRE(Approx(280.0) == pageT5XAttributes.last.value());
    REQUIRE(Approx(200.0) == pageT5XAttributes.min.value());
    REQUIRE(Approx(280.0) == pageT5XAttributes.max.value());
    REQUIRE(Approx(2.0) == pageT5XAttributes.factor.value());

    auto pageT5YAttributes = pageT5DataTable.getAttributes().yAttributes;
    REQUIRE("INTENSITY" == pageT5YAttributes.varName);
    REQUIRE("Y" == pageT5YAttributes.symbol);
    REQUIRE("DEPENDENT" == pageT5YAttributes.varType.value());
    REQUIRE("AFFN" == pageT5YAttributes.varForm);
    REQUIRE(5 == pageT5YAttributes.varDim.value());
    REQUIRE("YUNITS-TEST" == pageT5YAttributes.units);
    REQUIRE(Approx(150.0) == pageT5YAttributes.first.value());
    REQUIRE(Approx(270.0) == pageT5YAttributes.last.value());
    REQUIRE(Approx(150.0) == pageT5YAttributes.min.value());
    REQUIRE(Approx(270.0) == pageT5YAttributes.max.value());
    REQUIRE(Approx(3.0) == pageT5YAttributes.factor.value());
}

TEST_CASE(
    "uses page LDRs to fill missing or override NTUPLES variables", "[NTuples]")
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

    auto nextLine = std::optional<std::string>{};
    sciformats::jdx::NTuples nTuples{
        "NTUPLES", "MASS SPECTRUM", reader, blockLdrs, nextLine};

    auto pageT5 = nTuples.getPage(0);
    auto pageT5DataTable = pageT5.getDataTable().value();
    auto pageT5XAttributes = pageT5DataTable.getAttributes().xAttributes;
    REQUIRE("MASS" == pageT5XAttributes.varName);
    REQUIRE("X" == pageT5XAttributes.symbol);
    REQUIRE("INDEPENDENT" == pageT5XAttributes.varType.value());
    REQUIRE("AFFN" == pageT5XAttributes.varForm);
    REQUIRE(5 == pageT5XAttributes.varDim.value());
    REQUIRE("XUNITS-TEST" == pageT5XAttributes.units.value());
    REQUIRE(Approx(200.0) == pageT5XAttributes.first.value());
    REQUIRE(Approx(280.0) == pageT5XAttributes.last.value());
    REQUIRE(Approx(200.0) == pageT5XAttributes.min.value());
    REQUIRE(Approx(280.0) == pageT5XAttributes.max.value());
    REQUIRE(Approx(2.0) == pageT5XAttributes.factor.value());

    auto pageT5YAttributes = pageT5DataTable.getAttributes().yAttributes;
    REQUIRE("INTENSITY" == pageT5YAttributes.varName);
    REQUIRE("Y" == pageT5YAttributes.symbol);
    REQUIRE("DEPENDENT" == pageT5YAttributes.varType.value());
    REQUIRE("AFFN" == pageT5YAttributes.varForm);
    REQUIRE(5 == pageT5YAttributes.varDim.value());
    REQUIRE("YUNITS-TEST" == pageT5YAttributes.units.value());
    REQUIRE(Approx(150.0) == pageT5YAttributes.first.value());
    REQUIRE(Approx(270.0) == pageT5YAttributes.last.value());
    REQUIRE(Approx(150.0) == pageT5YAttributes.min.value());
    REQUIRE(Approx(270.0) == pageT5YAttributes.max.value());
    REQUIRE(Approx(3.0) == pageT5YAttributes.factor.value());
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
    auto nextLine = std::optional<std::string>{};

    REQUIRE_THROWS_WITH(sciformats::jdx::NTuples("NTUPLES", "NMR SPECTRUM",
                            reader, blockLdrs, nextLine),
        Catch::Matchers::Contains("VAR_NAME", Catch::CaseSensitive::Yes));
}

TEST_CASE("fails when NTUPLES record contains duplicate LDRs", "[NTuples]")
{
    // clang-format off
    // "##NTUPLES= NMR SPECTRUM"
    std::string input{
        "##VAR_NAME=   FREQUENCY,    SPECTRUM/REAL,   PAGE NUMBER\n"
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
    auto nextLine = std::optional<std::string>{};

    REQUIRE_THROWS_WITH(sciformats::jdx::NTuples("NTUPLES", "NMR SPECTRUM",
                            reader, blockLdrs, nextLine),
        Catch::Matchers::Contains("Duplicate", Catch::CaseSensitive::No)
            || Catch::Matchers::Contains("Multipe", Catch::CaseSensitive::No));
}

TEST_CASE("fails when NTUPLES standard variable LDR lacks columns", "[NTuples]")
{
    // clang-format off
    // "##NTUPLES= NMR SPECTRUM"
    std::string input{
        "##VAR_NAME=   FREQUENCY,    SPECTRUM/REAL,   PAGE NUMBER\n"
        "##SYMBOL=             X,                Y,             N\n"
        "##VAR_TYPE= INDEPENDENT,        DEPENDENT,          PAGE\n"
        "##VAR_FORM=        AFFN,             ASDF,          AFFN\n"
        "##VAR_DIM=            4,                4,             1\n"
        "##UNITS=             HZ\n" // only one column
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
    auto nextLine = std::optional<std::string>{};

    REQUIRE_THROWS_WITH(sciformats::jdx::NTuples("NTUPLES", "NMR SPECTRUM",
                            reader, blockLdrs, nextLine),
        Catch::Matchers::Contains("UNITS", Catch::CaseSensitive::Yes)
            || Catch::Matchers::Contains("column", Catch::CaseSensitive::No));
}

TEST_CASE("fails when NTUPLES custom variable LDR lacks columns", "[NTuples]")
{
    // clang-format off
    // "##NTUPLES= NMR SPECTRUM"
    std::string input{
        "##VAR_NAME=   FREQUENCY,    SPECTRUM/REAL,   PAGE NUMBER\n"
        "##SYMBOL=             X,                Y,             N\n"
        "##VAR_TYPE= INDEPENDENT,        DEPENDENT,          PAGE\n"
        "##VAR_FORM=        AFFN,             ASDF,          AFFN\n"
        "##VAR_DIM=            4,                4,             1\n"
        "##UNITS=             HZ,  ARBITRARY UNITS,              \n"
        "##$CUSTOM_LDR=     VAL1\n"
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
    auto nextLine = std::optional<std::string>{};

    REQUIRE_THROWS_WITH(sciformats::jdx::NTuples("NTUPLES", "NMR SPECTRUM",
                            reader, blockLdrs, nextLine),
        Catch::Matchers::Contains("CUSTOM_LDR", Catch::CaseSensitive::Yes)
            || Catch::Matchers::Contains("column", Catch::CaseSensitive::No));
}

TEST_CASE("fails when NTUPLES record ends prematurely", "[NTuples]")
{
    // clang-format off
    // "##NTUPLES= NMR SPECTRUM"
    std::string input{
        "##VAR_NAME=   FREQUENCY,    SPECTRUM/REAL,   PAGE NUMBER\n"
        "##SYMBOL=             X,                Y,             N\n"
        "##VAR_TYPE= INDEPENDENT,        DEPENDENT,          PAGE\n"
        "##VAR_FORM=        AFFN,             ASDF,          AFFN\n"
        "##VAR_DIM=            4,                4,             1\n"
        "##UNITS=             HZ,  ARBITRARY UNITS,              \n"
    };
    // clang-format on
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};
    std::vector<sciformats::jdx::StringLdr> blockLdrs;
    auto nextLine = std::optional<std::string>{};

    REQUIRE_THROWS_WITH(sciformats::jdx::NTuples("NTUPLES", "NMR SPECTRUM",
                            reader, blockLdrs, nextLine),
        Catch::Matchers::Contains("unexpected end", Catch::CaseSensitive::No));
}

TEST_CASE("fails when NTUPLES PAGE record ends prematurely", "[NTuples]")
{
    // clang-format off
    // "##NTUPLES= NMR SPECTRUM"
    std::string input{
        "##VAR_NAME=   FREQUENCY,    SPECTRUM/REAL,   PAGE NUMBER\n"
        "##SYMBOL=             X,                Y,             N\n"
        "##VAR_TYPE= INDEPENDENT,        DEPENDENT,          PAGE\n"
        "##VAR_FORM=        AFFN,             ASDF,          AFFN\n"
        "##VAR_DIM=            4,                4,             1\n"
        "##UNITS=             HZ,  ARBITRARY UNITS,              \n"
        "##PAGE= N=1\n"
    };
    // clang-format on
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};
    std::vector<sciformats::jdx::StringLdr> blockLdrs;
    auto nextLine = std::optional<std::string>{};

    REQUIRE_THROWS_WITH(sciformats::jdx::NTuples("NTUPLES", "NMR SPECTRUM",
                            reader, blockLdrs, nextLine),
        Catch::Matchers::Contains("unexpected", Catch::CaseSensitive::No));
}

TEST_CASE("fails for missing NTUPLES DATA TABLE variable list",
    "[NTuples][DataTable]")
{
    // clang-format off
    // "##NTUPLES= NMR SPECTRUM"
    std::string input{
        "##VAR_NAME=   FREQUENCY,    SPECTRUM/REAL,   PAGE NUMBER\n"
        "##SYMBOL=             X,                Y,             N\n"
        "##PAGE= N=1\n"
        "##DATA TABLE=                   $$ missing variable list\n"
        "##END NTUPLES= NMR SPECTRUM\n"
    };
    // clang-format on
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};
    std::vector<sciformats::jdx::StringLdr> blockLdrs;
    auto nextLine = std::optional<std::string>{};

    REQUIRE_THROWS_WITH(sciformats::jdx::NTuples("NTUPLES", "NMR SPECTRUM",
                            reader, blockLdrs, nextLine),
        Catch::Matchers::Contains("missing", Catch::CaseSensitive::No));
}

TEST_CASE("fails for illegal NTUPLES DATA TABLE variable list",
    "[NTuples][DataTable]")
{
    // clang-format off
    // "##NTUPLES= NMR SPECTRUM"
    std::string input{
        "##VAR_NAME=   FREQUENCY,    SPECTRUM/REAL,   PAGE NUMBER\n"
        "##SYMBOL=             X,                Y,             N\n"
        "##PAGE= N=1\n"
        "##DATA TABLE= a, b, c           $$ illegal variable list\n"
        "##END NTUPLES= NMR SPECTRUM\n"
    };
    // clang-format on
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};
    std::vector<sciformats::jdx::StringLdr> blockLdrs;
    auto nextLine = std::optional<std::string>{};

    REQUIRE_THROWS_WITH(sciformats::jdx::NTuples("NTUPLES", "NMR SPECTRUM",
                            reader, blockLdrs, nextLine),
        Catch::Matchers::Contains("illegal", Catch::CaseSensitive::No)
            || Catch::Matchers::Contains(
                "unexpected", Catch::CaseSensitive::No));
}
