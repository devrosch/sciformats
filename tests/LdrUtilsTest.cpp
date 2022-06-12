#include "util/LdrUtils.hpp"

#include "catch2/catch.hpp"

#include <iostream>
#include <sstream>
#include <string>

TEST_CASE("reads two lines with \\n endlines", "[util][readLine]")
{
    std::string input{"abc\ndef\n"};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    std::string line0 = sciformats::jdx::util::readLine(stream);
    std::string line1 = sciformats::jdx::util::readLine(stream);

    REQUIRE(std::string{"abc"} == line0);
    REQUIRE(std::string{"def"} == line1);
}

TEST_CASE("reads two lines with \\r\\n endlines", "[util][readLine]")
{
    std::string input{"abc\r\ndef\r\n"};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    std::string line0 = sciformats::jdx::util::readLine(stream);
    std::string line1 = sciformats::jdx::util::readLine(stream);

    REQUIRE(std::string{"abc"} == line0);
    REQUIRE(std::string{"def"} == line1);
}

TEST_CASE("reads line ending with EOF", "[util][readLine]")
{
    std::string input{"abc"};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);
    // the underlying getline() method sets failbit at end of file, so do not
    // set std::ios::eofbit
    stream.exceptions(std::ios::failbit | std::ios::badbit);

    std::string line0 = sciformats::jdx::util::readLine(stream);

    REQUIRE(std::string{"abc"} == line0);
}

TEST_CASE("throws when trying to read past end", "[util][readLine]")
{
    std::string input{};
    std::stringstream stream{std::ios_base::in};
    stream.str(input);

    REQUIRE_THROWS(sciformats::jdx::util::readLine(stream));
}

TEST_CASE("recognizes regular LDR start", "[util][isLdrStart]")
{
    std::string input{"##TITLE= abc"};

    REQUIRE(true == sciformats::jdx::util::isLdrStart(input));
}

TEST_CASE(
    "recognizes LDR start with leading white spaces", "[util][isLdrStart]")
{
    std::string input{"\t\n\v\f\r ##TITLE= abc"};

    REQUIRE(true == sciformats::jdx::util::isLdrStart(input));
}

TEST_CASE("recognizes LDR start with labels containing special characters and "
          "numbers",
    "[util][isLdrStart]")
{
    std::string input{"##.N_A/M2E$= abc"};

    REQUIRE(true == sciformats::jdx::util::isLdrStart(input));
}

TEST_CASE("rejects non LDR start", "[util][isLdrStart]")
{
    std::string input{"#NAME= ##NOT_LDR=abc"};

    REQUIRE(false == sciformats::jdx::util::isLdrStart(input));
}

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

TEST_CASE("normalize LDR start removes \" -/_\" from label",
    "[util][normalizeLdrLabel]")
{
    std::string input{"##A B-C/D_E= abc"};
    std::string expect{"##ABCDE= abc"};

    std::string actual = sciformats::jdx::util::normalizeLdrStart(input);

    REQUIRE(expect == actual);
}

TEST_CASE("normalize LDR start leaves normalized label intact",
    "[util][normalizeLdrLabel]")
{
    std::string input{"##ABCDE= abc"};
    std::string expect{"##ABCDE= abc"};

    std::string actual = sciformats::jdx::util::normalizeLdrStart(input);

    REQUIRE(expect == actual);
}

TEST_CASE("normalize LDR start removes leading white spaces",
    "[util][normalizeLdrLabel]")
{
    std::string input{"\t\n\v\f\r ##ABCDE= abc"};
    std::string expect{"##ABCDE= abc"};

    std::string actual = sciformats::jdx::util::normalizeLdrStart(input);

    REQUIRE(expect == actual);
}

TEST_CASE("normalize LDR start turns (only) ASCII letters to upper case",
    "[util][normalizeLdrLabel]")
{
    // label: abcdeäöüÄÖÜ in ISO-8859-1 encoding
    std::string input{"##abcde\xE4\xF6\xFC\xC4\xD6\xDC= abc"};
    std::string expect{"##ABCDE\xE4\xF6\xFC\xC4\xD6\xDC= abc"};

    std::string actual = sciformats::jdx::util::normalizeLdrStart(input);

    REQUIRE(expect == actual);
}

TEST_CASE(
    "rejects missing double hashes in LDR start", "[util][normalizeLdrLabel]")
{
    std::string input{"#LABEL= abc"};

    REQUIRE_THROWS(sciformats::jdx::util::normalizeLdrStart(input));
}

TEST_CASE("rejects missing equals in LDR start", "[util][normalizeLdrLabel]")
{
    std::string input{"##LABEL abc"};

    REQUIRE_THROWS(sciformats::jdx::util::normalizeLdrStart(input));
}

TEST_CASE("tokenizes regular LDR start", "[util][parseLdrStart]")
{
    std::string input{"##LABEL=abc"};

    auto [label, value] = sciformats::jdx::util::parseLdrStart(input);

    REQUIRE("LABEL" == label);
    REQUIRE("abc" == value);
}

TEST_CASE("tokenizes LDR start with missing value", "[util][parseLdrStart]")
{
    std::string input{"##LABEL="};

    auto [label, value] = sciformats::jdx::util::parseLdrStart(input);

    REQUIRE("LABEL" == label);
    REQUIRE(value.empty());
}

TEST_CASE("removes (only) first leading space LDR start value",
    "[util][parseLdrStart]")
{
    std::string input{"##LABEL=  abc"};

    auto [label, value] = sciformats::jdx::util::parseLdrStart(input);

    REQUIRE("LABEL" == label);
    REQUIRE(" abc" == value);
}

TEST_CASE("normalizes LDR start label", "[util][parseLdrStart]")
{
    std::string input{"\t\n\v\f\r ##abcde\xE4\xF6\xFC\xC4\xD6\xDC="};

    auto [label, value] = sciformats::jdx::util::parseLdrStart(input);

    REQUIRE("ABCDE\xE4\xF6\xFC\xC4\xD6\xDC" == label);
}

TEST_CASE("rejects malformed LDR start (missing hash)", "[util][parseLdrStart]")
{
    std::string input{"#LABEL="};

    REQUIRE_THROWS(sciformats::jdx::util::parseLdrStart(input));
}

TEST_CASE(
    "rejects malformed LDR start (missing equals)", "[util][parseLdrStart]")
{
    std::string input{"##LABEL"};

    REQUIRE_THROWS(sciformats::jdx::util::parseLdrStart(input));
}

TEST_CASE("strips line comment", "[util][stripLineComment]")
{
    std::string input{"line start $$ comment"};
    auto [content, comment] = sciformats::jdx::util::stripLineComment(input);

    REQUIRE("line start " == content);
    REQUIRE(comment.has_value());
    REQUIRE(" comment" == comment);
}

TEST_CASE("indicates missing comment with nullopt", "[util][stripLineComment]")
{
    std::string input{"line content"};
    auto [content, comment] = sciformats::jdx::util::stripLineComment(input);

    REQUIRE("line content" == content);
    REQUIRE(!comment.has_value());
}

TEST_CASE("indicates empty content if whole line is comment",
    "[util][stripLineComment]")
{
    std::string input{"$$line comment"};
    auto [content, comment] = sciformats::jdx::util::stripLineComment(input);

    REQUIRE(content.empty());
    REQUIRE(comment.has_value());
    REQUIRE("line comment" == comment);
}

TEST_CASE(
    "indicates empty comment with empty string", "[util][stripLineComment]")
{
    std::string input{"line content$$"};
    auto [content, comment] = sciformats::jdx::util::stripLineComment(input);

    REQUIRE(!content.empty());
    REQUIRE("line content" == content);
    REQUIRE(comment.has_value());
    REQUIRE(comment.value().empty());
}
