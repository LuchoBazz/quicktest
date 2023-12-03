/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021-present / Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::collections::VecDeque;
use std::path::PathBuf;
use std::process;
use std::time::Duration;

// dependencies
use exitfailure::ExitFailure;

use crate::cli::model::cmp_command::CmpCommand;
use crate::cli::model::traits::AdapterCommand;
// local library
use crate::error::handle_error::{
    throw_break_found_msg, throw_compiler_error_msg, throw_memory_limit_exceeded_msg,
    throw_runtime_error_msg, throw_time_limit_exceeded_msg,
};
use crate::file_handler::file::{
    create_folder_or_error, delete_test_case_folder, format_filename_test_case,
    load_test_cases_from_status, load_testcases_from_prefix, read_file, remove_files,
    save_test_case,
};
use crate::file_handler::path::get_root_path;
use crate::generator::generator::execute_generator;
use crate::language::get_language::{
    get_executor_correct, get_executor_generator, get_executor_target,
};
use crate::language::language_handler::LanguageHandler;
use crate::runner::types::{
    is_compiled_error, is_memory_limit_exceeded, is_runtime_error, Language,
};
use crate::views::diff_line_by_line::diff_line_by_line;
use crate::views::style::{
    show_accepted, show_input_test_case, show_memory_limit_exceeded_error, show_runtime_error,
    show_stats, show_time_limit_exceeded, show_time_limit_exceeded_correct, show_wrong_answer,
};

// Constants
use crate::constants::{
    CACHE_FOLDER, CORRECT_BINARY_FILE, GEN_BINARY_FILE, PREFIX_AC_FILES, PREFIX_MLE_FILES,
    PREFIX_RTE_FILES, PREFIX_TLE_FILES, PREFIX_WA_FILES, QTEST_ERROR_FILE, QTEST_EXPECTED_FILE,
    QTEST_INPUT_FILE, QTEST_OUTPUT_FILE, TARGET_BINARY_FILE, TEST_CASES_FOLDER,
};

#[allow(clippy::too_many_arguments)]
pub fn run(command: &CmpCommand) -> Result<(), ExitFailure> {
    // Check if the CACHE_FOLDER folder is already created
    create_folder_or_error(CACHE_FOLDER)?;

    let root = &get_root_path()[..];

    // Get the language depending on the extension of the target_file
    let target_file_lang: LanguageHandler = *get_executor_target(command)?;

    // Get the language depending on the extension of the checker_file_lang
    let correct_file_lang: LanguageHandler = *get_executor_correct(command)?;

    // Get the language depending on the extension of the gen_file
    let generator_file_lang: LanguageHandler = *get_executor_generator(command)?;

    // deletes the test cases folder, if and only if it is explicitly told to do so from the cli
    delete_test_case_folder(command);

    let mut cases: VecDeque<PathBuf> = VecDeque::new();
    load_test_cases_from_status(command, &mut cases)?;
    load_testcases_from_prefix(&mut cases, &command.get_prefix()[..])?;

    let mut tle_count: u32 = 0;
    let mut wa_count: u32 = 0;
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

        let response_correct = correct_file_lang.execute(
            command.get_timeout(),
            command.get_memory_limit(),
            test_number,
        );
        let time_correct: Duration = response_correct.time;

        if is_runtime_error(&response_correct.status) {
            return throw_runtime_error_msg("correct", "<correct-file>");
        } else if is_compiled_error(&response_correct.status) {
            return throw_compiler_error_msg("correct", "<correct-file>");
        } else if is_memory_limit_exceeded(&response_correct.status) {
            return throw_memory_limit_exceeded_msg("correct", "<correct-file>");
        }

        if time_correct >= Duration::from_millis(command.get_timeout() as u64) {
            // TLE Correct file
            show_time_limit_exceeded_correct(test_number, command.get_timeout());
            return throw_time_limit_exceeded_msg("correct", "<correct-file>");
        }

        let response_target = target_file_lang.execute(
            command.get_timeout(),
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
            if command.get_break_bad() {
                // remove input, output and error files
                remove_files(vec![
                    QTEST_INPUT_FILE,
                    QTEST_OUTPUT_FILE,
                    QTEST_ERROR_FILE,
                    QTEST_EXPECTED_FILE,
                    TARGET_BINARY_FILE,
                    GEN_BINARY_FILE,
                    CORRECT_BINARY_FILE,
                ]);

                return throw_break_found_msg("Wrong Answer", "WA", command.get_test_cases());
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
                    QTEST_EXPECTED_FILE,
                    TARGET_BINARY_FILE,
                    GEN_BINARY_FILE,
                    CORRECT_BINARY_FILE,
                ]);

                return throw_break_found_msg(
                    "Memory Limit Exceeded",
                    "MLE",
                    command.get_test_cases(),
                );
            }
            continue;
        }

        if time_target >= Duration::from_millis(command.get_timeout() as u64) {
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

            // check if the break_bad flag is high
            if command.get_break_bad() {
                // remove input, output and error files
                remove_files(vec![
                    QTEST_INPUT_FILE,
                    QTEST_OUTPUT_FILE,
                    QTEST_ERROR_FILE,
                    QTEST_EXPECTED_FILE,
                    TARGET_BINARY_FILE,
                    GEN_BINARY_FILE,
                    CORRECT_BINARY_FILE,
                ]);
                return Ok(());
            }
        } else {
            // The time is in the allowed range
            let file_in = format!("{}/{}", root, QTEST_INPUT_FILE);
            let file_out = format!("{}/{}", root, QTEST_OUTPUT_FILE);
            let file_expected = format!("{}/{}", root, QTEST_EXPECTED_FILE);

            // Check WA Status
            if compare_file(&file_out, &file_expected, true) {
                ac_count += 1;
                // is OK
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

                if command.get_diff() {
                    let mut tout = std::io::stdout();
                    let input = read_file(&file_in[..]).unwrap();
                    let expected = read_file(&file_expected[..]).unwrap();
                    let output = read_file(&file_out[..]).unwrap();
                    show_input_test_case(&mut tout, &input[..]);
                    diff_line_by_line(&mut tout, &expected[..], &output[..]);
                }

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
                        QTEST_EXPECTED_FILE,
                        TARGET_BINARY_FILE,
                        GEN_BINARY_FILE,
                        CORRECT_BINARY_FILE,
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
        QTEST_EXPECTED_FILE,
        TARGET_BINARY_FILE,
        GEN_BINARY_FILE,
        CORRECT_BINARY_FILE,
    ]);

    // check if the target file has errors
    if (wa_count + tle_count + rte_count + mle_count) > 0 {
        // exit status as error in software
        process::exit(exitcode::SOFTWARE);
    }

    Ok(())
}

pub fn compare_file(target_file: &str, correct_file: &str, ignore_space: bool) -> bool {
    let mut is_good = true;
    let target_content = match std::fs::read_to_string(target_file) {
        Ok(content) => Some(content),
        Err(_) => {
            is_good = false;
            None
        }
    };

    let correct_content = match std::fs::read_to_string(correct_file) {
        Ok(content) => Some(content),
        Err(_) => {
            is_good = false;
            None
        }
    };

    if !is_good {
        return false;
    }

    let mut target_content_str = String::new();
    let mut correct_content_str = String::new();

    is_good = match (target_content, correct_content) {
        (Some(_), None) => false,
        (None, Some(_)) => false,
        (None, None) => false,
        (Some(s1), Some(s2)) => {
            target_content_str = s1;
            correct_content_str = s2;
            true
        }
    };

    if !is_good {
        return false;
    }

    if ignore_space {
        let target_content = target_content_str.split('\n').collect::<Vec<&str>>();
        let correct_content = correct_content_str.split('\n').collect::<Vec<&str>>();

        let target_content = target_content
            .iter()
            .map(|&ch| ch.trim())
            .collect::<Vec<&str>>();

        let correct_content = correct_content
            .iter()
            .map(|&ch| ch.trim())
            .collect::<Vec<&str>>();

        return target_content == correct_content;
    }

    target_content_str == correct_content_str
}
