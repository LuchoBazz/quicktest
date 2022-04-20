/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use exitfailure::ExitFailure;
use failure::ResultExt;

pub fn throw_compiler_error_msg(file_name: &str, label: &str) -> Result<(), ExitFailure> {
    let error = Err(failure::err_msg(format!(
        "failed to compile the {} file / label {}",
        file_name, label
    )));
    Ok(error.context("QTEST_COMPILER_ERROR")?)
}

pub fn throw_runtime_error_msg(file_name: &str, label: &str) -> Result<(), ExitFailure> {
    let error = Err(failure::err_msg(format!(
        "{} file exited by Runtime Error / label {}",
        file_name, label
    )));
    Ok(error.context("QTEST_RUNTIME_ERROR")?)
}

pub fn throw_time_limit_exceeded_msg(file_name: &str, label: &str) -> Result<(), ExitFailure> {
    let error = Err(failure::err_msg(format!(
        "{} very slow / label {}",
        file_name, label
    )));
    Ok(error.context("QTEST_TIME_LIMIT_EXCEEDED")?)
}

pub fn throw_couldnt_create_folder_msg(file_name: &str) -> Result<(), ExitFailure> {
    let error = Err(failure::err_msg(format!(
        "Could not create folder {}",
        file_name
    )));
    Ok(error.context("QTEST_COULDNT_CREATE_FOLDER")?)
}

pub fn throw_couldnt_open_file_msg(file_name: &str, label: &str) -> Result<(), ExitFailure> {
    let error = Err(failure::err_msg(format!(
        "Can't open the file {} / label {}",
        file_name, label
    )));
    Ok(error.context("QTEST_COULDNT_OPEN_FILE")?)
}

pub fn throw_couldnt_write_to_file_msg(file_name: &str) -> Result<(), ExitFailure> {
    let error = Err(failure::err_msg(format!(
        "Could not write to folder {}",
        file_name
    )));
    Ok(error.context("QTEST_COULDNT_WRITE_TO_FILE")?)
}

pub fn throw_extension_not_supported_msg(file: &str, ext: &str) -> Result<(), ExitFailure> {
    let error = Err(failure::err_msg(format!(
        "extension '{}' of file '{}' is not supported",
        ext, file
    )));
    Ok(error.context("QTEST_EXTENSION_NOT_SOPPORTED_ERROR".to_string())?)
}

pub fn throw_configuration_file_error_msg(file_config: &str) -> Result<(), ExitFailure> {
    let error = Err(failure::err_msg(format!(
        "Configuration file {} contains errors",
        file_config
    )));
    Ok(error.context("QTEST_CONFIGURATION_FILE_ERROR".to_string())?)
}

pub fn throw_program_not_installed_msg(program: &str) -> Result<(), ExitFailure> {
    let error = Err(failure::err_msg(format!(
        "Can't run program {} because you don't have an application installed",
        program
    )));
    Ok(error.context("QTEST_PROGRAM_NOT_INSTALLED_ERROR".to_string())?)
}

pub fn throw_setup_label_is_not_correct_msg(label: &str) -> Result<(), ExitFailure> {
    let error = Err(failure::err_msg(format!(
        "Setup label '{}' is not correct",
        label
    )));
    Ok(error.context("QTEST_SETUP_LABEL_IS_NOT_CORRECT_ERROR".to_string())?)
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

#[allow(dead_code)]
pub fn throw_filename_cannot_be_empty() -> Result<(), ExitFailure> {
    let error = Err(failure::err_msg("File Name cannot be empty"));
    Ok(error.context("QTEST_FILE_NAME_CANNOT_BE_EMPTY")?)
}
