/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

pub const TARGET_CPP_CHECK: &str = r#"
#include <bits/stdc++.h>
using namespace std;
int main() {
    int n; cin >> n;
    int count = 0;
    int number = 2;
    while (count < n) {
        cout << number << " ";
        number += 2;
        count++;
    }
    cout << endl;
    return 0;
}"#;

pub const CHECKER_CPP_CHECK: &str = r#"
#include <bits/stdc++.h>
using namespace std;
int main() {
    ifstream in (".qtest/quicktest_input.txt");
    int n; in >> n;
    vector<int> A(n);
    for(auto &a: A) cin >> a;
    if(!is_sorted(A.begin(), A.end())) {
        cout << "NO" << endl;
        return 0;
    }
    int counter = 2;
    for(int i = 0; i < n; ++i) {
        if(A[i] != counter) {
            cout << "NO" << endl;
            return 0;
        }
        counter += 2;
    }
    cout << "YES" << endl;
    return 0;
}"#;

pub const GEN_CPP_CHECK: &str = r#"
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
    return 0;
}"#;

pub const TARGET_PY_CHECK: &str = r#"
n = int(input())
count, number = 0, 2
while count < n:
    print(number, end=" ")
    number += 2
    count += 1
print()"#;

pub const CHECKER_PY_CHECK: &str = r#"
def main():
    # n = int(intput())
    A = list(map(int, input().split()))
    sorted = A
    sorted.sort()
    if A != sorted:
        print("NO")
        return
    counter = 2
    for i in range(len(A)):
        if A[i] != counter:
            print("NO")
            return
        counter += 2
    print("YES")
if __name__ == '__main__':
    main()
"#;

pub const GEN_PY_CHECK: &str = r#"
from random import uniform
n = int(uniform(int(1e2), int(2e2)))
print(n)"#;
