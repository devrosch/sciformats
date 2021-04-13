#include "jdx/Ldr.hpp"

#include "catch2/catch.hpp"

TEST_CASE("LDR is initialized with two both arguments", "[Ldr]")
{
    std::string label{"LABEL"};
    std::string value{"value"};

    auto ldr = sciformats::jdx::Ldr{label, value};

    REQUIRE(label == ldr.getLabel());
    REQUIRE(value == ldr.getValue());
}

TEST_CASE("user defined LDRs are recognized", "[Ldr]")
{
    auto standardLdr = sciformats::jdx::Ldr{"TITLE", "value"};
    auto userDefinedLdr
        = sciformats::jdx::Ldr{"$USER_DEFINED_LABEL", "user value"};
    auto techniqueSpecificLdr
        = sciformats::jdx::Ldr{".OBSERVE_FREQUENCY", "50.0"};

    REQUIRE(false == standardLdr.isUserDefined());
    REQUIRE(true == userDefinedLdr.isUserDefined());
    REQUIRE(false == techniqueSpecificLdr.isUserDefined());
}

TEST_CASE("technique specific LDRs are recognized", "[Ldr]")
{
    auto standardLdr = sciformats::jdx::Ldr{"TITLE", "value"};
    auto userDefinedLdr
        = sciformats::jdx::Ldr{"$USER_DEFINED_LABEL", "user value"};
    auto techniqueSpecificLdr
        = sciformats::jdx::Ldr{".OBSERVE_FREQUENCY", "50.0"};

    REQUIRE(false == standardLdr.isTechniqueSpecific());
    REQUIRE(false == userDefinedLdr.isTechniqueSpecific());
    REQUIRE(true == techniqueSpecificLdr.isTechniqueSpecific());
}
