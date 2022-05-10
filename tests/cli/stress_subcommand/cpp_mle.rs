/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021-present / Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::{error::Error, process::Command};

use assert_cmd::assert::OutputAssertExt;
use predicates::prelude::predicate;

use crate::util::{
    test_command_handler::{execute_command_stress_with_timeout},
    test_constants::{BINARY, FOLDER_STRESS, GEN_FILE_CPP, MLE_CPP, TARGET_FILE_CPP},
    test_utilities::create_files_tle,
};

use super::codes::{GEN_CPP_STRESS, TARGET_CPP_STRESS};

#[test]
fn cmd_tle_target_mle_cpp() -> Result<(), Box<dyn Error>> {
    create_files_tle(
        TARGET_FILE_CPP,
        GEN_FILE_CPP,
        MLE_CPP,
        GEN_CPP_STRESS,
        FOLDER_STRESS,
    )?;
    let cases: usize = 3;

    let mut cmd = Command::new(BINARY);
    execute_command_stress_with_timeout(&mut cmd, TARGET_FILE_CPP, GEN_FILE_CPP, cases, 5000usize);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[MLE]").count(cases));

    Ok(())
}

#[test]
fn cmd_tle_gen_mle_cpp() -> Result<(), Box<dyn Error>> {
    create_files_tle(
        TARGET_FILE_CPP,
        GEN_FILE_CPP,
        TARGET_CPP_STRESS,
        MLE_CPP,
        FOLDER_STRESS,
    )?;
    let cases: usize = 3;

    let mut cmd = Command::new(BINARY);
    execute_command_stress_with_timeout(&mut cmd, TARGET_FILE_CPP, GEN_FILE_CPP, cases, 5000usize);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(
            "Error: QTEST_MEMORY_LIMIT_EXCEEDED\nInfo: caused by generator file give memory limit exceeded / label <gen-file>\n").count(1),
        );

    Ok(())
}
