#include "jdx/JdxLdrParser.hpp"

#include "catch2/catch.hpp"

#include <sstream>
#include <string>

TEST_CASE("reads two lines with \\n endlines", "[JdxLdrParser]")
{
    std::string input{"abc\ndef\n"};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    std::string line0 = sciformats::jdx::JdxLdrParser::readLine(stream);
    std::string line1 = sciformats::jdx::JdxLdrParser::readLine(stream);

    REQUIRE(std::string{"abc"} == line0);
    REQUIRE(std::string{"def"} == line1);
}

TEST_CASE("reads two lines with \\r\\n endlines", "[JdxLdrParser]")
{
    std::string input{"abc\r\ndef\r\n"};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    std::string line0 = sciformats::jdx::JdxLdrParser::readLine(stream);
    std::string line1 = sciformats::jdx::JdxLdrParser::readLine(stream);

    REQUIRE(std::string{"abc"} == line0);
    REQUIRE(std::string{"def"} == line1);
}

TEST_CASE("reads line ending with EOF", "[JdxLdrParser]")
{
    std::string input{"abc"};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    std::string line0 = sciformats::jdx::JdxLdrParser::readLine(stream);

    REQUIRE(std::string{"abc"} == line0);
}

TEST_CASE("throws when trying to read past end", "[JdxLdrParser]")
{
    std::string input{};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    REQUIRE_THROWS(sciformats::jdx::JdxLdrParser::readLine(stream));
}

TEST_CASE("left trims white space", "[JdxLdrParser]")
{
    std::string actual{"\t\n\v\f\r abc \t\n\v\f\r"};
    std::string expect{"abc \t\n\v\f\r"};

    sciformats::jdx::JdxLdrParser::trimLeft(actual);

    REQUIRE(expect == actual);
}

TEST_CASE("right trims white space", "[JdxLdrParser]")
{
    std::string actual{"\t\n\v\f\r abc \t\n\v\f\r"};
    std::string expect{"\t\n\v\f\r abc"};

    sciformats::jdx::JdxLdrParser::trimRight(actual);

    REQUIRE(expect == actual);
}

TEST_CASE("trims white space", "[JdxLdrParser]")
{
    std::string actual{"\t\n\v\f\r abc \t\n\v\f\r"};
    std::string expect{"abc"};

    sciformats::jdx::JdxLdrParser::trim(actual);

    REQUIRE(expect == actual);
}
