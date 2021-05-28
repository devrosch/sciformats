#ifndef LIBSCIWRAP_FILEPARSERSELECTOR_HPP
#define LIBSCIWRAP_FILEPARSERSELECTOR_HPP

#include "model/FileParser.hpp"
#include "model/Node.hpp"

#include <string>
#include <vector>

namespace sciformats::sciwrap::model
{
class FileParserSelector : public FileParser
{
public:
    explicit FileParserSelector(
        std::vector<std::shared_ptr<FileParser>> fileParsers);
    bool isRecognized(const std::string& path) override;
    std::unique_ptr<Node> parse(const std::string& path) override;

private:
    std::vector<std::shared_ptr<FileParser>> m_fileParsers;
};
} // sciformats::sciwrap::model

#endif // LIBSCIWRAP_FILEPARSERSELECTOR_HPP
