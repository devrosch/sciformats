#include "model/KeyValueParam.hpp"
#include "model/Node.hpp"

#ifdef __EMSCRIPTEN__
#include <emscripten/bind.h>
#endif

#ifdef __EMSCRIPTEN__
EMSCRIPTEN_BINDINGS(Node) {
    using namespace sciformats::sciwrap::model;
    using namespace emscripten;
    // see: https://github.com/emscripten-core/emscripten/issues/627
    class_<Node>("Node")
        .smart_ptr<std::shared_ptr<Node>>("Node")
        ;
    // cannot use unique_ptr in embind
    // see: https://stackoverflow.com/questions/31814092/cant-use-vector-of-unique-ptr-in-emscripten-bindings
    register_vector<KeyValueParam>("vector<KeyValueParam>");
    register_vector<std::shared_ptr<Node>>("vector<std::shared_ptr<Node>>");
    // as an elternative for mapping vectors to JS arrays, see: https://github.com/emscripten-core/emscripten/issues/11070
}
#endif
