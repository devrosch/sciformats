#include "jdx/JdxLdr.hpp"

#include "catch2/catch.hpp"

TEST_CASE("LDR is initialized with two both arguments", "[JdxLdr]")
{
    std::string label{"LABEL"};
    std::string value{"value"};

    auto ldr = sciformats::jdx::JdxLdr{label, value};

    REQUIRE(label == ldr.getLabel());
    REQUIRE(value == ldr.getValue());
}

TEST_CASE("user defined LDRs are recognized", "[JdxLdr]")
{
    auto standardLdr = sciformats::jdx::JdxLdr{"TITLE", "value"};
    auto userDefinedLdr
        = sciformats::jdx::JdxLdr{"$USER_DEFINED_LABEL", "user value"};
    auto techniqueSpecificLdr
        = sciformats::jdx::JdxLdr{".OBSERVE_FREQUENCY", "50.0"};

    REQUIRE(false == standardLdr.isUserDefined());
    REQUIRE(true == userDefinedLdr.isUserDefined());
    REQUIRE(false == techniqueSpecificLdr.isUserDefined());
}

TEST_CASE("technique specific LDRs are recognized", "[JdxLdr]")
{
    auto standardLdr = sciformats::jdx::JdxLdr{"TITLE", "value"};
    auto userDefinedLdr
        = sciformats::jdx::JdxLdr{"$USER_DEFINED_LABEL", "user value"};
    auto techniqueSpecificLdr
        = sciformats::jdx::JdxLdr{".OBSERVE_FREQUENCY", "50.0"};

    REQUIRE(false == standardLdr.isTechniqueSpecific());
    REQUIRE(false == userDefinedLdr.isTechniqueSpecific());
    REQUIRE(true == techniqueSpecificLdr.isTechniqueSpecific());
}
