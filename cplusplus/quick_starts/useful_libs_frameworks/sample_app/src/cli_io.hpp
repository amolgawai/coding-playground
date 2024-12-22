#ifndef CLI_IO_HPP
#define CLI_IO_HPP

#include <fmt/core.h>
#include <fmt/ranges.h>
#include <fmt/format.h>
#include <fmt/color.h>
#include <scn/scn.h>

#include <string>
#include <iostream>
#include <sstream>
#include <vector>

// stringstream can't handle uint_8, see following
// https://stackoverflow.com/questions/54627170/read-uint8-t-from-stdstingstream-as-a-numeric-type

namespace cli_io {

// show messages to user
void show_msg_title(const std::string &msg);
void show_msg_subtitle(const std::string &msg);
void show_msg_error(const std::string &msg);
template <typename... T>
void show_msg_info(fmt::format_string<T...> format, T &&...args) {
  fmt::print(format, args...);
};

// Get input from user
template <typename NUM_TYP>
std::vector<NUM_TYP> get_numbers(const std::string &msg) {
    fmt::print(fg(fmt::color::steel_blue) | fmt::emphasis::italic, "{}\n", msg);

    std::vector<NUM_TYP> input_nums;
    auto result = scn::scan_list_ex(scn::cstdin(), input_nums, scn::list_until('\n'));
    if (!result) {
        show_msg_error("Error occured during reading");
    }
    return input_nums;
};

} // namespace cli_io

#endif
