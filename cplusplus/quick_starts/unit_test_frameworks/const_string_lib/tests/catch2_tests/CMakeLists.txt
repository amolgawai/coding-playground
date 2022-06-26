cmake_minimum_required(VERSION 3.23 FATAL_ERROR)

add_executable(const_string_catch2_tests src/const_string_tests.cpp)
target_link_libraries(
  const_string_catch2_tests
  PRIVATE Catch2::Catch2WithMain
  const_str
)

include(Catch)
catch_discover_tests(const_string_catch2_tests)
