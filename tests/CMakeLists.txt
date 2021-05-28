# Tests need to be added as executables first
add_executable(sciwrap_test
    StubNodeTest.cpp
    StubFileParserTest.cpp
    FileParserSelectorTest.cpp
    )

# I'm using C++17 in the test
target_compile_features(sciwrap_test PRIVATE cxx_std_17)

# Should be linked to the library, as well as the Catch2 testing library
target_link_libraries(sciwrap_test PRIVATE sciwrap catch2 trompeloeil)

# https://stackoverflow.com/questions/12313258/cmake-how-to-install-test-files-with-unit-tests
# https://stackoverflow.com/questions/34799916/copy-file-from-source-directory-to-binary-directory-using-cmake
add_custom_command(
    TARGET sciwrap_test POST_BUILD
    COMMAND "${CMAKE_COMMAND}" -E copy_directory
            "${CMAKE_CURRENT_SOURCE_DIR}/resources/"
            "$<TARGET_FILE_DIR:sciwrap_test>/resources/"
# below works with gcc and clang but does not work with VS an Xcode as they create sub folder per build config (Debug, Release, ...)
#            "${CMAKE_CURRENT_BINARY_DIR}/resources/"
    VERBATIM # use VERBATIM to correctly handle paths that include spaces
)

add_custom_command(
    TARGET sciwrap_test POST_BUILD
    COMMAND "${CMAKE_COMMAND}" -E echo
    "target dir sciwrap_test: $<TARGET_FILE_DIR:sciwrap_test>")

if (EMSCRIPTEN)
    # Embed test files, make available in sub directory "resources" (specified after @)
    target_link_options(sciwrap_test PRIVATE "SHELL:--embed-file ${CMAKE_CURRENT_SOURCE_DIR}/resources@resources")
endif()

# Register test for ctest
if (EMSCRIPTEN)
    # run test with node.js
    add_test(NAME sciwrap_test COMMAND node "${CMAKE_CURRENT_BINARY_DIR}/sciwrap_test.js")
else()
    # run natively
    add_test(NAME sciwrap_test COMMAND sciwrap_test) # Command can be a target
endif()
