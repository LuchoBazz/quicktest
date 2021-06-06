/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021  Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::path::PathBuf;

use crate::runner::lang::cpp::Cpp;

pub fn default_gnucpp17(root: &str, file_name: &str, 
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
        vec!["-DLOCAL=1"],
        Some(stdin),
        Some(stdout),
        Some(stderr)
    )
}

pub fn default_set_output_gnucpp17(root: &str, file_name: &str, 
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