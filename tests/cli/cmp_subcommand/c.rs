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
    test_constants::{BINARY, CORRECT_FILE_C, FOLDER_CMP, GEN_FILE_C, TARGET_FILE_C},
    test_utilities::create_files_cmp,
};

use super::codes::{CORRECT_C_CMP, GEN_C_CMP, TARGET_C_CMP};

#[test]
fn cmp_target_c_correct_c_gen_c() -> Result<(), Box<dyn Error>> {
    create_files_cmp(
        TARGET_FILE_C,
        CORRECT_FILE_C,
        GEN_FILE_C,
        TARGET_C_CMP,
        CORRECT_C_CMP,
        GEN_C_CMP,
        FOLDER_CMP,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(&mut cmd, TARGET_FILE_C, CORRECT_FILE_C, GEN_FILE_C, cases);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));

    Ok(())
}
