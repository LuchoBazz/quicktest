/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

// reference I : https://mattgathu.github.io/2017/10/01/testing-rust-cli-apps.html
// reference II: https://www.duskborn.com/posts/rust-lit/

use std::process::Command;  // Run programs
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertion
use std::error::Error;
use crate::util::test_utilities::create_files_tle;
use crate::util::test_constants::FOLDER;

// // TLE CODES
pub const GEN_CPP_TLE: &str = r#"
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

pub const TARGET_CPP_TLE: &str = r#"
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

pub const GEN_PY_TLE: &str = r#"
from random import uniform
n = int(uniform(int(1e5), int(2e5)))
print(n)
A = [int(uniform(1, int(1e9))) for _ in range(n)]
print(*A)
"#;

pub const TARGET_PY_TLE: &str = r#"
n = int(input())
A = list(map(int, input().split()))
A.sort()
print(n)
print(*A)
"#;

#[test]
fn cmd_tle_gen_cpp_target_cpp() -> Result<(), Box<dyn Error>> {
    let target_file = "main.cpp";
    let gen_file = "gen.cpp";
    let folder = "tle";
    create_files_tle(
        target_file, gen_file,
        TARGET_CPP_TLE, GEN_CPP_TLE,
        folder
    )?;

    #[cfg(unix)]
    let mut cmd = Command::new("./target/debug/quicktest");

    #[cfg(windows)]
    let mut cmd = Command::new("./target/debug/quicktest.exe");

    let cases: usize = 10;

    cmd.arg("tle")
        .arg("--target-file")
        .arg(format!("{}/{}/{}", FOLDER, folder, target_file)) //target/.code/tle/main.cpp
        .arg("--gen-file")
        .arg(format!("{}/{}/{}", FOLDER, folder, gen_file)) // target/.code/tle/gen.cpp
        .arg("--timeout=1000")
        .arg(format!("--test-cases={}", cases));

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));
    
    Ok(())
}

#[test]
fn cmd_tle_gen_py_target_py() -> Result<(), Box<dyn Error>> {
    let target_file = "main.py";
    let gen_file = "gen.py";
    let folder = "tle";
    create_files_tle(
        target_file, gen_file,
        TARGET_PY_TLE, GEN_PY_TLE,
        folder
    )?;

    #[cfg(unix)]
    let mut cmd = Command::new("./target/debug/quicktest");

    #[cfg(windows)]
    let mut cmd = Command::new("./target/debug/quicktest.exe");

    let cases: usize = 10;

    cmd.arg("tle")
        .arg("--target-file")
        .arg(format!("{}/{}/{}", FOLDER, folder, target_file)) //target/.code/tle/main.cpp
        .arg("--gen-file")
        .arg(format!("{}/{}/{}", FOLDER, folder, gen_file)) // target/.code/tle/gen.cpp
        .arg("--timeout=1000")
        .arg(format!("--test-cases={}", cases));

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));
    
    Ok(())
}

#[test]
fn cmd_tle_gen_cpp_target_py() -> Result<(), Box<dyn Error>> {
    let target_file = "main.cpp";
    let gen_file = "gen.py";
    let folder = "tle";
    create_files_tle(
        target_file, gen_file,
        TARGET_CPP_TLE, GEN_PY_TLE,
        folder
    )?;

    #[cfg(unix)]
    let mut cmd = Command::new("./target/debug/quicktest");

    #[cfg(windows)]
    let mut cmd = Command::new("./target/debug/quicktest.exe");

    let cases: usize = 10;

    cmd.arg("tle")
        .arg("--target-file")
        .arg(format!("{}/{}/{}", FOLDER, folder, target_file)) //target/.code/tle/main.cpp
        .arg("--gen-file")
        .arg(format!("{}/{}/{}", FOLDER, folder, gen_file)) // target/.code/tle/gen.cpp
        .arg("--timeout=1000")
        .arg(format!("--test-cases={}", cases));

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));
    
    Ok(())
}
    
#[test]
fn cmd_tle_gen_py_target_cpp() -> Result<(), Box<dyn Error>> {
    let target_file = "main.py";
    let gen_file = "gen.cpp";
    let folder = "tle";
    create_files_tle(
        target_file, gen_file,
        TARGET_PY_TLE, GEN_CPP_TLE,
        folder
    )?;

    #[cfg(unix)]
    let mut cmd = Command::new("./target/debug/quicktest");

    #[cfg(windows)]
    let mut cmd = Command::new("./target/debug/quicktest.exe");

    let cases: usize = 10;

    cmd.arg("tle")
        .arg("--target-file")
        .arg(format!("{}/{}/{}", FOLDER, folder, target_file)) //target/.code/tle/main.cpp
        .arg("--gen-file")
        .arg(format!("{}/{}/{}", FOLDER, folder, gen_file)) // target/.code/tle/gen.cpp
        .arg("--timeout=1000")
        .arg(format!("--test-cases={}", cases));

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));
    
    Ok(())
}

// CHECK RTE in Subcommand tle

const RTE_CPP: &str = r#"
#include <bits/stdc++.h>
using namespace std;
int main() {
    // Generate divide by zero error
    for(int i = 0; i < 10; ++i) {
        int y = 10 / i;
    }
}
"#;

#[test]
fn cmd_tle_check_rte_cpp() -> Result<(), Box<dyn Error>> {
    let target_file = "main.cpp";
    let gen_file = "gen.cpp";
    let folder = "tle";
    create_files_tle(
        target_file, gen_file,
        RTE_CPP, GEN_CPP_TLE,
        folder
    )?;

    #[cfg(unix)]
    let mut cmd = Command::new("./target/debug/quicktest");

    #[cfg(windows)]
    let mut cmd = Command::new("./target/debug/quicktest.exe");

    let cases: usize = 10;

    cmd.arg("tle")
        .arg("--target-file")
        .arg(format!("{}/{}/{}", FOLDER, folder, target_file)) //target/.code/tle/main.cpp
        .arg("--gen-file")
        .arg(format!("{}/{}/{}", FOLDER, folder, gen_file)) // target/.code/tle/gen.cpp
        .arg("--timeout=1000")
        .arg(format!("--test-cases={}", cases));

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[RTE]").count(cases));
    
    Ok(())
}

const RTE_PY: &str = r#"
for i in range(10):
    print(10 / i)
"#;

#[test]
fn cmd_tle_check_rte_py() -> Result<(), Box<dyn Error>> {
    let target_file = "main.py";
    let gen_file = "gen.cpp";
    let folder = "tle";
    create_files_tle(
        target_file, gen_file,
        RTE_PY, GEN_CPP_TLE,
        folder
    )?;

    #[cfg(unix)]
    let mut cmd = Command::new("./target/debug/quicktest");

    #[cfg(windows)]
    let mut cmd = Command::new("./target/debug/quicktest.exe");

    let cases: usize = 10;

    cmd.arg("tle")
        .arg("--target-file")
        .arg(format!("{}/{}/{}", FOLDER, folder, target_file)) //target/.code/tle/main.cpp
        .arg("--gen-file")
        .arg(format!("{}/{}/{}", FOLDER, folder, gen_file)) // target/.code/tle/gen.cpp
        .arg("--timeout=1000")
        .arg(format!("--test-cases={}", cases));

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[RTE]").count(cases));
    
    Ok(())
}

// CHECK Compiler Error in Subcommand tle
const CE_CPP: &str = r#"
#include <bits/stdc++.h>
using namespace std;
int main() {
    Generate Compiler Error
}
"#;

#[test]
fn cmd_tle_check_ce_target_cpp() -> Result<(), Box<dyn Error>> {
    let target_file = "main.cpp";
    let gen_file = "gen.cpp";
    let folder = "tle";
    create_files_tle(
        target_file, gen_file,
        CE_CPP, GEN_CPP_TLE,
        folder
    )?;

    #[cfg(unix)]
    let mut cmd = Command::new("./target/debug/quicktest");

    #[cfg(windows)]
    let mut cmd = Command::new("./target/debug/quicktest.exe");

    let cases: usize = 10;

    cmd.arg("tle")
        .arg("--target-file")
        .arg(format!("{}/{}/{}", FOLDER, folder, target_file)) //target/.code/tle/main.cpp
        .arg("--gen-file")
        .arg(format!("{}/{}/{}", FOLDER, folder, gen_file)) // target/.code/tle/gen.cpp
        .arg("--timeout=1000")
        .arg(format!("--test-cases={}", cases));

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: compilation of <target-file> failed").count(1));
    
    Ok(())
}


#[test]
fn cmd_tle_check_ce_gen_cpp() -> Result<(), Box<dyn Error>> {
    let target_file = "main.cpp";
    let gen_file = "gen.cpp";
    let folder = "tle";
    create_files_tle(
        target_file, gen_file,
        TARGET_CPP_TLE, CE_CPP,
        folder
    )?;

    #[cfg(unix)]
    let mut cmd = Command::new("./target/debug/quicktest");

    #[cfg(windows)]
    let mut cmd = Command::new("./target/debug/quicktest.exe");

    let cases: usize = 10;

    cmd.arg("tle")
        .arg("--target-file")
        .arg(format!("{}/{}/{}", FOLDER, folder, target_file)) //target/.code/tle/main.cpp
        .arg("--gen-file")
        .arg(format!("{}/{}/{}", FOLDER, folder, gen_file)) // target/.code/tle/gen.cpp
        .arg("--timeout=1000")
        .arg(format!("--test-cases={}", cases));

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: compilation of <gen-file> failed").count(1));
    
    Ok(())
}
