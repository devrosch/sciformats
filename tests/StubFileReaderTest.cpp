#define CATCH_CONFIG_MAIN
#include "model/Node.hpp"
#include "stub/StubFileReader.hpp"

#include "catch2/catch.hpp"

TEST_CASE("StubFileReader returns dummy Node", "[StubFileReader]")
{
    using namespace sciformats::sciwrap::model;
    using namespace sciformats::sciwrap::stub;

    StubFileReader reader{};

    REQUIRE(reader.isResponsible("resources/dummy.txt"));
    REQUIRE_FALSE(reader.isResponsible("resources/non_existent.txt"));

    auto nodePtr = reader.read("resources/dummy.txt");

    REQUIRE(nodePtr != nullptr);
    REQUIRE("A Stub Node" == nodePtr->getName());
    REQUIRE(nodePtr->getParams().size() == 3);
}
