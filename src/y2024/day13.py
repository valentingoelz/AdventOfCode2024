import re
import numpy as np
from scipy.optimize import milp
from scipy.optimize import LinearConstraint


class Problem:
    def __init__(self, lines):
        lines = lines.split("\n")
        # get first number which comes after X+
        a_11 = int(re.match(r"Button A: X\+(\d+)", lines[0]).group(1))
        a_12 = int(re.match(r"Button A: X\+(\d+), Y\+(\d+)" , lines[0]).group(2))
        a_21 = int(re.match(r"Button B: X\+(\d+)", lines[1]).group(1))
        a_22 = int(re.match(r"Button B: X\+(\d+), Y\+(\d+)", lines[1]).group(2))
        b_1 = int(re.match(r"Prize: X=(\d+)", lines[2]).group(1))
        b_2 = int(re.match(r"Prize: X=(\d+), Y=(\d+)", lines[2]).group(2))
        # use int64
        self.A = np.array([[a_11, a_12],
                           [a_21, a_22],
                           ],
                           dtype=np.int64).transpose()
        self.b = np.array([b_1, b_2], dtype=np.int64)
        self.c = np.array([3, 1], dtype=np.int64)
    
    def find_integer_solution(self):
        # cramers rule
        det = self.A[0, 0] * self.A[1, 1] - self.A[0, 1] * self.A[1, 0]
        if det == 0:
            return None
        x1 = (self.b[0] * self.A[1, 1] - self.b[1] * self.A[0, 1]) / det
        x2 = (self.A[0, 0] * self.b[1] - self.A[1, 0] * self.b[0]) / det
        # chec if they are integer
        x1_int = x1 == int(x1)
        x2_int = x2 == int(x2)
        if x1_int and x2_int:
            return x1, x2
        return None


with open("../../input/day13.txt") as f:
    input = f.read().strip().split("\n\n")


def solve_part_1(input):
    total_sum = 0
    for lines in input:
        problem = Problem(lines)
        problem.b[0] += 10000000000000
        problem.b[1] += 10000000000000
        sol = problem.find_integer_solution()
        if sol is not None:
            total_sum += sum(sol * problem.c)
    print("Solution: ", total_sum)

solve_part_1(input)