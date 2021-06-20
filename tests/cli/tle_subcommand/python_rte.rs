/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::{error::Error, process::Command};

use assert_cmd::assert::OutputAssertExt;
use predicates::prelude::predicate;

use crate::util::{test_command_handler::execute_command_tle, test_constants::{BINARY, FOLDER_TLE, GEN_FILE_PY, RTE_PY, TARGET_FILE_PY}, test_utilities::create_files_tle};

use super::codes::{GEN_PY_TLE, TARGET_PY_TLE};

#[test]
fn cmd_tle_target_rte_py() -> Result<(), Box<dyn Error>> {
    create_files_tle(TARGET_FILE_PY, GEN_FILE_PY, RTE_PY, GEN_PY_TLE, FOLDER_TLE)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_tle(&mut cmd, TARGET_FILE_PY, GEN_FILE_PY, cases);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[RTE]").count(cases));
    
    Ok(())
}

#[test]
fn cmd_tle_gen_rte_py() -> Result<(), Box<dyn Error>> {
    create_files_tle(TARGET_FILE_PY, GEN_FILE_PY, TARGET_PY_TLE, RTE_PY, FOLDER_TLE)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_tle(&mut cmd, TARGET_FILE_PY, GEN_FILE_PY, cases);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: Runtime Error of <gen-file>"));
    
    Ok(())
}