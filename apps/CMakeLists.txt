# Tests need to be added as executables first
add_executable(sciwrap_main
    Main.cpp
    )

# I'm using C++17 in the test
target_compile_features(sciwrap_main PRIVATE cxx_std_17)

# Should be linked to the library, as well as the Catch2 testing library
target_link_libraries(sciwrap_main PRIVATE sciwrap)

