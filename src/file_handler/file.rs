/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::{fs, io::Write};

use glob::glob;
use exitfailure::ExitFailure;

use crate::{
    error::handle_error::{
        throw_couldnt_create_folder_msg, throw_couldnt_open_file_msg,
        throw_couldnt_write_to_file_msg
    },
    util::file::file_exists
};

pub fn remove_files(files: Vec<&str>) {
    for file in files {
        if let Ok(_) = file_exists(file) {
            if let Ok(_) = fs::remove_file(&file) {}
        }
    }
}

pub fn remove_files_with_prefix(prefix: &str) {
    if let Ok(paths) = glob(prefix) {
        for entry in paths {
            if let Ok(path) = entry {
                if let Ok(_ ) = fs::remove_file(path.to_str().unwrap()) {}
            }
        }
    }
}

pub fn write_file(file_name: &str, bytes: &[u8]) ->  Result<(), ExitFailure> {
    if let Ok(mut file) = std::fs::File::create(file_name) {
        if let Err(_) = file.write_all(bytes) {
            return throw_couldnt_write_to_file_msg(file_name);
        }
    }
    Ok(())
}

pub fn file_exists_or_error(file_name: &str, label: &str) -> Result<(), ExitFailure> {
    if let Err(_) = fs::File::open(file_name) {
        return throw_couldnt_open_file_msg(file_name, label);
    }
    Ok(())
}

pub fn create_folder_or_error(name: &str) -> Result<(), ExitFailure> {
    if let Err(_) = fs::read_dir(name) {
        if let Err(_) = fs::create_dir(name) {
            // If not, create the folder
            return throw_couldnt_create_folder_msg(name);
        }
    }
    Ok(())
}
