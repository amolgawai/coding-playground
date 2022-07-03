#include <boost/ut.hpp>

#include "const_string.h"

int main() {
  using namespace boost::ut;
  "constructors"_test =
      [] {
        using namespace std::literals::string_literals;
        const_string cs0{""};  // 1 //
        expect(cs0.length() == (size_t)0);
        expect(cs0.is_empty());

        const_string cs01{NULL};  // 2 //
        expect(cs01.length() == (size_t)0);
        expect(cs01.is_empty());

        const_string cs1{"test_string"};  // 3 //
        expect(cs1.data() == "test_string"s);
        expect(cs1.length() == std::strlen("test_string"));

        std::string s{"test_string"};  // 4 //
        const_string cs2(s);
        expect(std::strncmp(cs2.data(), "test_string", cs2.length()) == 0);
        // expect(cs2.data() == "test_string");

        const_string cs3{cs1};  // 5 //
        expect(cs3.data() == "test_string"s);

        const_string cs4{"test_string", 4};  // 6 //
        expect(std::strncmp(cs4.data(), "test", cs4.length()) == 0);

        const_string cs5{s.data(), s.data() + s.length()};  // 7 //
        expect(std::strncmp(cs5.data(), "test_string", cs5.length()) == 0);

        const_string cs_array[] = {"str1", "str2"};  // 8 //
        expect(cs_array[0].data() == "str1"s);
        expect(cs_array[1].data() == "str2"s);
      };

  return 0;
}
