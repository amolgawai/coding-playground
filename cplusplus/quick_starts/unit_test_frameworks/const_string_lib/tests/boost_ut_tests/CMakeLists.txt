cmake_minimum_required(VERSION 3.23 FATAL_ERROR)

add_executable(const_string_boostut_tests src/const_string_tests.cpp)
target_link_libraries(
  const_string_boostut_tests
  PRIVATE Boost::ut
  const_str
)

add_test(NAME boostut_tests COMMAND const_string_boostut_tests)
