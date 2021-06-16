/*
*  Quick Test: CLI for stress testing in competitive programming
*  Copyright (C) 2021 - Luis Miguel Báez
*  License: MIT (See the LICENSE file in the repository root directory)
*/

use std::env;
use std::path::PathBuf;
use std::fs;
use std::time::Duration;

use glob::glob;
use exitfailure::ExitFailure;
use failure::ResultExt;
use colored::*;

use crate::constants::QTEST_ERROR_FILE;
use crate::constants::QTEST_INPUT_FILE;
use crate::constants::QTEST_OUTPUT_FILE;
use crate::constants::TARGET_BINARY_FILE;
use crate::constants::TEST_CASES_FOLDER;
use crate::runner::types::Language;
use crate::util::file::file_exists;
use crate::util::lang::get_language_by_ext_default;

pub fn run(target_file: PathBuf, timeout: u32, all: bool, wa: bool, tle: bool, rte: bool) -> Result<(), ExitFailure>  {
    let mut wa = wa;
    let mut tle = tle;
    let mut rte = rte;
    if all { wa = true; tle = true; rte = true; }

    // Check if the CACHE_FOLDER folder is already created
    match fs::read_dir(TEST_CASES_FOLDER) {
        Ok(_) => (),
        Err(_) => match fs::create_dir(TEST_CASES_FOLDER) {
            Ok(_) => (),
            Err(_) => {
                // If not, create the folder
                let error: Result<(), failure::Error> = Err(failure::err_msg(format!("I can't create the test cases folder")));
                return Ok(error.context("Error creating the test cases folder".to_string())?);
            }
        },
    }

    // verify that the target file exists
    match fs::File::open(target_file.to_str().unwrap()) {
        Err(_) => {
            let error = Err(failure::err_msg(format!("Can't open the file {}", target_file.to_str().unwrap())));
            return Ok(error.context("<target-file> Not found".to_string())?);
        },
        _ => (),
    }

    let mut test_cases = Vec::new();

    if wa {
        let paths = glob(&format!("{}/testcase_wa*", TEST_CASES_FOLDER))?;
        for entry in paths {
            match entry {
                Ok(path) => {
                    test_cases.push(path);
                },
                Err(_) => (),
            }
        }
    }

    if tle {
        let paths = glob(&format!("{}/testcase_tle*", TEST_CASES_FOLDER))?;
        for entry in paths {
            match entry {
                Ok(path) => {
                    test_cases.push(path);
                },
                Err(_) => (),
            }
        }
    }

    if rte {
        let paths = glob(&format!("{}/testcase_rte*", TEST_CASES_FOLDER))?;
        for entry in paths {
            match entry {
                Ok(path) => {
                    test_cases.push(path);
                },
                Err(_) => (),
            }
        }
    }

    let root = match env::current_dir() {
        Ok(it) => it,
        _ => unreachable!(),
    };

    let root = match root.to_str() {
        Some(s) => s ,
        _ => unreachable!(),
    };

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

    target_file_lang.build();

    for case in test_cases {
        let path = case.to_str().unwrap();
        any_target.set_stdio(path);

        let response_case = any_target.execute(timeout);
        let time_target: std::time::Duration = response_case.time;

        let mills_target = time_target.as_millis();

        if time_target >= Duration::from_millis(timeout as u64) {
            // TLE Target file
            
            println!(
                "  {} [{}] {} {}ms",
                path,
                "TLE".bold().red(),
                "Time Limit Exceeded :".bold().red(),
                timeout
            );
        } else {
            println!(
                "  {} [{}] {} {}ms",
                path.to_string().bold().white(),
                "OK".bold().green(),
                "Finished in".bold().green(), mills_target
            );
        }
    }
    // remove input, output and error files
    fs::remove_file(&QTEST_INPUT_FILE)?;
    fs::remove_file(&QTEST_OUTPUT_FILE)?;
    fs::remove_file(&QTEST_ERROR_FILE)?;

    match file_exists(&TARGET_BINARY_FILE) {
        Ok(_) => fs::remove_file(&TARGET_BINARY_FILE)?,
        _ => (),
    }
    
    Ok(())
}