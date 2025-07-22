import os
import sys

sys.path.insert(0, os.path.abspath(os.path.dirname(__file__)))

import unittest

from mymodule import add, double, square


class TestSquare(unittest.TestCase):
    def test1(self):
        self.assertEqual(square(2), 4)  # test when 2 is given as input the output is 4.
        self.assertEqual(
            square(3.0), 9.0
        )  # test when 3.0 is given as input the output is 9.0.
        self.assertNotEqual(
            square(-3), -9
        )  # test when -3 is given as input the output is not -9.


class TestDouble(unittest.TestCase):
    def test1(self):
        self.assertEqual(double(2), 4)  # test when 2 is given as input the output is 4.
        self.assertEqual(
            double(-3.1), -6.2
        )  # test when -3.1 is given as input the output is -6.2.
        self.assertEqual(double(0), 0)  # test when 0 is given as input the output is 0.


class TestAdd(unittest.TestCase):
    def test1(self):
        self.assertEqual(add(1, 1), 2)
        self.assertEqual(add(-1, 1), 0)
        self.assertEqual(add(-3, -3), -6)
        self.assertEqual(add(-1, -2), 0)


if __name__ == "__main__":
    unittest.main()
