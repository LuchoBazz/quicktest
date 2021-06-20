/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::{collections::VecDeque, fs::{self, remove_dir_all}, io::Write, path::PathBuf};

use glob::glob;
use exitfailure::ExitFailure;

use crate::{constants::{PREFIX_AC_FILES, PREFIX_RTE_FILES, PREFIX_TLE_FILES, PREFIX_WA_FILES, TEST_CASES_FOLDER}, error::handle_error::{
        throw_couldnt_create_folder_msg, throw_couldnt_open_file_msg,
        throw_couldnt_write_to_file_msg
    }, util::file::file_exists};

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

pub fn remove_folder(path: &str) {
    if let Ok(_) = remove_dir_all(path) {}
}

pub fn write_file(file_name: &str, bytes: &[u8]) ->  Result<(), ExitFailure> {
    if let Ok(mut file) = std::fs::File::create(file_name) {
        if let Err(_) = file.write_all(bytes) {
            return throw_couldnt_write_to_file_msg(file_name);
        }
    }
    Ok(())
}

pub fn read_file(file_name: &str) ->  Option<String> {
    if let Ok(data) = fs::read_to_string(file_name) {
        return Some(data);
    }
    None
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

pub fn save_test_case(file_name: &str, from_path: &str) {
    // create test_cases folder
    if let Ok(_) = create_folder_or_error(TEST_CASES_FOLDER) {}
    // Example: test_cases/testcase_tle_1.txt
    //let file_name: &str = &format!( "{}/{}_{}.txt", TEST_CASES_FOLDER, PREFIX_TLE_FILES, tle_count)[..];
    if let Ok(_) = write_file(file_name, fs::read_to_string(from_path).unwrap().as_bytes()) {}
}

pub fn copy_file(from: &str, to: &str) -> Result<(), ExitFailure> {
    if let Some(data) = read_file(from) {
        write_file(to, data.as_bytes())?;
    }
    Ok(())
}

pub fn load_testcases(queue: &mut VecDeque<PathBuf>, run_all: bool, run_ac: bool, run_wa: bool, run_tle: bool, run_rte: bool) -> Result<(), ExitFailure> {
    let mut run_ac = run_ac;
    let mut run_wa = run_wa;
    let mut run_tle = run_tle;
    let mut run_rte = run_rte;

    if run_all { run_ac = true; run_wa = true; run_tle = true; run_rte = true; }

    if run_ac {
        let paths = glob(&format!("{}/{}*", TEST_CASES_FOLDER, PREFIX_AC_FILES))?;
        for entry in paths {
            if let Ok(path) = entry {
                queue.push_back(path);
            }
        }
    }

    if run_wa {
        let paths = glob(&format!("{}/{}*", TEST_CASES_FOLDER, PREFIX_WA_FILES))?;
        for entry in paths {
            if let Ok(path) = entry {
                queue.push_back(path);
            }
        }
    }

    if run_tle {
        let paths = glob(&format!("{}/{}*", TEST_CASES_FOLDER, PREFIX_TLE_FILES))?;
        for entry in paths {
            if let Ok(path) = entry {
                queue.push_back(path);
            }
        }
    }

    if run_rte {
        let paths = glob(&format!("{}/{}*", TEST_CASES_FOLDER, PREFIX_RTE_FILES))?;
        for entry in paths {
            if let Ok(path) = entry {
                queue.push_back(path);
            }
        }
    }

    Ok(())
}