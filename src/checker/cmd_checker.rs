/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::collections::VecDeque;
use std::path::Path;
use std::path::PathBuf;
use std::fs;
use std::env;
use std::time::Duration;
use std::io::Write;

use crate::runner::types::Language;
use crate::util::file::file_exists;

// Constants
use crate::constants::CACHE_FOLDER;
use crate::constants::TARGET_BINARY_FILE;
use crate::constants::CHECKER_BINARY_FILE;
use crate::constants::GEN_BINARY_FILE;
use crate::constants::QTEST_INPUT_FILE;
use crate::constants::QTEST_OUTPUT_FILE;
use crate::constants::QTEST_ERROR_FILE;
use crate::constants::QTEST_CHECKER_FILE;
use crate::util::lang::{
    get_language_by_ext_default,
    get_language_by_ext_set_output
};

use failure::ResultExt;
use exitfailure::ExitFailure;
use colored::*;
use glob::glob;

pub fn run(target_file: PathBuf, checker_file: PathBuf,
        gen_file: PathBuf, timeout: u32, test_cases: u32, wa_break: bool,
        save_cases: bool) -> Result<(), ExitFailure>  {
    
     // Check if the CACHE_FOLDER folder is already created
     match fs::read_dir(CACHE_FOLDER) {
        Ok(_) => (),
        Err(_) => match fs::create_dir(CACHE_FOLDER) {
            Ok(_) => (),
            Err(_) => {
                // If not, create the folder
                let error: Result<(), failure::Error> = Err(failure::err_msg(format!("Can't create internal cache files")));
                return Ok(error.context("Error creating internal cache files".to_string())?);
            }
        },
    }
    
    // verify that the target file exists
    let file_name = target_file.to_str().unwrap();
    match file_exists(file_name) {
        Ok(_) => (),
        Err(_) => {
            let error: Result<(), failure::Error> = Err(failure::err_msg(format!("Can't open the file '{}'", file_name)));
            return Ok(error.context("<target-file> Not found".to_string())?);
        }
    }

    // verify that the correct file exists
    let file_name = checker_file.to_str().unwrap();
    match file_exists(file_name) {
        Ok(_) => (),
        Err(_) => {
            let error: Result<(), failure::Error> = Err(failure::err_msg(format!("Can't open the file '{}'", file_name)));
            return Ok(error.context("<checker-file> Not found".to_string())?);
        }
    }

    // verify that the generator file exists
    let file_name = gen_file.to_str().unwrap();
    match file_exists(file_name) {
        Ok(_) => (),
        Err(_) => {
            let error: Result<(), failure::Error> = Err(failure::err_msg(format!("Can't open the file '{}'", file_name)));
            return Ok(error.context("<gen-file> Not found".to_string())?);
        }
    }

    // get root path
    let root: PathBuf = match env::current_dir() {
        Ok(path) => path,
        _ => unreachable!(),
    };

    let root: &str = match root.to_str() {
        Some(root_path) => root_path,
        _ => unreachable!(),
    };

    // Get the language depending on the extension of the gen_file
    let any_gen: Option<Box<dyn Language>> = get_language_by_ext_set_output(
        root,
        gen_file,
        &GEN_BINARY_FILE,
        &QTEST_INPUT_FILE,
    );
    let any_gen: Box<dyn Language> = any_gen.unwrap();
    let generator_file_lang: &dyn Language = any_gen.as_ref();
    
    // Get the language depending on the extension of the target_file
    let any_target: Option<Box<dyn Language>> = get_language_by_ext_default(
        root,
        target_file,
        &TARGET_BINARY_FILE,
        &QTEST_INPUT_FILE,
        &QTEST_OUTPUT_FILE,
        &QTEST_ERROR_FILE
    );
    let any_target: Box<dyn Language> = any_target.unwrap();
    let target_file_lang: &dyn Language = any_target.as_ref();
    
    // Get the language depending on the extension of the checker_file_lang
    let any_checker: Option<Box<dyn Language>> = get_language_by_ext_default(
        root,
        checker_file,
        &CHECKER_BINARY_FILE,
        &QTEST_OUTPUT_FILE,
        &QTEST_CHECKER_FILE,
        &QTEST_ERROR_FILE
    );
    let any_checker: Box<dyn Language> = any_checker.unwrap();
    let checker_file_lang_lang: &dyn Language = any_checker.as_ref();

    checker_file_lang_lang.build();

    target_file_lang.build();

    generator_file_lang.build();

    if save_cases {
        // remove test cases prefixed with testcase_wa*.txt
        let paths = glob("test_cases/testcase_wa*")?;
        for entry in paths {
            match entry {
                Ok(path) => {
                    fs::remove_file(path.to_str().unwrap())?;
                },
                Err(_) => (),
            }
        }
    }

    let mut tle_count: u32 = 0;
    let mut wa_count: u32 = 0;

    for test_number in 1..=test_cases {
        let time_gen: Duration = generator_file_lang.execute(timeout as u32);
        let time_target: Duration = target_file_lang.execute(timeout as u32);
        let time_checker: Duration = checker_file_lang_lang.execute(timeout as u32);

        let mills_target: u128 = time_target.as_millis();

        if time_gen >= Duration::from_millis(timeout as u64) {
            // TLE Generator
            println!(
                "  {} [{}] {} {}ms",
                test_number,
                "TLE".bold().red(),
                "Generator Time Limit Exceeded :".bold().red(),
                timeout
            );

            let error: Result<(), failure::Error> = Err(failure::err_msg("very slow generator"));
            return Ok(error.context("Generator TLE".to_string())?);
        }

        if time_checker >= Duration::from_millis(timeout as u64) {
            // TLE Correct file
            println!(
                "  {} [{}] {} {}ms",
                test_number,
                "TLE".bold().red(),
                "Correct File give Time Limit Exceeded :".bold().red(),
                timeout
            );

            let error: Result<(), failure::Error> = Err(failure::err_msg("Correct file very slow"));
            return Ok(error.context("Correct File TLE".to_string())?);
        }

        if time_target >= Duration::from_millis(timeout as u64) {
            // TLE Target file
            
            tle_count += 1;

            println!(
                "  {} [{}] {} {}ms",
                test_number,
                "TLE".bold().red(),
                "Time Limit Exceeded :".bold().red(),
                timeout
            );
        
            // Verify that the folder test_cases exists, in case it does not exist create it
            if save_cases && !Path::new("test_cases").exists() {
                match fs::create_dir("test_cases") {
                    Err(_) => {
                        let error = Err(failure::err_msg("Could not create folder test_cases"));
                        return Ok(error.context("test_cases folder".to_string())?);
                    }
                    _ => (),
                }
            }

            // Save the input of the test case that gave status tle
            if save_cases {
                let filename: String = format!("test_cases/testcase_tle_{}.txt", tle_count);
                let mut file = fs::File::create(filename)
                    .expect("Error creating file test_cases/testcase(i).txt");
                file.write_all(fs::read_to_string(QTEST_INPUT_FILE).unwrap().as_bytes()).unwrap();
            }
            
            // check if the wa_break flag is high
            if wa_break {
                // remove input, output and error files
                fs::remove_file(&QTEST_INPUT_FILE)?;
                fs::remove_file(&QTEST_OUTPUT_FILE)?;
                fs::remove_file(&QTEST_ERROR_FILE)?;
                fs::remove_file(&QTEST_CHECKER_FILE)?;

                match file_exists(&TARGET_BINARY_FILE) {
                    Ok(_) => fs::remove_file(&TARGET_BINARY_FILE)?,
                    _ => (),
                }
                match file_exists(&GEN_BINARY_FILE) {
                    Ok(_) => fs::remove_file(&GEN_BINARY_FILE)?,
                    _ => (),
                }
                match file_exists(&CHECKER_BINARY_FILE) {
                    Ok(_) => fs::remove_file(&CHECKER_BINARY_FILE)?,
                    _ => (),
                };
                return Ok(());
            }
        } else {
            // The time is in the allowed range
            let file_checker = format!("{}/{}", root, QTEST_CHECKER_FILE);
            
            // Check WA Status
            if check_answer(&file_checker, true) {
                // is OK
                println!(
                    "  {} [{}] {} {}ms",
                    test_number.to_string().bold().white(),
                    "OK".bold().green(),
                    "Finished in".bold().green(), mills_target
                );
            } else {
                // WA found
                wa_count += 1;
                println!(
                    "  {} [{}] {} {}ms",
                    test_number.to_string().bold().white(),
                    "WA".bold().red(),
                    "Finished in".bold().red(), mills_target
                );

                // Verify that the folder test_cases exists, in case it does not exist create it
                if save_cases && !Path::new("test_cases").exists() {
                    match fs::create_dir("test_cases") {
                        Err(_) => {
                            let error = Err(failure::err_msg("Could not create folder test_cases"));
                            return Ok(error.context("test_cases folder".to_string())?);
                        }
                        _ => (),
                    }
                }

                // Save the input of the test case that gave status tle
                if save_cases {
                    let filename: String = format!("test_cases/testcase_wa_{}.txt", wa_count);
                    let mut file = fs::File::create(filename)
                        .expect("Error creating file test_cases/testcase(i).txt");
                    file.write_all(fs::read_to_string(QTEST_INPUT_FILE).unwrap().as_bytes()).unwrap();
                }

                if wa_break {
                    // remove input, output and error files
                    fs::remove_file(&QTEST_INPUT_FILE)?;
                    fs::remove_file(&QTEST_OUTPUT_FILE)?;
                    fs::remove_file(&QTEST_ERROR_FILE)?;
                    fs::remove_file(&QTEST_CHECKER_FILE)?;
                    
                    match file_exists(&TARGET_BINARY_FILE) {
                        Ok(_) => fs::remove_file(&TARGET_BINARY_FILE)?,
                        _ => (),
                    }
                    match file_exists(&GEN_BINARY_FILE) {
                        Ok(_) => fs::remove_file(&GEN_BINARY_FILE)?,
                        _ => (),
                    }
                    match file_exists(&CHECKER_BINARY_FILE) {
                        Ok(_) => fs::remove_file(&CHECKER_BINARY_FILE)?,
                        _ => (),
                    };

                    let error = Err(failure::err_msg(format!("Wrong answer on test {}", test_number)));
                    return Ok(error.context("WA Status".to_string())?);
                }
            }
        }

        // remove input, output and error files
        fs::remove_file(&QTEST_INPUT_FILE)?;
        fs::remove_file(&QTEST_OUTPUT_FILE)?;
        fs::remove_file(&QTEST_ERROR_FILE)?;
        fs::remove_file(&QTEST_CHECKER_FILE)?;
        
    }

    match file_exists(&TARGET_BINARY_FILE) {
        Ok(_) => fs::remove_file(&TARGET_BINARY_FILE)?,
        _ => (),
    }
    match file_exists(&GEN_BINARY_FILE) {
        Ok(_) => fs::remove_file(&GEN_BINARY_FILE)?,
        _ => (),
    }
    match file_exists(&CHECKER_BINARY_FILE) {
        Ok(_) => fs::remove_file(&CHECKER_BINARY_FILE)?,
        _ => (),
    }

    Ok(())
}

// checker compare  compare_file

pub fn check_answer(answer_file: &String, ignore_space: bool) -> bool {
    let mut is_good = true;
    let target_content = match std::fs::read_to_string(answer_file) {
        Ok(content) => Some(content),
        Err(_) => {
            is_good = false;
            None
        },
    };

    if !is_good { return false; }

    let mut target_content_vec: VecDeque<char> = VecDeque::new();
    is_good = match target_content {
        Some(s1) => {
            target_content_vec = s1.to_lowercase().chars().collect::<VecDeque<char>>();
            true
        },
        _ => false,
    };
    
    if !is_good { return false; }

    if ignore_space {
        // Remove spaces at the beginning and end of the file
        // for target_content_vec
        while !target_content_vec.is_empty()
            && (*target_content_vec.back().unwrap()==' '||*target_content_vec.back().unwrap()=='\n') {
            target_content_vec.pop_back();
        }
        while !target_content_vec.is_empty()
            && (*target_content_vec.front().unwrap()==' '||*target_content_vec.front().unwrap()=='\n') {
            target_content_vec.pop_front();
        }
    }

    target_content_vec == vec!['y', 'e', 's']
}