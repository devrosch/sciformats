set(HEADER_LIST
    "${PROJECT_SOURCE_DIR}/include/jdx/JdxParser.hpp"
    # util
    "${PROJECT_SOURCE_DIR}/include/jdx/util/LdrUtils.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/util/DataParser.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/util/PeakTableParser.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/util/PeakAssignmentsParser.hpp"
    # ldr
    "${PROJECT_SOURCE_DIR}/include/jdx/Block.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/StringLdr.hpp"
    # data
    "${PROJECT_SOURCE_DIR}/include/jdx/DataLdr.hpp"
    # 2D data
    "${PROJECT_SOURCE_DIR}/include/jdx/XyParameters.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/RaParameters.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/Data2D.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/XyData.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/RaData.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/XyPoints.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/XyBase.hpp"
    # tabular data
    "${PROJECT_SOURCE_DIR}/include/jdx/Peak.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/PeakTable.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/PeakAssignment.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/PeakAssignments.hpp"
    )

# Make an automatic library - will be static or dynamic based on user setting
add_library(jdxparser
    JdxParser.cpp
    # util
    util/LdrUtils.cpp
    util/DataParser.cpp
    util/PeakTableParser.cpp
    util/PeakAssignmentsParser.cpp
    # ldr
    Block.cpp
    StringLdr.cpp
    # data
    DataLdr.cpp
    # 2D data
    Data2D.cpp
    XyData.cpp
    RaData.cpp
    XyPoints.cpp
    XyBase.cpp
    # tabular data
    Peak.cpp
    PeakTable.cpp
    PeakAssignment.cpp
    PeakAssignments.cpp
    ${HEADER_LIST})

# All users of this library will need at least C++17
target_compile_features(jdxparser PUBLIC cxx_std_17)

# We need this directory, and users of our library will need it too
target_include_directories(jdxparser PUBLIC ${PROJECT_SOURCE_DIR}/include)
