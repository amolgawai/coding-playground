#! /usr/bin/env python3
"""
methods, attributes starting with two underscores are mangled so that they
can't be overridden from subclasses
"""

_ManglingTest__global_var = 23


class ManglingTest():
    def __init__(self):
        self.__mangled = 'Hello'

    def __mangled_method(self):
        return 42

    def get_mangled_var(self):
        return self.__mangled

    def call_mangled(self):
        return self.__mangled_method()

    def global_becomes_member(self):
        return __global_var


def main():
    print("Testing name mangling on a class")
    the_class = ManglingTest()
    try:
        the_class.__mangled
    except AttributeError:
        print("No attribute named __mangled")
    print(f"But inside class works {the_class.get_mangled_var()}")

    try:
        the_class.__mangled_method()
    except AttributeError:
        print("Can't call a mangled method")
    print(f"But inside class works {the_class.call_mangled()}")

    print(
        f"Global variable will become class member {the_class.global_becomes_member()}"
    )


if __name__ == "__main__":
    main()

