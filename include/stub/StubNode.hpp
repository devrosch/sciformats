#ifndef LIBSCIWRAP_STUBNODE_HPP
#define LIBSCIWRAP_STUBNODE_HPP

#include "model/Node.hpp"

namespace sciformats::sciwrap::stub
{
class StubNode : public model::Node
{
public:
    virtual std::string getName() const;
    virtual std::vector<std::unique_ptr<model::Node>> getChildNodes();
    virtual ~StubNode() = default;

private:

};
} // sciformats::sciwrap::stub

#endif // LIBSCIWRAP_STUBNODE_HPP
