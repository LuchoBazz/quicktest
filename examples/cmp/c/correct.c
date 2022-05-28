#include <stdio.h>
#include <string.h>
#define N 100001
int values[N];

int max(int a, int b) { return a > b ? a : b; }
int min(int a, int b) { return a < b ? a : b; }

int main() {
    // Maximum Subarray Problem - slow version
    
    int n;
    scanf("%d", &n);
    for(int i = 0; i < n; i++)
        scanf("%d", &values[i]);
        
    int best = 0;
    for (int i = 0; i < n; i++) {
        int sum = 0;
        for (int j = i; j < n; j++) {
            sum += values[j];
            best = max(best, sum);
        }
    }
    printf("%d\n", best);
    return 0;
}