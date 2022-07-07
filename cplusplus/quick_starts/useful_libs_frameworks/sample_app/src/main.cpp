
#include <cxxopts.hpp>
#include <iostream>

auto main(int argc, char** argv) -> int {

    cxxopts::Options options(*argv, "A program to welcome the world!");

    // clang-format off
  options.add_options()
    ("h,help", "Show help")
    ("v,version", "Print the current version number")
  ;
    // clang-format on

    auto result = options.parse(argc, argv);

    if (result["help"].as<bool>()) {
        std::cout << options.help() << std::endl;
        return 0;
    }

    if (result["version"].as<bool>()) {
        std::cout << "1.0.0" << std::endl;
        return 0;
    }

    std::cout << "Hello popular c++ libs and framework" << std::endl;

    return 0;
}
