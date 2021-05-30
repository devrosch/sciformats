#include "jdx/JdxFileParser.hpp"
#include "model/Node.hpp"

#include "catch2/catch.hpp"

TEST_CASE(
    "JdxFileParser only accepts to parse valid JCAMP-DX", "[JdxFileParser]")
{
    using namespace sciformats::sciwrap::model;
    using namespace sciformats::sciwrap::jdx;
    JdxFileParser parser{};

    SECTION("Only recognizes existing file that contain JCAMP-DX data")
    {
        REQUIRE_FALSE(parser.isRecognized("resources/dummy.txt"));
        REQUIRE_FALSE(parser.isRecognized("resources/non_existent.txt"));
        REQUIRE(parser.isRecognized("resources/Claniline.jdx"));
    }

    SECTION("Parses valid JCAMP-DX file")
    {
        auto nodePtr = parser.parse("resources/Claniline.jdx");

        REQUIRE(nodePtr != nullptr);
        REQUIRE("Compound file, contains several data records"
                == nodePtr->getName());

        SECTION("Parses root level parameters")
        {
            REQUIRE(nodePtr->getParams().size() == 6);
        }

        SECTION("Parses nested nodes")
        {
            REQUIRE(nodePtr->getChildNodes().size() == 6);
        }
    }

    SECTION("Throws when trying to parse invalid or non-existing JCAMP-DX file")
    {
        REQUIRE_THROWS(parser.parse("resources/dummy.txt"));
        REQUIRE_THROWS(parser.parse("resources/non_existent.txt"));
    }
}
