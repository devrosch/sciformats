#ifndef LIBSCIWRAP_FILEPARSER_HPP
#define LIBSCIWRAP_FILEPARSER_HPP

#include "model/Node.hpp"

#include <string>

namespace sciformats::sciwrap::model
{
class FileParser
{
public:
    virtual bool isRecognized(const std::string& path) = 0;
    virtual std::unique_ptr<Node> parse(const std::string& path) = 0;

    // https://stackoverflow.com/questions/26039907/does-rule-of-three-five-apply-to-inheritance-and-virtual-destructors
    FileParser() = default;
    FileParser(const FileParser& node) = default;
    FileParser& operator=(const FileParser& node) = default;
    FileParser(FileParser&&) = default;
    FileParser& operator=(FileParser&&) = default;
    virtual ~FileParser() = default;
};
} // sciformats::sciwrap::model

#endif // LIBSCIWRAP_FILEPARSER_HPP
