#define CATCH_CONFIG_MAIN
#include "model/Node.hpp"
#include "stub/StubNode.hpp"

#include "catch2/catch.hpp"

TEST_CASE("parses all LDRs in block with XYDATA", "[Block]")
{
    using namespace sciformats::sciwrap::stub;
    StubNode node{};

    REQUIRE("A Node" == node.getName());

    auto childNodes = node.getChildNodes();
    REQUIRE(childNodes.size() == 3);
    REQUIRE("A Node" == childNodes.at(0)->getName());
}
