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
    test_command_handler::execute_command_cmp,
    test_constants::{
        BINARY, CORRECT_FILE_RUST, FOLDER_CMP, GEN_FILE_RUST, RTE_RUST, TARGET_FILE_RUST,
    },
    test_utilities::create_files_cmp,
};

use super::codes::{CORRECT_RUST_CMP, GEN_RUST_CMP, TARGET_RUST_CMP};

// CHECK RTE in Subcommand cmp
#[test]
#[cfg(not(target_os = "macos"))]
fn cmd_cmp_target_rte_rust() -> Result<(), Box<dyn Error>> {
    create_files_cmp(
        TARGET_FILE_RUST,
        CORRECT_FILE_RUST,
        GEN_FILE_RUST,
        RTE_RUST,
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

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("[RTE]").count(cases));

    Ok(())
}

#[test]
#[cfg(not(target_os = "macos"))]
fn cmd_cmp_correct_rte_rust() -> Result<(), Box<dyn Error>> {
    create_files_cmp(
        TARGET_FILE_RUST,
        CORRECT_FILE_RUST,
        GEN_FILE_RUST,
        TARGET_RUST_CMP,
        RTE_RUST,
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
        "Error: QTEST_RUNTIME_ERROR\nInfo: caused by correct file exited by Runtime Error / label <correct-file>\n",
    ));

    Ok(())
}

#[test]
fn cmd_cmp_gen_rte_rust() -> Result<(), Box<dyn Error>> {
    create_files_cmp(
        TARGET_FILE_RUST,
        CORRECT_FILE_RUST,
        GEN_FILE_RUST,
        TARGET_RUST_CMP,
        CORRECT_RUST_CMP,
        RTE_RUST,
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
        "Error: QTEST_RUNTIME_ERROR\nInfo: caused by generator file exited by Runtime Error / label <gen-file>\n",
    ));

    Ok(())
}
