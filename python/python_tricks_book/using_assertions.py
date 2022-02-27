msg = "Assertions using tuple as first argument will always succeed, \
don't use this way"
print(msg)
assert(1 == 3, "Should fail but never does")
