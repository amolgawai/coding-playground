cmake_minimum_required(VERSION 3.23 FATAL_ERROR)

# ---- Options ----

option(ENABLE_TEST_COVERAGE "Enable test coverage" OFF)
option(TEST_INSTALLED_VERSION "Test the version found by find_package" OFF)


# ---- Create binary ----

add_executable(const_string_boost_tests src/const_string_tests.cpp)
target_compile_features(const_string_boost_tests PRIVATE cxx_std_17)
# target_link_libraries(const_string_boost_tests Boost::Boost const_str)
target_link_libraries(const_string_boost_tests const_str)

add_test(NAME const_str_boost_test COMMAND const_string_boost_tests)
