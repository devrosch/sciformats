# Tests need to be added as executables first
add_executable(jdxparser_test
    JdxParserTest.cpp
    LdrParserTest.cpp
    LdrTest.cpp
    BlockTest.cpp
    DataParserTest.cpp
    XyDataTest.cpp
    RaDataTest.cpp
    XyPointsTest.cpp
    )

# I'm using C++17 in the test
target_compile_features(jdxparser_test PRIVATE cxx_std_17)

# Should be linked to the library, as well as the Catch2 testing library
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


if (EMSCRIPTEN)
    # Embed test files, make available in sub directory "resources" (specified after @)
    target_link_options(jdxparser_test PRIVATE "SHELL:--embed-file ${CMAKE_CURRENT_SOURCE_DIR}/resources@resources")
endif()

# Register test for ctest
if (EMSCRIPTEN)
    # run test with node.js
    add_test(NAME jdxparser_test COMMAND node "${CMAKE_CURRENT_BINARY_DIR}/jdxparser_test.js")
else()
    # run natively
    add_test(NAME jdxparser_test COMMAND jdxparser_test) # Command can be a target
endif()
