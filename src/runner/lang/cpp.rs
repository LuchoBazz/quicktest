/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021  Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::io::Write;
use std::process::Command;
use std::process::Stdio;
use std::path::PathBuf;
use std::fs::File;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use crate::runner::types::Compiler;
use std::time::Instant;

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

impl Compiler for Cpp {

    fn compile(&self) -> () {
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

        let now: Instant = Instant::now();

        let mut child: std::process::Child = match &self.stdin {
            Some(file) => {
                let input = File::open(file.to_str().unwrap()).unwrap();

                let child = Command::new(self.binary_file.to_str().unwrap())
                    .stdin(Stdio::from(input))
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .unwrap();
                child
            },
            _ => {
                let child = Command::new(self.binary_file.to_str().unwrap())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .unwrap();
                child
            }
        };

        let output_file: Option<Arc<Mutex<File>>> = match &self.stdout {
            Some(file) => {
                let output = File::create(file.to_str().unwrap()).unwrap();
                Some(Arc::new(Mutex::new(output)))
            },
            _ => None,
        };

        let err_file: Option<Arc<Mutex<File>>> = match &self.stderr {
            Some(file) =>{
                let err = File::create(file.to_str().unwrap()).unwrap();
                Some(Arc::new(Mutex::new(err)))
            },
            _ => None,
        };

        let thread: std::thread::JoinHandle<()> = std::thread::spawn(move || {
            for _ in 0..timeout {
                if let Ok(Some(_)) = child.try_wait() {
                    if let Ok(response) = child.wait_with_output() {
                        match output_file {
                            Some(f) => {
                                let mut file: std::sync::MutexGuard<File> = f.lock().unwrap();
                                file.write_all(&response.stdout).unwrap();
                            },
                            None => (),
                        }

                        match err_file {
                            Some(f) => {
                                let mut file: std::sync::MutexGuard<File> = f.lock().unwrap();
                                file.write_all(&response.stderr).unwrap();
                            },
                            None => (),
                        }
                    }
                    return;
                }
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
            child.kill().unwrap();
        });

        thread.join().unwrap();

        let new_now: Instant = Instant::now();

        let time: Duration = new_now.duration_since(now);

        time 
    }
}