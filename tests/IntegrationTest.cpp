#include "jdx/JdxParser.hpp"
#include "jdx/XyData.hpp"

#include "catch2/catch.hpp"

#include <fstream>
#include <sstream>

TEST_CASE("parses XyData from actual sample file", "[IntegrationTest][XyData]")
{
    const std::string path{"resources/Claniline.jdx"};
    std::ifstream istream{path};

    auto block = sciformats::jdx::JdxParser::parse(istream);

    REQUIRE(block.getBlocks().size() == 6);

    auto nestedBlock0 = block.getBlocks().at(0);
    REQUIRE(nestedBlock0.getXyData());

    auto xyData = nestedBlock0.getXyData().value();
    auto data = xyData.getData();
    REQUIRE(data.size() == 1801);
}
