# Maximum Subarray Problem

n = int(input())
values = list(map(int, input().split()))

best, sum = 0, 0

for i in range(n-1):
    sum = max(values[i], sum + values[i])
    best = max(best, sum)

print(best)