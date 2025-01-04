-- first file from the book Learn Physics with Functional Programming
-- Varaiable declaration
e :: Double
e = exp 1

-- Function declaration
square :: Double -> Double
square x = x ** 2

-- exercises
-- 2.1
sqrt_1_pls :: Double -> Double
sqrt_1_pls x = sqrt $ 1 + x

-- 2.2
-- height of rock as function of time
gravity = -9.8

initialVelocity = 30.0

yRock30 :: Double -> Double
yRock30 t = 0.5 * gravity * t ** 2 + initialVelocity * t

-- 2.3
-- velocity of rock as function of time
vRock30 :: Double -> Double
vRock30 t = initialVelocity + gravity * t

-- 2.4
-- sin of angle given in degrees
sinDeg :: Double -> Double
sinDeg angle = sin $ angle * pi / 180

-- 2.5
-- functions
f :: Double -> Double
f x = x ** (1 / 3)

g :: Double -> Double
g y = e ** y + 8 ** y

h :: Double -> Double
h x = 1 / sqrt ((x - 5) ** 2 + 16)
