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
    test_constants::{BINARY, CORRECT_FILE_GO, FOLDER_CMP, GEN_FILE_GO, RTE_GO, TARGET_FILE_GO},
    test_utilities::create_files_cmp,
};

use super::codes::{CORRECT_GO_CMP, GEN_GO_CMP, TARGET_GO_CMP};

// CHECK RTE in Subcommand cmp
#[test]
fn cmd_cmp_target_rte_go() -> Result<(), Box<dyn Error>> {
    create_files_cmp(
        TARGET_FILE_GO,
        CORRECT_FILE_GO,
        GEN_FILE_GO,
        RTE_GO,
        CORRECT_GO_CMP,
        GEN_GO_CMP,
        FOLDER_CMP,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(
        &mut cmd,
        TARGET_FILE_GO,
        CORRECT_FILE_GO,
        GEN_FILE_GO,
        cases,
    );

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("[RTE]").count(cases));

    Ok(())
}

#[test]
fn cmd_cmp_correct_rte_go() -> Result<(), Box<dyn Error>> {
    create_files_cmp(
        TARGET_FILE_GO,
        CORRECT_FILE_GO,
        GEN_FILE_GO,
        TARGET_GO_CMP,
        RTE_GO,
        GEN_GO_CMP,
        FOLDER_CMP,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(
        &mut cmd,
        TARGET_FILE_GO,
        CORRECT_FILE_GO,
        GEN_FILE_GO,
        cases,
    );

    cmd.assert().failure().stderr(predicate::str::contains(
        "Error: QTEST_RUNTIME_ERROR\nInfo: caused by correct file exited by Runtime Error / label <correct-file>\n",
    ));

    Ok(())
}

#[test]
fn cmd_cmp_gen_rte_go() -> Result<(), Box<dyn Error>> {
    create_files_cmp(
        TARGET_FILE_GO,
        CORRECT_FILE_GO,
        GEN_FILE_GO,
        TARGET_GO_CMP,
        CORRECT_GO_CMP,
        RTE_GO,
        FOLDER_CMP,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(
        &mut cmd,
        TARGET_FILE_GO,
        CORRECT_FILE_GO,
        GEN_FILE_GO,
        cases,
    );

    cmd.assert().failure().stderr(predicate::str::contains(
        "Error: QTEST_RUNTIME_ERROR\nInfo: caused by generator file exited by Runtime Error / label <gen-file>\n",
    ));

    Ok(())
}
