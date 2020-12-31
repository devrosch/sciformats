set(HEADER_LIST
    "${PROJECT_SOURCE_DIR}/include/jdx/JdxParser.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/JdxBlock.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/JdxLdrParser.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/JdxLdr.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/JdxDataParser.hpp"
    )

# Make an automatic library - will be static or dynamic based on user setting
add_library(jdxparser
    JdxParser.cpp
    JdxBlock.cpp
    JdxLdrParser.cpp
    JdxLdr.cpp
    JdxDataParser.cpp
    ${HEADER_LIST})

# All users of this library will need at least C++17
target_compile_features(jdxparser PUBLIC cxx_std_17)

# We need this directory, and users of our library will need it too
target_include_directories(jdxparser PUBLIC ${PROJECT_SOURCE_DIR}/include)
