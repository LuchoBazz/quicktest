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
    test_constants::{BINARY, CORRECT_FILE_PY, FOLDER_CMP, GEN_FILE_PY, TARGET_FILE_PY},
    test_utilities::create_files_cmp,
};

use super::codes::{CORRECT_PY_CMP, GEN_PY_CMP, TARGET_PY_CMP};

#[test]
fn cmp_target_py_correct_py_gen_py() -> Result<(), Box<dyn Error>> {
    create_files_cmp(
        TARGET_FILE_PY,
        CORRECT_FILE_PY,
        GEN_FILE_PY,
        TARGET_PY_CMP,
        CORRECT_PY_CMP,
        GEN_PY_CMP,
        FOLDER_CMP,
    )?;
    let cases: usize = 10;

    let mut cmd = Command::new(BINARY);
    execute_command_cmp(
        &mut cmd,
        TARGET_FILE_PY,
        CORRECT_FILE_PY,
        GEN_FILE_PY,
        cases,
    );

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK]").count(cases));

    Ok(())
}
