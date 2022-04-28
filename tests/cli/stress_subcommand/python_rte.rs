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
    test_constants::{BINARY, FOLDER_STRESS, GEN_FILE_PY, RTE_PY, TARGET_FILE_PY},
    test_utilities::create_files_tle,
};

use super::codes::{GEN_PY_STRESS, TARGET_PY_STRESS};

#[test]
fn cmd_tle_target_rte_py() -> Result<(), Box<dyn Error>> {
    create_files_tle(
        TARGET_FILE_PY,
        GEN_FILE_PY,
        RTE_PY,
        GEN_PY_STRESS,
        FOLDER_STRESS,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_stress(&mut cmd, TARGET_FILE_PY, GEN_FILE_PY, cases);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[RTE]").count(cases));

    Ok(())
}

#[test]
fn cmd_tle_gen_rte_py() -> Result<(), Box<dyn Error>> {
    create_files_tle(
        TARGET_FILE_PY,
        GEN_FILE_PY,
        TARGET_PY_STRESS,
        RTE_PY,
        FOLDER_STRESS,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_stress(&mut cmd, TARGET_FILE_PY, GEN_FILE_PY, cases);

    cmd.assert().failure().stderr(predicate::str::contains(
        "Error: QTEST_RUNTIME_ERROR\nInfo: caused by generator file exited by Runtime Error / label <gen-file>\n",
    ));

    Ok(())
}
