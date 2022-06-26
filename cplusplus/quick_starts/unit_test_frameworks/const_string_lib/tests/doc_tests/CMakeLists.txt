cmake_minimum_required(VERSION 3.23 FATAL_ERROR)

add_executable(const_string_doc_tests src/const_string_tests.cpp)
target_link_libraries(
  const_string_doc_tests
  PRIVATE doctest::doctest
  const_str
)

include(Catch)
catch_discover_tests(const_string_doc_tests)
