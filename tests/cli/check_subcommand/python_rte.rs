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
    test_constants::{BINARY, CHECKER_FILE_PY, FOLDER_CHECK, GEN_FILE_PY, RTE_PY, TARGET_FILE_PY},
    test_utilities::create_files_check,
};

use super::codes::{CHECKER_PY_CHECK, GEN_PY_CHECK, TARGET_PY_CHECK};

#[test]
fn cmd_check_target_rte_py() -> Result<(), Box<dyn Error>> {
    create_files_check(
        TARGET_FILE_PY,
        CHECKER_FILE_PY,
        GEN_FILE_PY,
        RTE_PY,
        CHECKER_PY_CHECK,
        GEN_PY_CHECK,
        FOLDER_CHECK,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_check(
        &mut cmd,
        TARGET_FILE_PY,
        CHECKER_FILE_PY,
        GEN_FILE_PY,
        cases,
    );

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[RTE]").count(cases));

    Ok(())
}

#[test]
fn cmd_check_checker_rte_py() -> Result<(), Box<dyn Error>> {
    create_files_check(
        TARGET_FILE_PY,
        CHECKER_FILE_PY,
        GEN_FILE_PY,
        TARGET_PY_CHECK,
        RTE_PY,
        GEN_PY_CHECK,
        FOLDER_CHECK,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_check(
        &mut cmd,
        TARGET_FILE_PY,
        CHECKER_FILE_PY,
        GEN_FILE_PY,
        cases,
    );

    cmd.assert().failure().stderr(predicate::str::contains(
        "Error: QTEST_RUNTIME_ERROR\nInfo: caused by checker file exited by Runtime Error / label <checker-file>\n",
    ));

    Ok(())
}

#[test]
fn cmd_check_gen_rte_py() -> Result<(), Box<dyn Error>> {
    create_files_check(
        TARGET_FILE_PY,
        CHECKER_FILE_PY,
        GEN_FILE_PY,
        TARGET_PY_CHECK,
        CHECKER_PY_CHECK,
        RTE_PY,
        FOLDER_CHECK,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_check(
        &mut cmd,
        TARGET_FILE_PY,
        CHECKER_FILE_PY,
        GEN_FILE_PY,
        cases,
    );

    cmd.assert().failure().stderr(predicate::str::contains(
        "Error: QTEST_RUNTIME_ERROR\nInfo: caused by generator file exited by Runtime Error / label <gen-file>\n",
    ));

    Ok(())
}
