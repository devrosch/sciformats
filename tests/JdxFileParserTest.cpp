#include "model/Node.hpp"
#include "jdx/JdxFileParser.hpp"

#include "catch2/catch.hpp"

TEST_CASE("JdxFileParser rejects invalid or non existent JCAMP-DX and parses valid JCAMP-DX file", "[JdxFileParser]")
{
    using namespace sciformats::sciwrap::model;
    using namespace sciformats::sciwrap::jdx;

    JdxFileParser parser{};

    REQUIRE_FALSE(parser.isRecognized("resources/dummy.txt"));
    REQUIRE_FALSE(parser.isRecognized("resources/non_existent.txt"));
    REQUIRE(parser.isRecognized("resources/Claniline.jdx"));

    auto nodePtr = parser.parse("resources/Claniline.jdx");

    REQUIRE(nodePtr != nullptr);
    REQUIRE("Compound file, contains several data records" == nodePtr->getName());
    REQUIRE(nodePtr->getParams().size() == 6);
}
