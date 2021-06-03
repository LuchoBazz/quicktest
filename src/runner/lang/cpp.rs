use std::io::Write;
use std::process::Command;
use std::process::Stdio;
use std::path::PathBuf;
use std::fs::{File, remove_file};

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
    
    stdin: PathBuf,

    stdout: PathBuf,

    stderr: PathBuf,
}

impl Cpp {
    pub fn new(program: &'static str, file_name: PathBuf,
            standard: &'static str, binary_file: PathBuf,
            flags: Vec<&'static str>, variables: Vec<&'static str>,
            stdin: PathBuf, stdout: PathBuf, stderr: PathBuf) -> Cpp {
        
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

    pub fn clean(&self) {
        match remove_file(self.stdin.to_str().unwrap()) {
            Ok(o) => o,
            Err(_) => println!("Error: No puedo eliminar el archivo stdin"),
        };

        match remove_file(self.stdout.to_str().unwrap()) {
            Ok(o) => o,
            Err(_) => println!("Error: No puedo eliminar el archivo stdout"),
        };

        match remove_file(self.stderr.to_str().unwrap()) {
            Ok(o) => o,
            Err(_) => println!("Error: No puedo eliminar el archivo stderr"),
        };
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
        let input = File::open(self.stdin.to_str().unwrap()).unwrap();
        let mut output = File::create(self.stdout.to_str().unwrap()).unwrap();
        let mut err = File::create(self.stderr.to_str().unwrap()).unwrap();

        let status = Command::new(self.binary_file.to_str().unwrap())
            .stdin(Stdio::from(input))
            .output()
            .expect("Error executing C++");

        output.write_all(&status.stdout).unwrap();
        err.write_all(&status.stderr).unwrap();
    }
}