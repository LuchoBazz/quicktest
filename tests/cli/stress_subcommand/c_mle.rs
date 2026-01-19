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
    test_command_handler::execute_command_stress_with_timeout,
    test_constants::{
        BINARY, FOLDER_STRESS, GEN_FILE_C, MLE_C, TARGET_FILE_C,
    },
    test_utilities::create_files_tle,
};

use super::codes::{GEN_C_STRESS, TARGET_C_STRESS};

#[test]
fn cmd_stress_target_mle_c() -> Result<(), Box<dyn Error>> {
    let folder = "stress_mle_c";
    create_files_tle(
        TARGET_FILE_C,
        GEN_FILE_C,
        MLE_C,
        GEN_C_STRESS,
        folder,
    )?;
    let cases: usize = 3;

    let mut cmd = Command::new(BINARY);
    execute_command_stress_with_timeout(&mut cmd, TARGET_FILE_C, GEN_FILE_C, cases, 5000usize, folder);

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("[MLE]").count(cases));

    Ok(())
}

#[test]
fn cmd_stress_gen_mle_c() -> Result<(), Box<dyn Error>> {
    let folder = "stress_mle_c_gen";
    create_files_tle(
        TARGET_FILE_C,
        GEN_FILE_C,
        TARGET_C_STRESS,
        MLE_C,
        folder,
    )?;
    let cases: usize = 3;

    let mut cmd = Command::new(BINARY);
    execute_command_stress_with_timeout(&mut cmd, TARGET_FILE_C, GEN_FILE_C, cases, 5000usize, folder);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(
            "Error: QTEST_MEMORY_LIMIT_EXCEEDED\nInfo: caused by generator file give memory limit exceeded / label <gen-file>\n").count(1),
        );

    Ok(())
}
