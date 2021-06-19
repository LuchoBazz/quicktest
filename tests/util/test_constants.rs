/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

pub const FOLDER:     &str = "target/.code";
pub const FOLDER_TLE: &str = "tle";
pub const FOLDER_CMP: &str = "cmp";

#[cfg(unix)]
pub const BINARY: &str = "./target/debug/quicktest";

#[cfg(windows)]
pub const BINARY: &str = "./target/debug/quicktest.exe";

pub const TARGET_FILE_CPP:  &str = "main.cpp";
pub const CORRECT_FILE_CPP: &str = "correct.cpp";
pub const GEN_FILE_CPP:     &str = "gen.cpp";

pub const TARGET_FILE_PY:   &str = "main.py";
pub const CORRECT_FILE_PY:  &str = "correct.py";
pub const GEN_FILE_PY:      &str = "gen.py";

// RTE CODES
pub const RTE_CPP: &str = r#"
#include <bits/stdc++.h>
using namespace std;
int main() {
    // Generate divide by zero error
    for(int i = 0; i < 10; ++i) {
        int y = 10 / i;
    }
}
"#;

pub const RTE_PY: &str = r#"
for i in range(10):
    print(10 / i)
"#;

// CE CODES
pub const CE_CPP: &str = r#"
#include <bits/stdc++.h>
using namespace std;
int main() {
    Generate Compiler Error
}
"#;

// CHECKER CODES

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
}
"#;

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
}
"#;

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
}
"#;