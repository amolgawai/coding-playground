"""
Various aspects of print function and printing strings
Note - Python 3 is required
ref - https://realpython.com/lessons/basic-usage-and-string-literals/
"""

import math
import time


def printSeparator():
    print(70 * "=")


print(__doc__)
printSeparator()
print("escaping with backslash")
print("new lines\nand\n\ttabs too")
print("Octal 275 is half: \275")
print("hex 0xBD is Octal 275: \xBD")
printSeparator()
print("string formatting")
# c-style
name = "James"
sirname = "Bond"
age = 34
print("Hello Mr. %s %s" % (name, sirname))
print("Mr bond is %d years old" % age)
print("pi: %f \nshort pi %0.2f" % (math.pi, math.pi))
# format with python 3.0
print("My name is {1}, {0} {1}".format("James", "Bond"))
# f-string - python 3.6
print(f"I am {age} years old")
print(f"f-string support arithmetic as in 5*pi is {math.pi*5}")
print(f"f-string also support formatting as in 5*pi till two digits is {math.pi*5:.2f}")

printSeparator()
# sep, flush
print(
    "different separator in print instead of default ' '",
    "a newline",
    6,
    "e.g.",
    sep="\n",
)
data = [["year", "last", "first"], [1943, "Cat", "Catty"], [1945, "Dog", "Dogbert"]]
for row in data:
    print(*row, sep=",")
# end, flush
def count_items(items):
    print("Counting", end="", flush=True)  # don't add \n but still flush the print
    num = 0
    for item in items:
        num += 1
        time.sleep(1)
        print(".", end="", flush=True)
    print(f"\nThere were {num} items")


count_items(data)
printSeparator()
