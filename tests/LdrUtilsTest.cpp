#include "util/LdrUtils.hpp"

#include "catch2/catch.hpp"

#include <iostream>
#include <sstream>
#include <string>

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

TEST_CASE("trims content and comment if indicated", "[util][stripLineComment]")
{
    std::string input{" content $$ comment "};
    auto [contentFull0, commentFull0]
        = sciformats::jdx::util::stripLineComment(input, false, false);
    auto [contentFull1, commentTrimmed1]
        = sciformats::jdx::util::stripLineComment(input, false, true);
    auto [contentTrimmed2, commentFull2]
        = sciformats::jdx::util::stripLineComment(input, true, false);
    auto [contentTrimmed3, commentTrimmed3]
        = sciformats::jdx::util::stripLineComment(input, true, true);

    REQUIRE(" content " == contentFull0);
    REQUIRE(" comment " == commentFull0.value());
    REQUIRE(" content " == contentFull1);
    REQUIRE("comment" == commentTrimmed1.value());
    REQUIRE("content" == contentTrimmed2);
    REQUIRE(" comment " == commentFull2.value());
    REQUIRE("content" == contentTrimmed3);
    REQUIRE("comment" == commentTrimmed3.value());
}
