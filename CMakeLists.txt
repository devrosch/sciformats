# --------------------------------------------------------------
# General project settings
# --------------------------------------------------------------

# Works with 3.15 and tested through 3.18
cmake_minimum_required(VERSION 3.15...3.18)

# Project name and a few useful settings. Other commands can pick up the results
project(
  libio
  VERSION 0.1
  DESCRIPTION "A library for reading and writing binary data"
  LANGUAGES CXX)

# --------------------------------------------------------------
# Collect sources info for clang-tidy and clang-format
# --------------------------------------------------------------

# export json database of processed files for use by clang-tidy during build
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
# include directories for use with clang-tidy and make as separate target
set(INCLUDE_DIRECTORIES
    "${CMAKE_CURRENT_SOURCE_DIR}/include")
set(TESTING_INCLUDE_DIRECTORIES
    "${CMAKE_CURRENT_SOURCE_DIR}/lib/catch2/include")
# source files for use with clang-tidy and make as separate target
file(GLOB_RECURSE ALL_SOURCE_FILES
    "${CMAKE_CURRENT_SOURCE_DIR}/apps/*.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/tests/*.cpp"
    "${INCLUDE_DIRECTORIES}/*.hpp")
message(STATUS "INCLUDE_DIRECTORIES: ${INCLUDE_DIRECTORIES}")
message(STATUS "ALL_SOURCE_FILES: ${ALL_SOURCE_FILES}")

# --------------------------------------------------------------
# Run clang-tidy during build and create separate target
# --------------------------------------------------------------

# clang-tidy rules to check
set(CLANG_TIDY_CHECKS "-*,bugprone-*,cert-*,clang-analyzer-*,cppcoreguidelines-*,hicpp-*,misc-*,modernize-*,performance-*,portability-*,readability-*")

# activate clang-tidy for all targets
# see: https://stackoverflow.com/questions/40433573/how-can-i-specify-additional-arguments-for-use-with-cmake-cxx-clang-tidy-variabl
set(CMAKE_CXX_CLANG_TIDY clang-tidy -checks=${CLANG_TIDY_CHECKS})
message(STATUS "CMAKE_CXX_CLANG_TIDY: ${CMAKE_CXX_CLANG_TIDY}")

# Including cmake script to create clang-tidy and clang-format targets
# see: https://stackoverflow.com/questions/32280717/cmake-clang-tidy-or-other-script-as-custom-target
# This allows to run "make clan-tidy" and "make clang-format" in the build directory
include(cmake/clang-dev-tools.cmake)
# Alternatively, in the build directory, the "run-clang-tidy.py" may be used:
# cmake -DCMAKE_EXPORT_COMPILE_COMMANDS=ON <path to project directory>/libio
# <path to script>/run-clang-tidy.py -header-filter='.hpp' -checks='-*,readability-*'

# --------------------------------------------------------------
# Add sub directories contaning sources
# --------------------------------------------------------------

# The compiled library code is here
add_subdirectory(src)
# The executable code is here
add_subdirectory(apps)

# --------------------------------------------------------------
# Add code for testing
# --------------------------------------------------------------

# Make testing available if this is the main app (and not included through add_subdirectory)
if(CMAKE_PROJECT_NAME STREQUAL PROJECT_NAME)
  # Let's ensure -std=c++xx instead of -std=g++xx
  set(CMAKE_CXX_EXTENSIONS OFF)

  # Testing only available if this is the main app
  # Note this needs to be done in the main CMakeLists
  # since it calls enable_testing, which must be in the
  # main CMakeLists.
  include(CTest)
endif()

# Make testing available if this is the main app
if((CMAKE_PROJECT_NAME STREQUAL PROJECT_NAME) AND BUILD_TESTING)
  add_subdirectory(lib/catch2)
  add_subdirectory(tests)
endif()
