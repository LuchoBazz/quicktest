#![allow(unused_imports)]
/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021-present / Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::{error::Error, process::Command};

use assert_cmd::assert::OutputAssertExt;
use predicates::prelude::predicate;

use crate::util::{
    test_command_handler::execute_command_cmp_with_timeout,
    test_constants::{BINARY, CORRECT_FILE_C, FOLDER_CMP, GEN_FILE_C, RTE_C, TARGET_FILE_C},
    test_utilities::create_files_cmp,
};

use super::codes::{CORRECT_C_CMP, GEN_C_CMP, TARGET_C_CMP};

// CHECK RTE in Subcommand cmp
#[test]
fn cmd_cmp_target_rte_c() -> Result<(), Box<dyn Error>> {
    create_files_cmp(
        TARGET_FILE_C,
        CORRECT_FILE_C,
        GEN_FILE_C,
        RTE_C,
        CORRECT_C_CMP,
        GEN_C_CMP,
        FOLDER_CMP,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp_with_timeout(
        &mut cmd,
        TARGET_FILE_C,
        CORRECT_FILE_C,
        GEN_FILE_C,
        cases,
        3000usize,
    );

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("[RTE]").count(cases));

    Ok(())
}

#[test]
fn cmd_cmp_correct_rte_c() -> Result<(), Box<dyn Error>> {
    create_files_cmp(
        TARGET_FILE_C,
        CORRECT_FILE_C,
        GEN_FILE_C,
        TARGET_C_CMP,
        RTE_C,
        GEN_C_CMP,
        FOLDER_CMP,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp_with_timeout(
        &mut cmd,
        TARGET_FILE_C,
        CORRECT_FILE_C,
        GEN_FILE_C,
        cases,
        3000usize,
    );

    cmd.assert().failure().stderr(predicate::str::contains(
        "Error: QTEST_RUNTIME_ERROR\nInfo: caused by correct file exited by Runtime Error / label <correct-file>\n",
    ));

    Ok(())
}

#[test]
fn cmd_cmp_gen_rte_c() -> Result<(), Box<dyn Error>> {
    create_files_cmp(
        TARGET_FILE_C,
        CORRECT_FILE_C,
        GEN_FILE_C,
        TARGET_C_CMP,
        CORRECT_C_CMP,
        RTE_C,
        FOLDER_CMP,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp_with_timeout(
        &mut cmd,
        TARGET_FILE_C,
        CORRECT_FILE_C,
        GEN_FILE_C,
        cases,
        3000usize,
    );

    cmd.assert().failure().stderr(predicate::str::contains(
        "Error: QTEST_RUNTIME_ERROR\nInfo: caused by generator file exited by Runtime Error / label <gen-file>\n",
    ));

    Ok(())
}
