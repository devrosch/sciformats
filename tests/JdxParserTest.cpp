#define CATCH_CONFIG_MAIN
#include "jdx/JdxParser.hpp"

#include "catch2/catch.hpp"

#include <array>
#include <climits>
#include <string>

// see: https://stackoverflow.com/a/47934240
constexpr auto operator"" _c(unsigned long long arg) noexcept
{
    return static_cast<char>(arg);
}

TEST_CASE("accepts legal file", "[JdxParser]")
{
    const std::string path{"resources/Claniline.jdx"};
    std::ifstream istream{path};

    bool canParse = sciformats::jdx::JdxParser::canParse(path, istream);
    REQUIRE(canParse == true);
}

TEST_CASE("rejects illegal file (wrong extension)", "[JdxParser]")
{
    const std::string path{"resources/dummy.txt"};
    std::ifstream istream{path};

    bool canParse = sciformats::jdx::JdxParser::canParse(path, istream);
    REQUIRE(canParse == false);
}

TEST_CASE("rejects illegal file (wrong magic bytes)", "[JdxParser]")
{
    const std::string path{"resources/dummy.jdx"};
    std::ifstream istream{path};

    bool canParse = sciformats::jdx::JdxParser::canParse(path, istream);
    REQUIRE(canParse == false);
}
