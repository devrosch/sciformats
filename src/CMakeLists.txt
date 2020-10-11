#set(HEADER_LIST "${binary_reader_SOURCE_DIR}/include/binaryreader/binary_reader.h")

# Make an automatic library - will be static or dynamic based on user setting
#add_library(stream_reader streamreader.cpp ${HEADER_LIST})
add_library(binary_reader binary_reader.cpp)

# All users of this library will need at least C++17
target_compile_features(binary_reader PUBLIC cxx_std_17)

# We need this directory, and users of our library will need it too
target_include_directories(binary_reader PUBLIC ${INCLUDE_DIRECTORIES})

