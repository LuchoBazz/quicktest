# Maximum Subarray Problem
# 1 <= n <= 1e5
# -1e5 <= a_i <= 1e5

n = int(input())
values = list(map(int, input().split()))

best = 0

for i in range(n):
    sum = 0
    for j in range(i, n):
        sum += values[j]
        best = max(best, sum)

print(best)