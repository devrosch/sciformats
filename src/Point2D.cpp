#include "model/Point2D.hpp"

#ifdef __EMSCRIPTEN__
#include <emscripten/bind.h>
#endif

#ifdef __EMSCRIPTEN__
EMSCRIPTEN_BINDINGS(Point2D)
{
    using namespace sciformats::sciwrap::model;
    using namespace emscripten;
    value_object<Point2D>("Point2D")
        .field("x", &KeyValueParam::x)
        .field("y", &KeyValueParam::y);
}
#endif
