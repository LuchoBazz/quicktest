/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use failure::ResultExt;
use exitfailure::ExitFailure;

pub fn throw_compiler_error_msg(file_name: &str, label: &str) -> Result<(), ExitFailure> {
    let error = Err(failure::err_msg(
        format!("failed to compile the {} file", file_name)
    ));
    return Ok(error.context(
        format!("compilation of {} failed", label)
    )?);
}

pub fn throw_runtime_error_msg(file_name: &str, label: &str) -> Result<(), ExitFailure> {
    let error = Err(failure::err_msg(
        format!("{} file exited by Runtime Error", file_name)
    ));
    return Ok(error.context(
        format!("Runtime Error of {}", label)
    )?);
}

pub fn throw_time_limit_exceeded_msg(file_name: &str, label: &str) -> Result<(), ExitFailure> {
    let error = Err(failure::err_msg(
        format!("{} very slow", file_name)
    ));
    return Ok(error.context(
        format!("{} TLE", label)
    )?);
}

pub fn throw_couldnt_create_folder_msg(file_name: &str) -> Result<(), ExitFailure> {
    let error = Err(failure::err_msg(
        format!("Could not create folder {}", file_name)
    ));
    return Ok(error.context(
        format!("{} folder", file_name)
    )?);
}

pub fn throw_couldnt_open_file_msg(file_name: &str, label: &str) -> Result<(), ExitFailure> {
    let error = Err(failure::err_msg(
        format!("Can't open the file {}", file_name)
    ));
    return Ok(error.context(
        format!("{} Not found", label)
    )?);
}