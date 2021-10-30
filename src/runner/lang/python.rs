/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::path::PathBuf;

use crate::runner::cmd::execute_program;
use crate::runner::types::{Language, StatusResponse};

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
    pub fn new(
        program: &'static str,
        file_name: PathBuf,
        flags: Vec<&'static str>,
        variables: Vec<&'static str>,
        stdin: Option<PathBuf>,
        stdout: Option<PathBuf>,
        stderr: Option<PathBuf>,
    ) -> Python {
        Python {
            program,
            file_name,
            flags,
            variables,
            stdin,
            stdout,
            stderr,
        }
    }
}

impl Language for Python {
    fn build(&self) -> bool {
        // no need to build
        true
    }

    fn execute(&self, timeout: u32, testcase: u32) -> StatusResponse {
        // Example: python3 main.py
        let commands = vec![self.program, self.file_name.to_str().unwrap()];
        execute_program(
            timeout,
            testcase,
            commands,
            self.stdin.clone(),
            self.stdout.clone(),
            self.stderr.clone(),
        )
    }

    fn set_stdio(&mut self, stdin: &str) {
        self.stdin = Some(PathBuf::from(stdin));
    }
}

pub mod default {
    use std::path::PathBuf;

    use super::Python;

    pub fn python3_default(
        root: &str,
        file_name: &str,
        input_file: &str,
        output_file: &str,
        error_file: &str,
    ) -> Python {
        let stdin = PathBuf::from(format!("{}/{}", root, input_file));
        let stdout = PathBuf::from(format!("{}/{}", root, output_file));
        let stderr = PathBuf::from(format!("{}/{}", root, error_file));

        Python::new(
            "python3",
            PathBuf::from(format!("{}/{}", root, file_name)),
            vec![],
            vec!["ONLINE_JUDGE=1"],
            Some(stdin),
            Some(stdout),
            Some(stderr),
        )
    }

    pub fn python3_set_output(root: &str, file_name: &str, output_file: &str) -> Python {
        let stdout = PathBuf::from(format!("{}/{}", root, output_file));

        Python::new(
            "python3",
            PathBuf::from(format!("{}/{}", root, file_name)),
            vec![],
            vec!["ONLINE_JUDGE=1"],
            None,
            Some(stdout),
            None,
        )
    }
}
