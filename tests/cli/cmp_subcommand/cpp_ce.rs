/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::{error::Error, process::Command};

use assert_cmd::assert::OutputAssertExt;
use predicates::prelude::predicate;

use crate::util::{test_command_handler::execute_command_cmp, test_constants::{BINARY, CE_CPP, CORRECT_FILE_CPP, FOLDER_CMP, GEN_FILE_CPP, TARGET_FILE_CPP}, test_utilities::create_files_cmp};

use super::codes::{CORRECT_CPP_CMP, GEN_CPP_CMP, TARGET_CPP_CMP};


#[test]
fn cmd_cmp_target_ce_cpp() -> Result<(), Box<dyn Error>> {
    create_files_cmp(TARGET_FILE_CPP, CORRECT_FILE_CPP, GEN_FILE_CPP, 
        CE_CPP, CORRECT_CPP_CMP, GEN_CPP_CMP, FOLDER_CMP)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(&mut cmd, TARGET_FILE_CPP, CORRECT_FILE_CPP, GEN_FILE_CPP, cases);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: compilation of <target-file> failed"));
    
    Ok(())
}

#[test]
fn cmd_cmp_correct_ce_cpp() -> Result<(), Box<dyn Error>> {
    create_files_cmp(TARGET_FILE_CPP, CORRECT_FILE_CPP, GEN_FILE_CPP, 
        TARGET_CPP_CMP, CE_CPP, GEN_CPP_CMP, FOLDER_CMP)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(&mut cmd, TARGET_FILE_CPP, CORRECT_FILE_CPP, GEN_FILE_CPP, cases);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: compilation of <correct-file> failed"));
    
    Ok(())
}

#[test]
fn cmd_cmp_gen_ce_cpp() -> Result<(), Box<dyn Error>> {
    create_files_cmp(TARGET_FILE_CPP, CORRECT_FILE_CPP, GEN_FILE_CPP, 
        TARGET_CPP_CMP, GEN_CPP_CMP, CE_CPP, FOLDER_CMP)?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(&mut cmd, TARGET_FILE_CPP, CORRECT_FILE_CPP, GEN_FILE_CPP, cases);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error: compilation of <gen-file> failed"));
    
    Ok(())
}
