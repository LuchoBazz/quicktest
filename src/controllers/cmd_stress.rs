/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021-present / Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::collections::VecDeque;

// std library
use std::path::PathBuf;
use std::process;
use std::time::Duration;

// dependencies
use exitfailure::ExitFailure;

use crate::cli::model::stress_command::StressCommand;
use crate::cli::model::traits::AdapterCommand;
// local library
use crate::error::handle_error::{throw_break_found_msg, throw_compiler_error_msg};
use crate::file_handler::file::{
    create_folder_or_error, delete_test_case_folder, format_filename_test_case,
    load_test_cases_from_status, load_testcases_from_prefix, remove_files, save_test_case,
};
use crate::generator::generator::execute_generator;
use crate::language::get_language::{get_executor_generator, get_executor_target};
use crate::language::language_handler::LanguageHandler;
use crate::runner::types::{
    is_compiled_error, is_memory_limit_exceeded, is_runtime_error, is_time_limit_exceeded, Language,
};
use crate::views::style::{
    show_accepted, show_memory_limit_exceeded_error, show_runtime_error, show_stats,
    show_time_limit_exceeded,
};

// Constants
use crate::constants::{
    CACHE_FOLDER, GEN_BINARY_FILE, PREFIX_AC_FILES, PREFIX_MLE_FILES, PREFIX_RTE_FILES,
    PREFIX_TLE_FILES, QTEST_ERROR_FILE, QTEST_INPUT_FILE, QTEST_OUTPUT_FILE, TARGET_BINARY_FILE,
    TEST_CASES_FOLDER,
};

pub fn run(command: &StressCommand) -> Result<(), ExitFailure> {
    // Check if the CACHE_FOLDER folder is already created
    create_folder_or_error(CACHE_FOLDER)?;

    // Get the language depending on the extension of the target_file
    let target_file_lang: LanguageHandler = *get_executor_target(command)?;

    // Get the language depending on the extension of the gen_file
    let generator_file_lang: LanguageHandler = *get_executor_generator(command)?;

    // deletes the test cases folder, if and only if it is explicitly told to do so from the cli
    delete_test_case_folder(command);

    let mut cases: VecDeque<PathBuf> = VecDeque::new();
    load_test_cases_from_status(command, &mut cases)?;
    load_testcases_from_prefix(&mut cases, &command.get_prefix()[..])?;

    let mut tle_count: u32 = 0;
    let mut rte_count: u32 = 0;
    let mut ac_count: u32 = 0;
    let mut mle_count: u32 = 0;

    let mut test_number: u32 = 0;
    while command.has_test_cases(test_number) {
        test_number += 1;

        let mut can_continue = false;

        // run generator or load testcases using prefix
        execute_generator(
            &generator_file_lang,
            command,
            &mut cases,
            test_number,
            &mut can_continue,
        )?;

        if !can_continue {
            break;
        }

        let response_target = target_file_lang.execute(
            command.get_timeout() as u32,
            command.get_memory_limit(),
            test_number,
        );
        let time_target: Duration = response_target.time;
        let mills_target = time_target.as_millis();

        if is_runtime_error(&response_target.status) {
            rte_count += 1;
            show_runtime_error(test_number, mills_target as u32);
            // Save the input of the test case that gave status tle
            if command.get_save_bad() || command.get_save_all() {
                // Example: test_cases/testcase_rte_01.txt
                let file_name: &str =
                    &format_filename_test_case(TEST_CASES_FOLDER, PREFIX_RTE_FILES, rte_count)[..];
                // save testcase
                save_test_case(file_name, QTEST_INPUT_FILE);
            }
            // check if the tle_breck flag is high
            if command.get_break_bad() {
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
        } else if is_memory_limit_exceeded(&response_target.status) {
            mle_count += 1;
            show_memory_limit_exceeded_error(test_number, mills_target as u32);

            if command.get_save_bad() || command.get_save_all() {
                // Example: test_cases/testcase_mle_01.txt
                let file_name: &str =
                    &format_filename_test_case(TEST_CASES_FOLDER, PREFIX_MLE_FILES, mle_count)[..];
                // save testcase
                save_test_case(file_name, QTEST_INPUT_FILE);
            }

            if command.get_break_bad() {
                // remove input, output and error files
                remove_files(vec![
                    QTEST_INPUT_FILE,
                    QTEST_OUTPUT_FILE,
                    QTEST_ERROR_FILE,
                    TARGET_BINARY_FILE,
                    GEN_BINARY_FILE,
                ]);

                return throw_break_found_msg(
                    "Memory Limit Exceeded",
                    "MLE",
                    command.get_test_cases(),
                );
            }
            continue;
        }

        if time_target >= Duration::from_millis(command.get_timeout() as u64)
            || is_time_limit_exceeded(&response_target.status)
        {
            // TLE Target file
            tle_count += 1;
            show_time_limit_exceeded(test_number, command.get_timeout());

            // Save the input of the test case that gave status tle
            if command.get_save_bad() || command.get_save_all() {
                // Example: test_cases/testcase_tle_01.txt
                let file_name: &str =
                    &format_filename_test_case(TEST_CASES_FOLDER, PREFIX_TLE_FILES, tle_count)[..];
                // save testcase
                save_test_case(file_name, QTEST_INPUT_FILE);
            }

            // check if the tle_breck flag is high
            if command.get_break_bad() {
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
            if command.get_save_all() {
                // Example: test_cases/testcase_ac_01.txt
                let file_name: &str =
                    &format_filename_test_case(TEST_CASES_FOLDER, PREFIX_AC_FILES, ac_count)[..];
                save_test_case(file_name, QTEST_INPUT_FILE);
            }
            show_accepted(test_number, mills_target as u32);
        }
    }
    show_stats(ac_count, 0, tle_count, rte_count, mle_count);

    // remove input, output and error files
    remove_files(vec![
        QTEST_INPUT_FILE,
        QTEST_OUTPUT_FILE,
        QTEST_ERROR_FILE,
        TARGET_BINARY_FILE,
        GEN_BINARY_FILE,
    ]);

    // check if the target file has errors
    if (tle_count + rte_count + mle_count) > 0 {
        // exit status as error in software
        process::exit(exitcode::SOFTWARE);
    }

    Ok(())
}
