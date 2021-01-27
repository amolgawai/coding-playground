"""Progression base class.
which does most of the work according to the framework design pattern

Ref - https://www.youtube.com/watch?v=S_ipdVNSFlo
Book - Data Structures and Algorithms in Python, Goodrich, Tamassia, Goldwasser
"""
from abc import ABC, abstractmethod


class Progression(ABC):
    """Iterator producing a Progression

    This base class does all the work and enforces the _advance method
    implementation
    """

    def __init__(self, start=0):
        "Initialize the class with start of progression"

        self._current = start

    @abstractmethod
    def _advance(self):
        """Advance to the next value in profression """

        self._current += 1

    def __next__(self):
        """Return the net element, Raise StopIteration if at end of progression"""
        if self._current is None:
            raise StopIteration()

        ret_val = self._current
        self._advance()
        return ret_val

    def __iter__(self):
        """As this is an Iterator, it returns self"""

        return self
