/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::runner::cmd::{execute_program, has_installed_controller};
use crate::runner::types::{Language, StatusResponse};

#[derive(Debug, Clone)]
pub struct Python {
    /// Example: python, python3, pypy2 or pypy3
    pub program: String,

    /// Example: main.py
    file_name: PathBuf,

    /// Example: ONLINE_JUDGE=1, etc
    #[allow(unused)]
    flags: Vec<String>,

    stdin: Option<PathBuf>,

    stdout: Option<PathBuf>,

    stderr: Option<PathBuf>,
}

impl Python {
    pub fn new(
        program: String,
        file_name: PathBuf,
        flags: Vec<String>,
        stdin: Option<PathBuf>,
        stdout: Option<PathBuf>,
        stderr: Option<PathBuf>,
    ) -> Python {
        Python {
            program,
            file_name,
            flags,
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
        let commands = vec![&self.program[..], self.file_name.to_str().unwrap()];
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

    fn is_installed(&self) -> bool {
        has_installed_controller(&self.program, vec!["--version"])
    }

    fn get_name(&self) -> String {
        "Python Language".to_string()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PythonConfig {
    pub program: String,
    pub flags: Vec<String>,
}

impl Default for PythonConfig {
    fn default() -> Self {
        PythonConfig {
            program: "python3".to_string(),
            flags: vec!["ONLINE_JUDGE=1".to_string()],
        }
    }
}

pub mod default {
    use std::path::PathBuf;

    use crate::config::scheme::load_default_config;

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

        let default_arg = load_default_config();
        let python = default_arg.python_config;

        Python::new(
            python.program,
            PathBuf::from(format!("{}/{}", root, file_name)),
            python.flags,
            Some(stdin),
            Some(stdout),
            Some(stderr),
        )
    }

    pub fn python3_set_output(root: &str, file_name: &str, output_file: &str) -> Python {
        let stdout = PathBuf::from(format!("{}/{}", root, output_file));

        let default_arg = load_default_config();
        let python = default_arg.python_config;

        Python::new(
            python.program,
            PathBuf::from(format!("{}/{}", root, file_name)),
            python.flags,
            None,
            Some(stdout),
            None,
        )
    }
}
