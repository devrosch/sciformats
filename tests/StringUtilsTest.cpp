#include "util/StringUtils.hpp"

#include "catch2/catch.hpp"

#include <iostream>
#include <sstream>
#include <string>

TEST_CASE("left trims white space", "[util][trimLeft]")
{
    std::string actual{"\t\n\v\f\r abc \t\n\v\f\r"};
    std::string expect{"abc \t\n\v\f\r"};

    sciformats::jdx::util::trimLeft(actual);

    REQUIRE(expect == actual);
}

TEST_CASE("right trims white space", "[util][trimRight]")
{
    std::string actual{"\t\n\v\f\r abc \t\n\v\f\r"};
    std::string expect{"\t\n\v\f\r abc"};

    sciformats::jdx::util::trimRight(actual);

    REQUIRE(expect == actual);
}

TEST_CASE("trims white space", "[util][trim]")
{
    std::string actual{"\t\n\v\f\r abc \t\n\v\f\r"};
    std::string expect{"abc"};

    sciformats::jdx::util::trim(actual);

    REQUIRE(expect == actual);
}

TEST_CASE("white spaces recognized", "[util][isSpace]")
{
    std::string actual{" \t\n\v\f\raA\x80\xFF"};

    REQUIRE(sciformats::jdx::util::isSpace(actual.at(0)));
    REQUIRE(sciformats::jdx::util::isSpace(actual.at(1)));
    REQUIRE(sciformats::jdx::util::isSpace(actual.at(2)));
    REQUIRE(sciformats::jdx::util::isSpace(actual.at(3)));
    REQUIRE(sciformats::jdx::util::isSpace(actual.at(4)));
    REQUIRE(sciformats::jdx::util::isSpace(actual.at(5)));
    REQUIRE_FALSE(sciformats::jdx::util::isSpace(actual.at(6)));
    REQUIRE_FALSE(sciformats::jdx::util::isSpace(actual.at(7)));
    REQUIRE_FALSE(sciformats::jdx::util::isSpace(actual.at(8)));
    REQUIRE_FALSE(sciformats::jdx::util::isSpace(actual.at(9)));
}

TEST_CASE("lower case letters for ASCII produced", "[util][toLower]")
{
    std::string actual{"\t\n\v\f\raAzZ%?"};
    const std::string expected{"\t\n\v\f\raazz%?"};
    // does not work for non ASCII characters such as umlauts
    sciformats::jdx::util::toLower(actual);

    REQUIRE(expected == actual);
}

TEST_CASE("splits string without trimming", "[util][split]")
{
    const std::string fixture{"  this, is , a ,test   "};
    const std::vector<std::string> expected{"  this", " is ", " a ", "test   "};
    const auto output = sciformats::jdx::util::split(fixture, ",");

    REQUIRE(expected == output);
}

TEST_CASE("splits string with trimming", "[util][split]")
{
    const std::string fixture{"  this, is , a ,test   "};
    const std::vector<std::string> expected{"this", "is", "a", "test"};
    const auto output = sciformats::jdx::util::split(fixture, ",", true);

    REQUIRE(expected == output);
}

TEST_CASE("split retains trailing empty segment", "[util][split]")
{
    const std::string fixture{", is, a,"};
    const std::vector<std::string> expected{"", "is", "a", ""};
    const auto output = sciformats::jdx::util::split(fixture, ",", true);

    REQUIRE(expected == output);
}

TEST_CASE("split returns full string if no delimiter present", "[util][split]")
{
    const std::string fixture{" test "};
    const std::vector<std::string> expected{" test "};
    const auto output = sciformats::jdx::util::split(fixture, ",", false);

    REQUIRE(expected == output);
}
