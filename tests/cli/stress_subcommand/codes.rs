/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021-present / Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

// STRESS CODES
pub const GEN_CPP_STRESS: &str = r#"
#include <bits/stdc++.h>
using namespace std;
template <typename T>
T random(const T from, const T to) {
    static random_device rdev;
    static default_random_engine re(rdev());

    using dist_type = typename conditional<
        is_floating_point<T>::value,
        uniform_real_distribution<T>,
        uniform_int_distribution<T>
    >::type;

    dist_type uni(from, to);
    return static_cast<T>(uni(re));
}
int main() {
    #define endl '\n'
    int n = random<int>(1e5, 2e5);
    cout << n << endl;
    for(int i=0;i<n;++i) cout << random<int>(1, 1e9) << " ";
    cout << endl;
    return 0;
}
"#;

pub const TARGET_CPP_STRESS: &str = r#"
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

pub const GEN_PY_STRESS: &str = r#"
from random import uniform
n = int(uniform(int(1e5), int(2e5)))
print(n)
A = [int(uniform(1, int(1e9))) for _ in range(n)]
print(*A)
"#;

pub const TARGET_PY_STRESS: &str = r#"
n = int(input())
A = list(map(int, input().split()))
A.sort()
print(n)
print(*A)
"#;

pub const GEN_C_STRESS: &str = r#"
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

int random_int(int min, int max) {
    return min + rand() % (max - min + 1);
}

int main() {
    srand(time(NULL));
    int n = random_int(100000, 200000);
    printf("%d\n", n);
    for (int i = 0; i < n; ++i) {
        printf("%d ", random_int(1, 1000000000));
    }
    printf("\n");
    return 0;
}
"#;

pub const TARGET_C_STRESS: &str = r#"
#include <stdio.h>
#include <stdlib.h>

int compare(const void *a, const void *b) {
    return (*(int *)a - *(int *)b);
}

int main() {
    int n;
    scanf("%d", &n);
    int *A = (int *)malloc(n * sizeof(int));
    for (int i = 0; i < n; ++i) {
        scanf("%d", &A[i]);
    }
    qsort(A, n, sizeof(int), compare);
    printf("%d\n", n);
    for (int i = 0; i < n; ++i) {
        printf("%d ", A[i]);
    }
    printf("\n");
    free(A);
    return 0;
}
"#;
