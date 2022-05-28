import pytest
from datetime import datetime
from src.example_for_mocking import (
    process_data,
    sleep_awhile,
    get_time_of_day,
    process_data,
    Engine,
    use_env_var,
)


def test_sleep_awhile(mocker):
    m = mocker.patch("src.example_for_mocking.time.sleep", return_value=None)
    sleep_awhile(3)
    m.assert_called_once_with(3)


def test_get_time_of_day(mocker):
    mock_now = mocker.patch("src.example_for_mocking.datetime")
    mock_now.now.return_value = datetime(2016, 5, 20, 14, 10, 0)
    assert get_time_of_day() == "Afternoon"


@pytest.mark.parametrize(
    "datetime_obj, expect",
    [
        (datetime(2016, 5, 20, 0, 0, 0), "Night"),
        (datetime(2016, 5, 20, 1, 10, 0), "Night"),
        (datetime(2016, 5, 20, 6, 10, 0), "Morning"),
        (datetime(2016, 5, 20, 12, 0, 0), "Afternoon"),
        (datetime(2016, 5, 20, 14, 10, 0), "Afternoon"),
        (datetime(2016, 5, 20, 18, 0, 0), "Evening"),
        (datetime(2016, 5, 20, 19, 10, 0), "Evening"),
    ],
)
def test_get_time_of_day_param(datetime_obj, expect, mocker):
    mock_now = mocker.patch("src.example_for_mocking.datetime")
    mock_now.now.return_value = datetime_obj

    assert get_time_of_day() == expect


def test_process_data(mocker):
    mocker.patch(
        "src.example_for_mocking.load_data",
        return_value={"key1": "valy", "key2": "val2"},
    )
    assert process_data() == "valy"


def test_engine_load_data(mocker):
    mocker.patch(
        "src.example_for_mocking.DBConnector.__init__", return_value=None
    )
    mocker.patch("src.example_for_mocking.DBConnector.get", return_value="xyz")
    output = Engine().load_data()
    assert output == "xyzxxx"


@pytest.mark.parametrize(
    "mock_contract_class,expect",
    [("en_cloud", "this is en_cloud"), ("en_onprem", "this is en_onprem")],
)
def test_mock_env_var(mock_contract_class, expect, monkeypatch):
    # more about monkeypatch
    # https://docs.pytest.org/en/6.2.x/monkeypatch.html
    monkeypatch.setenv("CONTRACT_CLASS", mock_contract_class)
    assert use_env_var() == expect


def test_exception(monkeypatch):
    monkeypatch.setenv("CONTRACT_CLASS", "something not existed")
    with pytest.raises(
        ValueError, match=r"contract class something not existed not found"
    ):
        use_env_var()
