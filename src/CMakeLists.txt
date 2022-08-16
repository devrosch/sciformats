set(HEADER_LIST
    # public data providers
    "${PROJECT_SOURCE_DIR}/include/jdx/JdxParser.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/TextReader.hpp"
    # util (private headers)
    "${PROJECT_SOURCE_DIR}/src/util/StringUtils.hpp"
    "${PROJECT_SOURCE_DIR}/src/util/LdrUtils.hpp"
    "${PROJECT_SOURCE_DIR}/src/util/DataParser.hpp"
    "${PROJECT_SOURCE_DIR}/src/util/PeakTableParser.hpp"
    "${PROJECT_SOURCE_DIR}/src/util/PeakAssignmentsParser.hpp"
    # ldr
    "${PROJECT_SOURCE_DIR}/include/jdx/LdrContainer.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/Block.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/Ldr.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/StringLdr.hpp"
    # data
    "${PROJECT_SOURCE_DIR}/include/jdx/DataLdr.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/NTuples.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/NTuplesVariables.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/NTuplesPage.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/DataTable.hpp"
    # 2D data
    "${PROJECT_SOURCE_DIR}/include/jdx/XyParameters.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/RaParameters.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/Array2DData.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/XyData.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/RaData.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/XyPoints.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/XyBase.hpp"
    # tabular data
    "${PROJECT_SOURCE_DIR}/include/jdx/TabularData.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/Peak.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/PeakTable.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/PeakAssignment.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/PeakAssignments.hpp"
    # exception
    "${PROJECT_SOURCE_DIR}/include/jdx/ParseException.hpp"
    "${PROJECT_SOURCE_DIR}/include/jdx/BlockParseException.hpp"
    )

# Make an automatic library - will be static or dynamic based on user setting
add_library(jdxparser
    # public data providers
    JdxParser.cpp
    TextReader.cpp
    # util
    util/StringUtils.cpp
    util/LdrUtils.cpp
    util/DataParser.cpp
    util/PeakTableParser.cpp
    util/PeakAssignmentsParser.cpp
    # ldr
    Block.cpp
    LdrContainer.cpp
    Ldr.cpp
    StringLdr.cpp
    # data
    DataLdr.cpp
    NTuples.cpp
    NTuplesPage.cpp
    DataTable.cpp
    # 2D data
    Array2DData.cpp
    XyData.cpp
    RaData.cpp
    XyPoints.cpp
    XyBase.cpp
    # tabular data
    TabularData.cpp
    PeakTable.cpp
    PeakAssignments.cpp
    # exception
    ParseException.cpp
    BlockParseException.cpp
    ${HEADER_LIST})

# All users of this library will need at least C++17
target_compile_features(jdxparser PUBLIC cxx_std_17)

if (CMAKE_COMPILER_IS_GNUCXX AND CMAKE_CXX_COMPILER_VERSION VERSION_LESS 9)
    # make filesystem available
    # see: https://stackoverflow.com/questions/33149878/experimentalfilesystem-linker-error
    message(STATUS "Linking stdc++fs to GNUCXX version: ${CMAKE_CXX_COMPILER_VERSION}")
    target_link_libraries(jdxparser stdc++fs)
endif()

# public headers
target_include_directories(jdxparser PUBLIC ${PROJECT_SOURCE_DIR}/include)
# private headers
target_include_directories(jdxparser PRIVATE ${PROJECT_SOURCE_DIR}/src)

# --------------------------------------------------------------
# Use include-what-you-use if available => deactivated due to some iwyu issues
# --------------------------------------------------------------
#find_program(iwyu_path NAMES include-what-you-use iwyu REQUIRED)
# if(NOT iwyu_path)
#   message(FATAL_ERROR "Could not find the program include-what-you-use")
# endif()
#set_property(TARGET jdxparser PROPERTY CXX_INCLUDE_WHAT_YOU_USE ${iwyu_path})
