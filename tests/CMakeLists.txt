set(HEADER_LIST
    "${PROJECT_SOURCE_DIR}/include/jdx/JdxParser.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/JdxLdrParser.hpp"
    "${PROJECT_SOURCE_DIR}/lib/catch2/include/catch2/catch.hpp")

# Tests need to be added as executables first
add_executable(jdxparser_test JdxParserTest.cpp JdxLdrParserTest.cpp)

# I'm using C++17 in the test
target_compile_features(jdxparser_test PRIVATE cxx_std_17)

# We need this directory, and users of our library will need it too
target_include_directories(jdxparser_test PRIVATE ${PROJECT_SOURCE_DIR}/lib/catch2/include)

# Should be linked to the stream_reader library, as well as the Catch2 testing library
target_link_libraries(jdxparser_test PRIVATE jdxparser catch2)

# https://stackoverflow.com/questions/12313258/cmake-how-to-install-test-files-with-unit-tests
# https://stackoverflow.com/questions/34799916/copy-file-from-source-directory-to-binary-directory-using-cmake
add_custom_command(
        TARGET jdxparser_test POST_BUILD
        COMMAND "${CMAKE_COMMAND}" -E copy_directory
                "${CMAKE_CURRENT_SOURCE_DIR}/resources/"
                "${CMAKE_CURRENT_BINARY_DIR}/resources/"
        VERBATIM # use VERBATIM to correctly handle paths that include spaces
)

# If you register a test, then ctest and make test will run it.
# You can also run examples and check the output, as well.
add_test(NAME jdxparser_test COMMAND jdxparser_test) # Command can be a target
