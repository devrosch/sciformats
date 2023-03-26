#ifndef LIBJDX_STRINGLDR_HPP
#define LIBJDX_STRINGLDR_HPP

#include "jdx/Ldr.hpp"

#include <string>

namespace sciformats::jdx
{
/**
 * @brief A JCAMP-DX labeled data record (LDR).
 */
class StringLdr : public Ldr
{
public:
    /**
     * @brief Constructs a Ldr from label and string value.
     * @param label The label of the LDR, e.g. "TITLE" for "##TITLE= abc".
     * @param value The value of the LDR, e.g. "abc" for "##TITLE= abc".
     */
    StringLdr(std::string label, std::string value);

    /**
     * @brief The value (without initial blank character if any) of the
     * LDR, e.g. "abc" for "##TITLE= abc".
     * @return The value of the LDR. If the value spans multiple lines, the
     * return value contains all lines, separated by \"\\n\".
     */
    [[nodiscard]] const std::string& getValue() const;

private:
    const std::string m_value;
};
} // namespace sciformats::jdx

#endif // LIBJDX_STRINGLDR_HPP
