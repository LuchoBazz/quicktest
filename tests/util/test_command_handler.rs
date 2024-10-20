/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021-present / Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::process::Command;

use crate::util::test_constants::{FOLDER, FOLDER_CHECK, FOLDER_CMP, FOLDER_STRESS};

use super::test_constants::FOLDER_OUTPUT;

pub fn execute_command_stress(cmd: &mut Command, target_file: &str, gen_file: &str, cases: usize) {
    execute_command_stress_with_timeout(cmd, target_file, gen_file, cases, 1000usize);
}

pub fn execute_command_stress_with_timeout(
    cmd: &mut Command,
    target_file: &str,
    gen_file: &str,
    cases: usize,
    timeout: usize,
) {
    cmd.arg("stress")
        .arg("--target-file")
        .arg(format!("{}/{}/{}", FOLDER, FOLDER_STRESS, target_file))
        .arg("--gen-file")
        .arg(format!("{}/{}/{}", FOLDER, FOLDER_STRESS, gen_file))
        .arg(format!("--timeout={}", timeout))
        .arg(format!("--test-cases={}", cases));
}

pub fn execute_command_cmp(
    cmd: &mut Command,
    target_file: &str,
    correct_file: &str,
    gen_file: &str,
    cases: usize,
) {
    execute_command_cmp_with_timeout(cmd, target_file, correct_file, gen_file, cases, 2000usize);
}

pub fn execute_command_cmp_with_timeout(
    cmd: &mut Command,
    target_file: &str,
    correct_file: &str,
    gen_file: &str,
    cases: usize,
    timeout: usize,
) {
    cmd.arg("cmp")
        .arg("--target-file")
        .arg(format!("{}/{}/{}", FOLDER, FOLDER_CMP, target_file))
        .arg("--correct-file")
        .arg(format!("{}/{}/{}", FOLDER, FOLDER_CMP, correct_file))
        .arg("--gen-file")
        .arg(format!("{}/{}/{}", FOLDER, FOLDER_CMP, gen_file))
        .arg(format!("--timeout={}", timeout))
        .arg(format!("--test-cases={}", cases));
}

pub fn execute_command_check(
    cmd: &mut Command,
    target_file: &str,
    checker_file: &str,
    gen_file: &str,
    cases: usize,
) {
    execute_command_check_with_timeout(cmd, target_file, checker_file, gen_file, cases, 1000usize);
}

pub fn execute_command_check_with_timeout(
    cmd: &mut Command,
    target_file: &str,
    checker_file: &str,
    gen_file: &str,
    cases: usize,
    timeout: usize,
) {
    cmd.arg("check")
        .arg("--target-file")
        .arg(format!("{}/{}/{}", FOLDER, FOLDER_CHECK, target_file))
        .arg("--checker-file")
        .arg(format!("{}/{}/{}", FOLDER, FOLDER_CHECK, checker_file))
        .arg("--gen-file")
        .arg(format!("{}/{}/{}", FOLDER, FOLDER_CHECK, gen_file))
        .arg(format!("--timeout={}", timeout))
        .arg(format!("--test-cases={}", cases));
}

pub fn execute_command_output(cmd: &mut Command, target_file: &str, prefix: &str) {
    cmd.arg("output")
        .arg("--target-file")
        .arg(format!("{}/{}/{}", FOLDER, FOLDER_OUTPUT, target_file))
        .arg(format!("--prefix={}/{}/{}", FOLDER, FOLDER_OUTPUT, prefix))
        .arg("--timeout=1000");
}

pub fn execute_command_setup(cmd: &mut Command, label: &str, value: &str) {
    cmd.arg("setup")
        .arg("config")
        .arg(format!("--label={}", label))
        .arg(format!("--value={}", value));
}
