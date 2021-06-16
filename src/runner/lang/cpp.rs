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

use crate::runner::types::Language;
use std::time::Instant;

use process_control::ChildExt;
use process_control::Timeout;

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
    pub fn new(program: &'static str, file_name: PathBuf,
            standard: &'static str, binary_file: PathBuf,
            flags: Vec<&'static str>, variables: Vec<&'static str>,
            stdin: Option<PathBuf>, stdout: Option<PathBuf>, stderr: Option<PathBuf>) -> Cpp {
        
        Cpp {
            program: program,
            file_name: file_name,
            standard: standard,
            binary_file: binary_file,
            flags: flags,
            variables: variables,
            stdin: stdin,
            stdout: stdout,
            stderr: stderr
        }
    }
}

impl Language for Cpp {

    fn build(&self) -> () {
        Command::new(self.program)
            .arg(self.standard)
            .args(&self.flags)
            .args(&self.variables)
            .arg("-o")
            .arg(self.binary_file.to_str().unwrap())
            .arg(self.file_name.to_str().unwrap())
            .status()
            .expect("Compiling C++ error");
    }

    fn execute(&self, timeout: u32) -> Duration {
        // TODO: Set pipe buffer size
        // ref1: https://stackoverflow.com/questions/5218741/set-pipe-buffer-size
        // ref2: https://unix.stackexchange.com/questions/328250/set-pipe-capacity-in-linux
        // ref3: https://unix.stackexchange.com/questions/353728/can-i-increase-the-system-pipe-buffer-max
        
        let now: Instant = Instant::now();

        let child: Result<std::process::Child, std::io::Error> = match &self.stdin {
            Some(file) => {
                let input = File::open(file.to_str().unwrap()).unwrap();
                let child = Command::new(self.binary_file.to_str().unwrap())
                    .stdin(Stdio::from(input))
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn();
                child
            },
            _ => {
                let child = Command::new(self.binary_file.to_str().unwrap())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn();
                child
            }
        };
        
        if let Ok(child_output) = child {
            let response = child_output
                .with_output_timeout(Duration::from_millis(timeout as u64))
                .terminating()
                .wait();
            
            if let Ok(output_option)= response {
                if let Some(output) = output_option {
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
                } else {
                    // TLE
                }
            }
        } else {
            // Compiler Error
        }

        let new_now: Instant = Instant::now();
        let time: Duration = new_now.duration_since(now);
        time
    }

    fn set_stdio(&mut self, stdin: &str) {
        self.stdin = Some(PathBuf::from(stdin));
    }
}


pub mod default {
    use std::path::PathBuf;

    use super::Cpp;
    
    pub fn gnucpp17_default(root: &str, file_name: &str, 
            binary_file: &str, input_file: &str, output_file: &str,
            error_file: &str) -> Cpp {
    
        let stdin = PathBuf::from(format!("{}/{}", root, input_file));
        let stdout =  PathBuf::from(format!("{}/{}", root, output_file));
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
            Some(stderr)
        )
    }

    pub fn gnucpp17_set_output(root: &str, file_name: &str, 
        binary_file: &str, output_file: &str) -> Cpp {

        let stdout =  PathBuf::from(format!("{}/{}", root, output_file));

        Cpp::new(
            "g++",
            PathBuf::from(format!("{}/{}", root, file_name)),
            "-std=c++17",
            PathBuf::from(format!("{}/{}", root, binary_file)),
            vec!["-Wall"],
            vec!["-DLOCAL=1"],
            None,
            Some(stdout),
            None
        )
    }
}