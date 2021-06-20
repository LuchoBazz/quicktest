/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::process::Command;
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertion
use std::error::Error;

use crate::util::test_command_handler::execute_command_cmp;
use crate::util::test_utilities::create_files_cmp;
use crate::util::test_constants::{
    BINARY, CE_CPP, CORRECT_FILE_CPP, CORRECT_FILE_PY, FOLDER_CMP,
    GEN_FILE_CPP, GEN_FILE_PY, RTE_CPP, RTE_PY, TARGET_FILE_CPP,
    TARGET_FILE_PY
};

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

#[test]
fn cmp_target_cpp_correct_cpp_gen_cpp() -> Result<(), Box<dyn Error>> {
    create_files_cmp(TARGET_FILE_CPP, CORRECT_FILE_CPP, GEN_FILE_CPP, 
        TARGET_CPP_CMP, CORRECT_CPP_CMP, GEN_CPP_CMP, FOLDER_CMP)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(&mut cmd, TARGET_FILE_CPP, CORRECT_FILE_CPP, GEN_FILE_CPP, cases);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));
    
    Ok(())
}


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

#[test]
fn cmp_target_py_correct_py_gen_py() -> Result<(), Box<dyn Error>> {
    create_files_cmp(TARGET_FILE_PY, CORRECT_FILE_PY, GEN_FILE_PY, 
        TARGET_PY_CMP, CORRECT_PY_CMP, GEN_PY_CMP, FOLDER_CMP)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(&mut cmd, TARGET_FILE_PY, CORRECT_FILE_PY, GEN_FILE_PY, cases);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));
    
    Ok(())
}

#[test]
fn cmp_target_cpp_correct_cpp_gen_py() -> Result<(), Box<dyn Error>> {
    create_files_cmp(TARGET_FILE_CPP, CORRECT_FILE_CPP, GEN_FILE_PY, 
        TARGET_CPP_CMP, CORRECT_CPP_CMP, GEN_PY_CMP, FOLDER_CMP)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(&mut cmd, TARGET_FILE_CPP, CORRECT_FILE_CPP, GEN_FILE_PY, cases);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));
    
    Ok(())
}

#[test]
fn cmp_target_cpp_correct_py_gen_py() -> Result<(), Box<dyn Error>> {
    create_files_cmp(TARGET_FILE_CPP, CORRECT_FILE_PY, GEN_FILE_PY, 
        TARGET_CPP_CMP, CORRECT_PY_CMP, GEN_PY_CMP, FOLDER_CMP)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(&mut cmd, TARGET_FILE_CPP, CORRECT_FILE_PY, GEN_FILE_PY, cases);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));
    
    Ok(())
}

#[test]
fn cmp_target_cpp_correct_py_gen_cpp() -> Result<(), Box<dyn Error>> {
    create_files_cmp(TARGET_FILE_CPP, CORRECT_FILE_PY, GEN_FILE_CPP, 
        TARGET_CPP_CMP, CORRECT_PY_CMP, GEN_CPP_CMP, FOLDER_CMP)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(&mut cmd, TARGET_FILE_CPP, CORRECT_FILE_PY, GEN_FILE_CPP, cases);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));
    
    Ok(())
}

#[test]
fn cmp_target_py_correct_py_gen_cpp() -> Result<(), Box<dyn Error>> {
    create_files_cmp(TARGET_FILE_PY, CORRECT_FILE_PY, GEN_FILE_CPP, 
        TARGET_PY_CMP, CORRECT_PY_CMP, GEN_CPP_CMP, FOLDER_CMP)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(&mut cmd, TARGET_FILE_PY, CORRECT_FILE_PY, GEN_FILE_CPP, cases);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));
    
    Ok(())
}

#[test]
fn cmp_target_py_correct_cpp_gen_cpp() -> Result<(), Box<dyn Error>> {
    create_files_cmp(TARGET_FILE_PY, CORRECT_FILE_CPP, GEN_FILE_CPP, 
        TARGET_PY_CMP, CORRECT_CPP_CMP, GEN_CPP_CMP, FOLDER_CMP)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(&mut cmd, TARGET_FILE_PY, CORRECT_FILE_CPP, GEN_FILE_CPP, cases);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));
    
    Ok(())
}

#[test]
fn cmp_target_py_correct_cpp_gen_py() -> Result<(), Box<dyn Error>> {
    create_files_cmp(TARGET_FILE_PY, CORRECT_FILE_CPP, GEN_FILE_PY, 
        TARGET_PY_CMP, CORRECT_CPP_CMP, GEN_PY_CMP, FOLDER_CMP)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(&mut cmd, TARGET_FILE_PY, CORRECT_FILE_CPP, GEN_FILE_PY, cases);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));
    
    Ok(())
}

// CHECK RTE in Subcommand cmp
#[test]
fn cmd_cmp_target_rte_cpp() -> Result<(), Box<dyn Error>> {
    create_files_cmp(TARGET_FILE_CPP, CORRECT_FILE_CPP, GEN_FILE_CPP, 
        RTE_CPP, CORRECT_CPP_CMP, GEN_CPP_CMP, FOLDER_CMP)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(&mut cmd, TARGET_FILE_CPP, CORRECT_FILE_CPP, GEN_FILE_CPP, cases);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[RTE]").count(cases));
    
    Ok(())
}

#[test]
fn cmd_cmp_correct_rte_cpp() -> Result<(), Box<dyn Error>> {
    create_files_cmp(TARGET_FILE_CPP, CORRECT_FILE_CPP, GEN_FILE_CPP, 
        TARGET_CPP_CMP, RTE_CPP, GEN_CPP_CMP, FOLDER_CMP)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(&mut cmd, TARGET_FILE_CPP, CORRECT_FILE_CPP, GEN_FILE_CPP, cases);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: Runtime Error of <correct-file>"));
    
    Ok(())
}

#[test]
fn cmd_cmp_gen_rte_cpp() -> Result<(), Box<dyn Error>> {
    create_files_cmp(TARGET_FILE_CPP, CORRECT_FILE_CPP, GEN_FILE_CPP, 
        TARGET_CPP_CMP, CORRECT_CPP_CMP, RTE_CPP, FOLDER_CMP)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(&mut cmd, TARGET_FILE_CPP, CORRECT_FILE_CPP, GEN_FILE_CPP, cases);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: Runtime Error of <gen-file>"));
    
    Ok(())
}

#[test]
fn cmd_cmp_target_rte_py() -> Result<(), Box<dyn Error>> {
    create_files_cmp(TARGET_FILE_PY, CORRECT_FILE_PY, GEN_FILE_PY, 
        RTE_PY, CORRECT_PY_CMP, GEN_PY_CMP, FOLDER_CMP)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(&mut cmd, TARGET_FILE_PY, CORRECT_FILE_PY, GEN_FILE_PY, cases);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[RTE]").count(cases));
    
    Ok(())
}

#[test]
fn cmd_cmp_correct_rte_py() -> Result<(), Box<dyn Error>> {
    create_files_cmp(TARGET_FILE_PY, CORRECT_FILE_PY, GEN_FILE_PY, 
        TARGET_PY_CMP, RTE_PY, GEN_PY_CMP, FOLDER_CMP)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(&mut cmd, TARGET_FILE_PY, CORRECT_FILE_PY, GEN_FILE_PY, cases);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: Runtime Error of <correct-file>"));
    
    Ok(())
}

#[test]
fn cmd_cmp_gen_rte_py() -> Result<(), Box<dyn Error>> {
    create_files_cmp(TARGET_FILE_PY, CORRECT_FILE_PY, GEN_FILE_PY, 
        TARGET_PY_CMP, CORRECT_PY_CMP, RTE_PY, FOLDER_CMP)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(&mut cmd, TARGET_FILE_PY, CORRECT_FILE_PY, GEN_FILE_PY, cases);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: Runtime Error of <gen-file>"));
    
    Ok(())
}

// CHECK CE in Subcommand cmp
#[test]
fn cmd_cmp_target_ce_cpp() -> Result<(), Box<dyn Error>> {
    create_files_cmp(TARGET_FILE_CPP, CORRECT_FILE_CPP, GEN_FILE_CPP, 
        CE_CPP, CORRECT_CPP_CMP, GEN_CPP_CMP, FOLDER_CMP)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(&mut cmd, TARGET_FILE_CPP, CORRECT_FILE_CPP, GEN_FILE_CPP, cases);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: compilation of <target-file> failed"));
    
    Ok(())
}

#[test]
fn cmd_cmp_correct_ce_cpp() -> Result<(), Box<dyn Error>> {
    create_files_cmp(TARGET_FILE_CPP, CORRECT_FILE_CPP, GEN_FILE_CPP, 
        TARGET_CPP_CMP, CE_CPP, GEN_CPP_CMP, FOLDER_CMP)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(&mut cmd, TARGET_FILE_CPP, CORRECT_FILE_CPP, GEN_FILE_CPP, cases);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: compilation of <correct-file> failed"));
    
    Ok(())
}

#[test]
fn cmd_cmp_gen_ce_cpp() -> Result<(), Box<dyn Error>> {
    create_files_cmp(TARGET_FILE_CPP, CORRECT_FILE_CPP, GEN_FILE_CPP, 
        TARGET_CPP_CMP, GEN_CPP_CMP, CE_CPP, FOLDER_CMP)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(&mut cmd, TARGET_FILE_CPP, CORRECT_FILE_CPP, GEN_FILE_CPP, cases);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: compilation of <gen-file> failed"));
    
    Ok(())
}
