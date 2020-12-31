#ifndef LIBJDX_JDXLDR_HPP
#define LIBJDX_JDXLDR_HPP

#include <string>

namespace sciformats::jdx
{
/**
 * @brief A JCAMP-DX labelled data record (LDR).
 */
class JdxLdr
{
public:
    explicit JdxLdr(const std::string& label);
    JdxLdr(const std::string& label, const std::string& value);
    void addValueLine(const std::string& line);
    [[nodiscard]] const std::string& getLabel() const;
    [[nodiscard]] const std::string& getValue() const;
    [[nodiscard]] bool isUserDefined() const;

private:
    std::string m_label;
    std::string m_value;
};
} // namespace sciformats::jdx

#endif // LIBJDX_JDXLDR_HPP
