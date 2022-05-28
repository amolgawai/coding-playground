import time
from datetime import datetime


def sleep_awhile(duration):
    """sleep for couple of seconds"""
    time.sleep(duration)
    # some other processing steps


def get_time_of_day():
    """return string Night/Morning/Afternoon/Evening depending on the hours range"""
    time = datetime.now()
    if 0 <= time.hour < 6:
        return "Night"
    if 6 <= time.hour < 12:
        return "Morning"
    if 12 <= time.hour < 18:
        return "Afternoon"
    return "Evening"
