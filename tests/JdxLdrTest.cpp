#include "jdx/JdxLdr.hpp"

#include "catch2/catch.hpp"

TEST_CASE("LDR is initialized with single init argument", "[JdxLdr]")
{
    std::string label{"LABEL"};

    auto ldr = sciformats::jdx::JdxLdr{label};

    REQUIRE(label == ldr.getLabel());
    REQUIRE(ldr.getValue().empty());
}

TEST_CASE("LDR is initialized with two init arguments", "[JdxLdr]")
{
    std::string label{"LABEL"};
    std::string value{"value"};

    auto ldr = sciformats::jdx::JdxLdr{label, value};

    REQUIRE(label == ldr.getLabel());
    REQUIRE(value == ldr.getValue());
}

TEST_CASE("LDR value can be added to", "[JdxLdr]")
{
    std::string line0{"value"};
    std::string line1{"addedLine"};

    auto ldr = sciformats::jdx::JdxLdr{"LABEL", line0};

    REQUIRE(line0 == ldr.getValue());
    ldr.addValueLine(line1);
    REQUIRE(line0 + "\n" + line1 == ldr.getValue());
}

TEST_CASE("user defined LDRs are recognized", "[JdxLdr]")
{
    auto standardLdr = sciformats::jdx::JdxLdr{"TITLE", "value"};
    auto userDefinedLdr
        = sciformats::jdx::JdxLdr{"$USER_DEFINED_LABEL", "user value"};

    REQUIRE(false == standardLdr.isUserDefined());
    REQUIRE(true == userDefinedLdr.isUserDefined());
}
