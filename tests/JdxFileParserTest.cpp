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
        REQUIRE(parser.isRecognized("resources/CompoundFile.jdx"));
    }

    SECTION("Parses valid JCAMP-DX file")
    {
        auto nodePtr = parser.parse("resources/CompoundFile.jdx");

        REQUIRE(nodePtr != nullptr);
        REQUIRE("Root LINK BLOCK" == nodePtr->getName());
        REQUIRE(nodePtr->getParams().size() == 4);
        REQUIRE(nodePtr->getData() == std::nullopt);
        REQUIRE(nodePtr->getChildNodes().size() == 4);

        SECTION("Parses nested XYDATA node")
        {
            auto nestedNode0 = nodePtr->getChildNodes().at(0);
            REQUIRE("Data XYDATA (PAC) Block" == nestedNode0->getName());
            REQUIRE(nestedNode0->getParams().size() == 12);
            REQUIRE(nestedNode0->getChildNodes().size() == 1);

            auto xyDataNode = nestedNode0->getChildNodes().at(0);
            REQUIRE(xyDataNode->getName() == "XYDATA");
            REQUIRE(xyDataNode->getParams().empty());
            REQUIRE(xyDataNode->getChildNodes().empty());

            auto dataOpt = xyDataNode->getData();
            REQUIRE(dataOpt.has_value());

            auto data = dataOpt.value();
            REQUIRE(data.size() == 2);
            REQUIRE(data.at(0).x == Approx(450.0));
            REQUIRE(data.at(0).y == Approx(10.0));
        }

        SECTION("Parses nested RADATA node")
        {
            auto nestedNode1 = nodePtr->getChildNodes().at(1);
            REQUIRE("Data RADATA (PAC) Block" == nestedNode1->getName());
            REQUIRE(nestedNode1->getParams().size() == 11);
            REQUIRE(nestedNode1->getChildNodes().size() == 1);

            auto raDataNode = nestedNode1->getChildNodes().at(0);
            REQUIRE(raDataNode->getName() == "RADATA");
            REQUIRE(raDataNode->getParams().empty());
            REQUIRE(raDataNode->getChildNodes().empty());

            auto dataOpt = raDataNode->getData();
            REQUIRE(dataOpt.has_value());

            auto data = dataOpt.value();
            REQUIRE(data.size() == 3);
            REQUIRE(data.at(0).x == Approx(0.0));
            REQUIRE(data.at(0).y == Approx(10.0));
        }

        SECTION("Parses nested XYPOINTS node")
        {
            auto nestedNode2 = nodePtr->getChildNodes().at(2);
            REQUIRE("Data XYPOINTS (AFFN) Block" == nestedNode2->getName());
            REQUIRE(nestedNode2->getParams().size() == 11);
            REQUIRE(nestedNode2->getChildNodes().size() == 1);

            auto xyPointsNode = nestedNode2->getChildNodes().at(0);
            REQUIRE(xyPointsNode->getName() == "XYPOINTS");
            REQUIRE(xyPointsNode->getParams().empty());
            REQUIRE(xyPointsNode->getChildNodes().empty());

            auto dataOpt = xyPointsNode->getData();
            REQUIRE(dataOpt.has_value());

            auto data = dataOpt.value();
            REQUIRE(data.size() == 4);
            REQUIRE(data.at(0).x == Approx(900.0));
            REQUIRE(data.at(0).y == Approx(100.0));
        }
    }

    SECTION("Throws when trying to parse invalid or non-existing JCAMP-DX file")
    {
        REQUIRE_THROWS(parser.parse("resources/dummy.txt"));
        REQUIRE_THROWS(parser.parse("resources/non_existent.txt"));
    }
}
