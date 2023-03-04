"""Demonstration of integer division and partial functions.

Original example taken from the book -
Python Programming Exercises, Gently Explained
EXERCISE #11: HOURS, MINUTES, SECONDS
"""
from functools import partial


def seconds_converter(seconds, hms_list, divisor, str_char):
    """converts seconds to hours or mnutes and appends to string"""
    if seconds >= divisor:
        hours_or_mins, seconds = divmod(seconds, divisor)
        hms_list.append(str(hours_or_mins) + str_char)

    return seconds


sec_to_hrs = partial(seconds_converter, divisor=3600, str_char='h')
sec_to_mins = partial(seconds_converter, divisor=60, str_char='m')


def convert_seconds_to_hrs_min_secs(totalSeconds):
    # If totalSeconds is 0, just return '0s':
    if totalSeconds == 0:
        return '0s'

    # Create an hms list that contains the string hour/minute/second amounts:
    hms = []

    totalSeconds = sec_to_hrs(totalSeconds, hms)
    totalSeconds = sec_to_mins(totalSeconds, hms)

    # If there are one or more seconds, add the amount with an 's' suffix:
    if totalSeconds:
        hms.append(str(totalSeconds) + 's')

    # Join the hour/minute/second strings with a space in between them:
    return ' '.join(hms)


if __name__ == "__main__":
    print(convert_seconds_to_hrs_min_secs(30))


def test_multi():
    assert convert_seconds_to_hrs_min_secs(30) == '30s'
    assert convert_seconds_to_hrs_min_secs(60) == '1m'
    assert convert_seconds_to_hrs_min_secs(90) == '1m 30s'
    assert convert_seconds_to_hrs_min_secs(3600) == '1h'
    assert convert_seconds_to_hrs_min_secs(3601) == '1h 1s'
    assert convert_seconds_to_hrs_min_secs(3661) == '1h 1m 1s'
    assert convert_seconds_to_hrs_min_secs(90042) == '25h 42s'
    assert convert_seconds_to_hrs_min_secs(0) == '0s'

