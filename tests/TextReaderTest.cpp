#include "jdx/TextReader.hpp"

#include "catch2/catch.hpp"

#include <memory>
#include <sstream>
#include <string>

TEST_CASE("reads file specified by path", "[TextReader]")
{
    const std::string path{"resources/dummy.txt"};
    sciformats::jdx::TextReader reader{path};

    REQUIRE(0 == reader.tellg());
    REQUIRE(20 == reader.getLength());
    REQUIRE(0 == reader.tellg());
    REQUIRE_FALSE(reader.eof());
    REQUIRE("not a JCAMP-DX file" == reader.readLine());
    REQUIRE(reader.eof());
    REQUIRE(20 == reader.tellg());
    reader.seekg(1);
    REQUIRE(1 == reader.tellg());
}

TEST_CASE("reads data provided by an istream", "[TextReader]")
{
    std::string input{"line 1\r\n"
                      "line 2\n"
                      "line 3"};
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};

    REQUIRE(0 == reader.tellg());
    REQUIRE(21 == reader.getLength());
    REQUIRE(0 == reader.tellg());
    REQUIRE_FALSE(reader.eof());
    REQUIRE("line 1" == reader.readLine());
    REQUIRE_FALSE(reader.eof());
    REQUIRE(8 == reader.tellg());
    REQUIRE("line 2" == reader.readLine());
    REQUIRE_FALSE(reader.eof());
    REQUIRE(15 == reader.tellg());
    REQUIRE("line 3" == reader.readLine());
    REQUIRE(reader.eof());
    REQUIRE(21 == reader.tellg());
    reader.seekg(1);
    REQUIRE(1 == reader.tellg());
    reader.seekg(21);
    REQUIRE(21 == reader.tellg());
}
