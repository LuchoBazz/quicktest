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
        BINARY, CORRECT_FILE_JAVA, FOLDER_CMP, GEN_FILE_JAVA, RTE_JAVA, TARGET_FILE_JAVA,
    },
    test_utilities::create_files_cmp,
};

use super::codes::{CORRECT_JAVA_CMP, GEN_JAVA_CMP, TARGET_JAVA_CMP};

// CHECK RTE in Subcommand cmp
#[test]
fn cmd_cmp_target_rte_java() -> Result<(), Box<dyn Error>> {
    create_files_cmp(
        TARGET_FILE_JAVA,
        CORRECT_FILE_JAVA,
        GEN_FILE_JAVA,
        RTE_JAVA,
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

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("[RTE]").count(cases));

    Ok(())
}

#[test]
fn cmd_cmp_correct_rte_java() -> Result<(), Box<dyn Error>> {
    let rte = RTE_JAVA.to_string().replace("Main", "Correct");

    create_files_cmp(
        TARGET_FILE_JAVA,
        CORRECT_FILE_JAVA,
        GEN_FILE_JAVA,
        TARGET_JAVA_CMP,
        &rte[..],
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
        "Error: QTEST_RUNTIME_ERROR\nInfo: caused by correct file exited by Runtime Error / label <correct-file>\n",
    ));

    Ok(())
}

#[test]
fn cmd_cmp_gen_rte_java() -> Result<(), Box<dyn Error>> {
    let rte = RTE_JAVA.to_string().replace("Main", "Gen");

    create_files_cmp(
        TARGET_FILE_JAVA,
        CORRECT_FILE_JAVA,
        GEN_FILE_JAVA,
        TARGET_JAVA_CMP,
        CORRECT_JAVA_CMP,
        &rte[..],
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
        "Error: QTEST_RUNTIME_ERROR\nInfo: caused by generator file exited by Runtime Error / label <gen-file>\n",
    ));

    Ok(())
}
