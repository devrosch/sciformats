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

        SECTION("Root level block contains no data LDRs")
        {
            REQUIRE(nodePtr->getData() == std::nullopt);
        }

        SECTION("Parses nested nodes")
        {
            REQUIRE(nodePtr->getChildNodes().size() == 6);

            auto nestedNode0 = nodePtr->getChildNodes().at(0);
            REQUIRE("4-chloroaniline" == nestedNode0->getName());
            REQUIRE(nestedNode0->getParams().size() == 30);
            REQUIRE(nestedNode0->getChildNodes().size() == 1);

            auto xyDataNode = nestedNode0->getChildNodes().at(0);
            REQUIRE(xyDataNode->getName() == "XYDATA");
            REQUIRE(xyDataNode->getParams().empty());

            auto dataOpt = xyDataNode->getData();
            REQUIRE(dataOpt.has_value());

            auto data = dataOpt.value();
            REQUIRE(data.size() == 1801);
            REQUIRE(data.at(0).x == Approx(111111 * 3.6E-3));
            REQUIRE(data.at(0).y == Approx(864977 * 8.11943557E-7));
        }
    }

    SECTION("Throws when trying to parse invalid or non-existing JCAMP-DX file")
    {
        REQUIRE_THROWS(parser.parse("resources/dummy.txt"));
        REQUIRE_THROWS(parser.parse("resources/non_existent.txt"));
    }
}
