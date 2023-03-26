#include "jdx/AuditTrail.hpp"

#include "catch2/catch.hpp"

#include <sstream>

TEST_CASE("parses well-formed 5 parameters audit trail", "[AuditTrail]")
{
    auto nextLine = std::optional<std::string>{
        "##AUDIT TRAIL=  $$ (NUMBER, WHEN, WHO, WHERE, WHAT)"};
    const auto* label = "AUDITTRAIL";
    const auto* variables = " $$ (NUMBER, WHEN, WHO, WHERE, WHAT)";
    // clang-format off
    std::string input{"(   1,<2022-09-01 09:10:11.123 -0200>,<testuser>,<location01>,\n"
                      "      <acquisition>)\n"
                      "(   2,<2022-09-01 19:10:12.123 -0200>,<testuser>,<location01>,\n"
                      "      <raw data processing\n"
                      "       line 2\n"
                      "       line 3>)\n"
                      "##END=\n"
                     };
    // clang-format on
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};

    auto auditTrail
        = sciformats::jdx::AuditTrail(label, variables, reader, nextLine);
    auto entries = auditTrail.getData();

    REQUIRE(2 == entries.size());
    auto entry1 = entries.at(0);
    REQUIRE(1 == entry1.number);
    REQUIRE("2022-09-01 09:10:11.123 -0200" == entry1.when);
    REQUIRE("testuser" == entry1.who);
    REQUIRE("location01" == entry1.where);
    REQUIRE_FALSE(entry1.process.has_value());
    REQUIRE_FALSE(entry1.version.has_value());
    REQUIRE("acquisition" == entry1.what);
    auto entry2 = entries.at(1);
    REQUIRE(2 == entry2.number);
    REQUIRE("2022-09-01 19:10:12.123 -0200" == entry2.when);
    REQUIRE("testuser" == entry2.who);
    REQUIRE("location01" == entry2.where);
    REQUIRE_FALSE(entry2.process.has_value());
    REQUIRE_FALSE(entry2.version.has_value());
    REQUIRE("raw data processing\nline 2\nline 3" == entry2.what);
}

TEST_CASE("parses well-formed 6 parameters audit trail", "[AuditTrail]")
{
    auto nextLine = std::optional<std::string>{
        "##AUDIT TRAIL= $$ (NUMBER, WHEN, WHO, WHERE, VERSION, WHAT)"};
    const auto* label = "AUDITTRAIL";
    const auto* variables = "$$ (NUMBER, WHEN, WHO, WHERE, VERSION, WHAT)";
    // clang-format off
    std::string input{"(   1,<2022-09-01 09:10:11.123 -0200>,<testuser>,<location01>,<SW 1.3>,\n"
                      "      <acquisition>)\n"
                      "(   2,<2022-09-01 19:10:12.123 -0200>,<testuser>,<location01>,<SW 1.3>,\n"
                      "      <raw data processing\n"
                      "       line 2\n"
                      "       line 3>)\n"
                      "##END=\n"
                     };
    // clang-format on
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};

    auto auditTrail
        = sciformats::jdx::AuditTrail(label, variables, reader, nextLine);
    auto entries = auditTrail.getData();

    REQUIRE(2 == entries.size());
    auto entry1 = entries.at(0);
    REQUIRE(1 == entry1.number);
    REQUIRE("2022-09-01 09:10:11.123 -0200" == entry1.when);
    REQUIRE("testuser" == entry1.who);
    REQUIRE("location01" == entry1.where);
    REQUIRE_FALSE(entry1.process.has_value());
    REQUIRE("SW 1.3" == entry1.version.value());
    REQUIRE("acquisition" == entry1.what);
    auto entry2 = entries.at(1);
    REQUIRE(2 == entry2.number);
    REQUIRE("2022-09-01 19:10:12.123 -0200" == entry2.when);
    REQUIRE("testuser" == entry2.who);
    REQUIRE("location01" == entry2.where);
    REQUIRE_FALSE(entry2.process.has_value());
    REQUIRE("SW 1.3" == entry2.version.value());
    REQUIRE("raw data processing\nline 2\nline 3" == entry2.what);
}

TEST_CASE("parses well-formed 7 parameters audit trail", "[AuditTrail]")
{
    auto nextLine = std::optional<std::string>{
        "##AUDIT TRAIL= $$ (NUMBER, WHEN, WHO, WHERE, PROCESS, VERSION, WHAT)"};
    const auto* label = "AUDITTRAIL";
    const auto* variables
        = " $$ (NUMBER, WHEN, WHO, WHERE, PROCESS, VERSION, WHAT)";
    // clang-format off
    std::string input{"(   1,<2022-09-01 09:10:11.123 -0200>,<testuser>,<location01>,<proc1>,<SW 1.3>,\n"
                      "      <acquisition>)\n"
                      "(   2,<2022-09-01 19:10:12.123 -0200>,<testuser>,<location01>,<proc1>,<SW 1.3>,\n"
                      "      <raw data processing\n"
                      "       line 2\n"
                      "       line 3>)\n"
                      "##END=\n"
                     };
    // clang-format on
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};

    auto auditTrail
        = sciformats::jdx::AuditTrail(label, variables, reader, nextLine);
    auto entries = auditTrail.getData();

    REQUIRE(2 == entries.size());
    auto entry1 = entries.at(0);
    REQUIRE(1 == entry1.number);
    REQUIRE("2022-09-01 09:10:11.123 -0200" == entry1.when);
    REQUIRE("testuser" == entry1.who);
    REQUIRE("location01" == entry1.where);
    REQUIRE("proc1" == entry1.process.value());
    REQUIRE("SW 1.3" == entry1.version.value());
    REQUIRE("acquisition" == entry1.what);
    auto entry2 = entries.at(1);
    REQUIRE(2 == entry2.number);
    REQUIRE("2022-09-01 19:10:12.123 -0200" == entry2.when);
    REQUIRE("testuser" == entry2.who);
    REQUIRE("location01" == entry2.where);
    REQUIRE("proc1" == entry1.process.value());
    REQUIRE("SW 1.3" == entry2.version.value());
    REQUIRE("raw data processing\nline 2\nline 3" == entry2.what);
}

TEST_CASE("parses Bruker NMR type audit trail", "[AuditTrail]")
{
    auto nextLine = std::optional<std::string>{
        "##AUDIT TRAIL= $$ (NUMBER, WHEN, WHO, WHERE, WHAT)"};
    // ##AUDIT TRAIL= $$ (NUMBER, WHEN, WHO, WHERE, PROCESS, VERSION, WHAT)
    const auto* label = "AUDITTRAIL";
    // variables list given may deviate between "##AUDIT TRAIL" and "$$ ##AUDIT
    // TRAIL" with "$$ ##AUDIT TRAIL" reflecting the actual structure
    const auto* variables = "$$ (NUMBER, WHEN, WHO, WHERE, WHAT)";
    // clang-format off
    std::string input{"$$ ##TITLE= Audit trail, TOPSPIN		Version 3.2\n"
                      "$$ ##JCAMPDX= 5.01\n"
                      "$$ ##ORIGIN= Bruker BioSpin GmbH\n"
                      "$$ ##OWNER= Test\n"
                      "$$ $$ C:\\Bruker\\TopSpin3.2/testpath/1/pdata/1/auditp.txt\n"
                      "$$ ##AUDIT TRAIL=  $$ (NUMBER, WHEN, WHO, WHERE, PROCESS, VERSION, WHAT)\n"
                      "(   1,<2022-01-02 03:04:05.999 +0001>,<testuser>,<location01>,<proc1>,<TOPSPIN 3.2>,\n"
                      "      <accumulate start offset = 0 scale = 1 ppm\n"
                      "       3 9876543 \"something" "/opt/topspin3.2/data/loc01/nmr\"\n"
                      "       data hash MD5: 64K\n"
                      "       01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F 10>)\n"
                      "(   2,<2022-01-02 04:04:05.999 +0001>,<testuser>,<location01>,<proc1>,<TOPSPIN 3.2>,\n"
                      "      <accumulate start offset = 0 scale = 1 ppm\n"
                      "       3 9876543 \"something" "/opt/topspin3.2/data/loc01/nmr\"\n"
                      "       data hash MD5: 64K\n"
                      "       02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F 10 11>)\n"
                      "(   3,<2022-01-02 05:04:05.999 +0001>,<testuser>,<location01>,<proc1>,<TOPSPIN 3.2>,\n"
                      "      <accumulate start offset = 0 scale = 1 ppm\n"
                      "       3 9876543 \"something" "/opt/topspin3.2/data/loc01/nmr\"\n"
                      "       data hash MD5: 64K\n"
                      "       03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F 10 11 12>)\n"
                      "$$ ##END=\n"
                      "$$\n"
                      "$$ $$ hash MD5\n"
                      "$$ $$ 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F 10 11 12 13\n"
                      "##$RELAX= \n"
                     };
    // clang-format on
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};

    auto auditTrail
        = sciformats::jdx::AuditTrail(label, variables, reader, nextLine);
    auto entries = auditTrail.getData();

    REQUIRE(3 == entries.size());
    auto entry1 = entries.at(0);
    REQUIRE(1 == entry1.number);
    REQUIRE("2022-01-02 03:04:05.999 +0001" == entry1.when);
    REQUIRE("testuser" == entry1.who);
    REQUIRE("location01" == entry1.where);
    REQUIRE("proc1" == entry1.process.value());
    REQUIRE("TOPSPIN 3.2" == entry1.version.value());
    REQUIRE("accumulate start offset = 0 scale = 1 ppm\n"
            "3 9876543 \"something"
            "/opt/topspin3.2/data/loc01/nmr\"\n"
            "data hash MD5: 64K\n"
            "01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F 10"
            == entry1.what);
    auto entry2 = entries.at(1);
    REQUIRE(2 == entry2.number);
    REQUIRE("2022-01-02 04:04:05.999 +0001" == entry2.when);
    auto entry3 = entries.at(2);
    REQUIRE(3 == entry3.number);
    REQUIRE("2022-01-02 05:04:05.999 +0001" == entry3.when);
}

TEST_CASE("fails when unclosed audit trail entry parenthesis", "[AuditTrail]")
{
    auto nextLine = std::optional<std::string>{
        "##AUDIT TRAIL= $$ (NUMBER, WHEN, WHO, WHERE, WHAT)"};
    // ##AUDIT TRAIL=  $$ (NUMBER, WHEN, WHO, WHERE, PROCESS, VERSION, WHAT)
    const auto* label = "AUDITTRAIL";
    const auto* variables
        = " $$ (NUMBER, WHEN, WHO, WHERE, PROCESS, VERSION, WHAT)";
    // clang-format off
    std::string input{"(   1,<2022-09-01 09:10:11.123 -0200>,<testuser>,<location01>,<proc1>,<SW 1.3>,\n"
                      "##END=\n"
                     };
    // clang-format on
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};

    auto auditTrail
        = sciformats::jdx::AuditTrail(label, variables, reader, nextLine);

    REQUIRE_THROWS_WITH(auditTrail.getData(),
        Catch::Matchers::Contains(
            "No closing parenthesis found for", Catch::CaseSensitive::No));
}

TEST_CASE("fails when file ends unexpectedly", "[AuditTrail]")
{
    auto nextLine = std::optional<std::string>{
        "##AUDIT TRAIL= $$ (NUMBER, WHEN, WHO, WHERE, WHAT)"};
    // ##AUDIT TRAIL=  $$ (NUMBER, WHEN, WHO, WHERE, PROCESS, VERSION, WHAT)
    const auto* label = "AUDITTRAIL";
    const auto* variables
        = " $$ (NUMBER, WHEN, WHO, WHERE, PROCESS, VERSION, WHAT)";
    // clang-format off
    std::string input{"(   1,<2022-09-01 09:10:11.123 -0200>,<testuser>,<location01>,<proc1>,<SW 1.3>,\n"
                      "      <acquisition>)\n"
                     };
    // clang-format on
    auto streamPtr = std::make_unique<std::stringstream>(std::ios_base::in);
    streamPtr->str(input);
    sciformats::jdx::TextReader reader{std::move(streamPtr)};

    auto auditTrail
        = sciformats::jdx::AuditTrail(label, variables, reader, nextLine);

    REQUIRE_THROWS_WITH(auditTrail.getData(),
        Catch::Matchers::Contains("end", Catch::CaseSensitive::No)
            && Catch::Matchers::Contains(
                "parenthesis", Catch::CaseSensitive::No));
}
