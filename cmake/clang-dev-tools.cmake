# Targets for running clang-tidy and clang-format
# Requires clang-format and clang-tidy to be on the path
# see: https://stackoverflow.com/questions/32280717/cmake-clang-tidy-or-other-script-as-custom-target

# prepend include directories with -I to pass them as multiple options to make
set(CLANG_TIDY_MAKE_INCLUDES ${INCLUDE_DIRECTORIES})
list(TRANSFORM CLANG_TIDY_MAKE_INCLUDES PREPEND -I)
message(STATUS "CLANG_TIDY_MAKE_INCLUDES: ${CLANG_TIDY_MAKE_INCLUDES}")

add_custom_target(
        clang-tidy
        COMMAND clang-tidy
        --checks=${CLANG_TIDY_CHECKS}
        --warnings-as-errors=${CLANG_TIDY_CHECKS}
        ${ALL_SOURCE_FILES}
        --
        -std=c++17
        ${CLANG_TIDY_MAKE_INCLUDES}
)

# TODO: untested
add_custom_target(
        clang-format
        COMMAND clang-format
        -style=file
        -i
        -I${ALL_SOURCE_FILES}
)
