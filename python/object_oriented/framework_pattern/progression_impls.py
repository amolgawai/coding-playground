"""Implementations of Progression"""
from progression import Progression
import itertools


class ArithmeticProgression(Progression):
    """Iterator producign Arithmetic Progression"""

    def __init__(self, increment=1, start=0):
        "Initialise with a start and increment value"

        super().__init__(start)
        self._increment = increment

    def _advance(self):
        """Advance the progression"""

        self._current += self._increment


class GeometricProgression(Progression):
    """The gemetric Progression iterator"""

    def __init__(self, base=2, start=1):
        """Initialise with base and start
        Progression is multiple of base"""

        super().__init__(start)
        self._base = base

    def _advance(self):
        """Advance the progression"""

        self._current *= self._base


class FibonacciProgression(Progression):
    """The Fionacci Progression Iterator"""

    def __init__(self, first=0, second=1):
        "Initialise Fibonacci series with the first two numbers"
        super().__init__(first)
        self._prev = second - first

    def _advance(self):
        """Advance the Pogression"""

        self._prev, self._current = self._current, self._prev + self._current


if __name__ == "__main__":
    progs = [ArithmeticProgression(), GeometricProgression(), FibonacciProgression()]
    for a_prog in progs:
        print(*(ele for ele in itertools.islice(a_prog, 10)))
