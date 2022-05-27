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
    test_constants::{BINARY, CE_C, CORRECT_FILE_C, FOLDER_CMP, GEN_FILE_C, TARGET_FILE_C},
    test_utilities::create_files_cmp,
};

use super::codes::{CORRECT_C_CMP, GEN_C_CMP, TARGET_C_CMP};

#[test]
fn cmd_cmp_target_ce_cpp() -> Result<(), Box<dyn Error>> {
    create_files_cmp(
        TARGET_FILE_C,
        CORRECT_FILE_C,
        GEN_FILE_C,
        CE_C,
        CORRECT_C_CMP,
        GEN_C_CMP,
        FOLDER_CMP,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(&mut cmd, TARGET_FILE_C, CORRECT_FILE_C, GEN_FILE_C, cases);

    cmd.assert().failure().stderr(predicate::str::contains(
        "Error: QTEST_COMPILER_ERROR\nInfo: caused by failed to compile the target file / label <target-file>",
    ));

    Ok(())
}

#[test]
fn cmd_cmp_correct_ce_cpp() -> Result<(), Box<dyn Error>> {
    create_files_cmp(
        TARGET_FILE_C,
        CORRECT_FILE_C,
        GEN_FILE_C,
        TARGET_C_CMP,
        CE_C,
        GEN_C_CMP,
        FOLDER_CMP,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(&mut cmd, TARGET_FILE_C, CORRECT_FILE_C, GEN_FILE_C, cases);

    cmd.assert().failure().stderr(predicate::str::contains(
        "Error: QTEST_COMPILER_ERROR\nInfo: caused by failed to compile the correct file / label <correct-file>",
    ));

    Ok(())
}

#[test]
fn cmd_cmp_gen_ce_cpp() -> Result<(), Box<dyn Error>> {
    create_files_cmp(
        TARGET_FILE_C,
        CORRECT_FILE_C,
        GEN_FILE_C,
        TARGET_C_CMP,
        GEN_C_CMP,
        CE_C,
        FOLDER_CMP,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(&mut cmd, TARGET_FILE_C, CORRECT_FILE_C, GEN_FILE_C, cases);

    cmd.assert().failure().stderr(predicate::str::contains(
        "Error: QTEST_COMPILER_ERROR\nInfo: caused by failed to compile the generator file / label <gen-file>",
    ));

    Ok(())
}
