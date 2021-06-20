/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

pub const TARGET_CPP_CMP: &str = r#"
#include <bits/stdc++.h>
using namespace std;
int main() {
    int n; cin >> n;
    vector<int> arr(n);
    for(auto &a: arr) cin >> a;
    sort(arr.begin(), arr.end());
    cout << n << endl;
    for(auto &a: arr) cout << a << " ";
    cout << endl;
    return 0;
}"#;

pub const CORRECT_CPP_CMP: &str = r#"
#include <bits/stdc++.h>
using namespace std;
int main() {
    int n; cin >> n;
    vector<int> arr(n);
    for(auto &a: arr) cin >> a;
    for (int i = 0; i < n-1; i++) {
        for (int j = 0; j < n-i-1; j++) {
            if (arr[j] > arr[j+1]) {
                swap(arr[j], arr[j+1]);
            }
        }
    }
    cout << n << endl;
    for(auto &a: arr) cout << a << " ";
    cout << endl;
    return 0;
}"#;

pub const GEN_CPP_CMP: &str = r#"
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
    int n = random<int>(1e2, 2e2);
    cout << n << endl;
    for(int i=0;i<n;++i) cout << random<int>(1, 1e9) << " ";
    cout << endl;
    return 0;
}"#;


pub const TARGET_PY_CMP: &str = r#"
n = int(input())
A = list(map(int, input().split()))
A.sort()
print(n)
print(*A)"#;

pub const CORRECT_PY_CMP: &str = r#"
n = int(input())
A = list(map(int, input().split()))

for i in range(n-1):
    for j in range(n-i-1):
        if A[j] > A[j+1]:
            A[j], A[j+1] = A[j+1], A[j]

print(n)
print(*A)"#;

pub const GEN_PY_CMP: &str = r#"
from random import uniform
n = int(uniform(int(1e2), int(2e2)))
print(n)
A = [int(uniform(1, int(1e9))) for _ in range(n)]
print(*A)"#;
