/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::collections::VecDeque;

// std library
use std::path::PathBuf;
use std::time::Duration;

// dependencies
use exitfailure::ExitFailure;

use crate::cli::structures::TLECommand;
// local library
use crate::error::handle_error::throw_compiler_error_msg;
use crate::file_handler::file::{
    can_run_language_or_error, copy_file, create_folder_or_error, file_exists_or_error,
    format_filename_test_case, is_extension_supported_or_error, load_testcases_from_states,
    remove_files, remove_files_with_prefix, remove_folder, save_test_case,
};
use crate::generator::generator::execute_generator;
use crate::language::language_handler::{get_generator_handler, get_language_handler};
use crate::painter::style::{
    show_accepted, show_runtime_error, show_stats, show_time_limit_exceeded,
};
use crate::runner::types::{is_compiled_error, is_runtime_error, is_time_limit_exceeded, Language};

// Constants
use crate::constants::{
    CACHE_FOLDER, GEN_BINARY_FILE, PREFIX_AC_FILES, PREFIX_RTE_FILES, PREFIX_TLE_FILES,
    QTEST_ERROR_FILE, QTEST_INPUT_FILE, QTEST_OUTPUT_FILE, TARGET_BINARY_FILE, TEST_CASES_FOLDER,
};

pub fn run(command: &TLECommand) -> Result<(), ExitFailure> {
    // create cache folder
    create_folder_or_error(CACHE_FOLDER)?;

    // verify that the target file exists
    file_exists_or_error(command.target_file.to_str().unwrap(), "<target-file>")?;

    // verify that the generator file exists
    file_exists_or_error(command.gen_file.to_str().unwrap(), "<gen-file>")?;

    // verify that the target file extension is supported
    is_extension_supported_or_error(command.target_file.to_str().unwrap())?;

    // verify that the generator file extension is supported
    is_extension_supported_or_error(command.gen_file.to_str().unwrap())?;

    // Get the language depending on the extension of the gen_file
    let generator_file_lang = *get_generator_handler(
        &command
            .gen_file
            .clone()
            .into_os_string()
            .into_string()
            .unwrap()[..],
        "<gen-file>",
        QTEST_INPUT_FILE,
    )?;

    // verify that the program to run the generator file is installed
    can_run_language_or_error(&generator_file_lang)?;

    // Get the language depending on the extension of the target_file
    let target_file_lang = *get_language_handler(
        &command
            .target_file
            .clone()
            .into_os_string()
            .into_string()
            .unwrap()[..],
        "<target-file>",
        QTEST_INPUT_FILE,
        QTEST_OUTPUT_FILE,
        QTEST_ERROR_FILE,
    )?;

    // verify that the program to run the target file is installed
    can_run_language_or_error(&target_file_lang)?;

    let can_compile_gen = generator_file_lang.build();
    if !can_compile_gen {
        return throw_compiler_error_msg("generator", "<gen-file>");
    }

    let can_compile_target = target_file_lang.build();
    if !can_compile_target {
        return throw_compiler_error_msg("target", "<target-file>");
    }

    if command.save_all {
        // Remove all previous test cases
        remove_folder(TEST_CASES_FOLDER);
    } else if command.save_bad {
        // remove test cases prefixed with test_cases/testcase_tle*.txt
        let prefix = &format!("{}/{}*", TEST_CASES_FOLDER, PREFIX_TLE_FILES)[..];
        remove_files_with_prefix(prefix);
    }

    let mut cases: VecDeque<PathBuf> = VecDeque::new();
    load_testcases_from_states(
        &mut cases,
        TEST_CASES_FOLDER,
        command.run_all,
        command.run_ac,
        command.run_wa,
        command.run_tle,
        command.run_rte,
    )?;

    let mut tle_count: u32 = 0;
    let mut rte_count: u32 = 0;
    let mut ac_count: u32 = 0;

    let load_case: bool =
        command.run_all || command.run_ac || command.run_wa || command.run_tle || command.run_rte;

    let mut test_number: u32 = 0;
    while test_number < command.test_cases || load_case {
        test_number += 1;

        if load_case {
            if !cases.is_empty() {
                // Load test case in stdin
                let case = cases.pop_front().unwrap();
                copy_file(case.to_str().unwrap(), QTEST_INPUT_FILE)?;
            } else {
                break;
            }
        } else {
            // run generator
            execute_generator(&generator_file_lang, command.timeout, test_number)?;
        }

        let response_target = target_file_lang.execute(command.timeout as u32, test_number);
        let time_target: Duration = response_target.time;
        let mills_target = time_target.as_millis();

        if is_runtime_error(&response_target.status) {
            rte_count += 1;
            show_runtime_error(test_number, mills_target as u32);
            // Save the input of the test case that gave status tle
            if command.save_bad || command.save_all {
                // Example: test_cases/testcase_rte_01.txt
                let file_name: &str =
                    &format_filename_test_case(TEST_CASES_FOLDER, PREFIX_RTE_FILES, rte_count)[..];
                // save testcase
                save_test_case(file_name, QTEST_INPUT_FILE);
            }
            // check if the tle_breck flag is high
            if command.break_bad {
                // remove input, output and error files
                remove_files(vec![
                    QTEST_INPUT_FILE,
                    QTEST_OUTPUT_FILE,
                    QTEST_ERROR_FILE,
                    TARGET_BINARY_FILE,
                    GEN_BINARY_FILE,
                ]);

                return Ok(());
            }
            continue;
        } else if is_compiled_error(&response_target.status) {
            return throw_compiler_error_msg("target", "<target-file>");
        }

        if time_target >= Duration::from_millis(command.timeout as u64)
            || is_time_limit_exceeded(&response_target.status)
        {
            // TLE Target file
            tle_count += 1;
            show_time_limit_exceeded(test_number, command.timeout);

            // Save the input of the test case that gave status tle
            if command.save_bad || command.save_all {
                // Example: test_cases/testcase_tle_01.txt
                let file_name: &str =
                    &format_filename_test_case(TEST_CASES_FOLDER, PREFIX_TLE_FILES, tle_count)[..];
                // save testcase
                save_test_case(file_name, QTEST_INPUT_FILE);
            }

            // check if the tle_breck flag is high
            if command.break_bad {
                // remove input, output and error files
                remove_files(vec![
                    QTEST_INPUT_FILE,
                    QTEST_OUTPUT_FILE,
                    QTEST_ERROR_FILE,
                    TARGET_BINARY_FILE,
                    GEN_BINARY_FILE,
                ]);

                return Ok(());
            }
        } else {
            ac_count += 1;
            if command.save_all {
                // Example: test_cases/testcase_ac_01.txt
                let file_name: &str =
                    &format_filename_test_case(TEST_CASES_FOLDER, PREFIX_AC_FILES, ac_count)[..];
                save_test_case(file_name, QTEST_INPUT_FILE);
            }
            show_accepted(test_number, mills_target as u32);
        }
    }
    show_stats(ac_count, 0, tle_count, rte_count);

    // remove input, output and error files
    remove_files(vec![
        QTEST_INPUT_FILE,
        QTEST_OUTPUT_FILE,
        QTEST_ERROR_FILE,
        TARGET_BINARY_FILE,
        GEN_BINARY_FILE,
    ]);

    Ok(())
}
