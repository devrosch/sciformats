#define CATCH_CONFIG_MAIN
#include "jdx/JdxParser.hpp"

#include "catch2/catch.hpp"

#include <array>
#include <climits>
#include <string>

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

TEST_CASE("parse succeeds for legal file", "[JdxParser]")
{
    const std::string path{"resources/Claniline.jdx"};
    auto istream = std::make_unique<std::ifstream>(path);

    REQUIRE_NOTHROW(sciformats::jdx::JdxParser::parse(std::move(istream)));
}

TEST_CASE("parse throws for illegal file", "[JdxParser]")
{
    const std::string path{"resources/dummy.jdx"};
    auto istream = std::make_unique<std::ifstream>(path);

    REQUIRE_THROWS(sciformats::jdx::JdxParser::parse(std::move(istream)));
}
