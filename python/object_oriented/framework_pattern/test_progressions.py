"""Tests for Progression class hierarchy"""
import itertools
import pytest
from progression import Progression
from progression_impls import (
    ArithmeticProgression,
    GeometricProgression,
    FibonacciProgression,
)


# @pytest.fixture
def get_progrssions():
    """Gets a list of concrete progressions"""
    return [
        ArithmeticProgression(),
        GeometricProgression(),
        FibonacciProgression(),
    ]


def test_abstract_exception():
    """Tests that the abstract class can't be instantiated """

    msg = "Can't instantiate abstract class Progression with abstract method _advance"
    with pytest.raises(TypeError, match=msg):
        Progression()


@pytest.mark.parametrize("progression", get_progrssions())
def test_progression_impls(progression):
    """Tests the implementations of Progression class"""

    assert isinstance(progression, Progression)


@pytest.mark.parametrize("progression", get_progrssions())
def test_progression_impls_iteration(progression):
    """Tests that progressions are iterable"""

    for a_num in itertools.islice(progression, 10):
        assert a_num is not None
