/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::path::PathBuf;
use std::process::Command;

use crate::runner::cmd::{execute_program, has_installed_controller};
use crate::runner::types::{Language, StatusResponse};

#[derive(Debug, Clone)]
pub struct Cpp {
    /// Example: g++
    pub program: &'static str,

    /// Example: main.cpp
    file_name: PathBuf,

    /// Example: -std=c++17
    standard: &'static str,

    /// Example: binary, binary.o, binary.exe etc..
    binary_file: PathBuf,

    /// Example: -Wall
    flags: Vec<&'static str>,

    /// Example -DLOCAL, -DONLINE_JUDGE, etc
    variables: Vec<&'static str>,

    stdin: Option<PathBuf>,

    stdout: Option<PathBuf>,

    stderr: Option<PathBuf>,
}

impl Cpp {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        program: &'static str,
        file_name: PathBuf,
        standard: &'static str,
        binary_file: PathBuf,
        flags: Vec<&'static str>,
        variables: Vec<&'static str>,
        stdin: Option<PathBuf>,
        stdout: Option<PathBuf>,
        stderr: Option<PathBuf>,
    ) -> Cpp {
        Cpp {
            program,
            file_name,
            standard,
            binary_file,
            flags,
            variables,
            stdin,
            stdout,
            stderr,
        }
    }
}

impl Language for Cpp {
    fn build(&self) -> bool {
        let status = Command::new(self.program)
            .arg(self.standard)
            .args(&self.flags)
            .args(&self.variables)
            .arg("-o")
            .arg(self.binary_file.to_str().unwrap())
            .arg(self.file_name.to_str().unwrap())
            .status()
            .expect("Compiling C++ error");
        status.code() == Some(0)
    }

    fn execute(&self, timeout: u32, testcase: u32) -> StatusResponse {
        let commands = vec![self.binary_file.to_str().unwrap()];
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
        "C++ Language".to_string()
    }
}

#[derive(Debug, PartialEq)]
pub struct CppConfig {
    pub program: String,
    pub standard: String,
}

impl Default for CppConfig {
    fn default() -> Self {
        CppConfig {
            program: "g++".to_string(),
            standard: "-std=c++17".to_string(),
        }
    }
}

pub mod default {
    use std::path::PathBuf;

    use super::Cpp;

    pub fn gnucpp17_default(
        root: &str,
        file_name: &str,
        binary_file: &str,
        input_file: &str,
        output_file: &str,
        error_file: &str,
    ) -> Cpp {
        let stdin = PathBuf::from(format!("{}/{}", root, input_file));
        let stdout = PathBuf::from(format!("{}/{}", root, output_file));
        let stderr = PathBuf::from(format!("{}/{}", root, error_file));

        Cpp::new(
            "g++",
            PathBuf::from(format!("{}/{}", root, file_name)),
            "-std=c++17",
            PathBuf::from(format!("{}/{}", root, binary_file)),
            vec!["-Wall"],
            vec!["-DONLINE_JUDGE=1"],
            Some(stdin),
            Some(stdout),
            Some(stderr),
        )
    }

    pub fn gnucpp17_set_output(
        root: &str,
        file_name: &str,
        binary_file: &str,
        output_file: &str,
    ) -> Cpp {
        let stdout = PathBuf::from(format!("{}/{}", root, output_file));

        Cpp::new(
            "g++",
            PathBuf::from(format!("{}/{}", root, file_name)),
            "-std=c++17",
            PathBuf::from(format!("{}/{}", root, binary_file)),
            vec!["-Wall"],
            vec!["-DLOCAL=1"],
            None,
            Some(stdout),
            None,
        )
    }
}
