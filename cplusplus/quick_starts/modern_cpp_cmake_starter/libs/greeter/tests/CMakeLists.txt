cmake_minimum_required(VERSION 3.23 FATAL_ERROR)

# ---- Options ----

option(ENABLE_TEST_COVERAGE "Enable test coverage" OFF)
option(TEST_INSTALLED_VERSION "Test the version found by find_package" OFF)

# ---- Create binary ----

file(GLOB sources CONFIGURE_DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp)
add_executable(greeter_tests ${sources})
target_link_libraries(greeter_tests doctest::doctest greeter::greeter)
target_compile_features(greeter_tests PRIVATE cxx_std_17)

# enable compiler warnings
if(NOT TEST_INSTALLED_VERSION)
  if(CMAKE_CXX_COMPILER_ID MATCHES "Clang" OR CMAKE_CXX_COMPILER_ID MATCHES "GNU")
    target_compile_options(greeter PUBLIC -Wall -Wpedantic -Wextra -Werror)
  elseif(MSVC)
    target_compile_options(greeter PUBLIC /W4 /WX)
    target_compile_definitions(greeter_tests PUBLIC DOCTEST_CONFIG_USE_STD_HEADERS)
  endif()
endif()

# ---- Add GreeterTests ----

add_test(greeter_tests greeter_tests)

# Note: doctest and similar testing frameworks can automatically configure CMake tests. For other
# testing frameworks add the tests target instead: add_test(NAME ${PROJECT_NAME} COMMAND
# ${PROJECT_NAME})

include(${doctest_SOURCE_DIR}/scripts/cmake/doctest.cmake)
doctest_discover_tests(greeter_tests)

# ---- code coverage ----

if(ENABLE_TEST_COVERAGE)
  target_compile_options(greeter PUBLIC -O0 -g -fprofile-arcs -ftest-coverage)
  target_link_options(greeter PUBLIC -fprofile-arcs -ftest-coverage)
endif()
