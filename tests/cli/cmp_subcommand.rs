/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::process::Command;
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertion
use std::error::Error;

use crate::util::test_utilities::create_files_cmp;
use crate::util::test_constants::{
    FOLDER,
    TARGET_CPP_CMP, CORRECT_CPP_CMP, GEN_CPP_CMP,
    TARGET_PY_CMP, CORRECT_PY_CMP, GEN_PY_CMP
};

#[test]
fn cmp_target_cpp_correct_cpp_gen_cpp() -> Result<(), Box<dyn Error>> {
    let target_file = "main.cpp";
    let correct_file = "correct.cpp";
    let gen_file = "gen.cpp";
    let folder = "cmp";
    create_files_cmp(
        target_file, correct_file, gen_file,
        TARGET_CPP_CMP, CORRECT_CPP_CMP, GEN_CPP_CMP,
        folder
    )?;

    #[cfg(unix)]
    let mut cmd = Command::new("./target/debug/quicktest");

    #[cfg(windows)]
    let mut cmd = Command::new("./target/debug/quicktest.exe");

    let cases: usize = 10;

    cmd.arg("cmp")
        .arg("--target-file")
        .arg(format!("{}/{}/{}", FOLDER, folder, target_file)) //target/.code/cmp/main.cpp
        .arg("--correct-file")
        .arg(format!("{}/{}/{}", FOLDER, folder, correct_file)) // target/.code/cmp/correct.cpp
        .arg("--gen-file")
        .arg(format!("{}/{}/{}", FOLDER, folder, gen_file)) // target/.code/cmp/gen.cpp
        .arg("--timeout=1000")
        .arg(format!("--test-cases={}", cases));
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));

    Ok(())
}


#[test]
fn cmp_target_py_correct_py_gen_py() -> Result<(), Box<dyn Error>> {
    let target_file = "main.py";
    let correct_file = "correct.py";
    let gen_file = "gen.py";
    let folder = "cmp";
    create_files_cmp(
        target_file, correct_file, gen_file,
        TARGET_PY_CMP, CORRECT_PY_CMP, GEN_PY_CMP,
        folder
    )?;

    #[cfg(unix)]
    let mut cmd = Command::new("./target/debug/quicktest");

    #[cfg(windows)]
    let mut cmd = Command::new("./target/debug/quicktest.exe");

    let cases: usize = 10;

    cmd.arg("cmp")
        .arg("--target-file")
        .arg(format!("{}/{}/{}", FOLDER, folder, target_file)) //target/.code/cmp/main.cpp
        .arg("--correct-file")
        .arg(format!("{}/{}/{}", FOLDER, folder, correct_file)) // target/.code/cmp/correct.cpp
        .arg("--gen-file")
        .arg(format!("{}/{}/{}", FOLDER, folder, gen_file)) // target/.code/cmp/gen.cpp
        .arg("--timeout=1000")
        .arg(format!("--test-cases={}", cases));
    
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));

    Ok(())
}