"""Examples for unit testing
Note - Python 3.5+
"""

from unittest.mock import Mock
import json

# Mocking
the_mock = Mock()
print(the_mock)

# mock json
json = Mock()
json.dumps({'a':1})
json.dumps({'a':1})
json.loads('{"a":1}')
print(json.dumps.call_args)
print(json.dumps.call_count)
print(json.method_calls)
