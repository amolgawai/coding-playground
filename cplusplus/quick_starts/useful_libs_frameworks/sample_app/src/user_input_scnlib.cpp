#include "cli_io.hpp"
#include "run_examples.hpp"
#include <vector>

void run_user_input_examples() {

  cli_io::show_msg_subtitle("Running integer list user input examples\n");

  auto numbers =
      cli_io::get_numbers<uint8_t>("input integers separated by space: ");

  if (!numbers.empty()) {
    cli_io::show_msg_info("You entered a list of integers as following\n {}",
                          numbers);
  } else {
      cli_io::show_msg_error("Invalid input");
  }
}
