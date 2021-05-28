#include "model/FileParser.hpp"

#ifdef __EMSCRIPTEN__
#include <emscripten/bind.h>
#endif

#ifdef __EMSCRIPTEN__
EMSCRIPTEN_BINDINGS(FileParser)
{
    using namespace sciformats::sciwrap::model;
    using namespace emscripten;
    class_<FileParser>("FileParser")
        .smart_ptr<std::shared_ptr<FileParser>>("std::shared_ptr<FileParser>");
}
#endif
