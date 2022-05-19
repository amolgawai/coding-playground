#!/usr/bin/env python3
"""Additonal data structures

   Data Structures apart from the regular list, dict, map, str and bytes
"""
import collections

# Ordered dict - remembers insertion order of keys
d = collections.OrderedDict(one=1, two=2, three=3)
print(d)
d['four'] = 4
print(d.keys())

# defaultdict - Return default values for missing keys

dd = collections.defaultdict(list)

# Accessing a missing key creates it and
# initializes it using the default factory,
# i.e. list() in this example:
dd['dogs'].append('Rufus')
dd['dogs'].append('Kathrin')
dd['dogs'].append('Mr Sniffles')
print(dd['dogs'])

# ChainMap Search multiple dictionaries as a single mapping
dict1 = {'one': 1, 'two': 2}
dict2 = {'three': 3, 'four': 4}
chain = collections.ChainMap(dict1, dict2)

print(chain)
# ChainMap searches each collection in the chain
# from left to right until it finds the key (or fails):
print(chain['three'])
print(chain['one'])
try:
    print(chain['missing'])
except KeyError:
    print("Searching for misiing key in chained dictionaries")

# MappingProxyType - A wrapper for making Read-Only dictionaries
from types import MappingProxyType  # noqa: E305
writable = {'one': 1, 'two': 2}
read_only = MappingProxyType(writable)
print(read_only['one'])
try:
    read_only['one'] = 23
except TypeError:
    print("Can't modify readonly dictionary'")
# Updates to the original are reflected in the proxy:
writable['one'] = 42
print(read_only)

# array - typed arrays that provide space-efficient storage
import array  # noqa: E305

arr = array.array('f', (1.0, 1.5, 2.0, 2.5))
print(arr[1])
# Arrays have a nice repr:
print(arr)
# Arrays are mutable:
arr[1] = 23.0
print(arr)
del arr[1]
print(arr)
arr.append(42.0)
print(arr)
# Arrays are "typed":
try:
    arr[1] = 'hello'
except TypeError:
    print("typed array - insertion must be real number, not str")

# bytearray - Mutable arrays of Single Bytes
arr = bytearray((0, 1, 2, 3))
print(arr[1])
# The bytearray repr:
print(arr)
bytearray(b'\x00\x01\x02\x03')
# Bytearrays are mutable:
arr[1] = 23
print(arr, arr[1])
# Bytearrays can grow and shrink in size:
del arr[1]
arr.append(42)
print(arr)
# Bytearrays can only hold "bytes"
# (integers in the range 0 <= x <= 255)
try:
    arr[1] = 'hello'
except TypeError:
    print("bytearray can obly hold bytes")

try:
    arr[1] = 300
except ValueError as err:
    print(err)

# Bytearrays can be converted back into bytes objects:
# (This will copy the data)
print(bytes(arr))

# struct.Struct - Serialized C Structs
from struct import Struct  # noqa: E305
MyStruct = Struct('i?f')
data = MyStruct.pack(23, False, 42.0)

# All you get is a blob of data:
print(data)
# Data blobs can be unpacked again:
print(MyStruct.unpack(data))

# types.SimpleNamespace - Fancy attribute Access
from types import SimpleNamespace  # noqa: E305
car1 = SimpleNamespace(color='red', mileage=3812.4, automatic=True)

# The default repr:
print(car1)
# Instances support attribute access and are mutable:
car1.mileage = 12
car1.windshield = 'broken'
del car1.automatic
print(car1)

# frozenset - Immutable sets
vowels = frozenset({'a', 'e', 'i', 'o', 'u'})
try:
    vowels.add('p')
except AttributeError as err:
    print(err)

# Frozensets are hashable and can
# be used as dictionary keys:
d = {frozenset({1, 2, 3}): 'hello'}
print(d[frozenset({1, 2, 3})])

# collections.Counter - Multisets
from collections import Counter  # noqa: E305
inventory = Counter()

loot = {'sword': 1, 'bread': 3}
inventory.update(loot)
print(inventory)

more_loot = {'sword': 1, 'apple': 1}
inventory.update(more_loot)
print(inventory)
print(f'Unique elements {len(inventory)}')
print(f'Total no. of elements {sum(inventory.values())}')

# collections.deque - fast and robust stacks
# are implemented as doubly-linked lists
from collections import deque  # noqa: E305
s = deque()
s.append('eat')
s.append('sleep')
s.append('code')

print(s)
print(s.pop(), s.pop(), s.pop())
try:
    s.pop()
except IndexError as err:
    print(err)

# queue.LifoQueue - Locking Semantics for parallel computing
# besides LifoQueue, queue contains several other classes that implement multi-producer/multi-consumer queues for parallel computing
from queue import LifoQueue
s = LifoQueue()
s.put('eat')
s.put('sleep')
s.put('code')
print(s)
print(s.get(), s.get(), s.get())
try:
    print(s.get_nowait())
except Exception:
    print("Queue is empty")
# s.get()
# Blocks / waits forever...

# queue.Queue - Locking semantics for parallel computing
from queue import Queue  # noqa: E305
q = Queue()
q.put('eat')
q.put('sleep')
q.put('code')
print(q)
print(q.get(), q.get(), q.get())
try:
    print(q.get_nowait())
except Exception:
    print("Empty Queue")
# q.get() # Blocks/ waits forever

# queue.PriorityQueue - thread safe priority queue
from queue import PriorityQueue  # noqa: E305

q = PriorityQueue()

q.put((2, 'code'))
q.put((1, 'eat'))
q.put((3, 'sleep'))

while not q.empty():
    print(q.get())

