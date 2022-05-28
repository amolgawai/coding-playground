import pytest
from src.calc import add


def test_add_simple():
    # Arrange
    a = 2
    b = 5
    expected = 7

    # Act
    output = add(a, b)

    # Assert
    assert output == expected


@pytest.mark.parametrize(
    "a,b,expected", [(10, 5, 15), (-1, 1, 0), (-1, -1, -2)]
)
def test_add_param(a, b, expected):
    assert add(a, b) == expected
