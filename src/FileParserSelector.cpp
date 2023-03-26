#include "model/FileParserSelector.hpp"

#include <vector>

#ifdef __EMSCRIPTEN__
#include <emscripten/bind.h>
#endif

sciformats::sciwrap::model::FileParserSelector::FileParserSelector(
    std::vector<std::shared_ptr<sciformats::sciwrap::model::FileParser>>
        fileParsers)
    : m_fileParsers{std::move(fileParsers)}
{
}

bool sciformats::sciwrap::model::FileParserSelector::isRecognized(
    const std::string& path)
{
    for (auto const& parserPtr : m_fileParsers)
    {
        if (parserPtr->isRecognized(path))
        {
            return true;
        }
    }
    return false;
}

std::unique_ptr<sciformats::sciwrap::model::Node>
sciformats::sciwrap::model::FileParserSelector::parse(const std::string& path)
{
    auto parseErrors = std::vector<std::string>{};

    for (auto const& parserPtr : m_fileParsers)
    {
        if (parserPtr->isRecognized(path))
        {
            try
            {
                auto node = parserPtr->parse(path);
                return node;
            }
            catch (const std::exception& ex)
            {
                parseErrors.emplace_back(ex.what());
            }
        }
    }
    if (parseErrors.empty())
    {
        // no suitable file parser was found
        throw std::runtime_error("No suitable parser found for: " + path);
    }
    // suitable file parser(s) found but all threw exceptions while parsing
    // => collect exception messages and throw new umbrella exception
    std::string message{"Errors encountered while parsing: " + path + '\n'};
    size_t i = 0;
    for (auto const& error : parseErrors)
    {
        message.append("Parser " + std::to_string(++i) + ": ")
            .append(error)
            .append("\n");
    }
    throw std::runtime_error(message);
}

#ifdef __EMSCRIPTEN__
EMSCRIPTEN_BINDINGS(FileParserSelector)
{
    using namespace sciformats::sciwrap::model;
    using namespace emscripten;
    class_<FileParserSelector, base<FileParser>>("FileParserSelector")
        .constructor<std::vector<
            std::shared_ptr<sciformats::sciwrap::model::FileParser>>>()
        .function("isRecognized", &FileParserSelector::isRecognized)
        .function("parse", &FileParserSelector::parse);
    register_vector<std::shared_ptr<sciformats::sciwrap::model::FileParser>>(
        "vector<std::shared_ptr<sciformats::sciwrap::model::FileParser>>");
}
#endif
