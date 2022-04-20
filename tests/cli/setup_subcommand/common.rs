/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::{error::Error, process::Command};

use assert_cmd::assert::OutputAssertExt;
use predicates::prelude::predicate;

use crate::util::{test_command_handler::execute_command_setup, test_constants::BINARY};

#[test]
fn cmd_setup_cpp_program() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::new(BINARY);
    execute_command_setup(&mut cmd, "Language::Cpp.PROGRAM", "g++");

    cmd.assert().success().stdout(
        predicate::str::contains(
            " [INFO] Argument PROGRAM in Language::Cpp was updated to g++ successfully",
        )
        .count(1),
    );

    Ok(())
}

#[test]
fn cmd_setup_cpp_standard() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::new(BINARY);
    execute_command_setup(&mut cmd, "Language::Cpp.STANDARD", "-std=c++17");

    cmd.assert().success().stdout(
        predicate::str::contains(
            "  [INFO] Argument STANDARD in Language::Cpp was updated to -std=c++17 successfully",
        )
        .count(1),
    );

    Ok(())
}

#[test]
fn cmd_setup_python_program() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::new(BINARY);
    execute_command_setup(&mut cmd, "Language::Python.PROGRAM", "python");

    cmd.assert().success().stdout(
        predicate::str::contains(
            " [INFO] Argument PROGRAM in Language::Python was updated to python successfully",
        )
        .count(1),
    );

    Ok(())
}
