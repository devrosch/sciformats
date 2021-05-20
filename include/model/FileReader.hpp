#ifndef LIBSCIWRAP_FILEREADER_HPP
#define LIBSCIWRAP_FILEREADER_HPP

#include "model/Node.hpp"

#include <string>

namespace sciformats::sciwrap::model
{
class FileReader
{
public:
    virtual bool isResponsible(const std::string& path) = 0;
    virtual std::unique_ptr<Node> read(const std::string& path) = 0;
    virtual ~FileReader() = default;
};
} // sciformats::sciwrap::model

#endif // LIBSCIWRAP_FILEREADER_HPP
