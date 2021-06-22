/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::{error::Error, process::Command};

use assert_cmd::assert::OutputAssertExt;
use predicates::prelude::predicate;

use crate::util::{
    test_command_handler::execute_command_cmp,
    test_constants::{
        BINARY, CORRECT_FILE_CPP, CORRECT_FILE_PY, FOLDER_CMP, GEN_FILE_CPP, GEN_FILE_PY,
        TARGET_FILE_CPP, TARGET_FILE_PY,
    },
    test_utilities::create_files_cmp,
};

use super::codes::{
    CORRECT_CPP_CMP, CORRECT_PY_CMP, GEN_CPP_CMP, GEN_PY_CMP, TARGET_CPP_CMP, TARGET_PY_CMP,
};

#[test]
fn cmp_target_cpp_correct_cpp_gen_cpp() -> Result<(), Box<dyn Error>> {
    create_files_cmp(
        TARGET_FILE_CPP,
        CORRECT_FILE_CPP,
        GEN_FILE_CPP,
        TARGET_CPP_CMP,
        CORRECT_CPP_CMP,
        GEN_CPP_CMP,
        FOLDER_CMP,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(
        &mut cmd,
        TARGET_FILE_CPP,
        CORRECT_FILE_CPP,
        GEN_FILE_CPP,
        cases,
    );

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));

    Ok(())
}

#[test]
fn cmp_target_py_correct_py_gen_py() -> Result<(), Box<dyn Error>> {
    create_files_cmp(
        TARGET_FILE_PY,
        CORRECT_FILE_PY,
        GEN_FILE_PY,
        TARGET_PY_CMP,
        CORRECT_PY_CMP,
        GEN_PY_CMP,
        FOLDER_CMP,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(
        &mut cmd,
        TARGET_FILE_PY,
        CORRECT_FILE_PY,
        GEN_FILE_PY,
        cases,
    );

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));

    Ok(())
}

#[test]
fn cmp_target_cpp_correct_cpp_gen_py() -> Result<(), Box<dyn Error>> {
    create_files_cmp(
        TARGET_FILE_CPP,
        CORRECT_FILE_CPP,
        GEN_FILE_PY,
        TARGET_CPP_CMP,
        CORRECT_CPP_CMP,
        GEN_PY_CMP,
        FOLDER_CMP,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(
        &mut cmd,
        TARGET_FILE_CPP,
        CORRECT_FILE_CPP,
        GEN_FILE_PY,
        cases,
    );

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));

    Ok(())
}

#[test]
fn cmp_target_cpp_correct_py_gen_py() -> Result<(), Box<dyn Error>> {
    create_files_cmp(
        TARGET_FILE_CPP,
        CORRECT_FILE_PY,
        GEN_FILE_PY,
        TARGET_CPP_CMP,
        CORRECT_PY_CMP,
        GEN_PY_CMP,
        FOLDER_CMP,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(
        &mut cmd,
        TARGET_FILE_CPP,
        CORRECT_FILE_PY,
        GEN_FILE_PY,
        cases,
    );

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));

    Ok(())
}

#[test]
fn cmp_target_cpp_correct_py_gen_cpp() -> Result<(), Box<dyn Error>> {
    create_files_cmp(
        TARGET_FILE_CPP,
        CORRECT_FILE_PY,
        GEN_FILE_CPP,
        TARGET_CPP_CMP,
        CORRECT_PY_CMP,
        GEN_CPP_CMP,
        FOLDER_CMP,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(
        &mut cmd,
        TARGET_FILE_CPP,
        CORRECT_FILE_PY,
        GEN_FILE_CPP,
        cases,
    );

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));

    Ok(())
}

#[test]
fn cmp_target_py_correct_py_gen_cpp() -> Result<(), Box<dyn Error>> {
    create_files_cmp(
        TARGET_FILE_PY,
        CORRECT_FILE_PY,
        GEN_FILE_CPP,
        TARGET_PY_CMP,
        CORRECT_PY_CMP,
        GEN_CPP_CMP,
        FOLDER_CMP,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(
        &mut cmd,
        TARGET_FILE_PY,
        CORRECT_FILE_PY,
        GEN_FILE_CPP,
        cases,
    );

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));

    Ok(())
}

#[test]
fn cmp_target_py_correct_cpp_gen_cpp() -> Result<(), Box<dyn Error>> {
    create_files_cmp(
        TARGET_FILE_PY,
        CORRECT_FILE_CPP,
        GEN_FILE_CPP,
        TARGET_PY_CMP,
        CORRECT_CPP_CMP,
        GEN_CPP_CMP,
        FOLDER_CMP,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(
        &mut cmd,
        TARGET_FILE_PY,
        CORRECT_FILE_CPP,
        GEN_FILE_CPP,
        cases,
    );

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));

    Ok(())
}

#[test]
fn cmp_target_py_correct_cpp_gen_py() -> Result<(), Box<dyn Error>> {
    create_files_cmp(
        TARGET_FILE_PY,
        CORRECT_FILE_CPP,
        GEN_FILE_PY,
        TARGET_PY_CMP,
        CORRECT_CPP_CMP,
        GEN_PY_CMP,
        FOLDER_CMP,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(
        &mut cmd,
        TARGET_FILE_PY,
        CORRECT_FILE_CPP,
        GEN_FILE_PY,
        cases,
    );

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));

    Ok(())
}
