#include "jdx/TabularData.hpp"
#include "util/LdrUtils.hpp"
#include "util/PeakAssignmentsParser.hpp"
#include "util/StringUtils.hpp"

sciformats::jdx::TabularData::TabularData(
    std::string label, std::string variableList, TextReader& reader)
    : DataLdr(std::move(label), std::move(variableList), reader)
{
}

std::optional<std::string> sciformats::jdx::TabularData::getWidthFunction()
{
    auto appendToDescription
        = [](std::string comment, std::string& description) {
              if (!description.empty())
              {
                  description += '\n';
              }
              util::trim(comment);
              description.append(comment);
          };

    auto getCommentLines = [this, &appendToDescription]() {
        // comment $$ in line(s) following LDR start may contain peak function
        auto& reader = getReader();
        auto readerPos = reader.tellg();
        std::string line{};
        std::string functionDescription{};
        while (!reader.eof() && !util::isLdrStart(line = reader.readLine())
               && util::isPureComment(line))
        {
            readerPos = reader.tellg();
            auto [content, comment] = util::stripLineComment(line);
            appendToDescription(comment.value(), functionDescription);
        }
        // reset reader position to start of first assignment or start of next
        // LDR
        reader.seekg(readerPos);
        // return
        return functionDescription.empty()
                   ? std::nullopt
                   : std::optional<std::string>{functionDescription};
    };

    return callAndResetStreamPos<std::optional<std::string>>(getCommentLines);
}
