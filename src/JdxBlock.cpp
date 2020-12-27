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
            // TODO: add special treatment for data LDRs (e.g. XYDATA, RADATA,
            // NTUPLES, PEAK TABLE, ...
            auto [it, success] = m_ldrs.emplace(label, value);
            if (!success)
            {
                // TODO: log warning or throw
                // reference implementation seems to overwrite LDR with
                // duplicate
                //                throw std::runtime_error(
                //                    std::string{"Duplicate LDR in Block \""} +
                //                    m_ldrs["TITLE"] + "\": " + line);
            }
        }
        else
        {
            m_ldrs.at(lastLabel).append('\n' + line);
        }
    }
}

const std::map<std::string, std::string>& sciformats::jdx::JdxBlock::getLdrs()
{
    return m_ldrs;
}

const std::vector<sciformats::jdx::JdxBlock>&
sciformats::jdx::JdxBlock::getBlocks()
{
    return m_blocks;
}
