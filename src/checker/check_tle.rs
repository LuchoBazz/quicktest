/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

// std library
use std::path::PathBuf;
use std::time::Duration;
use std::fs;

// dependencies
use exitfailure::ExitFailure;

// local library
use crate::file_handler::path::get_root_path;
use crate::error::handle_error::{
    throw_compiler_error_msg, throw_runtime_error_msg,
    throw_time_limit_exceeded_msg
};
use crate::file_handler::file::{
    create_folder_or_error, file_exists_or_error,
    remove_files, remove_files_with_prefix, write_file
};
use crate::runner::types::{
    Language, is_time_limit_exceeded,
    is_compiled_error, is_runtime_error
};
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
    CACHE_FOLDER, GEN_BINARY_FILE, PREFIX_TLE_FILES,
    QTEST_ERROR_FILE, QTEST_INPUT_FILE, QTEST_OUTPUT_FILE,
    TARGET_BINARY_FILE, TEST_CASES_FOLDER
};

pub fn run(target_file: PathBuf, gen_file: PathBuf,
        test_cases: u32, timeout: u32, tle_break: bool, save_cases: bool) -> Result<(), ExitFailure> {
    
    // create cache folder
    create_folder_or_error(CACHE_FOLDER)?;

    // verify that the target file exists
    file_exists_or_error(target_file.to_str().unwrap(), "<target-file>")?;
    
    // verify that the generator file exists
    file_exists_or_error(gen_file.to_str().unwrap(), "<gen-file>")?;
    
    let root = &get_root_path()[..];

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
        // remove test cases prefixed with test_cases/testcase_tle*.txt
        let prefix = &format!("{}/{}*", TEST_CASES_FOLDER, PREFIX_TLE_FILES)[..];
        remove_files_with_prefix(prefix);
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

            // Save the input of the test case that gave status tle
            if save_cases {
                // create test_cases folder
                create_folder_or_error(TEST_CASES_FOLDER)?;
                // Example: test_cases/testcase_tle_1.txt
                let file_name: &str = &format!( "{}/{}_{}.txt", TEST_CASES_FOLDER, PREFIX_TLE_FILES, tle_count)[..];
                write_file(file_name, fs::read_to_string(QTEST_INPUT_FILE).unwrap().as_bytes())?;
            }
            
            // check if the tle_breck flag is high
            if tle_break {
                // remove input, output and error files
                remove_files(vec![
                    QTEST_INPUT_FILE,
                    QTEST_OUTPUT_FILE,
                    QTEST_ERROR_FILE,
                    TARGET_BINARY_FILE,
                    GEN_BINARY_FILE
                ]);
               return Ok(());
            }
        } else {
            show_accepted(test_number, mills_target as u32);
        }
    }

    // remove input, output and error files
    remove_files(vec![
        QTEST_INPUT_FILE,
        QTEST_OUTPUT_FILE,
        QTEST_ERROR_FILE,
        TARGET_BINARY_FILE,
        GEN_BINARY_FILE
    ]);

    Ok(())
}
