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

use crate::cli::test_constants::{
    TARGET_CPP_TLE, TARGET_PY_TLE,
    GEN_CPP_TLE,  GEN_PY_TLE,
    FOLDER
};
use crate::cli::test_utilities::create_files_tle;

#[test]
fn tle_gen_cpp_target_cpp() -> Result<(), Box<dyn Error>> {
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
fn tle_gen_py_target_py() -> Result<(), Box<dyn Error>> {
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
fn tle_gen_cpp_target_py() -> Result<(), Box<dyn Error>> {
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
fn tle_gen_py_target_cpp() -> Result<(), Box<dyn Error>> {
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