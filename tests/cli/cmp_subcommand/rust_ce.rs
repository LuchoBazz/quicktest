/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021-present / Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::{error::Error, process::Command};

use assert_cmd::assert::OutputAssertExt;
use predicates::prelude::predicate;

use crate::util::{
    test_command_handler::execute_command_cmp,
    test_constants::{
        BINARY, CE_RUST, CORRECT_FILE_RUST, FOLDER_CMP, GEN_FILE_RUST, TARGET_FILE_RUST,
    },
    test_utilities::create_files_cmp,
};

use super::codes::{CORRECT_RUST_CMP, GEN_RUST_CMP, TARGET_RUST_CMP};

#[test]
fn cmd_cmp_target_ce_rust() -> Result<(), Box<dyn Error>> {
    create_files_cmp(
        TARGET_FILE_RUST,
        CORRECT_FILE_RUST,
        GEN_FILE_RUST,
        CE_RUST,
        CORRECT_RUST_CMP,
        GEN_RUST_CMP,
        FOLDER_CMP,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(
        &mut cmd,
        TARGET_FILE_RUST,
        CORRECT_FILE_RUST,
        GEN_FILE_RUST,
        cases,
    );

    cmd.assert().failure().stderr(predicate::str::contains(
        "Error: QTEST_COMPILER_ERROR\nInfo: caused by failed to compile the target file / label <target-file>",
    ));

    Ok(())
}

#[test]
fn cmd_cmp_correct_ce_rust() -> Result<(), Box<dyn Error>> {
    create_files_cmp(
        TARGET_FILE_RUST,
        CORRECT_FILE_RUST,
        GEN_FILE_RUST,
        TARGET_RUST_CMP,
        CE_RUST,
        GEN_RUST_CMP,
        FOLDER_CMP,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(
        &mut cmd,
        TARGET_FILE_RUST,
        CORRECT_FILE_RUST,
        GEN_FILE_RUST,
        cases,
    );

    cmd.assert().failure().stderr(predicate::str::contains(
        "Error: QTEST_COMPILER_ERROR\nInfo: caused by failed to compile the correct file / label <correct-file>",
    ));

    Ok(())
}

#[test]
fn cmd_cmp_gen_ce_rust() -> Result<(), Box<dyn Error>> {
    create_files_cmp(
        TARGET_FILE_RUST,
        CORRECT_FILE_RUST,
        GEN_FILE_RUST,
        TARGET_RUST_CMP,
        GEN_RUST_CMP,
        CE_RUST,
        FOLDER_CMP,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(
        &mut cmd,
        TARGET_FILE_RUST,
        CORRECT_FILE_RUST,
        GEN_FILE_RUST,
        cases,
    );

    cmd.assert().failure().stderr(predicate::str::contains(
        "Error: QTEST_COMPILER_ERROR\nInfo: caused by failed to compile the generator file / label <gen-file>",
    ));

    Ok(())
}
