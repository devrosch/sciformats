#include "model/Node.hpp"

#ifdef __EMSCRIPTEN__
#include <emscripten/bind.h>
#endif

#ifdef __EMSCRIPTEN__
EMSCRIPTEN_BINDINGS(Node) {
    using namespace sciformats::sciwrap::model;
    using namespace emscripten;
    class_<std::pair<std::string, std::string>>("std::pair<std::string, std::string>")
        .constructor<std::string, std::string>()
        .property("key", &std::pair<std::string, std::string>::first)
        .property("value", &std::pair<std::string, std::string>::second)
    ;
    // see: https://github.com/emscripten-core/emscripten/issues/627
    class_<Node>("Node")
        .smart_ptr<std::shared_ptr<Node>>("Node")
    ;
    // cannot use unique_ptr in embind
    // see: https://stackoverflow.com/questions/31814092/cant-use-vector-of-unique-ptr-in-emscripten-bindings
    register_vector<std::pair<std::string, std::string>>("vector<std::pair<std::string, std::string>>");
    register_vector<std::shared_ptr<Node>>("vector<std::shared_ptr<Node>>");
}
#endif
