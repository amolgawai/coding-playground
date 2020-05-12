"""
Overview of features introduced in python 3.8
ref - https://realpython.com/lessons/cool-new-features-python-38-overview/
"""

from typing import Literal

# The walrus operator - assignment expressions
# := -> Allows assignment and return of value in same expression
# more examples - https://www.python.org/dev/peps/pep-0572/#examples
print(walrus := True)
print(type(walrus))

inputs = list()
while (current := input('Write something, type quit to quit - ')) != 'quit':
    inputs.append(current)
print(f"you said {inputs}")

# positional only arguments
def greet(name, /, greeting = 'Hello'): # the '/' separates positional only and named args
    print(f"{greeting} {name}")
greet("Jon")
greet("Jon", greeting = "Hi")
# greet(name = "Jon") - does not work
# keyword only arguments are available in python 3 which can be separated by '*'
def headline(text, /, border="~", *, width=50):
    """Example of regular positional-only and keyword-only args

    :param text: positional-only
    :param border: regular
    :param width: keyword-only
    :returns: formatted headline
    :rtype: text

    """
    return f" {text} ".center(width, border)

# Since text is positional-only, you can’t use the keyword text:
print(headline("Positional-only Arguments"))
# headline(text="This doesn't work!")

# border, on the other hand, can be specified both with and without the keyword:
print(headline("Python 3.8", "="))
print(headline("Real Python", border=":"))

# Finally, width must be specified using the keyword:
print(headline("Python", "@", width=38))
# headline("Python", "@", 38) - This doesn't work

# More precise types
# type checking as introduced in 3.5
def double_it(number: float) -> float:
	return 2*number
# check with mypy on cli
double_it(10.5) # passes mypy
double_it("type check failed") # mypy error
# 3.8 introduced 'Literal' type which restricts possible values of an argument
# or return
def draw_line(direction: Literal["horizontal", "vertical"]) -> None:
	if direction == "horizontal":
		print("Drawing horizontal line")
	elif direction == "vertical":
		print("Drawing vertical line")
	else:
		raise ValueError(f"Invalid direction {direction!r}")

# following shall produce error with mypy
draw_line("up")
