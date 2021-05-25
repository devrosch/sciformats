#ifndef LIBSCIWRAP_FILEREADERSELECTOR_HPP
#define LIBSCIWRAP_FILEREADERSELECTOR_HPP

#include "model/FileReader.hpp"
#include "model/Node.hpp"

#include <string>
#include <vector>

namespace sciformats::sciwrap::model
{
class FileReaderSelector : public FileReader
{
public:
    explicit FileReaderSelector(
        std::vector<std::shared_ptr<FileReader>> fileReaders);
    bool isResponsible(const std::string& path) override;
    std::unique_ptr<Node> read(const std::string& path) override;

private:
    std::vector<std::shared_ptr<FileReader>> m_fileReaders;
};
} // sciformats::sciwrap::model

#endif // LIBSCIWRAP_FILEREADERSELECTOR_HPP
