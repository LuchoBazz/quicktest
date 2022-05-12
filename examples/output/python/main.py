# Maximum Subarray Problem - slow version

n = int(input())
values = list(map(int, input().split()))

best = 0

for i in range(n):
    sum = 0
    for j in range(i, n):
        sum += values[j]
        best = max(best, sum)

print(best)