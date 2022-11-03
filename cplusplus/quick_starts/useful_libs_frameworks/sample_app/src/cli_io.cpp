#include "cli_io.hpp"
#include <fmt/color.h>
#include <fmt/core.h>

#include <string>

namespace cli_io {
void show_msg_title(const std::string &msg) {
  fmt::print(fg(fmt::color::crimson) | fmt::emphasis::bold |
                 fmt::emphasis::underline,
             "{}\n", msg);
}

void show_msg_subtitle(const std::string &msg) {
  fmt::print(fg(fmt::color::dark_golden_rod) | fmt::emphasis::italic, "{}\n",
             msg);
}

void show_msg_error(const std::string &msg) {
  fmt::print(fg(fmt::color::orange_red), "{}\n", msg);
}
} // namespace cli_io
