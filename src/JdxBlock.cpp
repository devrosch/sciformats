#include "jdx/JdxBlock.hpp"
#include "jdx/JdxLdrParser.hpp"

#include <array>
#include <climits>
#include <cstring>
#include <limits>

sciformats::jdx::JdxBlock::JdxBlock(std::istream& inputStream)
    : m_istream{inputStream}
{
    auto firstLine = JdxLdrParser::readLine(m_istream);
    if (!JdxLdrParser::isLdrStart(firstLine))
    {
        throw std::runtime_error(
            std::string{"Malformed LDR start: "} + firstLine);
    }
    auto [label, title] = JdxLdrParser::parseLdrStart(firstLine);
    if (label != "TITLE")
    {
        throw std::runtime_error(
            std::string{"Malformed Block start, wrong label: "} + firstLine);
    }
    m_ldrs.insert(std::make_pair("TITLE", title));
    parseInput();
}

sciformats::jdx::JdxBlock::JdxBlock(
    const std::string& title, std::istream& inputStream)
    : m_istream{inputStream}
{
    m_ldrs.insert(std::make_pair("TITLE", title));
    parseInput();
}

void sciformats::jdx::JdxBlock::parseInput()
{
    std::string lastLabel;
    while (!m_istream.eof())
    {
        auto line = JdxLdrParser::readLine(m_istream);
        if (JdxLdrParser::isLdrStart(line))
        {
            auto [label, value] = JdxLdrParser::parseLdrStart(line);
            lastLabel = label;
            if ("END" == label)
            {
                // end of block
                break;
            }
            if ("TITLE" == label)
            {
                // nested block
                auto block = JdxBlock(value, m_istream);
                m_blocks.push_back(std::move(block));
                continue;
            }
            if (label.empty())
            {
                // start of block comment "##="
                m_ldrComments.push_back(value);
                continue;
            }
            // TODO: add special treatment for data LDRs (e.g. XYDATA,
            // XYPOINTS, RADATA, PEAK TABLE, PEAK ASSIGNMENTS, NTUPLES, ...)
            auto [it, success] = m_ldrs.emplace(label, value);
            if (!success)
            {
                // reference implementation seems to overwrite LDR with
                // duplicate, but spec (JCAMP-DX IR 3.2) says
                // a duplicate LDR is illegal in a block => throw
                throw std::runtime_error(
                    std::string{"Duplicate LDR in Block \""} + m_ldrs["TITLE"]
                    + "\": " + line);
            }
        }
        else
        {
            if (lastLabel.empty())
            {
                // TODO: check case that no LDR has yet been encountered
                // last LDR start was an LDR comment "##="
                m_ldrComments.back().append('\n' + line);
            }
            else
            {
                // last LDR was a regular LDR
                m_ldrs.at(lastLabel).append('\n' + line);
            }
        }
    }
}

const std::map<std::string, std::string>&
sciformats::jdx::JdxBlock::getLdrs() const
{
    return m_ldrs;
}

const std::vector<sciformats::jdx::JdxBlock>&
sciformats::jdx::JdxBlock::getBlocks() const
{
    return m_blocks;
}

const std::vector<std::string>&
sciformats::jdx::JdxBlock::getLdrComments() const
{
    return m_ldrComments;
}
