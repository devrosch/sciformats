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
