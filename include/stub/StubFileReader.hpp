#ifndef LIBSCIWRAP_STUBFILEREADER_HPP
#define LIBSCIWRAP_STUBFILEREADER_HPP

#include "model/FileReader.hpp"
#include "model/Node.hpp"

namespace sciformats::sciwrap::stub
{
class StubFileReader: public model::FileReader
{
public:
    virtual bool isResponsible(const std::string& path);
    virtual std::unique_ptr<model::Node> read(const std::string& path);
    virtual ~StubFileReader() = default;

private:

};
} // sciformats::sciwrap::stub

#endif // LIBSCIWRAP_STUBFILEREADER_HPP
