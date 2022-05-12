import random
import sys

# Generator for Maximum Subarray Problem

# quicktest passes a seed as an argument: $ python3 main.py seed testcase
seed = int(sys.argv[1])
random.seed(seed)

N = int(1e3)
Ai = int(1e2)

n = int(random.uniform(1, N))
print(n)
values = [int(random.uniform(-Ai, Ai)) for _ in range(n)]
print(*values)