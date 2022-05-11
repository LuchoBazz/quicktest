/*
*  Quick Test: CLI for stress testing in competitive programming
*  Copyright (C) 2021-present / Luis Miguel Báez
*  License: MIT (See the LICENSE file in the repository root directory)
*/

use std::{error::Error, process::Command};

use assert_cmd::assert::OutputAssertExt;
use predicates::prelude::predicate;

use crate::util::{
    test_command_handler::execute_command_check,
    test_constants::{
        BINARY, CHECKER_FILE_CPP, FOLDER_CHECK, GEN_FILE_CPP, RTE_CPP, TARGET_FILE_CPP,
    },
    test_utilities::create_files_check,
};

use super::codes::{CHECKER_CPP_CHECK, GEN_CPP_CHECK, TARGET_CPP_CHECK};

#[test]
fn cmd_check_target_rte_cpp() -> Result<(), Box<dyn Error>> {
    create_files_check(
        TARGET_FILE_CPP,
        CHECKER_FILE_CPP,
        GEN_FILE_CPP,
        RTE_CPP,
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

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("[RTE]").count(cases));

    Ok(())
}

#[test]
fn cmd_check_checker_rte_cpp() -> Result<(), Box<dyn Error>> {
    create_files_check(
        TARGET_FILE_CPP,
        CHECKER_FILE_CPP,
        GEN_FILE_CPP,
        TARGET_CPP_CHECK,
        RTE_CPP,
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
        "Error: QTEST_RUNTIME_ERROR\nInfo: caused by checker file exited by Runtime Error / label <checker-file>\n",
    ));

    Ok(())
}

#[test]
fn cmd_check_gen_rte_cpp() -> Result<(), Box<dyn Error>> {
    create_files_check(
        TARGET_FILE_CPP,
        CHECKER_FILE_CPP,
        GEN_FILE_CPP,
        TARGET_CPP_CHECK,
        CHECKER_CPP_CHECK,
        RTE_CPP,
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
        "Error: QTEST_RUNTIME_ERROR\nInfo: caused by generator file exited by Runtime Error / label <gen-file>\n",
    ));

    Ok(())
}
