#define CATCH_CONFIG_MAIN
#include "stub/StubFileParser.hpp"
#include "model/Node.hpp"

#include "catch2/catch.hpp"

TEST_CASE("StubFileParser returns dummy Node", "[StubFileParser]")
{
    using namespace sciformats::sciwrap::model;
    using namespace sciformats::sciwrap::stub;

    StubFileParser parser{};

    REQUIRE(parser.isRecognized("resources/dummy.txt"));
    REQUIRE_FALSE(parser.isRecognized("resources/non_existent.txt"));

    auto nodePtr = parser.parse("resources/dummy.txt");

    REQUIRE(nodePtr != nullptr);
    REQUIRE("A Stub Node" == nodePtr->getName());
    REQUIRE(nodePtr->getParams().size() == 3);
}
