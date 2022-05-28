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
        BINARY, CORRECT_FILE_KOTLIN, FOLDER_CMP, GEN_FILE_KOTLIN, TARGET_FILE_KOTLIN,
    },
    test_utilities::create_files_cmp,
};

use super::codes::{CORRECT_KOTLIN_CMP, GEN_KOTLIN_CMP, TARGET_KOTLIN_CMP};

#[test]
fn cmp_target_cpp_correct_cpp_gen_cpp() -> Result<(), Box<dyn Error>> {
    create_files_cmp(
        TARGET_FILE_KOTLIN,
        CORRECT_FILE_KOTLIN,
        GEN_FILE_KOTLIN,
        TARGET_KOTLIN_CMP,
        CORRECT_KOTLIN_CMP,
        GEN_KOTLIN_CMP,
        FOLDER_CMP,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(
        &mut cmd,
        TARGET_FILE_KOTLIN,
        CORRECT_FILE_KOTLIN,
        GEN_FILE_KOTLIN,
        cases,
    );

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));

    Ok(())
}
