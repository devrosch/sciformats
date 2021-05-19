#include "model/KeyValueParam.hpp"

#ifdef __EMSCRIPTEN__
#include <emscripten/bind.h>
#endif

#ifdef __EMSCRIPTEN__
EMSCRIPTEN_BINDINGS(KeyValueParam) {
    using namespace sciformats::sciwrap::model;
    using namespace emscripten;
    value_object<KeyValueParam>("KeyValueParam")
        .field("key", &KeyValueParam::key)
        .field("value", &KeyValueParam::value)
        ;
}
#endif
