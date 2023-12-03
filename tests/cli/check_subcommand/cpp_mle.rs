/*
*  Quick Test: CLI for stress testing in competitive programming
*  Copyright (C) 2021-present / Luis Miguel Báez
*  License: MIT (See the LICENSE file in the repository root directory)
*/

#![allow(unused_imports)]

use std::{error::Error, process::Command};

use assert_cmd::assert::OutputAssertExt;
use predicates::prelude::predicate;

use crate::util::{
    test_command_handler::execute_command_check_with_timeout,
    test_constants::{
        BINARY, CHECKER_FILE_CPP, FOLDER_CHECK, GEN_FILE_CPP, MLE_CPP, TARGET_FILE_CPP,
    },
    test_utilities::create_files_check,
};

use super::codes::{CHECKER_CPP_CHECK, GEN_CPP_CHECK, TARGET_CPP_CHECK};

#[test]
#[cfg(not(target_os = "macos"))]
fn cmd_check_target_mle_cpp() -> Result<(), Box<dyn Error>> {
    create_files_check(
        TARGET_FILE_CPP,
        CHECKER_FILE_CPP,
        GEN_FILE_CPP,
        MLE_CPP,
        CHECKER_CPP_CHECK,
        GEN_CPP_CHECK,
        FOLDER_CHECK,
    )?;
    let cases: usize = 3;

    let mut cmd = Command::new(BINARY);
    execute_command_check_with_timeout(
        &mut cmd,
        TARGET_FILE_CPP,
        CHECKER_FILE_CPP,
        GEN_FILE_CPP,
        cases,
        5000usize,
    );

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("[MLE]").count(cases));

    Ok(())
}

#[test]
#[cfg(not(target_os = "macos"))]
fn cmd_check_checker_mle_cpp() -> Result<(), Box<dyn Error>> {
    create_files_check(
        TARGET_FILE_CPP,
        CHECKER_FILE_CPP,
        GEN_FILE_CPP,
        TARGET_CPP_CHECK,
        MLE_CPP,
        GEN_CPP_CHECK,
        FOLDER_CHECK,
    )?;
    let cases: usize = 3;

    let mut cmd = Command::new(BINARY);
    execute_command_check_with_timeout(
        &mut cmd,
        TARGET_FILE_CPP,
        CHECKER_FILE_CPP,
        GEN_FILE_CPP,
        cases,
        5000usize,
    );

    cmd.assert().failure().stderr(predicate::str::contains(
        "Error: QTEST_MEMORY_LIMIT_EXCEEDED\nInfo: caused by checker file give memory limit exceeded / label <checker-file>\n",
    ));

    Ok(())
}

#[test]
#[cfg(not(target_os = "macos"))]
fn cmd_check_gen_mle_cpp() -> Result<(), Box<dyn Error>> {
    create_files_check(
        TARGET_FILE_CPP,
        CHECKER_FILE_CPP,
        GEN_FILE_CPP,
        TARGET_CPP_CHECK,
        CHECKER_CPP_CHECK,
        MLE_CPP,
        FOLDER_CHECK,
    )?;
    let cases: usize = 3;

    let mut cmd = Command::new(BINARY);
    execute_command_check_with_timeout(
        &mut cmd,
        TARGET_FILE_CPP,
        CHECKER_FILE_CPP,
        GEN_FILE_CPP,
        cases,
        5000usize,
    );

    cmd.assert().failure().stderr(predicate::str::contains(
        "Error: QTEST_MEMORY_LIMIT_EXCEEDED\nInfo: caused by generator file give memory limit exceeded / label <gen-file>\n",
    ));

    Ok(())
}
