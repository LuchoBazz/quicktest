/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use exitfailure::ExitFailure;
use failure::ResultExt;

pub fn throw_compiler_error_msg(file_name: &str, label: &str) -> Result<(), ExitFailure> {
    let error = Err(failure::err_msg(format!(
        "failed to compile the {} file",
        file_name
    )));
    Ok(error.context(format!("compilation of {} failed", label))?)
}

pub fn throw_runtime_error_msg(file_name: &str, label: &str) -> Result<(), ExitFailure> {
    let error = Err(failure::err_msg(format!(
        "{} file exited by Runtime Error",
        file_name
    )));
    Ok(error.context(format!("Runtime Error of {}", label))?)
}

pub fn throw_time_limit_exceeded_msg(file_name: &str, label: &str) -> Result<(), ExitFailure> {
    let error = Err(failure::err_msg(format!("{} very slow", file_name)));
    Ok(error.context(format!("{} TLE", label))?)
}

pub fn throw_couldnt_create_folder_msg(file_name: &str) -> Result<(), ExitFailure> {
    let error = Err(failure::err_msg(format!(
        "Could not create folder {}",
        file_name
    )));
    Ok(error.context(format!("{} folder", file_name))?)
}

pub fn throw_couldnt_open_file_msg(file_name: &str, label: &str) -> Result<(), ExitFailure> {
    let error = Err(failure::err_msg(format!(
        "Can't open the file {}",
        file_name
    )));
    Ok(error.context(format!("{} Not found", label))?)
}

pub fn throw_couldnt_write_to_file_msg(file_name: &str) -> Result<(), ExitFailure> {
    let error = Err(failure::err_msg(format!(
        "Could not write to folder {}",
        file_name
    )));
    Ok(error.context(format!("{} file", file_name))?)
}

pub fn throw_extension_not_supported_msg(file: &str, ext: &str) -> Result<(), ExitFailure> {
    let error = Err(failure::err_msg(format!(
        "extension '{}' of file '{}' is not supported",
        ext, file
    )));
    Ok(error.context("EXTENSION_NOT_SOPPORTED_ERROR".to_string())?)
}

pub fn throw_program_not_installed_msg(program: &str) -> Result<(), ExitFailure> {
    let error = Err(failure::err_msg(format!(
        "Can't run program {} because you don't have an application installed",
        program
    )));
    Ok(error.context("PROGRAM_NOT_INSTALLED_ERROR".to_string())?)
}

pub fn throw_break_found_msg(
    status_name: &str,
    status: &str,
    test_number: u32,
) -> Result<(), ExitFailure> {
    let error = Err(failure::err_msg(format!(
        "Wrong answer {} on test {}",
        status_name, test_number
    )));
    Ok(error.context(format!("{} status - break flag on", status))?)
}
