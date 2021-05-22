#include "model/FileReader.hpp"

#ifdef __EMSCRIPTEN__
#include <emscripten/bind.h>
#endif

#ifdef __EMSCRIPTEN__
EMSCRIPTEN_BINDINGS(FileReader)
{
    using namespace sciformats::sciwrap::model;
    using namespace emscripten;
    class_<FileReader>("FileReader");
}
#endif
