set(HEADER_LIST
    "${PROJECT_SOURCE_DIR}/include/model/File.hpp"
    "${PROJECT_SOURCE_DIR}/include/model/Dir.hpp"
    "${PROJECT_SOURCE_DIR}/include/stub/StubFile.hpp"
    "${PROJECT_SOURCE_DIR}/include/stub/StubDir.hpp"
    )

# Make an automatic library - will be static or dynamic based on user setting
add_library(sciwrap
    StubFile.cpp
    StubDir.cpp
    ${HEADER_LIST})

# All users of this library will need at least C++17
target_compile_features(sciwrap PUBLIC cxx_std_17)

# We need this directory, and users of our library will need it too
target_include_directories(sciwrap PUBLIC ${PROJECT_SOURCE_DIR}/include)
