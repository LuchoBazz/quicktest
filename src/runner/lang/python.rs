/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::io::Write;
use std::process::Command;
use std::process::Stdio;
use std::path::PathBuf;
use std::fs::File;
use std::time::Duration;
use std::time::Instant;

use crate::runner::types::{Language, CPStatus, StatusResponse};

use process_control::ChildExt;
use process_control::Timeout;

#[derive(Debug, Clone)]
pub struct Python {

    /// Example: python, python3, pypy2 or pypy3
    pub program: &'static str,

    /// Example: main.py
    file_name: PathBuf,

    /// Example: -Wall
    flags: Vec<&'static str>,

    /// Example ONLINE_JUDGE=1, etc
    #[allow(unused)]
    variables: Vec<&'static str>,
    
    stdin: Option<PathBuf>,

    stdout: Option<PathBuf>,

    stderr: Option<PathBuf>,
}

impl Python {
    pub fn new(program: &'static str, file_name: PathBuf, flags: Vec<&'static str>,
        variables: Vec<&'static str>, stdin: Option<PathBuf>, stdout: Option<PathBuf>,
        stderr: Option<PathBuf>) -> Python {
        
        Python {
            program: program,
            file_name: file_name,
            flags: flags,
            variables: variables,
            stdin: stdin,
            stdout: stdout,
            stderr: stderr
        }
    }
}

impl Language for Python {

    fn build(&self) -> bool {
        // no need to build
        true
    }

    fn execute(&self, timeout: u32) -> StatusResponse {
        let now: Instant = Instant::now();

        let child: Result<std::process::Child, std::io::Error> = match &self.stdin {
            Some(file) => {
                let input = File::open(file.to_str().unwrap()).unwrap();

                let child = Command::new(self.program)
                    .arg(&self.file_name)
                    .stdin(Stdio::from(input))
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn();
                child
            },
            _ => {
                let child = Command::new(self.program)
                    .arg(&self.file_name)
                    .stdout(Stdio::piped())
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
                        match &self.stdout {
                            Some(file) => {
                                let mut writer = File::create(file.to_str().unwrap()).unwrap();
                                writer.write_all(&output.stdout).unwrap();
                            },
                            _ => (),
                        }

                        match &self.stderr {
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

    fn set_stdio(&mut self, stdin: &str) {
        self.stdin = Some(PathBuf::from(stdin));
    }
}

pub mod default {
    use std::path::PathBuf;

    use super::Python;
    
    pub fn python3_default(root: &str, file_name: &str,
            input_file: &str, output_file: &str,
            error_file: &str) -> Python {
    
        let stdin = PathBuf::from(format!("{}/{}", root, input_file));
        let stdout =  PathBuf::from(format!("{}/{}", root, output_file));
        let stderr = PathBuf::from(format!("{}/{}", root, error_file));

        Python::new(
            "python3",
            PathBuf::from(format!("{}/{}", root, file_name)),
            vec![],
            vec!["ONLINE_JUDGE=1"],
            Some(stdin),
            Some(stdout),
            Some(stderr)
        )
    }

    pub fn python3_set_output(root: &str, file_name: &str,
            output_file: &str) -> Python {

        let stdout =  PathBuf::from(format!("{}/{}", root, output_file));

        Python::new(
            "python3",
            PathBuf::from(format!("{}/{}", root, file_name)),
            vec![],
            vec!["ONLINE_JUDGE=1"],
            None,
            Some(stdout),
            None
        )
    }
}