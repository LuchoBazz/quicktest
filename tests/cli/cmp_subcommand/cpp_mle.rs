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
    test_command_handler::execute_command_cmp_with_timeout,
    test_constants::{
        BINARY, CORRECT_FILE_CPP, FOLDER_CMP, GEN_FILE_CPP, MLE_CPP, TARGET_FILE_CPP,
    },
    test_utilities::create_files_cmp,
};

use super::codes::{CORRECT_CPP_CMP, GEN_CPP_CMP, TARGET_CPP_CMP};

// CHECK MLE in Subcommand cmp
#[test]
#[cfg(not(target_os = "macos"))]
fn cmd_cmp_target_mle_cpp() -> Result<(), Box<dyn Error>> {
    create_files_cmp(
        TARGET_FILE_CPP,
        CORRECT_FILE_CPP,
        GEN_FILE_CPP,
        MLE_CPP,
        CORRECT_CPP_CMP,
        GEN_CPP_CMP,
        FOLDER_CMP,
    )?;
    let cases: usize = 3;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp_with_timeout(
        &mut cmd,
        TARGET_FILE_CPP,
        CORRECT_FILE_CPP,
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
fn cmd_cmp_correct_mle_cpp() -> Result<(), Box<dyn Error>> {
    create_files_cmp(
        TARGET_FILE_CPP,
        CORRECT_FILE_CPP,
        GEN_FILE_CPP,
        TARGET_CPP_CMP,
        MLE_CPP,
        GEN_CPP_CMP,
        FOLDER_CMP,
    )?;
    let cases: usize = 3;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp_with_timeout(
        &mut cmd,
        TARGET_FILE_CPP,
        CORRECT_FILE_CPP,
        GEN_FILE_CPP,
        cases,
        5000usize,
    );

    cmd.assert().failure().stderr(predicate::str::contains(
        "Error: QTEST_MEMORY_LIMIT_EXCEEDED\nInfo: caused by correct file give memory limit exceeded / label <correct-file>\n",
    ));

    Ok(())
}

#[test]
#[cfg(not(target_os = "macos"))]
fn cmd_cmp_gen_mle_cpp() -> Result<(), Box<dyn Error>> {
    create_files_cmp(
        TARGET_FILE_CPP,
        CORRECT_FILE_CPP,
        GEN_FILE_CPP,
        TARGET_CPP_CMP,
        CORRECT_CPP_CMP,
        MLE_CPP,
        FOLDER_CMP,
    )?;
    let cases: usize = 3;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp_with_timeout(
        &mut cmd,
        TARGET_FILE_CPP,
        CORRECT_FILE_CPP,
        GEN_FILE_CPP,
        cases,
        5000usize,
    );

    cmd.assert().failure().stderr(predicate::str::contains(
        "Error: QTEST_MEMORY_LIMIT_EXCEEDED\nInfo: caused by generator file give memory limit exceeded / label <gen-file>\n",
    ));

    Ok(())
}
