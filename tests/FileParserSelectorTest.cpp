#include "model/FileParserSelector.hpp"
#include "model/FileParser.hpp"
#include "model/Node.hpp"
#include "stub/StubFileParser.hpp"
#include "stub/StubNode.hpp"

#include "catch2/catch.hpp"
#include "catch2/trompeloeil.hpp"

// -Wweak-vtable warning due to QTCreator bug, see:
// https://stackoverflow.com/questions/50463374/avoid-weak-vtable-warnings-for-classes-only-defined-in-a-source-file
// , also: https://bugreports.qt.io/browse/QTCREATORBUG-19741
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wweak-vtables"
class MockFileParser : public sciformats::sciwrap::model::FileParser
{
public:
    // dangerous, see
    // https://github.com/rollbear/trompeloeil/blob/master/docs/reference.md/#-trompeloeil_movable_mock
    static constexpr bool trompeloeil_movable_mock = true;

    MAKE_MOCK1(isRecognized, bool(const std::string&), override);
    MAKE_MOCK1(parse,
        std::unique_ptr<sciformats::sciwrap::model::Node>(const std::string&),
        override);
};
#pragma clang diagnostic pop

TEST_CASE("FileParserSelector sequentially checks parsers for applicability",
    "[FileParserSelector]")
{
    using namespace sciformats::sciwrap::model;
    using namespace sciformats::sciwrap::stub;

    auto mockParser0 = MockFileParser();
    REQUIRE_CALL(mockParser0, isRecognized(ANY(const std::string&)))
        .TIMES(1)
        .RETURN(false);
    auto mockParserPtr0
        = std::make_shared<MockFileParser>(std::move(mockParser0));
    // alternatively:
    // auto mockParserPtr0
    //    = std::shared_ptr<MockFileParser>(&mockParser0,
    //    [](MockFileParser*){});

    auto mockParser1 = MockFileParser();
    REQUIRE_CALL(mockParser1, isRecognized(ANY(const std::string&)))
        .TIMES(1)
        .RETURN(true);
    auto mockParserPtr1
        = std::make_shared<MockFileParser>(std::move(mockParser1));

    auto mockParser2 = MockFileParser();
    REQUIRE_CALL(mockParser2, isRecognized(ANY(const std::string&))).TIMES(0);
    auto mockParserPtr2
        = std::make_shared<MockFileParser>(std::move(mockParser2));

    FileParserSelector selector{
        {mockParserPtr0, mockParserPtr1, mockParserPtr2}};

    REQUIRE(selector.isRecognized("resources/dummy.txt"));
}

TEST_CASE("FileParserSelector collects failed parsing error messages",
    "[FileParserSelector]")
{
    using namespace sciformats::sciwrap::model;
    using namespace sciformats::sciwrap::stub;

    auto mockParser0 = MockFileParser();
    REQUIRE_CALL(mockParser0, isRecognized(ANY(const std::string&)))
        .TIMES(1)
        .RETURN(true);
    REQUIRE_CALL(mockParser0, parse(ANY(const std::string&)))
        .TIMES(1)
        .THROW(std::runtime_error("Error 1."));
    auto mockParserPtr0
        = std::make_shared<MockFileParser>(std::move(mockParser0));

    auto mockParser1 = MockFileParser();
    REQUIRE_CALL(mockParser1, isRecognized(ANY(const std::string&)))
        .TIMES(1)
        .RETURN(false);
    REQUIRE_CALL(mockParser1, parse(ANY(const std::string&))).TIMES(0);
    auto mockParserPtr1
        = std::make_shared<MockFileParser>(std::move(mockParser1));

    auto mockParser2 = MockFileParser();
    REQUIRE_CALL(mockParser2, isRecognized(ANY(const std::string&)))
        .TIMES(1)
        .RETURN(true);
    REQUIRE_CALL(mockParser2, parse(ANY(const std::string&)))
        .TIMES(1)
        .THROW(std::runtime_error("Error 3."));
    auto mockParserPtr2
        = std::make_shared<MockFileParser>(std::move(mockParser2));

    FileParserSelector selector{
        {mockParserPtr0, mockParserPtr1, mockParserPtr2}};

    REQUIRE_THROWS_WITH(selector.parse("resources/dummy.txt"),
        Catch::Matchers::Contains("Error 1")
            && Catch::Matchers::Contains("Error 3"));
}

TEST_CASE("FileParserSelector provides generic error when no suitable parser "
          "is found",
    "[FileParserSelector]")
{
    using namespace sciformats::sciwrap::model;
    using namespace sciformats::sciwrap::stub;

    auto mockParser0 = MockFileParser();
    REQUIRE_CALL(mockParser0, isRecognized(ANY(const std::string&)))
        .TIMES(1)
        .RETURN(false);
    REQUIRE_CALL(mockParser0, parse(ANY(const std::string&))).TIMES(0);
    auto mockParserPtr0
        = std::make_shared<MockFileParser>(std::move(mockParser0));

    auto mockParser1 = MockFileParser();
    REQUIRE_CALL(mockParser1, isRecognized(ANY(const std::string&)))
        .TIMES(1)
        .RETURN(false);
    REQUIRE_CALL(mockParser1, parse(ANY(const std::string&))).TIMES(0);
    auto mockParserPtr1
        = std::make_shared<MockFileParser>(std::move(mockParser1));

    FileParserSelector selector{{mockParserPtr0, mockParserPtr1}};

    REQUIRE_THROWS_WITH(selector.parse("resources/dummy.txt"),
        Catch::Matchers::Contains(
            "No suitable parser", Catch::CaseSensitive::No));
}

TEST_CASE(
    "FileParserSelector shallow check returns false when no suitable parser "
    "is found",
    "[FileParserSelector]")
{
    using namespace sciformats::sciwrap::model;
    using namespace sciformats::sciwrap::stub;

    auto mockParser0 = MockFileParser();
    REQUIRE_CALL(mockParser0, isRecognized(ANY(const std::string&)))
        .TIMES(1)
        .RETURN(false);
    REQUIRE_CALL(mockParser0, parse(ANY(const std::string&))).TIMES(0);
    auto mockParserPtr0
        = std::make_shared<MockFileParser>(std::move(mockParser0));

    FileParserSelector selector{{mockParserPtr0}};

    REQUIRE_FALSE(selector.isRecognized("resources/dummy.txt"));
}

TEST_CASE("FileParserSelector returns node from applicable underlying parser",
    "[FileParserSelector]")
{
    using namespace sciformats::sciwrap::model;
    using namespace sciformats::sciwrap::stub;

    auto parserPtr = std::make_shared<StubFileParser>(StubFileParser());
    FileParserSelector selector{{parserPtr}};

    auto node = selector.parse("resources/dummy.txt");

    REQUIRE(node != nullptr);
    REQUIRE(node->getName() == "A Stub Node");
}
