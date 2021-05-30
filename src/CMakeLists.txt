set(HEADER_LIST
    "${PROJECT_SOURCE_DIR}/include/model/KeyValueParam.hpp"
    "${PROJECT_SOURCE_DIR}/include/model/Node.hpp"
    "${PROJECT_SOURCE_DIR}/include/model/FileParser.hpp"
    "${PROJECT_SOURCE_DIR}/include/model/FileParserSelector.hpp"
    "${PROJECT_SOURCE_DIR}/include/stub/StubNode.hpp"
    "${PROJECT_SOURCE_DIR}/include/stub/StubFileParser.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/JdxBlockNode.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/JdxFileParser.hpp"
    )

# Make an automatic library - will be static or dynamic based on user setting
add_library(sciwrap
    KeyValueParam.cpp
    Node.cpp
    FileParser.cpp
    FileParserSelector.cpp
    StubNode.cpp
    StubFileParser.cpp
    JdxBlockNode.cpp
    JdxFileParser.cpp
    ${HEADER_LIST})

target_link_libraries(sciwrap PRIVATE jdxparser)

# All users of this library will need at least C++17
target_compile_features(sciwrap PUBLIC cxx_std_17)

if (EMSCRIPTEN)
    # make sure that code not used in this target is retained, see: https://emscripten.org/docs/porting/connecting_cpp_and_javascript/embind.html
    # for making this work in CMake, see: https://stackoverflow.com/a/59994837
    # alternative discussion https://stackoverflow.com/questions/53071878/using-whole-archive-linker-option-with-cmake-and-libraries-with-other-library
    target_link_options(sciwrap INTERFACE "LINKER:--whole-archive")
endif()

# We need this directory, and users of our library will need it too
target_include_directories(sciwrap PUBLIC ${PROJECT_SOURCE_DIR}/include)
