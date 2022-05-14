/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021-present / Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::process::Command;

use assert_cmd::assert::OutputAssertExt;
use predicates::prelude::predicate;
use std::error::Error;

use crate::util::test_constants::{GEN_FILE_PY, TARGET_FILE_PY};
use crate::util::{
    test_command_handler::execute_command_stress,
    test_constants::{BINARY, FOLDER_STRESS, GEN_FILE_CPP, TARGET_FILE_CPP},
    test_utilities::create_files_tle,
};

use super::codes::{GEN_CPP_STRESS, GEN_PY_STRESS, TARGET_CPP_STRESS, TARGET_PY_STRESS};

#[test]
fn cmd_stress_gen_cpp_target_cpp() -> Result<(), Box<dyn Error>> {
    create_files_tle(
        TARGET_FILE_CPP,
        GEN_FILE_CPP,
        TARGET_CPP_STRESS,
        GEN_CPP_STRESS,
        FOLDER_STRESS,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_stress(&mut cmd, TARGET_FILE_CPP, GEN_FILE_CPP, cases);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));

    Ok(())
}

#[test]
fn cmd_stress_gen_py_target_py() -> Result<(), Box<dyn Error>> {
    create_files_tle(
        TARGET_FILE_PY,
        GEN_FILE_PY,
        TARGET_PY_STRESS,
        GEN_PY_STRESS,
        FOLDER_STRESS,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_stress(&mut cmd, TARGET_FILE_PY, GEN_FILE_PY, cases);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));

    Ok(())
}

#[test]
fn cmd_stress_gen_cpp_target_py() -> Result<(), Box<dyn Error>> {
    create_files_tle(
        TARGET_FILE_CPP,
        GEN_FILE_PY,
        TARGET_CPP_STRESS,
        GEN_PY_STRESS,
        FOLDER_STRESS,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_stress(&mut cmd, TARGET_FILE_CPP, GEN_FILE_PY, cases);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));

    Ok(())
}

#[test]
fn cmd_stress_gen_py_target_cpp() -> Result<(), Box<dyn Error>> {
    create_files_tle(
        TARGET_FILE_PY,
        GEN_FILE_CPP,
        TARGET_PY_STRESS,
        GEN_CPP_STRESS,
        FOLDER_STRESS,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_stress(&mut cmd, TARGET_FILE_PY, GEN_FILE_CPP, cases);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));

    Ok(())
}
