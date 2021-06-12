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
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::time::Instant;

use crate::runner::types::Language;

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

    fn build(&self) -> () {
        // no need to build
    }

    fn execute(&self, timeout: u32) -> Duration {
        let now: Instant = Instant::now();

        let process_output: std::process::Output = match &self.stdin {
            Some(file) => {
                let input = File::open(file.to_str().unwrap()).unwrap();

                let process_output = Command::new(self.program)
                    .arg(&self.file_name)
                    .stdin(Stdio::from(input))
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .output()
                    .unwrap();
                process_output
            },
            _ => {
                let process_output = Command::new(self.program)
                    .arg(&self.file_name)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .output()
                    .unwrap();
                process_output
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
                if process_output.status.success() {
                    match output_file {
                        Some(f) => {
                            let mut file: std::sync::MutexGuard<File> = f.lock().unwrap();
                            file.write_all(&process_output.stdout).unwrap();
                        },
                        None => (),
                    }

                    match err_file {
                        Some(f) => {
                            let mut file: std::sync::MutexGuard<File> = f.lock().unwrap();
                            file.write_all(&process_output.stderr).unwrap();
                        },
                        None => (),
                    }
                    return;
                }
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
        });

        thread.join().unwrap();

        let new_now: Instant = Instant::now();

        let time: Duration = new_now.duration_since(now);

        time 
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