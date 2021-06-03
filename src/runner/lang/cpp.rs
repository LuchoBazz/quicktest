use std::io::Write;
use std::process::Command;
use std::process::Stdio;
use std::path::PathBuf;
use std::fs::File;

use crate::runner::types::Compiler;

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

    fn compile(&self) {
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

    fn execute(&self) {

        let status = match &self.stdin {
            Some(file) => {
                let input = File::open(file.to_str().unwrap()).unwrap();

                Command::new(self.binary_file.to_str().unwrap())
                    .stdin(Stdio::from(input))
                    .output()
                    .expect("Error executing C++")
            },
            _ => {
                Command::new(self.binary_file.to_str().unwrap())
                    .output()
                    .expect("Error executing C++")
            }
        };

        match &self.stdout {
            Some(file) => {
                let mut output = File::create(file.to_str().unwrap()).unwrap();
                output.write_all(&status.stdout).unwrap();
            },
            _ => (),
        }

        match &self.stderr {
            Some(file) =>{
                let mut err = File::create(file.to_str().unwrap()).unwrap();
                err.write_all(&status.stderr).unwrap();
            },
            _ => (),
        }
    }
}