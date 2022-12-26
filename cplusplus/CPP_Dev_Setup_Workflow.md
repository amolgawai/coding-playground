# C++ Development Tools and Workflow

## Introduction
This describes development setup for C++

## Compilers
- Primary - clang++ from [The llvm Compiler Infrastructure](https://llvm.org)
- Secondary - g++ from [Gnu Compile Collection](https://gcc.gnu.org)

## Build system
- [bazel](https://bazel.build/)
- [CMake](https://cmake.org/)


## Formatting with clang-format
Create a format config file
``` clang-format -style=llvm -dump-config > .clang-format```
Available styles are
* llvm -  A style complying with the LLVM coding standards
* Google A style complying with Google’s C++ style guide
* Chromium A style complying with Chromium’s style guide
* Mozilla A style complying with Mozilla’s style guide
* WebKit A style complying with WebKit’s style guide
* Microsoft A style complying with Microsoft’s style guide
[Reference Documentation](https://clang.llvm.org/docs/ClangFormatStyleOptions.html)
