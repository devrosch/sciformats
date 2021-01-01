# --------------------------------------------------------------
# General project settings
# --------------------------------------------------------------

# Works with 3.15 and tested through 3.18
cmake_minimum_required(VERSION 3.15...3.18)

# Project name and a few useful settings. Other commands can pick up the results
project(
  libjdx
  VERSION 0.1
  DESCRIPTION "A library for reading JCAMP-DX data"
  LANGUAGES CXX)

# --------------------------------------------------------------
# Add code coverage support
# --------------------------------------------------------------

if (CMAKE_BUILD_TYPE MATCHES DEBUG)
    if (CMAKE_COMPILER_IS_GNUCXX)
        # we're using GCC
        add_compile_options(-g -O0 --coverage -fprofile-arcs -ftest-coverage)
        add_link_options(-lgcov --coverage -fprofile-arcs -ftest-coverage -p)
        message(STATUS "set compile options: -g -O0 --coverage -fprofile-arcs -ftest-coverage")
        message(STATUS "set link options: -lgcov --coverage -fprofile-arcs -ftest-coverage -p")
    endif()
endif()

# --------------------------------------------------------------
# Run clang-tidy during build and create separate target
# --------------------------------------------------------------

# clang-tidy rules to check
# deactivate checks for magic-numbers as they are
# too sensitive in bit oprations and sample data
set(CLANG_TIDY_CHECKS
    "-*\
    ,bugprone-*\
    ,cert-*\
    ,clang-analyzer-*\
    ,cppcoreguidelines-*,-cppcoreguidelines-avoid-magic-numbers\
    ,hicpp-*,misc-*\
    ,modernize-*,-modernize-use-trailing-return-type\
    ,performance-*\
    ,portability-*\
    ,readability-*,-readability-magic-numbers")

# activate clang-tidy for all targets
# see: https://stackoverflow.com/questions/40433573/how-can-i-specify-additional-arguments-for-use-with-cmake-cxx-clang-tidy-variabl
set(CMAKE_CXX_CLANG_TIDY clang-tidy -checks=${CLANG_TIDY_CHECKS})
message(STATUS "CMAKE_CXX_CLANG_TIDY: ${CMAKE_CXX_CLANG_TIDY}")

# Including cmake script to create clang-tidy and clang-format targets
# see: https://stackoverflow.com/questions/32280717/cmake-clang-tidy-or-other-script-as-custom-target
# This allows to run "make clan-tidy" and "make clang-format" in the build directory
include(cmake/clang-dev-tools.cmake)

# --------------------------------------------------------------
# Add sub directories contaning sources
# --------------------------------------------------------------

# The compiled library code is here
add_subdirectory(src)
# The executable code is here
# add_subdirectory(apps) # not used by this library

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

# Including cmake script to create coverage-prepare and coverage targets
include(cmake/coverage.cmake)
