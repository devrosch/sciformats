#ifndef LIBSCIWRAP_NODE_HPP
#define LIBSCIWRAP_NODE_HPP

#include <string>
#include <vector>
#include <memory>

namespace sciformats::sciwrap::model
{
class Node
{
public:
    virtual std::string getName() const = 0;
    virtual std::vector<std::shared_ptr<Node>> getChildNodes() = 0;
//    virtual std::shared_ptr<Node> getSingleChild() = 0;
    virtual ~Node() = default;
};
} // sciformats::sciwrap::model

#endif // LIBSCIWRAP_NODE_HPP
