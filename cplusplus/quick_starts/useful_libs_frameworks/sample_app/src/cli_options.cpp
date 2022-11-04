#include "cli_options.hpp"
#include <cxxopts.hpp>

bool CLIOptions::scan_options(int argc, char **argv) {
    cxxopts::Options opts{*argv, app_des};

    // clang-format off
  opts.add_options()
    ("h,help", "Show help")
    ("v,version", "Print the current version number")
  ;
    // clang-format on

    auto result = opts.parse(argc, argv);

    if (result["help"].as<bool>()) {
        std::cout << opts.help() << std::endl;
        return false;
    }

    if (result["version"].as<bool>()) {
        std::cout << "1.0.0" << std::endl;
        return false;
    }

    return true;
}
