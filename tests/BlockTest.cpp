#include "jdx/Block.hpp"

#include "catch2/catch.hpp"

TEST_CASE("parses all LDRs in block with XYDATA", "[Block]")
{
    std::string input{"##TITLE= Test\r\n"
                      "##JCAMP-DX= 4.24\r\n"
                      "##DATA TYPE= INFRARED SPECTRUM\r\n"
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
                      "##END="};
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
    REQUIRE(true == block.getRaData().has_value());
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
