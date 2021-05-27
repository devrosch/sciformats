#include "model/FileReaderSelector.hpp"
#include "model/FileReader.hpp"
#include "model/Node.hpp"
#include "stub/StubFileReader.hpp"
#include "stub/StubNode.hpp"

#include "catch2/catch.hpp"
#include "catch2/trompeloeil.hpp"

// -Wweak-vtable warning due to QTCreator bug, see:
// https://bugreports.qt.io/browse/QTCREATORBUG-19741
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wweak-vtables"
class MockFileReader : public sciformats::sciwrap::model::FileReader
{
public:
    // dangerous, see
    // https://github.com/rollbear/trompeloeil/blob/master/docs/reference.md/#-trompeloeil_movable_mock
    static constexpr bool trompeloeil_movable_mock = true;

    MAKE_MOCK1(isResponsible, bool(const std::string&), override);
    MAKE_MOCK1(read,
        std::unique_ptr<sciformats::sciwrap::model::Node>(const std::string&),
        override);
};
#pragma clang diagnostic pop

TEST_CASE("FileReaderSelector sequentially checks parsers for applicability",
    "[FileReaderSelector]")
{
    using namespace sciformats::sciwrap::model;
    using namespace sciformats::sciwrap::stub;

    auto mockReader0 = MockFileReader();
    REQUIRE_CALL(mockReader0, isResponsible(ANY(const std::string&)))
        .TIMES(1)
        .RETURN(false);
    auto mockReaderPtr0
        = std::make_shared<MockFileReader>(std::move(mockReader0));

    auto mockReader1 = MockFileReader();
    REQUIRE_CALL(mockReader1, isResponsible(ANY(const std::string&)))
        .TIMES(1)
        .RETURN(true);
    auto mockReaderPtr1
        = std::make_shared<MockFileReader>(std::move(mockReader1));

    auto mockReader2 = MockFileReader();
    REQUIRE_CALL(mockReader2, isResponsible(ANY(const std::string&))).TIMES(0);
    auto mockReaderPtr2
        = std::make_shared<MockFileReader>(std::move(mockReader2));

    FileReaderSelector selector{
        {mockReaderPtr0, mockReaderPtr1, mockReaderPtr2}};

    REQUIRE(selector.isResponsible("resources/dummy.txt"));
}

TEST_CASE("FileReaderSelector collects failed parsing error messages",
    "[FileReaderSelector]")
{
    using namespace sciformats::sciwrap::model;
    using namespace sciformats::sciwrap::stub;

    auto mockReader0 = MockFileReader();
    REQUIRE_CALL(mockReader0, isResponsible(ANY(const std::string&)))
        .TIMES(1)
        .RETURN(true);
    REQUIRE_CALL(mockReader0, read(ANY(const std::string&)))
        .TIMES(1)
        .THROW(std::runtime_error("Error 1."));
    auto mockReaderPtr0
        = std::make_shared<MockFileReader>(std::move(mockReader0));

    auto mockReader1 = MockFileReader();
    REQUIRE_CALL(mockReader1, isResponsible(ANY(const std::string&)))
        .TIMES(1)
        .RETURN(false);
    REQUIRE_CALL(mockReader1, read(ANY(const std::string&))).TIMES(0);
    auto mockReaderPtr1
        = std::make_shared<MockFileReader>(std::move(mockReader1));

    auto mockReader2 = MockFileReader();
    REQUIRE_CALL(mockReader2, isResponsible(ANY(const std::string&)))
        .TIMES(1)
        .RETURN(true);
    REQUIRE_CALL(mockReader2, read(ANY(const std::string&)))
        .TIMES(1)
        .THROW(std::runtime_error("Error 3."));
    auto mockReaderPtr2
        = std::make_shared<MockFileReader>(std::move(mockReader2));

    FileReaderSelector selector{
        {mockReaderPtr0, mockReaderPtr1, mockReaderPtr2}};

    REQUIRE_THROWS_WITH(selector.read("resources/dummy.txt"),
        Catch::Matchers::Contains("Error 1")
            && Catch::Matchers::Contains("Error 3"));
}

TEST_CASE("FileReaderSelector provides generic error when no suitable parser is found",
    "[FileReaderSelector]")
{
    using namespace sciformats::sciwrap::model;
    using namespace sciformats::sciwrap::stub;

    auto mockReader0 = MockFileReader();
    REQUIRE_CALL(mockReader0, isResponsible(ANY(const std::string&)))
        .TIMES(1)
        .RETURN(false);
    REQUIRE_CALL(mockReader0, read(ANY(const std::string&)))
        .TIMES(0);
    auto mockReaderPtr0
        = std::make_shared<MockFileReader>(std::move(mockReader0));

    auto mockReader1 = MockFileReader();
    REQUIRE_CALL(mockReader1, isResponsible(ANY(const std::string&)))
        .TIMES(1)
        .RETURN(false);
    REQUIRE_CALL(mockReader1, read(ANY(const std::string&)))
        .TIMES(0);
    auto mockReaderPtr1
        = std::make_shared<MockFileReader>(std::move(mockReader1));

    FileReaderSelector selector{
        {mockReaderPtr0, mockReaderPtr1}};

    REQUIRE_THROWS_WITH(selector.read("resources/dummy.txt"),
        Catch::Matchers::Contains("No suitable parser", Catch::CaseSensitive::No));
}
