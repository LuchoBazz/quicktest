/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021-present / Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

// std library
use std::collections::VecDeque;
use std::path::PathBuf;
use std::process;
use std::time::Duration;

use crate::cli::model::traits::AdapterCommand;
// local library
use crate::error::handle_error::{
    throw_break_found_msg, throw_compiler_error_msg, throw_memory_limit_exceeded_msg,
    throw_runtime_error_msg, throw_time_limit_exceeded_msg,
};
use crate::file_handler::file::{
    copy_file, create_folder_or_error, delete_test_case_folder, format_filename_test_case,
    load_test_cases_from_status, remove_files, save_test_case,
};
use crate::file_handler::path::get_root_path;
use crate::generator::generator::execute_generator;
use crate::language::get_language::{
    get_executor_checker, get_executor_generator, get_executor_target,
};
use crate::language::language_handler::LanguageHandler;
use crate::runner::types::{
    is_compiled_error, is_memory_limit_exceeded, is_runtime_error, is_time_limit_exceeded, Language,
};
use crate::views::style::{
    show_accepted, show_memory_limit_exceeded_error, show_runtime_error, show_stats,
    show_time_limit_exceeded, show_time_limit_exceeded_checker, show_wrong_answer,
};

// dependencies
use exitfailure::ExitFailure;

// Constants
use crate::constants::{
    CACHE_FOLDER, CHECKER_BINARY_FILE, GEN_BINARY_FILE, PREFIX_AC_FILES, PREFIX_MLE_FILES,
    PREFIX_RTE_FILES, PREFIX_TLE_FILES, PREFIX_WA_FILES, QTEST_CHECKER_FILE, QTEST_ERROR_FILE,
    QTEST_INPUT_FILE, QTEST_OUTPUT_FILE, TARGET_BINARY_FILE, TEST_CASES_FOLDER,
};

#[allow(clippy::too_many_arguments)]
pub fn run(command: &dyn AdapterCommand) -> Result<(), ExitFailure> {
    // create cache folder
    create_folder_or_error(CACHE_FOLDER)?;

    let root = &get_root_path()[..];

    // Get the language depending on the extension of the gen_file
    let generator_file_lang: LanguageHandler = *get_executor_generator(command)?;

    // Get the language depending on the extension of the target_file
    let target_file_lang: LanguageHandler = *get_executor_target(command)?;

    // Get the language depending on the extension of the checker_file_lang
    let checker_file_lang_lang: LanguageHandler = *get_executor_checker(command)?;

    // deletes the test cases folder, if and only if it is explicitly told to do so from the cli
    delete_test_case_folder(command);

    let mut cases: VecDeque<PathBuf> = VecDeque::new();
    load_test_cases_from_status(command, &mut cases)?;

    let mut tle_count: u32 = 0;
    let mut wa_count: u32 = 0;
    let mut rte_count: u32 = 0;
    let mut ac_count: u32 = 0;
    let mut mle_count: u32 = 0;

    let mut test_number: u32 = 0;

    while command.has_test_cases(test_number) {
        test_number += 1;

        if command.can_run_cases() {
            if !cases.is_empty() {
                // Load test case in stdin
                let case = cases.pop_front().unwrap();
                copy_file(case.to_str().unwrap(), QTEST_INPUT_FILE)?;
            } else {
                break;
            }
        } else {
            // run generator
            execute_generator(
                &generator_file_lang,
                command.get_timeout(),
                command.get_memory_limit(),
                test_number,
            )?;
        }

        let response_target = target_file_lang.execute(
            command.get_timeout() as u32,
            command.get_memory_limit(),
            test_number,
        );
        let time_target: Duration = response_target.time;
        let mills_target: u128 = time_target.as_millis();

        if is_runtime_error(&response_target.status) {
            rte_count += 1;
            show_runtime_error(test_number, mills_target as u32);
            if command.get_save_bad() || command.get_save_all() {
                // Example: test_cases/testcase_rte_01.txt
                let file_name: &str =
                    &format_filename_test_case(TEST_CASES_FOLDER, PREFIX_RTE_FILES, rte_count)[..];
                // save testcase
                save_test_case(file_name, QTEST_INPUT_FILE);
            }
            // check if the command.get_break_bad() flag is high
            if command.get_break_bad() {
                // remove input, output and error files
                remove_files(vec![
                    QTEST_INPUT_FILE,
                    QTEST_OUTPUT_FILE,
                    QTEST_ERROR_FILE,
                    QTEST_CHECKER_FILE,
                    TARGET_BINARY_FILE,
                    GEN_BINARY_FILE,
                    CHECKER_BINARY_FILE,
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
                    QTEST_CHECKER_FILE,
                    TARGET_BINARY_FILE,
                    GEN_BINARY_FILE,
                    CHECKER_BINARY_FILE,
                ]);

                return throw_break_found_msg(
                    "Memory Limit Exceeded",
                    "MLE",
                    command.get_test_cases(),
                );
            }
            continue;
        }

        let response_checker = checker_file_lang_lang.execute(
            command.get_timeout() as u32,
            command.get_memory_limit(),
            test_number,
        );
        let time_checker: Duration = response_checker.time;

        if is_runtime_error(&response_checker.status) {
            return throw_runtime_error_msg("checker", "<checker-file>");
        } else if is_compiled_error(&response_checker.status) {
            return throw_compiler_error_msg("checker", "<checker-file>");
        } else if is_memory_limit_exceeded(&response_checker.status) {
            return throw_memory_limit_exceeded_msg("checker", "<checker-file>");
        }

        if time_checker >= Duration::from_millis(command.get_timeout() as u64) {
            // TLE checker file
            show_time_limit_exceeded_checker(test_number, command.get_timeout());
            return throw_time_limit_exceeded_msg("checker", "<checker-file>");
        }

        if time_target >= Duration::from_millis(command.get_timeout() as u64)
            || is_time_limit_exceeded(&response_target.status)
        {
            // TLE Target file
            tle_count += 1;
            show_time_limit_exceeded(test_number, command.get_timeout());

            if command.get_save_bad() || command.get_save_all() {
                // Example: test_cases/testcase_tle_01.txt
                let file_name: &str =
                    &format_filename_test_case(TEST_CASES_FOLDER, PREFIX_TLE_FILES, tle_count)[..];
                // save testcase
                save_test_case(file_name, QTEST_INPUT_FILE);
            }

            // check if the command.get_break_bad() flag is high
            if command.get_break_bad() {
                // remove input, output and error files
                remove_files(vec![
                    QTEST_INPUT_FILE,
                    QTEST_OUTPUT_FILE,
                    QTEST_ERROR_FILE,
                    QTEST_CHECKER_FILE,
                    TARGET_BINARY_FILE,
                    GEN_BINARY_FILE,
                    CHECKER_BINARY_FILE,
                ]);
                return Ok(());
            }
        } else {
            // The time is in the allowed range
            let file_checker = format!("{}/{}", root, QTEST_CHECKER_FILE);

            // Check WA Status
            if check_answer(&file_checker, true) {
                // is OK
                ac_count += 1;
                show_accepted(test_number, mills_target as u32);
                if command.get_save_all() {
                    // Example: test_cases/testcase_ac_01.txt
                    let file_name: &str = &format_filename_test_case(
                        TEST_CASES_FOLDER,
                        PREFIX_AC_FILES,
                        ac_count,
                    )[..];
                    // save testcase
                    save_test_case(file_name, QTEST_INPUT_FILE);
                }
            } else {
                // WA found
                wa_count += 1;

                show_wrong_answer(test_number, mills_target as u32);

                if command.get_save_bad() || command.get_save_all() {
                    // Example: test_cases/testcase_wa_01.txt
                    let file_name: &str = &format_filename_test_case(
                        TEST_CASES_FOLDER,
                        PREFIX_WA_FILES,
                        wa_count,
                    )[..];
                    // save testcase
                    save_test_case(file_name, QTEST_INPUT_FILE);
                }

                if command.get_break_bad() {
                    // remove input, output and error files
                    remove_files(vec![
                        QTEST_INPUT_FILE,
                        QTEST_OUTPUT_FILE,
                        QTEST_ERROR_FILE,
                        QTEST_CHECKER_FILE,
                        TARGET_BINARY_FILE,
                        GEN_BINARY_FILE,
                        CHECKER_BINARY_FILE,
                    ]);

                    return throw_break_found_msg("Wrong Answer", "WA", command.get_test_cases());
                }
            }
        }
    }

    show_stats(ac_count, wa_count, tle_count, rte_count, mle_count);

    // remove input, output, error and binary files
    remove_files(vec![
        QTEST_INPUT_FILE,
        QTEST_OUTPUT_FILE,
        QTEST_ERROR_FILE,
        QTEST_CHECKER_FILE,
        TARGET_BINARY_FILE,
        GEN_BINARY_FILE,
        CHECKER_BINARY_FILE,
    ]);

    // check if the target file has errors
    if (wa_count + tle_count + rte_count + mle_count) > 0 {
        // exit status as error in software
        process::exit(exitcode::SOFTWARE);
    }

    Ok(())
}

// checker compare  compare_file

pub fn check_answer(answer_file: &str, ignore_space: bool) -> bool {
    let mut target_content_str = String::new();

    if let Ok(s1) = std::fs::read_to_string(answer_file) {
        target_content_str.push_str(&s1.to_lowercase());
    } else {
        return false;
    }

    if ignore_space {
        // Remove spaces at the beginning and end of the file
        // for target_content_vec
        target_content_str = target_content_str.trim().to_string();
    }

    "yes" == &target_content_str[..]
}
