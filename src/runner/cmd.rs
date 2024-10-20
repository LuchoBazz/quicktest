/*
*  Quick Test: CLI for stress testing in competitive programming
*  Copyright (C) 2021 - Luis Miguel Báez
*  License: MIT (See the LICENSE file in the repository root directory)
*/

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use process_control::{ChildExt, Control};
use rand::distributions::{Distribution, Uniform};

use super::types::{CPStatus, StatusResponse};

#[allow(unused_variables)]
pub fn execute_program(
    timeout: u32,
    memory_limit: u64,
    testcase: u32,
    commands: Vec<&str>,
    stdin: Option<PathBuf>,
    stdout: Option<PathBuf>,
    stderr: Option<PathBuf>,
) -> StatusResponse {
    assert!(!commands.is_empty());

    let now: Instant = Instant::now();

    let mut cmd = Command::new(commands[0]);

    #[cfg(any(target_os = "linux", target_os = "windows"))]
    let mut memory_factor_needed = 1usize;

    #[cfg(any(target_os = "linux", target_os = "windows"))]
    if commands[0] == "java" {
        memory_factor_needed *= 10usize;
    }

    if commands.len() > 1 {
        cmd.args(&commands[1..]);
    }

    if let Some(file) = &stdin {
        // set output file, only exists
        let input = File::open(file.to_str().unwrap()).unwrap();
        cmd.stdin(Stdio::from(input));
    } else {
        // Only for generator file

        // Initialize random generator
        let mut rng = rand::thread_rng();
        let die = Uniform::from(0..(i32::MAX / 2));

        // add seed and test case
        cmd.args(&[die.sample(&mut rng).to_string(), testcase.to_string()]);
    }

    let child: Result<std::process::Child, std::io::Error> =
        cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn();

    if child.is_err() {
        return execute_program_response(CPStatus::CE, now);
    }

    let child_output = child.unwrap();

    #[cfg(any(target_os = "linux", target_os = "windows"))]
    let response = child_output
        .controlled_with_output()
        .memory_limit(memory_limit as usize * memory_factor_needed) // bytes
        .time_limit(Duration::from_millis(timeout as u64))
        .terminate_for_timeout()
        .wait();

    #[cfg(target_os = "macos")]
    let response = child_output
        .controlled_with_output()
        .time_limit(Duration::from_millis(timeout as u64))
        .terminate_for_timeout()
        .wait();

    if response.is_err() {
        return execute_program_response(CPStatus::TLE, now);
    }

    let output_option = response.unwrap();

    if output_option.is_none() {
        return execute_program_response(CPStatus::TLE, now);
    }

    let output = output_option.unwrap();
    let mut res_status = CPStatus::AC;

    if output.status.success() {
        // OK
        if let Some(file) = stdout {
            let mut writer = File::create(file.to_str().unwrap()).unwrap();
            writer.write_all(&output.stdout).unwrap();
        }

        if let Some(file) = stderr {
            let mut writer = File::create(file.to_str().unwrap()).unwrap();
            writer.write_all(&output.stderr).unwrap();
        }
    } else {
        println!("{:?}", output.status.code());
        println!("{:#?}", output.status.code());

        #[cfg(unix)]
        if let Some(6) = output.status.signal() {
            res_status = CPStatus::MLE;
        } else {
            res_status = CPStatus::RTE;
        }

        #[cfg(windows)]
        match output.status.code() {
            Some(3221226505) | Some(3) => res_status = CPStatus::MLE, // STATUS_HEAP_CORRUPTION: 3221226505, ERROR_PATH_NOT_FOUND: 3
            Some(_) => res_status = CPStatus::RTE,
            None => res_status = CPStatus::RTE,
        }
    }

    execute_program_response(res_status, now)
}

pub fn execute_program_response(res_status: CPStatus, now: Instant) -> StatusResponse {
    let new_now: Instant = Instant::now();
    let time: Duration = new_now.duration_since(now);
    StatusResponse::new(time, res_status)
}

pub fn has_installed_controller(program: &str, args: Vec<&str>) -> bool {
    let mut cmd = Command::new(program);
    cmd.args(args);

    let child: Result<std::process::Child, std::io::Error> =
        cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn();

    if let Ok(child_output) = child {
        let status = child_output
            .controlled_with_output()
            .time_limit(Duration::from_millis(1000_u64))
            .terminate_for_timeout()
            .wait();
        return status.is_ok();
    }
    false
}
