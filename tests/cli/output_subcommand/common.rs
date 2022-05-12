/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021-present / Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::{error::Error, process::Command};

use assert_cmd::assert::OutputAssertExt;
use predicates::prelude::predicate;

use crate::util::{
    test_command_handler::execute_command_output,
    test_constants::{BINARY, FOLDER_OUTPUT, TARGET_FILE_CPP, TARGET_FILE_PY},
    test_utilities::create_files_output_cmd,
};

use super::codes::{TARGET_CPP_OUTPUT_CMD, TARGET_PY_OUTPUT_CMD};

#[test]
fn cmd_output_cpp() -> Result<(), Box<dyn Error>> {
    create_files_output_cmd(
        vec![
            ("testcase_ac_01.txt", "10\n1 -2 3 -4 5 -6 7 -8 9 -10"),
            ("testcase_ac_02.txt", "5\n10 20 -100 1 2"),
            ("testcase_ac_03.txt", "5\n-100 100 -100 100 -100"),
            (TARGET_FILE_CPP, TARGET_CPP_OUTPUT_CMD),
        ],
        FOLDER_OUTPUT,
    )?;

    let mut cmd = Command::new(BINARY);
    execute_command_output(&mut cmd, TARGET_FILE_CPP, "testcase_ac");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK] Ran Successfully").count(3));

    Ok(())
}

#[test]
fn cmd_output_python() -> Result<(), Box<dyn Error>> {
    create_files_output_cmd(
        vec![
            ("testcase_ac_01.txt", "10\n1 -2 3 -4 5 -6 7 -8 9 -10"),
            ("testcase_ac_02.txt", "5\n10 20 -100 1 2"),
            ("testcase_ac_03.txt", "5\n-100 100 -100 100 -100"),
            (TARGET_FILE_PY, TARGET_PY_OUTPUT_CMD),
        ],
        FOLDER_OUTPUT,
    )?;

    let mut cmd = Command::new(BINARY);
    execute_command_output(&mut cmd, TARGET_FILE_PY, "testcase_ac");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK] Ran Successfully").count(3));

    Ok(())
}
