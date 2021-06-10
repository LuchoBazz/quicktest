/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::io::Write;
use std::path::{PathBuf, Path};
use std::time::Duration;
use std::env;
use std::fs;

use colored::*;
use failure::ResultExt;
use exitfailure::ExitFailure;

use crate::runner::types::Language;
use crate::util::lang::{
    get_language_by_ext_default,
    get_language_by_ext_set_output
};

// Constants
use crate::constants::CACHE_FOLDER;
use crate::constants::TARGET_BINARY_FILE;
use crate::constants::GEN_BINARY_FILE;
use crate::constants::QTEST_INPUT_FILE;
use crate::constants::QTEST_OUTPUT_FILE;
use crate::constants::QTEST_ERROR_FILE;

pub fn run(target_file: PathBuf, gen_file: PathBuf,
        test_cases: u32, timeout: u32, tle_break: bool, save_cases: bool) -> Result<(), ExitFailure> {
    
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
    match fs::File::open(target_file.to_str().unwrap()) {
        Err(_) => {
            let error = Err(failure::err_msg(format!("Can't open the file {}", target_file.to_str().unwrap())));
            return Ok(error.context("<target-file> Not found".to_string())?);
        },
        _ => (),
    }

    // verify that the generator file exists
    match fs::File::open(gen_file.to_str().unwrap()) {
        Err(_) => {
            let error = Err(failure::err_msg(format!("Can't open the file {}", gen_file.to_str().unwrap())));
            return Ok(error.context("<gen-file> Not found".to_string())?);
        },
        _ => (),
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
    let any_target: Box<dyn Language> = any_target.unwrap();
    let target_file_lang: &dyn Language = any_target.as_ref();

    // Get the language depending on the extension of the gen_file
    let any_gen: Option<Box<dyn Language>> = get_language_by_ext_set_output(
        root,
        gen_file,
        &GEN_BINARY_FILE,
        &QTEST_INPUT_FILE,
    );
    let any_gen: Box<dyn Language> = any_gen.unwrap();
    let generator_file_lang: &dyn Language = any_gen.as_ref();

    target_file_lang.build();

    generator_file_lang.build();

    let mut tle_count: u32 = 0;

    for test_number in 1..=test_cases {
        let time_gen: Duration = generator_file_lang.execute(timeout as u32);
        let time_target: Duration = target_file_lang.execute(timeout as u32);
        
        let mills_target = time_target.as_millis();
        
        if time_gen >= Duration::from_millis(timeout as u64) {
            // TLE Generator

            println!(
                "  {} [{}] {} {}ms",
                test_number,
                "TLE".bold().red(),
                "Generator Time Limit Exceeded :".bold().red(),
                timeout
            );

            let error = Err(failure::err_msg("very slow generator"));
            return Ok(error.context("Generator TLE".to_string())?);
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
                let filename = format!("test_cases/testcase_tle_{}.txt", tle_count);

                let mut file = fs::File::create(filename)
                    .expect("Error creating file test_cases/testcase_tle_(i).txt");

                file.write_all(fs::read_to_string(QTEST_INPUT_FILE).unwrap().as_bytes()).unwrap();
            }
            
            // check if the tle_breck flag is high
            if tle_break {
                // remove input, output and error files
                fs::remove_file(&QTEST_INPUT_FILE)?;
                fs::remove_file(&QTEST_OUTPUT_FILE)?;
                fs::remove_file(&QTEST_ERROR_FILE)?;
                fs::remove_file(&TARGET_BINARY_FILE)?;
                fs::remove_file(&GEN_BINARY_FILE)?;
               return Ok(());
            }
        } else {
            println!(
                "  {} [{}] {} {}ms",
                test_number.to_string().bold().white(),
                "OK".bold().green(),
                "Finished in".bold().green(), mills_target
            );
        }

        // remove input, output and error files
        fs::remove_file(&QTEST_INPUT_FILE)?;
        fs::remove_file(&QTEST_OUTPUT_FILE)?;
        fs::remove_file(&QTEST_ERROR_FILE)?;
    }

    fs::remove_file(&TARGET_BINARY_FILE)?;
    fs::remove_file(&GEN_BINARY_FILE)?;

    Ok(())
}


