/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021-present / Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

// CODES
pub const TARGET_CPP_OUTPUT_CMD: &str = r#"
#include <bits/stdc++.h>
using namespace std;
int n;
vector<int> A;
int main() {
    cin >> n;
    A.resize(n);
    for(auto &ref: A) cin >> ref;
    sort(A.begin(), A.end());
    cout << n << endl;
    for(auto &a: A) cout << a << " ";
    cout << endl;
    return 0;
}
"#;

pub const TARGET_PY_OUTPUT_CMD: &str = r#"
n = int(input())
A = list(map(int, input().split()))
A.sort()
print(n)
print(*A)
"#;

pub const TARGET_C_OUTPUT_CMD: &str = r#"
#include <stdio.h>
#include <stdlib.h>
int a[100001];
int cmp (const void * a, const void * b) { return ( *(int*)a - *(int*)b ); }
int main() {
    int n;
    scanf("%d", &n);
    for(int i = 0; i < n; ++i) scanf("%d", &a[i]);
    qsort(a, n, sizeof(int), cmp);
    printf("%d\n", n);
    for(int i = 0; i < n; ++i) {
        if(i > 0) printf(" ");
        printf("%d", a[i]);
    }
    return 0;
}"#;
