#ifndef LIBJDX_JDXLDR_HPP
#define LIBJDX_JDXLDR_HPP

#include <string>

namespace sciformats::jdx
{
/**
 * @brief A JCAMP-DX labeled data record (LDR).
 */
class JdxLdr
{
public:
    explicit JdxLdr(const std::string& label);
    /**
     * @brief Constructs a JdxLdr from label and value.
     * @param label The label of the LDR, e.g. "TITLE" for "##TITLE= abc".
     * @param value The value of the LDR, e.g. "abc" for "##TITLE= abc".
     */
    JdxLdr(const std::string& label, const std::string& value);
    void addValueLine(const std::string& line);
    /**
     * @brief The label of the LDR, e.g. "TITLE" for "##TITLE= abc".
     * @return The label of the LDR.
     */
    [[nodiscard]] const std::string& getLabel() const;
    /**
     * @brief The value (without initial blank character if any) of the
     * LDR, e.g. "abc" for "##TITLE= abc".
     * @return The value of the LDR. If the value spans multiple lines, the
     * return value contains all lines, separated by \"\\n\".
     */
    [[nodiscard]] const std::string& getValue() const;
    /**
     * @brief Whether LDR is user defined, i.e. the label starts
     * with "$", e.g. "##$USER_DEFINED_LABEL= abc".
     * @return "true" if label is user defined, otherwise "false".
     */
    [[nodiscard]] bool isUserDefined() const;

private:
    std::string m_label;
    std::string m_value;
};
} // namespace sciformats::jdx

#endif // LIBJDX_JDXLDR_HPP
