// determine the availability of the filesystem header
// inspired by:
// https://stackoverflow.com/questions/53365538/how-to-determine-whether-to-use-filesystem-or-experimental-filesystem
#ifndef LIBJDX_USE_EXPERIMENTAL_FILESYSTEM
#if defined(__cpp_lib_filesystem)
// NOLINTNEXTLINE(cppcoreguidelines-macro-usage)
#define LIBJDX_USE_EXPERIMENTAL_FILESYSTEM 0
#elif defined(__cpp_lib_experimental_filesystem)
// NOLINTNEXTLINE(cppcoreguidelines-macro-usage)
#define LIBJDX_USE_EXPERIMENTAL_FILESYSTEM 1
#elif !defined(__has_include)
// NOLINTNEXTLINE(cppcoreguidelines-macro-usage)
#define LIBJDX_USE_EXPERIMENTAL_FILESYSTEM 1
#elif __has_include(<filesystem>)
// NOLINTNEXTLINE(cppcoreguidelines-macro-usage)
#define LIBJDX_USE_EXPERIMENTAL_FILESYSTEM 0
#elif __has_include(<experimental/filesystem>)
// NOLINTNEXTLINE(cppcoreguidelines-macro-usage)
#define LIBJDX_USE_EXPERIMENTAL_FILESYSTEM 1
#endif
#endif

#ifndef LIBJDX_USE_EXPERIMENTAL_FILESYSTEM
#error Required <filesystem> header not available.
#endif

#include "jdx/JdxParser.hpp"
#include "util/StringUtils.hpp"

#include <algorithm>
#include <array>
#include <climits>
#include <cstring>
#include <limits>
// include and alias filesystem header
#if LIBJDX_USE_EXPERIMENTAL_FILESYSTEM
#include <experimental/filesystem>
namespace fs = std::experimental::filesystem;
#else
#include <filesystem>
namespace fs = std::filesystem;
#endif

bool sciformats::jdx::JdxParser::canParse(
    const std::string& filePath, std::istream& iStream)
{
    // check extension
    std::string extension = fs::path(filePath).extension();
    util::toLower(extension);
    if (std::find(std::begin(s_acceptedExtensions),
            std::end(s_acceptedExtensions), extension)
        == std::end(s_acceptedExtensions))
    {
        return false;
    }

    // check magic bytes
    std::ios::pos_type position = iStream.tellg();
    std::string magic{"##TITLE="};
    bool match = true;
    for (size_t i{0}; i < magic.size(); i++)
    {
        // TODO: label should be normalized before comparison
        if (iStream.eof() || magic.at(i) != iStream.get())
        {
            match = false;
            break;
        }
    }
    iStream.seekg(position, std::ios_base::beg);
    return match;
}

sciformats::jdx::Block sciformats::jdx::JdxParser::parse(
    std::unique_ptr<std::istream> streamPtr)
{
    // the underlying getline() method sets failbit at end of file, so do
    // not set std::ios::eofbit
    streamPtr->exceptions(std::ios::failbit | std::ios::badbit);
    sciformats::jdx::Block block{std::move(streamPtr)};
    return block;
}
