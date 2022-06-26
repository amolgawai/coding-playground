cmake_minimum_required(VERSION 3.23 FATAL_ERROR)

add_executable(const_string_google_tests src/const_string_tests.cpp)
target_link_libraries(
  const_string_google_tests
  gtest_main
  const_str
)

include(GoogleTest)
gtest_discover_tests(const_string_google_tests)
