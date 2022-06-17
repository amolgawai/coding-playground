cmake_minimum_required(VERSION 3.23 FATAL_ERROR)

# ---- Options ----

option(ENABLE_TEST_COVERAGE "Enable test coverage" OFF)
option(TEST_INSTALLED_VERSION "Test the version found by find_package" OFF)

# ---- Create binary ----

add_executable(const_string_boost_tests src/const_string_tests.cpp)
target_link_libraries(const_string_boost_tests doctest::doctest const_string::const_string)
target_compile_features(const_string_boost_tests PRIVATE cxx_std_17)
