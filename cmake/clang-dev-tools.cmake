# Targets for running clang-tidy and clang-format
# Requires clang-format (^10.0.0) and clang-tidy (^6.0.0) to be on the path
# see: https://stackoverflow.com/questions/32280717/cmake-clang-tidy-or-other-script-as-custom-target

# prepend include directories with -I to pass them as multiple options to make
set(CLANG_TIDY_MAKE_INCLUDES ${INCLUDE_DIRECTORIES} ${TESTING_INCLUDE_DIRECTORIES})
list(TRANSFORM CLANG_TIDY_MAKE_INCLUDES PREPEND -I)
message(STATUS "CLANG_TIDY_MAKE_INCLUDES: ${CLANG_TIDY_MAKE_INCLUDES}")

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
