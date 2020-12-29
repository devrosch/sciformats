#include "jdx/JdxBlock.hpp"

#include "catch2/catch.hpp"

TEST_CASE("parses all LDRs in block", "[JdxBlock]")
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
                      "##FIRSTX= 450\r\n"
                      "##LASTX= 451\r\n"
                      "##NPOINTS= 2\r\n"
                      "##FIRSTY= 10\r\n"
                      "##XYPOINTS= (XY..XY)\r\n"
                      "450.0, 10.0"
                      "451.0, 11.0"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    auto block = sciformats::jdx::JdxBlock(stream);
    const auto& ldrs = block.getLdrs();

    // does not contain "##END=" even though technically an LDR
    REQUIRE(14 == ldrs.size());
    REQUIRE("Test" == ldrs.at("TITLE"));
}

TEST_CASE("parses nested blocks", "[JdxBlock]")
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
                      "450.0, 10.0"
                      "451.0, 11.0"
                      "##END="

                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    auto block = sciformats::jdx::JdxBlock(stream);
    const auto& ldrs = block.getLdrs();
    const auto& innerBlocks = block.getBlocks();

    // does not contain "##END=" even though technically an LDR
    REQUIRE(4 == ldrs.size());
    REQUIRE("Test Link Block" == ldrs.at("TITLE"));
    REQUIRE("LINK" == ldrs.at("DATATYPE"));

    REQUIRE(1 == innerBlocks.size());
    auto innerBlock = innerBlocks.at(0);
    REQUIRE("Test Nested Block" == innerBlock.getLdrs().at("TITLE"));
}

TEST_CASE("treats block comments different from other LDRs", "[JdxBlock]")
{
    std::string input{"##TITLE= Test Block\r\n"
                      "##= comment 1\r\n"
                      "##JCAMP-DX= 4.24\r\n"
                      "##= comment 2 line 1\r\n"
                      "comment 2 line 2\r\n"
                      "##END="};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    auto block = sciformats::jdx::JdxBlock(stream);
    const auto& ldrs = block.getLdrs();
    const auto& ldrComments = block.getLdrComments();

    REQUIRE(2 == ldrs.size());
    REQUIRE(2 == ldrComments.size());
    REQUIRE("comment 1" == ldrComments.at(0));
    REQUIRE("comment 2 line 1\ncomment 2 line 2" == ldrComments.at(1));
}
