#include "stub/StubNode.hpp"
#include "model/KeyValueParam.hpp"
#include "model/Node.hpp"

#include "catch2/catch.hpp"

TEST_CASE("StubNode returns dummy data", "[StubNode]")
{
    using namespace sciformats::sciwrap::stub;
    StubNode node{};

    REQUIRE("A Stub Node" == node.getName());

    auto params = node.getParams();
    REQUIRE(params.size() == 3);
    REQUIRE("key0" == params.at(0).key);
    REQUIRE("value0" == params.at(0).value);

    auto data = node.getData();
    REQUIRE(data->size() == 3);
    auto point0 = data->at(0);
    REQUIRE(point0.x == Approx(1.0));
    REQUIRE(point0.y == Approx(10.0));

    auto childNodes = node.getChildNodes();
    REQUIRE(childNodes.size() == 3);
    REQUIRE("A Stub Node" == childNodes.at(0)->getName());
}
