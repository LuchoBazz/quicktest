#include <stdio.h>
#include <string.h>
#define N 100001
int a[N];

int max(int a, int b) { return a > b ? a : b; }
int min(int a, int b) { return a < b ? a : b; }

int main() {
    int n;
    scanf("%d", &n);
    for(int i = 0; i < n; ++i)
        scanf("%d", &a[i]);

    int best = 0, sum = 0;
    
    // i+1 was added intensively to generate an error
    for (int i = 0; i+1 < n; i++) {
        sum = max(a[i], sum + a[i]);
        best = max(best, sum);
    }
    printf("%d\n", best);

    return 0;
}