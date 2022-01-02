#include "jdx/Block.hpp"

#include "catch2/catch.hpp"

TEST_CASE("parses all LDRs in block with XYDATA", "[Block]")
{
    std::string input{"##TITLE= Test\r\n"
                      "##JCAMP-DX= 4.24\r\n"
                      "##DATA TYPE= INFRARED SPECTRUM\r\n"
                      "$$ random comment #1\r\n"
                      "##ORIGIN= devrosch\r\n"
                      "##OWNER= PUBLIC DOMAIN\r\n"
                      "##SPECTROMETER/DATA SYSTEM= Dum=\r\n"
                      "my\r\n"
                      "##XUNITS= 1/CM\r\n"
                      "##YUNITS= ABSORBANCE\r\n"
                      "##XFACTOR= 1.0\r\n"
                      "##YFACTOR= 1.0\r\n"
                      "##FIRSTX= 450\r\n"
                      "##LASTX= 451\r\n"
                      "##NPOINTS= 2\r\n"
                      "##FIRSTY= 10\r\n"
                      "##XYDATA= (X++(Y..Y))\r\n"
                      "450.0, 10.0\r\n"
                      "451.0, 11.0\r\n"
                      "$$ random comment #2\r\n"
                      "##END="
                      "$$ random comment #3\r\n"};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    auto block = sciformats::jdx::Block(stream);
    const auto& ldrs = block.getLdrs();

    // does NOT contain "##END=" even though technically an LDR
    // DOES contain "##XYDATA=" with its variable list as value
    REQUIRE(15 == ldrs.size());
    REQUIRE("Test" == block.getLdr("TITLE").value().getValue());
    REQUIRE(
        "Dummy" == block.getLdr("SPECTROMETERDATASYSTEM").value().getValue());
    REQUIRE(true == block.getXyData().has_value());
    REQUIRE(
        "Dummy" == block.getLdr("Spectrometer/DATA SYSTEM").value().getValue());
    REQUIRE(true == block.getXyData().has_value());
    auto data = block.getXyData().value();
    REQUIRE(2 == data.getData().size());
}

TEST_CASE("parses all LDRs in block with RADATA", "[Block]")
{
    std::string input{"##TITLE= Test\r\n"
                      "##JCAMP-DX= 4.24\r\n"
                      "##DATA TYPE= INFRARED INTERFEROGRAM\r\n"
                      "##ORIGIN= devrosch\r\n"
                      "##OWNER= PUBLIC DOMAIN\r\n"
                      "##RUNITS= MICROMETERS\r\n"
                      "##AUNITS= ARBITRARY UNITS\r\n"
                      "##RFACTOR= 1.0\r\n"
                      "##AFACTOR= 1.0\r\n"
                      "##FIRSTR= 0\r\n"
                      "##LASTR= 1\r\n"
                      "##NPOINTS= 2\r\n"
                      "##FIRSTA= 10\r\n"
                      "##RADATA= (R++(A..A))\r\n"
                      "0, 10.0\r\n"
                      "1, 11.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    auto block = sciformats::jdx::Block(stream);
    const auto& ldrs = block.getLdrs();

    // does NOT contain "##END=" even though technically an LDR
    // DOES contain "##RADATA=" with its variable list as value
    REQUIRE(14 == ldrs.size());
    REQUIRE("Test" == block.getLdr("TITLE").value().getValue());
    REQUIRE(block.getRaData().has_value());
    auto data = block.getRaData().value();
    REQUIRE(2 == data.getData().size());
}

TEST_CASE("parses block with XYPOINTS", "[Block]")
{
    std::string input{"##TITLE= Test\r\n"
                      "##JCAMP-DX= 4.24\r\n"
                      "##DATA TYPE= INFRARED SPECTRUM\r\n"
                      "##XUNITS= 1/CM\r\n"
                      "##YUNITS= ABSORBANCE\r\n"
                      "##XFACTOR= 1.0\r\n"
                      "##YFACTOR= 1.0\r\n"
                      "##FIRSTX= 450\r\n"
                      "##LASTX= 461\r\n"
                      "##NPOINTS= 4\r\n"
                      "##FIRSTY= 10\r\n"
                      "##XYPOINTS= (XY..XY)\r\n"
                      "450.0, 10.0; 451.0, 11.0\r\n"
                      "460.0, ?; 461.0, 21.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    auto block = sciformats::jdx::Block(stream);
    REQUIRE(block.getXyPoints().has_value());
}

TEST_CASE("parses block with PEAK TABLE", "[Block]")
{
    std::string input{"##TITLE= Test\r\n"
                      "##JCAMP-DX= 4.24\r\n"
                      "##PEAK TABLE= (XY..XY)\r\n"
                      "0, 10.0\r\n"
                      "1, 11.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    auto block = sciformats::jdx::Block(stream);
    const auto& ldrs = block.getLdrs();

    // does NOT contain "##END=" even though technically an LDR
    // DOES contain "##PEAKTABLE=" with its variable list as value
    REQUIRE(3 == ldrs.size());
    REQUIRE(block.getPeakTable().has_value());
    auto peakTable = block.getPeakTable().value();
    REQUIRE(2 == peakTable.getData().size());
}

TEST_CASE("parses block with PEAK ASSIGNMENTS", "[Block]")
{
    std::string input{"##TITLE= Test\r\n"
                      "##JCAMP-DX= 4.24\r\n"
                      "##PEAK ASSIGNMENTS= (XYA)\r\n"
                      "$$ peak width function\r\n"
                      "(1.0, 10.0, <peak assignment 1>)\r\n"
                      "(2.0, 20.0, <peak assignment 2> )\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    auto block = sciformats::jdx::Block(stream);
    const auto& ldrs = block.getLdrs();

    // does NOT contain "##END=" even though technically an LDR
    // DOES contain "##PEAKASSIGNMENTS=" with its variable list as value
    REQUIRE(3 == ldrs.size());
    REQUIRE(block.getPeakAssignments().has_value());
    auto peakAssignments = block.getPeakAssignments().value();
    REQUIRE(2 == peakAssignments.getData().size());
}

TEST_CASE("parses LINK block", "[Block]")
{
    std::string input{"##TITLE= Root LINK BLOCK\r\n"
                      "##JCAMP-DX= 4.24\r\n"
                      "##DATA TYPE= LINK\r\n"
                      "##BLOCKS= 3\r\n"
                      "##TITLE= Data XYDATA (PAC) Block\r\n"
                      "##JCAMP-DX= 4.24\r\n"
                      "##DATA TYPE= INFRARED SPECTRUM\r\n"
                      "##XUNITS= 1/CM\r\n"
                      "##YUNITS= ABSORBANCE\r\n"
                      "##XFACTOR= 1.0\r\n"
                      "##YFACTOR= 1.0\r\n"
                      "##FIRSTX= 450\r\n"
                      "##LASTX= 451\r\n"
                      "##NPOINTS= 2\r\n"
                      "##FIRSTY= 10\r\n"
                      "##XYDATA= (X++(Y..Y))\r\n"
                      "+450+10\r\n"
                      "+451+11\r\n"
                      "##END=\r\n"
                      "##TITLE= Data RADATA (PAC) Block\r\n"
                      "##JCAMP-DX= 4.24\r\n"
                      "##DATA TYPE= INFRARED INTERFEROGRAM\r\n"
                      "##RUNITS= MICROMETERS\r\n"
                      "##AUNITS= ARBITRARY UNITS\r\n"
                      "##FIRSTR= 0\r\n"
                      "##LASTR= 2\r\n"
                      "##RFACTOR= 1.0\r\n"
                      "##AFACTOR= 1.0\r\n"
                      "##NPOINTS= 3\r\n"
                      "##RADATA= (R++(A..A))\r\n"
                      "+0+10\r\n"
                      "+1+11\r\n"
                      "+2+12\r\n"
                      "##END=\r\n"
                      "$$ potentially problematic comment\r\n"
                      "##END=\r\n"};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    auto block = sciformats::jdx::Block(stream);
    const auto& ldrs = block.getLdrs();

    // does NOT contain "##END=" even though technically an LDR
    // does NOT contain nested block LDRs
    REQUIRE(4 == ldrs.size());
    REQUIRE("Root LINK BLOCK" == block.getLdr("TITLE").value().getValue());
    REQUIRE_FALSE(block.getXyData().has_value());
    REQUIRE_FALSE(block.getRaData().has_value());
    REQUIRE_FALSE(block.getXyPoints().has_value());
    REQUIRE_FALSE(block.getPeakTable().has_value());
    REQUIRE(2 == block.getBlocks().size());
}

TEST_CASE("throws if required LDRs for xy data are missing", "[Block]")
{
    std::string input{"##TITLE= Test\r\n"
                      "##JCAMP-DX= 4.24\r\n"
                      "##DATA TYPE= INFRARED SPECTRUM\r\n"
                      "##ORIGIN= devrosch\r\n"
                      "##OWNER= PUBLIC DOMAIN\r\n"
                      "##XUNITS= 1/CM\r\n"
                      "##YUNITS= ABSORBANCE\r\n"
                      "##XFACTOR= 1.0\r\n"
                      "##YFACTOR= 1.0\r\n"
                      // "##FIRSTX= 450\r\n" // required for XYDATA
                      "##LASTX= 451\r\n"
                      // "##NPOINTS= 2\r\n" // required for XYDATA
                      "##FIRSTY= 10\r\n"
                      "##XYDATA= (X++(Y..Y))\r\n"
                      "450.0, 10.0\r\n"
                      "451.0, 11.0\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    REQUIRE_THROWS_WITH(sciformats::jdx::Block(stream),
        Catch::Matchers::Contains("NPOINTS")
            && Catch::Matchers::Contains("FIRSTX"));
}

TEST_CASE("parses nested blocks", "[Block]")
{
    std::string input{"##TITLE= Test Link Block\r\n"
                      "##JCAMP-DX= 4.24\r\n"
                      "##DATA TYPE= LINK\r\n"
                      "##BLOCKS= 1\r\n"

                      "##TITLE= Test Nested Block\r\n"
                      "##JCAMP-DX= 4.24\r\n"
                      "##DATA TYPE= INFRARED SPECTRUM\r\n"
                      "##ORIGIN= devrosch\r\n"
                      "##OWNER= PUBLIC DOMAIN\r\n"
                      "##XUNITS= 1/CM\r\n"
                      "##YUNITS= ABSORBANCE\r\n"
                      "##XFACTOR= 1.0\r\n"
                      "##YFACTOR= 1.0\r\n"
                      "##FIRSTX= 450\r\n"
                      "##LASTX= 451\r\n"
                      "##NPOINTS= 2\r\n"
                      "##FIRSTY= 10\r\n"
                      "##XYPOINTS= (XY..XY)\r\n"
                      "450.0, 10.0\r\n"
                      "451.0, 11.0\r\n"
                      "##END=\r\n"

                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    auto block = sciformats::jdx::Block(stream);
    const auto& ldrs = block.getLdrs();
    const auto& innerBlocks = block.getBlocks();

    // does not contain "##END=" even though technically an LDR
    REQUIRE(4 == ldrs.size());
    REQUIRE("Test Link Block" == block.getLdr("TITLE").value().getValue());
    REQUIRE("LINK" == block.getLdr("DATATYPE").value().getValue());

    REQUIRE(1 == innerBlocks.size());
    auto innerBlock = innerBlocks.at(0);
    REQUIRE(
        "Test Nested Block" == innerBlock.getLdr("TITLE").value().getValue());
}

TEST_CASE("treats block comments different from other LDRs", "[Block]")
{
    std::string input{"##TITLE= Test Block\r\n"
                      "##= comment 1\r\n"
                      "##JCAMP-DX= 4.24\r\n"
                      "##= comment 2 line 1\r\n"
                      "comment 2 line 2\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    auto block = sciformats::jdx::Block(stream);
    const auto& ldrs = block.getLdrs();
    const auto& ldrComments = block.getLdrComments();

    REQUIRE(2 == ldrs.size());
    REQUIRE(2 == ldrComments.size());
    REQUIRE("comment 1" == ldrComments.at(0));
    REQUIRE("comment 2 line 1\ncomment 2 line 2" == ldrComments.at(1));
}

TEST_CASE("throws on illegal block start", "[Block]")
{
    std::string input{"##ILLEGAL_BLOCK_START= Test Block\r\n"
                      "##JCAMP-DX= 4.24\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    REQUIRE_THROWS(sciformats::jdx::Block(stream));
}

TEST_CASE("throws on duplicate LDRs in block", "[Block]")
{
    std::string input{"##TITLE= Test Block\r\n"
                      "##JCAMP-DX= 4.24\r\n"
                      "##JCAMP-DX= 5.00\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    REQUIRE_THROWS(sciformats::jdx::Block(stream));
}

TEST_CASE("throws on missing END LDR in block", "[Block]")
{
    std::string input{"##TITLE= Test Block\r\n"
                      "##JCAMP-DX= 5.00\r\n"};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    REQUIRE_THROWS(sciformats::jdx::Block(stream));
}
