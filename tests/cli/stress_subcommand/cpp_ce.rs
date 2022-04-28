/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021-present / Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::{error::Error, process::Command};

use assert_cmd::assert::OutputAssertExt;
use predicates::prelude::predicate;

use crate::util::{
    test_command_handler::execute_command_stress,
    test_constants::{BINARY, CE_CPP, FOLDER_STRESS, GEN_FILE_CPP, TARGET_FILE_CPP},
    test_utilities::create_files_tle,
};

use super::codes::{GEN_CPP_STRESS, TARGET_CPP_STRESS};

#[test]
fn cmd_tle_target_ce_cpp() -> Result<(), Box<dyn Error>> {
    create_files_tle(
        TARGET_FILE_CPP,
        GEN_FILE_CPP,
        CE_CPP,
        GEN_CPP_STRESS,
        FOLDER_STRESS,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_stress(&mut cmd, TARGET_FILE_CPP, GEN_FILE_CPP, cases);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: QTEST_COMPILER_ERROR\nInfo: caused by failed to compile the target file / label <target-file>").count(1));

    Ok(())
}

#[test]
fn cmd_tle_gen_ce_cpp() -> Result<(), Box<dyn Error>> {
    create_files_tle(
        TARGET_FILE_CPP,
        GEN_FILE_CPP,
        TARGET_CPP_STRESS,
        CE_CPP,
        FOLDER_STRESS,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_stress(&mut cmd, TARGET_FILE_CPP, GEN_FILE_CPP, cases);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: QTEST_COMPILER_ERROR\nInfo: caused by failed to compile the generator file / label <gen-file>").count(1));

    Ok(())
}
