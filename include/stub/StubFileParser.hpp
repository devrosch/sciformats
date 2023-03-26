#ifndef LIBSCIWRAP_STUBFILEPARSER_HPP
#define LIBSCIWRAP_STUBFILEPARSER_HPP

#include "model/FileParser.hpp"
#include "model/Node.hpp"

namespace sciformats::sciwrap::stub
{
class StubFileParser : public model::FileParser
{
public:
    bool isRecognized(const std::string& path) override;
    std::unique_ptr<model::Node> parse(const std::string& path) override;

private:
};
} // sciformats::sciwrap::stub

#endif // LIBSCIWRAP_STUBFILEPARSER_HPP
