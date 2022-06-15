#include "jdx/ParseException.hpp"

sciformats::jdx::ParseException::ParseException(const std::string& what)
    : std::invalid_argument(what)
{
}
