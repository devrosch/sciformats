#ifndef LIBJDX_LDR_HPP
#define LIBJDX_LDR_HPP

#include <string>

namespace sciformats::jdx
{
/**
 * @brief A JCAMP-DX labeled data record (LDR).
 */
class Ldr
{
public:
    /**
     * @brief Constructs a Ldr from label and string value.
     * @param label The label of the LDR, e.g. "TITLE" for "##TITLE= abc".
     */
    explicit Ldr(std::string label);
    /**
     * @brief The label of the LDR, e.g. "TITLE" for "##TITLE= abc".
     * @return The label of the LDR.
     */
    [[nodiscard]] const std::string& getLabel() const;
    /**
     * @brief Whether LDR is user defined, i.e. the label starts
     * with "$", e.g. "##$USER_DEFINED_LABEL= abc".
     * @return "true" if label is user defined, otherwise "false".
     */
    [[nodiscard]] bool isUserDefined() const;
    /**
     * @brief Whether LDR is technique specific, i.e. the label starts
     * with ".", e.g. "##.OBSERVE_FREQUENCY= 50.0".
     * @return "true" if label is user defined, otherwise "false".
     */
    [[nodiscard]] bool isTechniqueSpecific() const;

private:
    const std::string m_label;
};
} // namespace sciformats::jdx

#endif // LIBJDX_LDR_HPP
