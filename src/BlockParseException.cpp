#include "jdx/BlockParseException.hpp"

sciformats::jdx::BlockParseException::BlockParseException(
    const std::string& what)
    : ParseException{what}
{
}

sciformats::jdx::BlockParseException::BlockParseException(
    const std::string& issueMsg, const std::string& label,
    const std::string& blockTitle)
    : ParseException{issueMsg + " " + label + " LDR(s) encountered in block: \""
                     + blockTitle}
{
}
