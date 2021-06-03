use std::path::PathBuf;

use crate::runner::lang::cpp::Cpp;

pub fn default_gnucpp17(root: &str, file_name: &str, 
        binary_file: &str, input_file: &str, output_file: &str,
        error_file: &str) -> Cpp {
    Cpp::new(
        "g++",
        PathBuf::from(format!("{}/{}", root, file_name)),
        "-std=c++17",
        PathBuf::from(format!("{}/{}", root, binary_file)),
        vec!["-Wall"],
        vec!["-DLOCAL=1"],
        PathBuf::from(format!("{}/{}", root, input_file)),
        PathBuf::from(format!("{}/{}", root, output_file)),
        PathBuf::from(format!("{}/{}", root, error_file))
    )
}