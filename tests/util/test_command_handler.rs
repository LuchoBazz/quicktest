/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::process::Command;

use crate::util::test_constants::{FOLDER, FOLDER_CHECK, FOLDER_CMP, FOLDER_TLE};

pub fn execute_command_tle(cmd: &mut Command, target_file: &str, gen_file: &str, cases: usize) {
    cmd.arg("tle")
        .arg("--target-file")
        .arg(format!("{}/{}/{}", FOLDER, FOLDER_TLE, target_file))
        .arg("--gen-file")
        .arg(format!("{}/{}/{}", FOLDER, FOLDER_TLE, gen_file))
        .arg("--timeout=1000")
        .arg(format!("--test-cases={}", cases));
}

pub fn execute_command_cmp(
    cmd: &mut Command,
    target_file: &str,
    correct_file: &str,
    gen_file: &str,
    cases: usize,
) {
    cmd.arg("cmp")
        .arg("--target-file")
        .arg(format!("{}/{}/{}", FOLDER, FOLDER_CMP, target_file))
        .arg("--correct-file")
        .arg(format!("{}/{}/{}", FOLDER, FOLDER_CMP, correct_file))
        .arg("--gen-file")
        .arg(format!("{}/{}/{}", FOLDER, FOLDER_CMP, gen_file))
        .arg("--timeout=1000")
        .arg(format!("--test-cases={}", cases));
}

pub fn execute_command_check(
    cmd: &mut Command,
    target_file: &str,
    checker_file: &str,
    gen_file: &str,
    cases: usize,
) {
    cmd.arg("check")
        .arg("--target-file")
        .arg(format!("{}/{}/{}", FOLDER, FOLDER_CHECK, target_file))
        .arg("--checker-file")
        .arg(format!("{}/{}/{}", FOLDER, FOLDER_CHECK, checker_file))
        .arg("--gen-file")
        .arg(format!("{}/{}/{}", FOLDER, FOLDER_CHECK, gen_file))
        .arg("--timeout=1000")
        .arg(format!("--test-cases={}", cases));
}
