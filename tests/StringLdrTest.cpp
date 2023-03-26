#include "jdx/StringLdr.hpp"

#include "catch2/catch.hpp"

TEST_CASE("LDR is initialized with two both arguments", "[StringLdr]")
{
    std::string label{"LABEL"};
    std::string value{"value"};

    auto ldr = sciformats::jdx::StringLdr{label, value};

    REQUIRE(label == ldr.getLabel());
    REQUIRE(value == ldr.getValue());
}

TEST_CASE("user defined LDRs are recognized", "[StringLdr]")
{
    auto standardLdr = sciformats::jdx::StringLdr{"TITLE", "value"};
    auto userDefinedLdr
        = sciformats::jdx::StringLdr{"$USER_DEFINED_LABEL", "user value"};
    auto techniqueSpecificLdr
        = sciformats::jdx::StringLdr{".OBSERVE_FREQUENCY", "50.0"};

    REQUIRE(false == standardLdr.isUserDefined());
    REQUIRE(true == userDefinedLdr.isUserDefined());
    REQUIRE(false == techniqueSpecificLdr.isUserDefined());
}

TEST_CASE("technique specific LDRs are recognized", "[StringLdr]")
{
    auto standardLdr = sciformats::jdx::StringLdr{"TITLE", "value"};
    auto userDefinedLdr
        = sciformats::jdx::StringLdr{"$USER_DEFINED_LABEL", "user value"};
    auto techniqueSpecificLdr
        = sciformats::jdx::StringLdr{".OBSERVE_FREQUENCY", "50.0"};

    REQUIRE(false == standardLdr.isTechniqueSpecific());
    REQUIRE(false == userDefinedLdr.isTechniqueSpecific());
    REQUIRE(true == techniqueSpecificLdr.isTechniqueSpecific());
}
