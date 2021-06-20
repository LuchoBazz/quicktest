/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::process::Command;
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertion
use std::error::Error;

use crate::util::test_command_handler::execute_command_check;
use crate::util::test_utilities::create_files_check;
use crate::util::test_constants::{BINARY, CE_CPP, CHECKER_FILE_CPP, CHECKER_FILE_PY, FOLDER_CHECK, GEN_FILE_CPP, GEN_FILE_PY, RTE_CPP, RTE_PY, TARGET_FILE_CPP, TARGET_FILE_PY};

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

#[test]
fn cmd_check_target_cpp_check_cpp_gen_cpp() -> Result<(), Box<dyn Error>> {
    create_files_check(TARGET_FILE_CPP, CHECKER_FILE_CPP, GEN_FILE_CPP, 
        TARGET_CPP_CHECK, CHECKER_CPP_CHECK, GEN_CPP_CHECK, FOLDER_CHECK)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_check(&mut cmd, TARGET_FILE_CPP, CHECKER_FILE_CPP, GEN_FILE_CPP, cases);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));
    
    Ok(())
}

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

#[test]
fn cmd_check_target_py_check_py_gen_py() -> Result<(), Box<dyn Error>> {
    create_files_check(TARGET_FILE_PY, CHECKER_FILE_PY, GEN_FILE_PY, 
        TARGET_PY_CHECK, CHECKER_PY_CHECK, GEN_PY_CHECK, FOLDER_CHECK)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_check(&mut cmd, TARGET_FILE_PY, CHECKER_FILE_PY, GEN_FILE_PY, cases);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));
    
    Ok(())
}

#[test]
fn cmd_check_target_cpp_checker_cpp_gen_py() -> Result<(), Box<dyn Error>> {
    create_files_check(TARGET_FILE_CPP, CHECKER_FILE_CPP, GEN_FILE_PY, 
        TARGET_CPP_CHECK, CHECKER_CPP_CHECK, GEN_PY_CHECK, FOLDER_CHECK)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_check(&mut cmd, TARGET_FILE_CPP, CHECKER_FILE_CPP, GEN_FILE_PY, cases);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));
    
    Ok(())
}

#[test]
fn cmd_check_target_cpp_checker_py_gen_py() -> Result<(), Box<dyn Error>> {
    create_files_check(TARGET_FILE_CPP, CHECKER_FILE_PY, GEN_FILE_PY, 
        TARGET_CPP_CHECK, CHECKER_PY_CHECK, GEN_PY_CHECK, FOLDER_CHECK)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_check(&mut cmd, TARGET_FILE_CPP, CHECKER_FILE_PY, GEN_FILE_PY, cases);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));
    
    Ok(())
}

#[test]
fn cmd_check_target_cpp_checker_py_gen_cpp() -> Result<(), Box<dyn Error>> {
    create_files_check(TARGET_FILE_CPP, CHECKER_FILE_PY, GEN_FILE_CPP, 
        TARGET_CPP_CHECK, CHECKER_PY_CHECK, GEN_CPP_CHECK, FOLDER_CHECK)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_check(&mut cmd, TARGET_FILE_CPP, CHECKER_FILE_PY, GEN_FILE_CPP, cases);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));
    
    Ok(())
}

#[test]
fn cmd_check_target_py_checker_py_gen_cpp() -> Result<(), Box<dyn Error>> {
    create_files_check(TARGET_FILE_PY, CHECKER_FILE_PY, GEN_FILE_CPP, 
        TARGET_PY_CHECK, CHECKER_PY_CHECK, GEN_CPP_CHECK, FOLDER_CHECK)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_check(&mut cmd, TARGET_FILE_PY, CHECKER_FILE_PY, GEN_FILE_CPP, cases);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));
    
    Ok(())
}

#[test]
fn cmd_check_target_py_checker_cpp_gen_cpp() -> Result<(), Box<dyn Error>> {
    create_files_check(TARGET_FILE_PY, CHECKER_FILE_CPP, GEN_FILE_CPP, 
        TARGET_PY_CHECK, CHECKER_CPP_CHECK, GEN_CPP_CHECK, FOLDER_CHECK)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_check(&mut cmd, TARGET_FILE_PY, CHECKER_FILE_CPP, GEN_FILE_CPP, cases);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));
    
    Ok(())
}

#[test]
fn cmd_check_target_py_checker_cpp_gen_py() -> Result<(), Box<dyn Error>> {
    create_files_check(TARGET_FILE_PY, CHECKER_FILE_CPP, GEN_FILE_PY, 
        TARGET_PY_CHECK, CHECKER_CPP_CHECK, GEN_PY_CHECK, FOLDER_CHECK)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_check(&mut cmd, TARGET_FILE_PY, CHECKER_FILE_CPP, GEN_FILE_PY, cases);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));
    
    Ok(())
}

// CHECK RTE in Subcommand check
#[test]
fn cmd_check_target_rte_cpp() -> Result<(), Box<dyn Error>> {
    create_files_check(TARGET_FILE_CPP, CHECKER_FILE_CPP, GEN_FILE_CPP, 
        RTE_CPP, CHECKER_CPP_CHECK, GEN_CPP_CHECK, FOLDER_CHECK)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_check(&mut cmd, TARGET_FILE_CPP, CHECKER_FILE_CPP, GEN_FILE_CPP, cases);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[RTE]").count(cases));
    
    Ok(())
}

#[test]
fn cmd_check_checker_rte_cpp() -> Result<(), Box<dyn Error>> {
    create_files_check(TARGET_FILE_CPP, CHECKER_FILE_CPP, GEN_FILE_CPP, 
        TARGET_CPP_CHECK, RTE_CPP, GEN_CPP_CHECK, FOLDER_CHECK)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_check(&mut cmd, TARGET_FILE_CPP, CHECKER_FILE_CPP, GEN_FILE_CPP, cases);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: Runtime Error of <checker-file>"));
    
    Ok(())
}

#[test]
fn cmd_check_gen_rte_cpp() -> Result<(), Box<dyn Error>> {
    create_files_check(TARGET_FILE_CPP, CHECKER_FILE_CPP, GEN_FILE_CPP, 
        TARGET_CPP_CHECK, CHECKER_CPP_CHECK, RTE_CPP, FOLDER_CHECK)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_check(&mut cmd, TARGET_FILE_CPP, CHECKER_FILE_CPP, GEN_FILE_CPP, cases);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: Runtime Error of <gen-file>"));
    
    Ok(())
}

#[test]
fn cmd_check_target_rte_py() -> Result<(), Box<dyn Error>> {
    create_files_check(TARGET_FILE_PY, CHECKER_FILE_PY, GEN_FILE_PY, 
        RTE_PY, CHECKER_PY_CHECK, GEN_PY_CHECK, FOLDER_CHECK)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_check(&mut cmd, TARGET_FILE_PY, CHECKER_FILE_PY, GEN_FILE_PY, cases);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[RTE]").count(cases));
    
    Ok(())
}


#[test]
fn cmd_check_checker_rte_py() -> Result<(), Box<dyn Error>> {
    create_files_check(TARGET_FILE_PY, CHECKER_FILE_PY, GEN_FILE_PY, 
        TARGET_PY_CHECK, RTE_PY, GEN_PY_CHECK, FOLDER_CHECK)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_check(&mut cmd, TARGET_FILE_PY, CHECKER_FILE_PY, GEN_FILE_PY, cases);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: Runtime Error of <checker-file>"));
    
    Ok(())
}

#[test]
fn cmd_check_gen_rte_py() -> Result<(), Box<dyn Error>> {
    create_files_check(TARGET_FILE_PY, CHECKER_FILE_PY, GEN_FILE_PY, 
        TARGET_PY_CHECK, CHECKER_PY_CHECK, RTE_PY, FOLDER_CHECK)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_check(&mut cmd, TARGET_FILE_PY, CHECKER_FILE_PY, GEN_FILE_PY, cases);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: Runtime Error of <gen-file>"));
    
    Ok(())
}

// CHECK CE in Subcommand check
#[test]
fn cmd_check_target_ce_cpp() -> Result<(), Box<dyn Error>> {
    create_files_check(TARGET_FILE_CPP, CHECKER_FILE_CPP, GEN_FILE_CPP, 
        CE_CPP, CHECKER_CPP_CHECK, GEN_CPP_CHECK, FOLDER_CHECK)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_check(&mut cmd, TARGET_FILE_CPP, CHECKER_FILE_CPP, GEN_FILE_CPP, cases);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: compilation of <target-file> failed"));
    
    Ok(())
}

#[test]
fn cmd_check_checker_ce_cpp() -> Result<(), Box<dyn Error>> {
    create_files_check(TARGET_FILE_CPP, CHECKER_FILE_CPP, GEN_FILE_CPP, 
        TARGET_CPP_CHECK, CE_CPP, GEN_CPP_CHECK, FOLDER_CHECK)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_check(&mut cmd, TARGET_FILE_CPP, CHECKER_FILE_CPP, GEN_FILE_CPP, cases);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: compilation of <checker-file> failed"));
    
    Ok(())
}

#[test]
fn cmd_check_gen_ce_cpp() -> Result<(), Box<dyn Error>> {
    create_files_check(TARGET_FILE_CPP, CHECKER_FILE_CPP, GEN_FILE_CPP, 
        TARGET_CPP_CHECK, CHECKER_CPP_CHECK, CE_CPP, FOLDER_CHECK)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_check(&mut cmd, TARGET_FILE_CPP, CHECKER_FILE_CPP, GEN_FILE_CPP, cases);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: compilation of <gen-file> failed"));
    
    Ok(())
}