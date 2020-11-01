#set(HEADER_LIST
#	"${binary_reader_SOURCE_DIR}/include/binaryreader/binary_reader.hpp"
#	"${binary_reader_SOURCE_DIR}/include/catch2/catch.hpp")

# Tests need to be added as executables first
#add_executable(binary_reader_test binary_reader_test.cpp ${HEADER_LIST})
add_executable(binaryreader_test BinaryReaderTest.cpp)

# I'm using C++17 in the test
target_compile_features(binaryreader_test PRIVATE cxx_std_17)

# We need this directory, and users of our library will need it too
target_include_directories(binaryreader_test PRIVATE ${INCLUDE_DIRECTORIES} ${TESTING_INCLUDE_DIRECTORIES})

# Should be linked to the stream_reader library, as well as the Catch2 testing library
target_link_libraries(binaryreader_test PRIVATE binaryreader catch2)

# https://stackoverflow.com/questions/12313258/cmake-how-to-install-test-files-with-unit-tests
# https://stackoverflow.com/questions/34799916/copy-file-from-source-directory-to-binary-directory-using-cmake
add_custom_command(
        TARGET binaryreader_test POST_BUILD
        COMMAND "${CMAKE_COMMAND}" -E copy_directory
                "${CMAKE_CURRENT_SOURCE_DIR}/resources/"
                "${CMAKE_CURRENT_BINARY_DIR}/resources/"
        VERBATIM # use VERBATIM to correctly handle paths that include spaces
)

# If you register a test, then ctest and make test will run it.
# You can also run examples and check the output, as well.
add_test(NAME binaryreader_test COMMAND binaryreader_test) # Command can be a target
