/*
*  Quick Test: CLI for stress testing in competitive programming
*  Copyright (C) 2021 - Luis Miguel Báez
*  License: MIT (See the LICENSE file in the repository root directory)
*/

use std::{error::Error, process::Command};

use assert_cmd::assert::OutputAssertExt;
use predicates::prelude::predicate;

use crate::util::{
    test_command_handler::execute_command_check,
    test_constants::{
        BINARY, CE_CPP, CHECKER_FILE_CPP, FOLDER_CHECK, GEN_FILE_CPP, TARGET_FILE_CPP,
    },
    test_utilities::create_files_check,
};

use super::codes::{CHECKER_CPP_CHECK, GEN_CPP_CHECK, TARGET_CPP_CHECK};

#[test]
fn cmd_check_target_ce_cpp() -> Result<(), Box<dyn Error>> {
    create_files_check(
        TARGET_FILE_CPP,
        CHECKER_FILE_CPP,
        GEN_FILE_CPP,
        CE_CPP,
        CHECKER_CPP_CHECK,
        GEN_CPP_CHECK,
        FOLDER_CHECK,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_check(
        &mut cmd,
        TARGET_FILE_CPP,
        CHECKER_FILE_CPP,
        GEN_FILE_CPP,
        cases,
    );

    cmd.assert().failure().stderr(predicate::str::contains(
        "Error: QTEST_COMPILER_ERROR\nInfo: caused by failed to compile the target file / label <target-file>",
    ));

    Ok(())
}

#[test]
fn cmd_check_checker_ce_cpp() -> Result<(), Box<dyn Error>> {
    create_files_check(
        TARGET_FILE_CPP,
        CHECKER_FILE_CPP,
        GEN_FILE_CPP,
        TARGET_CPP_CHECK,
        CE_CPP,
        GEN_CPP_CHECK,
        FOLDER_CHECK,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_check(
        &mut cmd,
        TARGET_FILE_CPP,
        CHECKER_FILE_CPP,
        GEN_FILE_CPP,
        cases,
    );

    cmd.assert().failure().stderr(predicate::str::contains(
        "Error: QTEST_COMPILER_ERROR\nInfo: caused by failed to compile the checker file / label <checker-file>",
    ));

    Ok(())
}

#[test]
fn cmd_check_gen_ce_cpp() -> Result<(), Box<dyn Error>> {
    create_files_check(
        TARGET_FILE_CPP,
        CHECKER_FILE_CPP,
        GEN_FILE_CPP,
        TARGET_CPP_CHECK,
        CHECKER_CPP_CHECK,
        CE_CPP,
        FOLDER_CHECK,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_check(
        &mut cmd,
        TARGET_FILE_CPP,
        CHECKER_FILE_CPP,
        GEN_FILE_CPP,
        cases,
    );

    cmd.assert().failure().stderr(predicate::str::contains(
        "Error: QTEST_COMPILER_ERROR\nInfo: caused by failed to compile the generator file / label <gen-file>",
    ));

    Ok(())
}
