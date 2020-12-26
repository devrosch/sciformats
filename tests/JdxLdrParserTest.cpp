#include "jdx/JdxLdrParser.hpp"

#include "catch2/catch.hpp"

#include <iostream>
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

TEST_CASE("recognizes regular LDR start", "[JdxLdrParser]")
{
    std::string input{"##TITLE= abc"};

    REQUIRE(true == sciformats::jdx::JdxLdrParser::isLdrStart(input));
}

TEST_CASE("recognizes LDR start with leading white spaces", "[JdxLdrParser]")
{
    std::string input{"\t\n\v\f\r ##TITLE= abc"};

    REQUIRE(true == sciformats::jdx::JdxLdrParser::isLdrStart(input));
}

TEST_CASE("recognizes LDR start with labels containing special characters and "
          "numbers",
    "[JdxLdrParser]")
{
    std::string input{"##.N_A/M2E$= abc"};

    REQUIRE(true == sciformats::jdx::JdxLdrParser::isLdrStart(input));
}

TEST_CASE("rejects non LDR start", "[JdxLdrParser]")
{
    std::string input{"#NAME= ##NOT_LDR=abc"};

    REQUIRE(false == sciformats::jdx::JdxLdrParser::isLdrStart(input));
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

TEST_CASE("normalize LDR label removes \" -/_\" from label", "[JdxLdrParser]")
{
    std::string input{"##A B-C/D_E= abc"};
    std::string expect{"##ABCDE= abc"};

    std::string actual
        = sciformats::jdx::JdxLdrParser::normalizeLdrLabel(input);

    REQUIRE(expect == actual);
}

TEST_CASE(
    "normalize LDR label leaves normalized label intact", "[JdxLdrParser]")
{
    std::string input{"##ABCDE= abc"};
    std::string expect{"##ABCDE= abc"};

    std::string actual
        = sciformats::jdx::JdxLdrParser::normalizeLdrLabel(input);

    REQUIRE(expect == actual);
}

TEST_CASE("normalize LDR label removes leading white spaces", "[JdxLdrParser]")
{
    std::string input{"\t\n\v\f\r ##ABCDE= abc"};
    std::string expect{"##ABCDE= abc"};

    std::string actual
        = sciformats::jdx::JdxLdrParser::normalizeLdrLabel(input);

    REQUIRE(expect == actual);
}

TEST_CASE("normalize LDR label turns (only) ASCII letters to upper case",
    "[JdxLdrParser]")
{
    // label: abcdeäöüÄÖÜ in ISO-8859-1 encoding
    std::string input{"##abcde\xE4\xF6\xFC\xC4\xD6\xDC= abc"};
    std::string expect{"##ABCDE\xE4\xF6\xFC\xC4\xD6\xDC= abc"};

    std::string actual
        = sciformats::jdx::JdxLdrParser::normalizeLdrLabel(input);

    REQUIRE(expect == actual);
}

TEST_CASE("rejects missing double hashes in LDR start", "[JdxLdrParser]")
{
    std::string input{"#LABEL= abc"};

    REQUIRE_THROWS(sciformats::jdx::JdxLdrParser::normalizeLdrLabel(input));
}

TEST_CASE("rejects missing enquals in LDR start", "[JdxLdrParser]")
{
    std::string input{"##LABEL abc"};

    REQUIRE_THROWS(sciformats::jdx::JdxLdrParser::normalizeLdrLabel(input));
}
