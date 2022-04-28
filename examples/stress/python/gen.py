import random
import sys

# Generator for Maximum Subarray Problem
# 1 <= n <= 1e5
# -1e5 <= a_i <= 1e5

# quicktest passes a seed as an argument
seed = int(sys.argv[1])
random.seed(seed)

n = int(random.uniform(int(1), int(1e5)))
print(n)
values = [int(random.uniform(-int(1e5), int(1e5))) for _ in range(n)]
print(*values)