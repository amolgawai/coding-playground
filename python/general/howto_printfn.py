"""
Various aspects of print function and printing strings
Note - Python 3 is required
ref - https://realpython.com/lessons/basic-usage-and-string-literals/
"""

import math
import time


def print_separator(sep_char="="):
    """Prints a line 70 chars wide"""
    print(70 * sep_char)


print(__doc__)
print_separator()
print("escaping with backslash")
print("new lines\nand\n\ttabs too")
print("Octal 275 is half: \275")
print("hex 0xBD is Octal 275: \xBD")
print_separator()
print("string formatting")
# c-style
NAME = "James"
SURNAME = "Bond"
AGE = 34
print("Hello Mr. %s %s" % (NAME, SURNAME))
print("Mr bond is %d years old" % AGE)
print("pi: %f \nshort pi %0.2f" % (math.pi, math.pi))
# format with python 3.0
print("My name is {1}, {0} {1}".format("James", "Bond"))
# f-string - python 3.6
print(f"I am {AGE} years old")
print(f"f-string support arithmetic as in 5*pi is {math.pi*5}")
print(
    f"f-string also support formatting as in 5*pi till two digits is {math.pi*5:.2f}"
)

print_separator()
# sep, flush
print(
    "different separator in print instead of default ' '",
    "a newline",
    6,
    "e.g.",
    sep="\n",
)
data = [
    ["year", "last", "first"],
    [1943, "Cat", "Catty"],
    [1945, "Dog", "Dogbert"],
]
for row in data:
    print(*row, sep=",")
# end, flush
def count_items(items):
    """Counts items in the list"""
    print(
        "Counting", end="", flush=True
    )  # don't add \n but still flush the print
    num = 0
    for _ in items:
        num += 1
        time.sleep(1)
        print(".", end="", flush=True)
    print(f"\nThere were {num} items")


count_items(data)
print_separator()
