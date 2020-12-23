#define CATCH_CONFIG_MAIN
#include "jdxparser/JdxParser.hpp"

#include "catch2/catch.hpp"

#include <array>
#include <climits>
#include <string>

// see: https://stackoverflow.com/a/47934240
constexpr auto operator"" _c(unsigned long long arg) noexcept
{
    return static_cast<char>(arg);
}

TEST_CASE("correctly reads file", "[JdxParser]")
{
    const std::string path{"resources/Claniline.jdx"};
    sciformats::jdx::JdxParser reader(path);

    REQUIRE(true);
}
