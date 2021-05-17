#include "stub/StubNode.hpp"

#ifdef __EMSCRIPTEN__
#include <emscripten/bind.h>
#endif

#ifdef __EMSCRIPTEN__
//EMSCRIPTEN_BINDINGS(Node) {
//    using namespace sciformats::sciwrap::model;
//    using namespace sciformats::sciwrap::stub;
//    using namespace emscripten;
//    // see: https://github.com/emscripten-core/emscripten/issues/627
//    class_<Node>("Node")
//        .constructor<>()
//        .property("name", &StubNode::getName)
//        // embind fails mapping getChildNodes to property
//        .function("getChildNodes", &StubNode::getChildNodes)
//    ;
//    // cannot use unique_ptr in embind
//    // see: https://stackoverflow.com/questions/31814092/cant-use-vector-of-unique-ptr-in-emscripten-bindings
//    register_vector<std::shared_ptr<Node>>("vector<std::shared_ptr<Node>>");
//}
#endif
