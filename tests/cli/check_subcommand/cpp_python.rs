/*
*  Quick Test: CLI for stress testing in competitive programming
*  Copyright (C) 2021 - Luis Miguel Báez
*  License: MIT (See the LICENSE file in the repository root directory)
*/

use std::{error::Error, process::Command};

use assert_cmd::assert::OutputAssertExt;
use predicates::prelude::predicate;

use crate::util::{test_command_handler::execute_command_check, test_constants::{BINARY, CHECKER_FILE_CPP, CHECKER_FILE_PY, FOLDER_CHECK, GEN_FILE_CPP, GEN_FILE_PY, TARGET_FILE_CPP, TARGET_FILE_PY}, test_utilities::create_files_check};

use super::codes::{CHECKER_CPP_CHECK, CHECKER_PY_CHECK, GEN_CPP_CHECK, GEN_PY_CHECK, TARGET_CPP_CHECK, TARGET_PY_CHECK};

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