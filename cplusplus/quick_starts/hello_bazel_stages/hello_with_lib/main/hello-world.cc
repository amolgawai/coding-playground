#include "hello-greet.h"
#include "lib/hello-time.h"
#include <iostream>
#include <string>

int main(int argc, char **argv) {
  std::string who = "World";

  if (argc > 1) {
    who = argv[1];
  }

  std::cout << get_greet(who) << std::endl;
  print_localtime();

  return 0;
}
