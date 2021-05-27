#include "model/FileReaderSelector.hpp"

#include <vector>

#ifdef __EMSCRIPTEN__
#include <emscripten/bind.h>
#endif

sciformats::sciwrap::model::FileReaderSelector::FileReaderSelector(
    std::vector<std::shared_ptr<sciformats::sciwrap::model::FileReader>>
        fileReaders)
    : m_fileReaders{std::move(fileReaders)}
{
}

bool sciformats::sciwrap::model::FileReaderSelector::isResponsible(
    const std::string& path)
{
    for (auto const& readerPtr : m_fileReaders)
    {
        if (readerPtr->isResponsible(path))
        {
            return true;
        }
    }
    return false;
}

std::unique_ptr<sciformats::sciwrap::model::Node>
sciformats::sciwrap::model::FileReaderSelector::read(const std::string& path)
{
    auto parseErrors = std::vector<std::string>{};

    for (auto const& readerPtr : m_fileReaders)
    {
        if (readerPtr->isResponsible(path))
        {
            try
            {
                auto node = readerPtr->read(path);
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
EMSCRIPTEN_BINDINGS(FileReaderSelector)
{
    using namespace sciformats::sciwrap::model;
    using namespace emscripten;
    class_<FileReaderSelector, base<FileReader>>("FileReaderSelector")
        .constructor<std::vector<
            std::shared_ptr<sciformats::sciwrap::model::FileReader>>>()
        .function("isResponsible", &FileReaderSelector::isResponsible)
        .function("read", &FileReaderSelector::read);
    register_vector<std::shared_ptr<sciformats::sciwrap::model::FileReader>>(
        "vector<std::shared_ptr<sciformats::sciwrap::model::FileReader>>");
}
#endif
