#ifndef LIBSCIWRAP_JDXFILEPARSER_HPP
#define LIBSCIWRAP_JDXFILEPARSER_HPP

#include "model/FileParser.hpp"
#include "model/Node.hpp"

namespace sciformats::sciwrap::jdx
{
class JdxFileParser : public model::FileParser
{
public:
    bool isRecognized(const std::string& path) override;
    std::unique_ptr<model::Node> parse(const std::string& path) override;

private:
};
} // sciformats::sciwrap::jdx

#endif // LIBSCIWRAP_JDXFILEPARSER_HPP
