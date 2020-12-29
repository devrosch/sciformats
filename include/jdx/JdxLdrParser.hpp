#ifndef LIBJDX_JDXLDRPARSER_HPP
#define LIBJDX_JDXLDRPARSER_HPP

#include "jdx/JdxLdr.hpp"

#include <istream>
#include <optional>

namespace sciformats::jdx
{
class JdxLdrParser
{
public:
    std::string static readLine(std::istream& istream);
    bool static isLdrStart(const std::string& line);
    void static trim(std::string& s);
    void static trimLeft(std::string& s);
    void static trimRight(std::string& s);
    std::string static normalizeLdrLabel(const std::string& ldr);
    std::pair<std::string, std::string> static parseLdrStart(const std::string& ldrStart);
    std::pair<std::string, std::optional<std::string>> static stripLineComment(const std::string& line);
};
} // namespace sciformats::jdx

#endif // LIBJDX_JDXLDRPARSER_HPP
