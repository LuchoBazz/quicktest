/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::{error::Error, process::Command};

use assert_cmd::assert::OutputAssertExt;
use predicates::prelude::predicate;

use crate::util::{
    test_command_handler::execute_command_run,
    test_constants::{BINARY, FOLDER_RUN, TARGET_FILE_CPP, TARGET_FILE_PY},
    test_utilities::create_files_run,
};

use super::codes::{TARGET_CPP_RUN, TARGET_PY_RUN};

#[test]
fn cmd_run_cpp() -> Result<(), Box<dyn Error>> {
    create_files_run(
        vec![
            ("testcase_ac_01.txt", "10\n1 -2 3 -4 5 -6 7 -8 9 -10"),
            ("testcase_ac_02.txt", "5\n10 20 -100 1 2"),
            ("testcase_ac_03.txt", "5\n-100 100 -100 100 -100"),
            (TARGET_FILE_CPP, TARGET_CPP_RUN),
        ],
        FOLDER_RUN,
    )?;

    let mut cmd = Command::new(BINARY);
    execute_command_run(&mut cmd, TARGET_FILE_CPP, "testcase_ac");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK] Ran Successfully").count(3));

    Ok(())
}

#[test]
fn cmd_run_python() -> Result<(), Box<dyn Error>> {
    create_files_run(
        vec![
            ("testcase_ac_01.txt", "10\n1 -2 3 -4 5 -6 7 -8 9 -10"),
            ("testcase_ac_02.txt", "5\n10 20 -100 1 2"),
            ("testcase_ac_03.txt", "5\n-100 100 -100 100 -100"),
            (TARGET_FILE_PY, TARGET_PY_RUN),
        ],
        FOLDER_RUN,
    )?;

    let mut cmd = Command::new(BINARY);
    execute_command_run(&mut cmd, TARGET_FILE_PY, "testcase_ac");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[OK] Ran Successfully").count(3));

    Ok(())
}
