# Targets for running clang-tidy and clang-format
# Requires clang-format (^10.0.0) and clang-tidy (^6.0.0) to be on the path
# see: https://stackoverflow.com/questions/32280717/cmake-clang-tidy-or-other-script-as-custom-target

# --------------------------------------------------------------
# Collect sources info for clang-tidy and clang-format
# --------------------------------------------------------------

# export json database of processed files for use by clang-tidy during build
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
# include directories for use with clang-tidy and make as separate target
set(INCLUDE_DIRECTORIES
    "${PROJECT_SOURCE_DIR}/include"
    "${PROJECT_SOURCE_DIR}/lib/catch2/include")
# source files for use with clang-tidy and make as separate target
file(GLOB_RECURSE ALL_SOURCE_FILES
    "${PROJECT_SOURCE_DIR}/apps/*.cpp"
    "${PROJECT_SOURCE_DIR}/src/*.cpp"
    "${PROJECT_SOURCE_DIR}/tests/*.cpp"
    "${PROJECT_SOURCE_DIR}/include/*.hpp")
message(STATUS "clang-tidy INCLUDE_DIRECTORIES: ${INCLUDE_DIRECTORIES}")
message(STATUS "clang-tidy/clang-format ALL_SOURCE_FILES: ${ALL_SOURCE_FILES}")

# prepend include directories with -I to pass them as multiple options to make
set(CLANG_TIDY_MAKE_INCLUDES ${INCLUDE_DIRECTORIES})
list(TRANSFORM CLANG_TIDY_MAKE_INCLUDES PREPEND -I)
message(STATUS "CLANG_TIDY_MAKE_INCLUDES: ${CLANG_TIDY_MAKE_INCLUDES}")

# --------------------------------------------------------------
# Add targets
# --------------------------------------------------------------

add_custom_target(
        clang-tidy
        VERBATIM # use VERBATIM to correctly handle paths that include spaces
        COMMAND clang-tidy
        --checks=${CLANG_TIDY_CHECKS}
        --warnings-as-errors=${CLANG_TIDY_CHECKS}
        ${ALL_SOURCE_FILES}
        --
        -std=c++17
        ${CLANG_TIDY_MAKE_INCLUDES}
)

add_custom_target(
        clang-format-check
        COMMAND clang-format
        --style=file
        --dry-run
        --Werror
        ${ALL_SOURCE_FILES}
)

add_custom_target(
        clang-format-fix
        COMMAND clang-format
        --style=file
        -i
        ${ALL_SOURCE_FILES}
)
