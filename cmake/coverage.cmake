# --------------------------------------------------------------
# Targets for creating code coverage reports
# --------------------------------------------------------------

# --------------------------------------------------------------
# Prerequisites
# --------------------------------------------------------------
# Requires lcov and genhtml command to be on the path
# compiler must be GNU g++ (i.e. CMAKE_COMPILER_IS_GNUCXX)
# build must be made in DEBUG mode and the following flags need to be set:
# add_compile_options(-g -O0 --coverage -fprofile-arcs -ftest-coverage)
# add_link_options(-lgcov --coverage -fprofile-arcs -ftest-coverage -p)

# --------------------------------------------------------------
# Run AFTER make and BEFORE executing tests to generate baseline
# --------------------------------------------------------------
add_custom_target(
    coverage-base
    COMMAND lcov
    --capture
    --initial
    --base-directory ${CMAKE_CURRENT_SOURCE_DIR}
    --directory ${CMAKE_CURRENT_BINARY_DIR}
    --output-file "${CMAKE_CURRENT_BINARY_DIR}/coverage_base.info"
)

# --------------------------------------------------------------
# Run AFTER executing tests to report coverage
# --------------------------------------------------------------
add_custom_target(
    coverage-capture
    COMMAND lcov --capture
    --base-directory ${CMAKE_CURRENT_SOURCE_DIR}
    --directory ${CMAKE_CURRENT_BINARY_DIR}
    --output-file "${CMAKE_CURRENT_BINARY_DIR}/coverage.info"
    # combine coverage baseline with captured results
    COMMAND lcov
    --add-tracefile "${CMAKE_CURRENT_BINARY_DIR}/coverage_base.info"
    --add-tracefile "${CMAKE_CURRENT_BINARY_DIR}/coverage.info"
    --output-file "${CMAKE_CURRENT_BINARY_DIR}/coverage_total.info"
    # remove system and test code from coverage
    COMMAND lcov
    --remove "${CMAKE_CURRENT_BINARY_DIR}/coverage_total.info"
    # paths to be removed must be enclosed by SINGLE quotes
    '/usr/*' '*/lib/*' '*/tests/*'
    --output-file "${CMAKE_CURRENT_BINARY_DIR}/coverage_filtered.info"
)

# --------------------------------------------------------------
# Run AFTER coverage-capture to generate HTML report
# --------------------------------------------------------------
add_custom_target(
    coverage-report
    COMMAND genhtml
    "${CMAKE_CURRENT_BINARY_DIR}/coverage_filtered.info"
    --output-directory "${CMAKE_CURRENT_BINARY_DIR}/coverage"
)
