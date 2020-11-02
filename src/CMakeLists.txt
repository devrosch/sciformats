find_package(ICU REQUIRED COMPONENTS uc)

# Glob header(s), supported in CMake 3.12 or later, so they show up in IDE
file(GLOB HEADER_LIST CONFIGURE_DEPENDS
    "${libio_SOURCE_DIR}/include/binaryreader/*.hpp")

# Make an automatic library - will be static or dynamic based on user setting
add_library(binaryreader BinaryReader.cpp ${HEADER_LIST})

target_link_libraries(binaryreader PRIVATE ICU::uc)

# All users of this library will need at least C++17
target_compile_features(binaryreader PUBLIC cxx_std_17)

# We need this directory, and users of our library will need it too
target_include_directories(binaryreader PUBLIC ${INCLUDE_DIRECTORIES})
