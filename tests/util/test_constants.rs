/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021-present / Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

pub const FOLDER: &str = "target/.code";
pub const FOLDER_STRESS: &str = "stress";
pub const FOLDER_CMP: &str = "cmp";
pub const FOLDER_CHECK: &str = "checker";
pub const FOLDER_RUN: &str = "run";

#[cfg(unix)]
pub const BINARY: &str = "./target/debug/quicktest";

#[cfg(windows)]
pub const BINARY: &str = "./target/debug/quicktest.exe";

pub const TARGET_FILE_CPP: &str = "main.cpp";
pub const CORRECT_FILE_CPP: &str = "correct.cpp";
pub const CHECKER_FILE_CPP: &str = "checker.cpp";
pub const GEN_FILE_CPP: &str = "gen.cpp";

pub const TARGET_FILE_PY: &str = "main.py";
pub const CORRECT_FILE_PY: &str = "correct.py";
pub const CHECKER_FILE_PY: &str = "checker.py";
pub const GEN_FILE_PY: &str = "gen.py";

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

// MLE CODES
pub const MLE_CPP: &str = r#"
#include <bits/stdc++.h>
using namespace std;
int main() {
    vector<long long> vec;
    while(true) vec.push_back(1LL);
}
"#;

// not used because python throws an RTE (MemoryError) when it exceeds memory
pub const MLE_PY: &str = r#"
data = []
while True:
    data.append(1 << 128)
"#;
