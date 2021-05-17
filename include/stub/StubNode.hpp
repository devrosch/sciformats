#ifndef LIBSCIWRAP_STUBNODE_HPP
#define LIBSCIWRAP_STUBNODE_HPP

#include "model/Node.hpp"

namespace sciformats::sciwrap::stub
{
class StubNode : public model::Node
{
public:
    virtual std::string getName() const;
    virtual std::vector<std::shared_ptr<model::Node>> getChildNodes();
    virtual std::shared_ptr<Node> getSingleChild();
    virtual ~StubNode() = default;

private:

};
} // sciformats::sciwrap::stub

#endif // LIBSCIWRAP_STUBNODE_HPP
