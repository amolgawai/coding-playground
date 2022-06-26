#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers_string.hpp>
#include "const_string.h"

TEST_CASE( "const_string_tests", "[constructors_test]") {
  const_string cs0{ "" };  // 1 //
  REQUIRE(cs0.length() == (size_t)0);
  REQUIRE(cs0.is_empty());

  const_string cs01{ NULL };  // 2 //
  REQUIRE(cs01.length() == (size_t)0);
  REQUIRE(cs01.is_empty());

  const_string cs1{ "test_string" };  // 3 //
  REQUIRE_THAT(cs1.data(), Catch::Matchers::Equals("test_string"));
  REQUIRE(cs1.length() == std::strlen("test_string"));

  std::string s{ "test_string" };  // 4 //
  const_string cs2(s);
  REQUIRE_THAT(cs2.data(), Catch::Matchers::Equals("test_string"));

  const_string cs3{ cs1 };  // 5 //
  REQUIRE_THAT(cs3.data(), Catch::Matchers::Equals("test_string"));

  const_string cs4{ "test_string", 4 };  // 6 //
  REQUIRE (std::strncmp(cs4.data(), "test", cs4.length()) == 0);

  const_string cs5{ s.data(), s.data() + s.length() };  // 7 //
  REQUIRE (std::strncmp(cs5.data(), "test_string", cs5.length()) == 0);

  const_string cs_array[] = {"str1", "str2"};  // 8 //
  REQUIRE_THAT(cs_array[0].data(), Catch::Matchers::Equals("str1"));
  REQUIRE_THAT(cs_array[1].data(), Catch::Matchers::Equals("str2"));
}
