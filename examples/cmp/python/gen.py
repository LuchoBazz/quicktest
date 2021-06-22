import random
import sys

# Generator for Maximum Subarray Problem

# quicktest passes a seed as an argument
seed = int(sys.argv[1])
random.seed(seed)

n = int(random.uniform(int(1), int(1e3)))
print(n)
values = [int(random.uniform(-int(1e2), int(1e2))) for _ in range(n)]
print(*values)