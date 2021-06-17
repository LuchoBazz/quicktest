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

use exitfailure::ExitFailure;
use glob::glob;

use crate::error::handle_error::{
    throw_compiler_error_msg, throw_couldnt_create_folder_msg,
    throw_couldnt_open_file_msg, throw_runtime_error_msg,
    throw_time_limit_exceeded_msg
};
use crate::runner::types::{
    Language, is_time_limit_exceeded,
    is_compiled_error, is_runtime_error
};
use crate::util::file::file_exists;
use crate::util::lang::{
    get_language_by_ext_default,
    get_language_by_ext_set_output
};
use crate::painter::style::{
    show_accepted, show_time_limit_exceeded,
    show_time_limit_exceeded_generator, show_runtime_error
};

// Constants
use crate::constants::{
    CACHE_FOLDER, TARGET_BINARY_FILE, GEN_BINARY_FILE,
    QTEST_INPUT_FILE, QTEST_OUTPUT_FILE, QTEST_ERROR_FILE
};

pub fn run(target_file: PathBuf, gen_file: PathBuf,
        test_cases: u32, timeout: u32, tle_break: bool, save_cases: bool) -> Result<(), ExitFailure> {
    
    // Check if the CACHE_FOLDER folder is already created
    if let Err(_) = fs::read_dir(CACHE_FOLDER) {
        if let Err(_) = fs::create_dir(CACHE_FOLDER) {
            // If not, create the folder
            return throw_couldnt_create_folder_msg(CACHE_FOLDER);
        }
    }

    // verify that the target file exists
    if let Err(_) = fs::File::open(target_file.to_str().unwrap()) {
        return throw_couldnt_open_file_msg(target_file.to_str().unwrap(), "<target-file>");
    }
    
    // verify that the generator file exists
    if let Err(_) = fs::File::open(gen_file.to_str().unwrap()) {
        return throw_couldnt_open_file_msg(gen_file.to_str().unwrap(), "<gen-file>");
    }

    let root = match env::current_dir() {
        Ok(it) => it,
        _ => unreachable!(),
    };

    let root = match root.to_str() {
        Some(s) => s ,
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

    let can_compile_gen = generator_file_lang.build();
    if !can_compile_gen {
        return throw_compiler_error_msg("generator", "<gen-file>");
    }
    
    let can_compile_target = target_file_lang.build();
    if !can_compile_target {
        return throw_compiler_error_msg("target", "<target-file>");
    }

    if save_cases {
        // remove test cases prefixed with testcase_tle*.txt
        let paths = glob("test_cases/testcase_tle*")?;
        for entry in paths {
            if let Ok(path) = entry {
                fs::remove_file(path.to_str().unwrap())?;
            }
        }
    }

    let mut tle_count: u32 = 0;

    for test_number in 1..=test_cases {
        let response_gen = generator_file_lang.execute(timeout as u32);
        let time_gen = response_gen.time;

        if is_runtime_error(&response_gen.status) {
            return throw_runtime_error_msg("generator", "<gen-file>");
        } else if is_compiled_error(&response_gen.status) {
            return throw_compiler_error_msg("generator", "<gen-file>");
        }

        if time_gen >= Duration::from_millis(timeout as u64) || is_time_limit_exceeded(&response_gen.status) {
            // TLE Generator
            show_time_limit_exceeded_generator(test_number, timeout);
            return throw_time_limit_exceeded_msg("generator", "<gen-file>");
        }

        let response_target = target_file_lang.execute(timeout as u32);
        let time_target: Duration = response_target.time;
        let mills_target = time_target.as_millis();

        if is_runtime_error(&response_target.status) {
            show_runtime_error(test_number, mills_target as u32);
            continue;
        } else if is_compiled_error(&response_target.status) {
            return throw_compiler_error_msg("target", "<target-file>");
        }

        if time_target >= Duration::from_millis(timeout as u64) || is_time_limit_exceeded(&response_gen.status) {
            // TLE Target file
            tle_count += 1;
            show_time_limit_exceeded(test_number, timeout);
        
            // Verify that the folder test_cases exists, in case it does not exist create it
            if save_cases && !Path::new("test_cases").exists() {
                if let Err(_) = fs::create_dir("test_cases") {
                    return throw_couldnt_create_folder_msg("test_cases");
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

                if let Ok(_) = file_exists(&TARGET_BINARY_FILE) {
                    fs::remove_file(&TARGET_BINARY_FILE)?;
                }
                if let Ok(_) = file_exists(&GEN_BINARY_FILE) {
                    fs::remove_file(&GEN_BINARY_FILE)?;
                }
               return Ok(());
            }
        } else {
            show_accepted(test_number, mills_target as u32);
        }
    }
    // remove input, output and error files
    match fs::remove_file(&QTEST_INPUT_FILE) {_=>()}
    match fs::remove_file(&QTEST_OUTPUT_FILE) {_=>()}
    match fs::remove_file(&QTEST_ERROR_FILE) {_=>()}

    if let Ok(_) = file_exists(&TARGET_BINARY_FILE) {
        fs::remove_file(&TARGET_BINARY_FILE)?;
    }
    if let Ok(_) = file_exists(&GEN_BINARY_FILE) {
        fs::remove_file(&GEN_BINARY_FILE)?;
    }

    Ok(())
}
