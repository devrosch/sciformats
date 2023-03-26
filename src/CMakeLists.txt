#set(HEADER_LIST
#        "${PROJECT_SOURCE_DIR}/include/binaryreader/binary_reader.hpp"
#    )

# alternatively, checking with "if(${CMAKE_SYSTEM_NAME} MATCHES "[Ee]mscripten")"
if(NOT EMSCRIPTEN)
    message(STATUS "Trying to find native ICU package...")
    find_package(ICU REQUIRED COMPONENTS uc)
endif()
# if EMSCRIPTEN, USE_ICU flag will have been set

# Glob header(s), supported in CMake 3.12 or later, so they show up in IDE
file(GLOB HEADER_LIST CONFIGURE_DEPENDS
    "${libio_SOURCE_DIR}/include/binaryreader/*.hpp")

# Make an automatic library - will be static or dynamic based on user setting
add_library(binaryreader BinaryReader.cpp ${HEADER_LIST})

if(NOT ${CMAKE_SYSTEM_NAME} MATCHES "[Ee]mscripten")
    # we need to link with ICU, for Emscripten, this is managed through a compile flag
    target_link_libraries(binaryreader PRIVATE ICU::uc)
endif()

# All users of this library will need at least C++17
target_compile_features(binaryreader PUBLIC cxx_std_17)

message(STATUS "incude: ${PROJECT_SOURCE_DIR}/include")
# We need this directory, and users of our library will need it too
target_include_directories(binaryreader PUBLIC "${PROJECT_SOURCE_DIR}/include")
