# Header only library, therefore INTERFACE
add_library(trompeloeil INTERFACE)

# INTERFACE targets only have INTERFACE properties
target_include_directories(trompeloeil INTERFACE ${CMAKE_CURRENT_SOURCE_DIR}/include)
