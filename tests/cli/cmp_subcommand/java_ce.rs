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
    test_constants::{
        BINARY, CE_JAVA, CORRECT_FILE_JAVA, FOLDER_CMP, GEN_FILE_JAVA, TARGET_FILE_JAVA,
    },
    test_utilities::create_files_cmp,
};

use super::codes::{CORRECT_JAVA_CMP, GEN_JAVA_CMP, TARGET_JAVA_CMP};

#[test]
fn cmd_cmp_target_ce_java() -> Result<(), Box<dyn Error>> {
    create_files_cmp(
        TARGET_FILE_JAVA,
        CORRECT_FILE_JAVA,
        GEN_FILE_JAVA,
        CE_JAVA,
        CORRECT_JAVA_CMP,
        GEN_JAVA_CMP,
        FOLDER_CMP,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(
        &mut cmd,
        TARGET_FILE_JAVA,
        CORRECT_FILE_JAVA,
        GEN_FILE_JAVA,
        cases,
    );

    cmd.assert().failure().stderr(predicate::str::contains(
        "Error: QTEST_COMPILER_ERROR\nInfo: caused by failed to compile the target file / label <target-file>",
    ));

    Ok(())
}

#[test]
fn cmd_cmp_correct_ce_java() -> Result<(), Box<dyn Error>> {
    let ce = CE_JAVA.to_string().replace("Main", "Correct");
    create_files_cmp(
        TARGET_FILE_JAVA,
        CORRECT_FILE_JAVA,
        GEN_FILE_JAVA,
        TARGET_JAVA_CMP,
        &ce[..],
        GEN_JAVA_CMP,
        FOLDER_CMP,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(
        &mut cmd,
        TARGET_FILE_JAVA,
        CORRECT_FILE_JAVA,
        GEN_FILE_JAVA,
        cases,
    );

    cmd.assert().failure().stderr(predicate::str::contains(
        "Error: QTEST_COMPILER_ERROR\nInfo: caused by failed to compile the correct file / label <correct-file>",
    ));

    Ok(())
}

#[test]
fn cmd_cmp_gen_ce_java() -> Result<(), Box<dyn Error>> {
    let ce = CE_JAVA.to_string().replace("Main", "Gen");
    create_files_cmp(
        TARGET_FILE_JAVA,
        CORRECT_FILE_JAVA,
        GEN_FILE_JAVA,
        TARGET_JAVA_CMP,
        CORRECT_JAVA_CMP,
        &ce[..],
        FOLDER_CMP,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(
        &mut cmd,
        TARGET_FILE_JAVA,
        CORRECT_FILE_JAVA,
        GEN_FILE_JAVA,
        cases,
    );

    cmd.assert().failure().stderr(predicate::str::contains(
        "Error: QTEST_COMPILER_ERROR\nInfo: caused by failed to compile the generator file / label <gen-file>",
    ));

    Ok(())
}
