#include "run_examples.hpp"
#include <iostream>
#include <scn/scn.h>
#include <vector>

void run_user_input_examples() {

  std::cout << "Running integer list user input examples\n";

  std::cout << "input integers separated by space: ";
  std::string line{};
  std::ignore = scn::getline(scn::cstdin(), line);

  std::vector<int> vec{};
  auto result = scn::scan_list(line, vec);

  if (result.empty()) {

    std::cout << "You entered a list of integers as following\n";
    for (auto const &i : vec) {
      std::cout << i << " ";
    }
  }

  else {
    std::cerr << "Invalid input\n";
  }
}
