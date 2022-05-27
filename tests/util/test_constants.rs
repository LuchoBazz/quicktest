/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021-present / Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

pub const FOLDER: &str = "target/.code";
pub const FOLDER_STRESS: &str = "stress";
pub const FOLDER_CMP: &str = "cmp";
pub const FOLDER_CHECK: &str = "checker";
pub const FOLDER_OUTPUT: &str = "output";

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

pub const TARGET_FILE_C: &str = "main.c";
pub const CORRECT_FILE_C: &str = "correct.c";
pub const CHECKER_FILE_C: &str = "checker.c";
pub const GEN_FILE_C: &str = "gen.c";

pub const TARGET_FILE_RUST: &str = "main.rs";
pub const CORRECT_FILE_RUST: &str = "correct.rs";
pub const CHECKER_FILE_RUST: &str = "checker.rs";
pub const GEN_FILE_RUST: &str = "gen.rs";

// CPP
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

pub const CE_CPP: &str = r#"
#include <bits/stdc++.h>
using namespace std;
int main() {
    Generate Compiler Error
}
"#;

pub const MLE_CPP: &str = r#"
#include <bits/stdc++.h>
using namespace std;
int main() {
    vector<long long> vec;
    while(true) vec.push_back(1LL);
}
"#;

// C
pub const CE_C: &str = r#"
#include <stdio.h>
int main() {
    Generate Compiler Error
}
"#;

pub const MLE_C: &str = r#"
#include <stdio.h>
int * a[100000];
int main() {
    int n = 1000000000;
    for(int i = 0; i < n; ++i) {
        a[i % 100000] = (int*)malloc(sizeof(int));
    }
    return 0;
}
"#;

pub const RTE_C: &str = r#"
#include <stdio.h>
int main() {
    // Generate divide by zero error
    for(int i = 0; i < 10; ++i) {
        int y = 10 / i;
    }
    return 0;
}
"#;

// PYTHON
pub const RTE_PY: &str = r#"
for i in range(10):
    print(10 / i)
"#;

// not used because python throws an RTE (MemoryError) when it exceeds memory
pub const MLE_PY: &str = r#"
data = []
while True:
    data.append(1 << 128)
"#;

// RUST
pub const RTE_RUST: &str = r#"
fn main() {
   for i in -10..10 {
       println!("{}", 100 / i);
   }
}
"#;

pub const CE_RUST: &str = r#"
fn main() {
    Generate Compiler Error
}
"#;

pub const MLE_RUST: &str = r#"
fn main() {
    let mut vec = Vec::new();
    loop {
        vec.push(1_i64 << 60_i64);
    }
}"#;
