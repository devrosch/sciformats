#include "jdx/JdxParser.hpp"
#include "jdx/XyData.hpp"

#include "catch2/catch.hpp"

#include <fstream>
#include <sstream>

TEST_CASE("parses XyData from actual sample file", "[IntegrationTest][XyData]")
{
    const std::string path{"resources/Claniline.jdx"};
    auto istream = std::make_unique<std::ifstream>(path);

    auto block = sciformats::jdx::JdxParser::parse(std::move(istream));

    REQUIRE(block.getBlocks().size() == 6);

    const auto& nestedBlock0 = block.getBlocks().at(0);
    REQUIRE(nestedBlock0.getXyData());

    auto xyData = nestedBlock0.getXyData().value();
    auto data = xyData.getData();
    REQUIRE(data.size() == 1801);
}
