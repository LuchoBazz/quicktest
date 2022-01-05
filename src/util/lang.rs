use std::any::Any;
use std::path::PathBuf;

use crate::runner::lang::cpp::default::{gnucpp17_default, gnucpp17_set_output};
use crate::runner::lang::cpp::Cpp;
use crate::runner::lang::python::default::{python3_default, python3_set_output};
use crate::runner::lang::python::Python;
use crate::runner::types::Language;
use crate::util::file::get_extension;

// Reference: https://stackoverflow.com/questions/26126683/how-to-match-trait-implementors

pub fn get_language_by_ext_default(
    root: &str,
    file_name: PathBuf,
    binary_file: &str,
    stdin: &str,
    stdout: &str,
    stderr: &str,
) -> Option<Box<dyn Language>> {
    let any: Box<dyn Any> =
        _get_language_by_ext_default(root, file_name, binary_file, stdin, stdout, stderr);
    if let Some(python_lang) = any.downcast_ref::<Python>() {
        let x = python_lang.clone();
        Some(Box::new(x))
    } else if let Some(cpp_lang) = any.downcast_ref::<Cpp>() {
        let x = cpp_lang.clone();
        Some(Box::new(x))
    } else {
        unreachable!();
    }
}

fn _get_language_by_ext_default(
    root: &str,
    file_name: PathBuf,
    binary_file: &str,
    stdin: &str,
    stdout: &str,
    stderr: &str,
) -> Box<dyn Any> {
    let ext: &str = get_extension(&file_name).unwrap();

    match ext {
        "cpp" => Box::new(gnucpp17_default(
            root,
            file_name.to_str().unwrap(),
            binary_file,
            stdin,
            stdout,
            stderr,
        )),
        "py" => Box::new(python3_default(
            root,
            file_name.to_str().unwrap(),
            stdin,
            stdout,
            stderr,
        )),
        _ => unreachable!(),
    }
}

pub fn get_language_by_ext_set_output(
    root: &str,
    file_name: PathBuf,
    binary_file: &str,
    stdout: &str,
) -> Option<Box<dyn Language>> {
    let any: Box<dyn Any> = _get_language_by_ext_set_output(root, file_name, binary_file, stdout);
    if let Some(python_lang) = any.downcast_ref::<Python>() {
        let x = python_lang.clone();
        Some(Box::new(x))
    } else if let Some(cpp_lang) = any.downcast_ref::<Cpp>() {
        let x = cpp_lang.clone();
        Some(Box::new(x))
    } else {
        unreachable!();
    }
}

fn _get_language_by_ext_set_output(
    root: &str,
    file_name: PathBuf,
    binary_file: &str,
    stdout: &str,
) -> Box<dyn Any> {
    let ext: &str = get_extension(&file_name).unwrap();

    match ext {
        "cpp" => Box::new(gnucpp17_set_output(
            root,
            file_name.to_str().unwrap(),
            binary_file,
            stdout,
        )),
        "py" => Box::new(python3_set_output(
            root,
            file_name.to_str().unwrap(),
            stdout,
        )),
        _ => unreachable!(),
    }
}
