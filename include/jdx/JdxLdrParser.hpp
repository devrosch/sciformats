#ifndef LIBJDX_JDXLDRPARSER_HPP
#define LIBJDX_JDXLDRPARSER_HPP

#include "jdx/JdxLdr.hpp"

#include <istream>

namespace sciformats::jdx
{
class JdxLdrParser
{
public:
    bool static isLdrStart(std::string& line);
    JdxLdr static readLdr(std::istream& istream);
    std::string static readLine(std::istream& istream);
    void static trim(std::string& s);
    void static trimLeft(std::string& s);
    void static trimRight(std::string& s);
    std::string normalizeLdrLabel(std::string ldr);
private:
};
} // namespace sciformats::jdx

#endif // LIBJDX_JDXLDRPARSER_HPP
