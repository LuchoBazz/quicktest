/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::{error::Error, process::Command};

use assert_cmd::assert::OutputAssertExt;
use predicates::prelude::predicate;

use crate::util::{
    test_command_handler::{execute_command_setup_cpp, execute_command_setup_python},
    test_constants::BINARY,
};

#[test]
fn cmd_setup_cpp_program() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::new(BINARY);
    execute_command_setup_cpp(&mut cmd, "g++", "", "");

    cmd.assert().success().stdout(
        predicate::str::contains(" [INFO] Argument program in C++ was updated to g++ successfully")
            .count(1),
    );

    Ok(())
}

#[test]
fn cmd_setup_cpp_standard() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::new(BINARY);
    execute_command_setup_cpp(&mut cmd, "", "-std=c++17", "");

    cmd.assert().success().stdout(
        predicate::str::contains(
            " [INFO] Argument standard in C++ was updated to -std=c++17 successfully",
        )
        .count(1),
    );

    Ok(())
}

#[test]
fn cmd_setup_cpp_program_standard() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::new(BINARY);
    execute_command_setup_cpp(&mut cmd, "g++", "-std=c++17", "");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("  [INFO] Argument program in C++ was updated to g++ successfully\n  [INFO] Argument standard in C++ was updated to -std=c++17 successfully\n").count(1));

    Ok(())
}

#[test]
fn cmd_setup_cpp_flags() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::new(BINARY);
    execute_command_setup_cpp(&mut cmd, "", "", "-Wall;-DONLINE_JUDGE=1");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("  [INFO] Argument flags in C++ was updated to [\"-Wall\", \"-DONLINE_JUDGE=1\"] successfully\n").count(1));

    Ok(())
}

#[test]
fn cmd_setup_python_program() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::new(BINARY);
    execute_command_setup_python(&mut cmd, "python", "");

    cmd.assert().success().stdout(
        predicate::str::contains(
            " [INFO] Argument program in Python was updated to python successfully",
        )
        .count(1),
    );

    Ok(())
}

#[test]
fn cmd_setup_python_flags() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::new(BINARY);
    execute_command_setup_python(&mut cmd, "", "ONLINE_JUDGE=1;ENV=1");

    cmd.assert().success().stdout(
        predicate::str::contains(
            " [INFO] Argument flags in Python was updated to [\"ONLINE_JUDGE=1\", \"ENV=1\"] successfully",
        )
        .count(1),
    );

    Ok(())
}
