/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::process::Command;
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertion
use std::error::Error;

use crate::util::test_utilities::create_files_check;
use crate::util::test_constants::{
    FOLDER,
    TARGET_CPP_CHECK, CHECKER_CPP_CHECK, GEN_CPP_CHECK
};

#[test]
fn check_target_cpp_check_cpp_gen_cpp() -> Result<(), Box<dyn Error>> {
    let target_file = "main.cpp";
    let checker_file = "checker.cpp";
    let gen_file = "gen.cpp";
    let folder = "check";
    create_files_check(
        target_file, checker_file, gen_file,
        TARGET_CPP_CHECK, CHECKER_CPP_CHECK, GEN_CPP_CHECK,
        folder
    )?;

    #[cfg(unix)]
    let mut cmd = Command::new("./target/debug/quicktest");

    #[cfg(windows)]
    let mut cmd = Command::new("./target/debug/quicktest.exe");

    let cases: usize = 10;

    cmd.arg("check")
        .arg("--target-file")
        .arg(format!("{}/{}/{}", FOLDER, folder, target_file)) //target/.code/cmp/main.cpp
        .arg("--checker-file")
        .arg(format!("{}/{}/{}", FOLDER, folder, checker_file)) // target/.code/cmp/correct.cpp
        .arg("--gen-file")
        .arg(format!("{}/{}/{}", FOLDER, folder, gen_file)) // target/.code/cmp/gen.cpp
        .arg("--timeout=1000")
        .arg(format!("--test-cases={}", cases));
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));

    Ok(())
}
