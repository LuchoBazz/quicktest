/*
*  Quick Test: CLI for stress testing in competitive programming
*  Copyright (C) 2021 - Luis Miguel Báez
*  License: MIT (See the LICENSE file in the repository root directory)
*/

// std library
use std::path::PathBuf;
use std::time::Duration;

// dependencies
use glob::glob;
use exitfailure::ExitFailure;

// local library
use crate::error::handle_error::{
    throw_compiler_error_msg, throw_runtime_error_msg
};
use crate::file_handler::file::{
    create_folder_or_error, file_exists_or_error,
    remove_files
};
use crate::file_handler::path::get_root_path;
use crate::painter::style::{
    show_accepted_case, show_time_limit_exceeded_case
};
use crate::runner::types::{
    Language, is_compiled_error, is_runtime_error
};
use crate::util::lang::get_language_by_ext_default;

use crate::constants::{
    PREFIX_RTE_FILES, PREFIX_TLE_FILES, PREFIX_WA_FILES,
    QTEST_ERROR_FILE, QTEST_INPUT_FILE, QTEST_OUTPUT_FILE,
    TARGET_BINARY_FILE, TEST_CASES_FOLDER
};

pub fn run(target_file: PathBuf, timeout: u32, all: bool, wa: bool, tle: bool, rte: bool) -> Result<(), ExitFailure>  {
    let mut wa = wa;
    let mut tle = tle;
    let mut rte = rte;
    if all { wa = true; tle = true; rte = true; }

    // Check if the TEST_CASES_FOLDER folder is already created
    create_folder_or_error(TEST_CASES_FOLDER)?;

    // verify that the target file exists
    file_exists_or_error(target_file.to_str().unwrap(), "<target-file>")?;

    let mut test_cases = Vec::new();

    if wa {
        let paths = glob(&format!("{}/{}*", TEST_CASES_FOLDER, PREFIX_WA_FILES))?;
        for entry in paths {
            if let Ok(path) = entry {
                test_cases.push(path);
            }
        }
    }

    if tle {
        let paths = glob(&format!("{}/{}*", TEST_CASES_FOLDER, PREFIX_TLE_FILES))?;
        for entry in paths {
            if let Ok(path) = entry {
                test_cases.push(path);
            }
        }
    }

    if rte {
        let paths = glob(&format!("{}/{}*", TEST_CASES_FOLDER, PREFIX_RTE_FILES))?;
        for entry in paths {
            if let Ok(path) = entry {
                test_cases.push(path);
            }
        }
    }

    let root = &get_root_path()[..];

    // Get the language depending on the extension of the target_file
    let any_target: Option<Box<dyn Language>> = get_language_by_ext_default(
        root,
        target_file,
        &TARGET_BINARY_FILE,
        &QTEST_INPUT_FILE,
        &QTEST_OUTPUT_FILE,
        &QTEST_ERROR_FILE
    );
    let mut any_target: Box<dyn Language> = any_target.unwrap();
    let target_file_lang = any_target.as_mut();

    let can_compile_target = target_file_lang.build();
    if !can_compile_target {
        return throw_compiler_error_msg("target", "<target-file>");
    }

    for case in test_cases {
        let path = case.to_str().unwrap();
        any_target.set_stdio(path);

        let response_target = any_target.execute(timeout);
        let time_target: std::time::Duration = response_target.time;

        let mills_target = time_target.as_millis();

        if is_runtime_error(&response_target.status) {
            return throw_runtime_error_msg("target", "<target-file>");
        } else if is_compiled_error(&response_target.status) {
            return throw_compiler_error_msg("target", "<target-file>");
        }

        if time_target >= Duration::from_millis(timeout as u64) {
            // TLE Target file
            show_time_limit_exceeded_case(path, timeout);
        } else {
            show_accepted_case(path, mills_target as u32);
        }
    }
    // remove input, output and error files
    remove_files(vec![
        QTEST_INPUT_FILE,
        QTEST_OUTPUT_FILE,
        QTEST_ERROR_FILE,
        TARGET_BINARY_FILE
    ]);
    
    Ok(())
}