/*
*  Quick Test: CLI for stress testing in competitive programming
*  Copyright (C) 2021 - Luis Miguel Báez
*  License: MIT (See the LICENSE file in the repository root directory)
*/

use std::{fs::File, io::Write, path::PathBuf, process::{Command, Stdio}, time::{Duration, Instant}};

use process_control::{ChildExt, Timeout};

use super::types::{CPStatus, StatusResponse};

pub fn execute_program(timeout: u32, commands: Vec<&str>,
        stdin: Option<PathBuf>, stdout: Option<PathBuf>, 
        stderr: Option<PathBuf>) -> StatusResponse {
    
    let now: Instant = Instant::now();

    let child: Result<std::process::Child, std::io::Error> = match &stdin {
        Some(file) => {
            let input = File::open(file.to_str().unwrap()).unwrap();
            let mut cmd = Command::new(commands[0]);
            if commands.len() > 1 {
                cmd.args(&commands[1..]);
            }
            let child = cmd.stdin(Stdio::from(input))
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn();
            child
        },
        _ => {
            let mut cmd = Command::new(commands[0]);
            if commands.len() > 1 {
                cmd.args(&commands[1..]);
            }
            let child = cmd.stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn();
            child
        }
    };

    let mut res_status = CPStatus::AC;

    if let Ok(child_output) = child {
        let response = child_output
            .with_output_timeout(Duration::from_millis(timeout as u64))
            .terminating()
            .wait();
        
        if let Ok(output_option)= response {
            if let Some(output) = output_option {

                if output.status.success() {
                    // OK
                    match stdout {
                        Some(file) => {
                            let mut writer = File::create(file.to_str().unwrap()).unwrap();
                            writer.write_all(&output.stdout).unwrap();
                        },
                        _ => (),
                    }

                    match stderr {
                        Some(file) => {
                            let mut writer = File::create(file.to_str().unwrap()).unwrap();
                            writer.write_all(&output.stderr).unwrap();
                        },
                        _ => (),
                    }
                    res_status = CPStatus::AC;
                } else {
                    res_status = CPStatus::RTE;
                }
            } else {
                res_status = CPStatus::TLE;
            }
        }
    } else {
        res_status = CPStatus::CE;
    }

    let new_now: Instant = Instant::now();
    let time: Duration = new_now.duration_since(now);

    StatusResponse::new(time, res_status)
}