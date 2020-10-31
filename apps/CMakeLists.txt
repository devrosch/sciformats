find_package(Boost REQUIRED COMPONENTS locale)
find_package(ICU REQUIRED COMPONENTS uc)

add_executable(main main.cpp)

target_compile_features(main PRIVATE cxx_std_17)

#target_link_libraries(main PRIVATE binary_reader)
#target_link_libraries(main PRIVATE Boost::locale binary_reader)
target_link_libraries(main PRIVATE Boost::locale ICU::uc binary_reader)
