#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#define DOCTEST_CONFIG_SUPER_FAST_ASSERTS
#include <doctest/doctest.h>
#include "const_string.h"

TEST_CASE( "const_string constructor test") {
  const_string cs0{ "" };  // 1 //
  REQUIRE(cs0.length() == (size_t)0);
  REQUIRE(cs0.is_empty());

  const_string cs01{ NULL };  // 2 //
  REQUIRE(cs01.length() == (size_t)0);
  REQUIRE(cs01.is_empty());

  const_string cs1{ "test_string" };  // 3 //
  REQUIRE(cs1.data() == "test_string");
  REQUIRE(cs1.length() == std::strlen("test_string"));

  std::string s{ "test_string" };  // 4 //
  const_string cs2(s);
  REQUIRE (std::strncmp(cs2.data(), "test_string", cs2.length()) == 0);
  // REQUIRE(cs2.data() == "test_string");

  const_string cs3{ cs1 };  // 5 //
  REQUIRE(cs3.data() == "test_string");

  const_string cs4{ "test_string", 4 };  // 6 //
  REQUIRE (std::strncmp(cs4.data(), "test", cs4.length()) == 0);

  const_string cs5{ s.data(), s.data() + s.length() };  // 7 //
  REQUIRE (std::strncmp(cs5.data(), "test_string", cs5.length()) == 0);

  const_string cs_array[] = {"str1", "str2"};  // 8 //
  REQUIRE(cs_array[0].data() == "str1");
  REQUIRE(cs_array[1].data() == "str2");
}
