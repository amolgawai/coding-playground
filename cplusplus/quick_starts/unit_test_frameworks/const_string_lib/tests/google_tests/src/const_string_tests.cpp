#include <gtest/gtest.h>
#include "const_string.h"

TEST(const_string_tests, constructors_test) {
  const_string cs0{ "" };  // 1 //
  EXPECT_EQ(cs0.length(), (size_t)0);
  EXPECT_TRUE(cs0.is_empty());

  const_string cs01{ NULL };  // 2 //
  EXPECT_EQ(cs01.length(), (size_t)0);
  EXPECT_TRUE(cs01.is_empty());

  const_string cs1{ "test_string" };  // 3 //
  EXPECT_STRCASEEQ(cs1.data(), "test_string");
  EXPECT_EQ(cs1.length(), std::strlen("test_string"));

  std::string s{ "test_string" };  // 4 //
  const_string cs2(s);
  EXPECT_STRCASEEQ(cs2.data(), "test_string");

  const_string cs3{ cs1 };  // 5 //
  EXPECT_STRCASEEQ(cs1.data(), "test_string");

  const_string cs4{ "test_string", 4 };  // 6 //
  EXPECT_EQ (std::strncmp(cs4.data(), "test", cs4.length()), 0);

  const_string cs5{ s.data(), s.data() + s.length() };  // 7 //
  EXPECT_EQ (std::strncmp(cs5.data(), "test_string", cs5.length()), 0);

  const_string cs_array[] = {"str1", "str2"};  // 8 //
  EXPECT_STRCASEEQ(cs_array[0].data(), "str1");
  EXPECT_STRCASEEQ(cs_array[1].data(), "str2");
}
