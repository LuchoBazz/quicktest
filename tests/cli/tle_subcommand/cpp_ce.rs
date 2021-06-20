/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::{error::Error, process::Command};

use assert_cmd::assert::OutputAssertExt;
use predicates::prelude::predicate;

use crate::util::{test_command_handler::execute_command_tle, test_constants::{BINARY, CE_CPP, FOLDER_TLE, GEN_FILE_CPP, TARGET_FILE_CPP}, test_utilities::create_files_tle};

use super::codes::{GEN_CPP_TLE, TARGET_CPP_TLE};

#[test]
fn cmd_tle_target_ce_cpp() -> Result<(), Box<dyn Error>> {
    create_files_tle(TARGET_FILE_CPP, GEN_FILE_CPP, CE_CPP, GEN_CPP_TLE, FOLDER_TLE)?;
    let cases: usize = 10;
    
    let mut cmd = Command::new(BINARY);
    execute_command_tle(&mut cmd, TARGET_FILE_CPP, GEN_FILE_CPP, cases);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: compilation of <target-file> failed").count(1));
    
    Ok(())
}

#[test]
fn cmd_tle_gen_ce_cpp() -> Result<(), Box<dyn Error>> {
    create_files_tle(TARGET_FILE_CPP, GEN_FILE_CPP, TARGET_CPP_TLE, CE_CPP, FOLDER_TLE)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_tle(&mut cmd, TARGET_FILE_CPP, GEN_FILE_CPP, cases);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: compilation of <gen-file> failed").count(1));
    
    Ok(())
}
