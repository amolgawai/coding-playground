
#include "cli_options.hpp"
#include "cli_io.hpp"
#include "run_examples.hpp"

auto main(int argc, char** argv) -> int {

    CLIOptions options{"A program to demonstrate useful c++ libraries and frameworks!"};

    if (!options.scan_options(argc, argv)) {
        return 0;
    }

    cli_io::show_msg_title("Hello popular c++ libs and framework");

    run_user_input_examples();

    return 0;
}
