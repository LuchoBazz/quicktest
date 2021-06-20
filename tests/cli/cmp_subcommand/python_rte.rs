/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::{error::Error, process::Command};

use assert_cmd::assert::OutputAssertExt;
use predicates::prelude::predicate;

use crate::util::{test_command_handler::execute_command_cmp, test_constants::{BINARY, CORRECT_FILE_PY, FOLDER_CMP, GEN_FILE_PY, RTE_PY, TARGET_FILE_PY}, test_utilities::create_files_cmp};

use super::codes::{CORRECT_PY_CMP, GEN_PY_CMP, TARGET_PY_CMP};


#[test]
fn cmd_cmp_target_rte_py() -> Result<(), Box<dyn Error>> {
    create_files_cmp(TARGET_FILE_PY, CORRECT_FILE_PY, GEN_FILE_PY, 
        RTE_PY, CORRECT_PY_CMP, GEN_PY_CMP, FOLDER_CMP)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(&mut cmd, TARGET_FILE_PY, CORRECT_FILE_PY, GEN_FILE_PY, cases);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[RTE]").count(cases));
    
    Ok(())
}

#[test]
fn cmd_cmp_correct_rte_py() -> Result<(), Box<dyn Error>> {
    create_files_cmp(TARGET_FILE_PY, CORRECT_FILE_PY, GEN_FILE_PY, 
        TARGET_PY_CMP, RTE_PY, GEN_PY_CMP, FOLDER_CMP)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(&mut cmd, TARGET_FILE_PY, CORRECT_FILE_PY, GEN_FILE_PY, cases);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: Runtime Error of <correct-file>"));
    
    Ok(())
}

#[test]
fn cmd_cmp_gen_rte_py() -> Result<(), Box<dyn Error>> {
    create_files_cmp(TARGET_FILE_PY, CORRECT_FILE_PY, GEN_FILE_PY, 
        TARGET_PY_CMP, CORRECT_PY_CMP, RTE_PY, FOLDER_CMP)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(&mut cmd, TARGET_FILE_PY, CORRECT_FILE_PY, GEN_FILE_PY, cases);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: Runtime Error of <gen-file>"));
    
    Ok(())
}