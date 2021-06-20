/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::collections::VecDeque;
use std::path::PathBuf;
use std::time::Duration;

// dependencies
use exitfailure::ExitFailure;

// local library
use crate::file_handler::path::get_root_path;
use crate::generator::generator::execute_generator;
use crate::error::handle_error::{
    throw_break_found_msg, throw_compiler_error_msg,
    throw_runtime_error_msg, throw_time_limit_exceeded_msg
};
use crate::file_handler::file::{
    copy_file, create_folder_or_error, file_exists_or_error, format_filename_test_case,
    load_testcases, remove_files, remove_folder, save_test_case
};
use crate::painter::style::{
    show_accepted, show_runtime_error, show_stats,
    show_time_limit_exceeded, show_time_limit_exceeded_correct,
    show_wrong_answer
};
use crate::runner::types::{
    Language,
    is_compiled_error,
    is_runtime_error
};
use crate::util::lang::{
    get_language_by_ext_default,
    get_language_by_ext_set_output
};

// Constants
use crate::constants::{
    CACHE_FOLDER, CORRECT_BINARY_FILE, GEN_BINARY_FILE, PREFIX_AC_FILES, PREFIX_RTE_FILES,
    PREFIX_TLE_FILES, PREFIX_WA_FILES, QTEST_ERROR_FILE, QTEST_EXPECTED_FILE,
    QTEST_INPUT_FILE, QTEST_OUTPUT_FILE, TARGET_BINARY_FILE, TEST_CASES_FOLDER
};

pub fn run(target_file: PathBuf, correct_file: PathBuf,
        gen_file: PathBuf, timeout: u32, test_cases: u32, break_bad: bool,
        save_bad: bool, save_all: bool, run_all: bool, run_ac: bool,
        run_wa: bool, run_tle: bool, run_rte: bool) -> Result<(), ExitFailure>  {

    // Check if the CACHE_FOLDER folder is already created
    create_folder_or_error(CACHE_FOLDER)?;
    
    // verify that the target file exists
    file_exists_or_error(target_file.to_str().unwrap(), "<target-file>")?;

    // verify that the correct file exists
    file_exists_or_error(correct_file.to_str().unwrap(), "<correct-file>")?;

    // verify that the generator file exists
    file_exists_or_error(gen_file.to_str().unwrap(), "<gen-file>")?;

    let root = &get_root_path()[..];

    // Get the language depending on the extension of the correct_file
    let any_correct: Option<Box<dyn Language>> = get_language_by_ext_default(
        root,
        correct_file,
        &CORRECT_BINARY_FILE,
        &QTEST_INPUT_FILE,
        &QTEST_EXPECTED_FILE,
        &QTEST_ERROR_FILE
    );
    let any_correct: Box<dyn Language> = any_correct.unwrap();
    let correct_file_lang: &dyn Language = any_correct.as_ref();

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

    let can_compile_gen = generator_file_lang.build();
    if !can_compile_gen {
        return throw_compiler_error_msg("generator", "<gen-file>");
    }

    let can_compile_target = target_file_lang.build();
    if !can_compile_target {
        return throw_compiler_error_msg("target", "<target-file>");
    }

    let can_compile_correct = correct_file_lang.build();
    if !can_compile_correct {
        return throw_compiler_error_msg("correct", "<correct-file>");
    }

    if save_bad || save_all {
        // Remove all previous test cases
        remove_folder(TEST_CASES_FOLDER);
    }

    let mut cases: VecDeque<PathBuf> = VecDeque::new();
    load_testcases(&mut cases, run_all, run_ac, run_wa, run_tle, run_rte)?;
    
    let mut tle_count: u32 = 0;
    let mut wa_count: u32 = 0;
    let mut rte_count: u32 = 0;
    let mut ac_count: u32 = 0;

    let load_case: bool = run_all || run_ac || run_wa || run_tle || run_rte;
    
    let mut test_number: u32 = 0;

    while test_number < test_cases || load_case {
        test_number += 1;

        if load_case {
            if !cases.is_empty() {
                // Load test case in stdin
                let case = cases.pop_front().unwrap();
                copy_file(case.to_str().unwrap(), QTEST_INPUT_FILE)?;
            } else { break; }
        } else {
            // run generator
            execute_generator(generator_file_lang, timeout, test_cases)?;
        }

        let response_correct = correct_file_lang.execute(timeout as u32);
        let time_correct: Duration = response_correct.time;

        if is_runtime_error(&response_correct.status) {
            return throw_runtime_error_msg("correct", "<correct-file>");
        } else if is_compiled_error(&response_correct.status) {
            return throw_compiler_error_msg("correct", "<correct-file>");
        }

        if time_correct >= Duration::from_millis(timeout as u64) {
            // TLE Correct file
            show_time_limit_exceeded_correct(test_number, timeout);
            return throw_time_limit_exceeded_msg("correct", "<correct-file>");
        }

        let response_target = target_file_lang.execute(timeout as u32);
        let time_target: Duration = response_target.time;
        let mills_target: u128 = time_target.as_millis();

        if is_runtime_error(&response_target.status) {
            rte_count += 1;
            show_runtime_error(test_number, mills_target as u32);
            if save_bad || save_all {
                // Example: test_cases/testcase_rte_01.txt
                let file_name: &str = &format_filename_test_case(TEST_CASES_FOLDER, PREFIX_RTE_FILES, rte_count)[..];
                // save testcase
                save_test_case(file_name, QTEST_INPUT_FILE);
            }
            if break_bad {
                // remove input, output and error files
                remove_files(vec![QTEST_INPUT_FILE, QTEST_OUTPUT_FILE, QTEST_ERROR_FILE, QTEST_EXPECTED_FILE,
                    TARGET_BINARY_FILE, GEN_BINARY_FILE, CORRECT_BINARY_FILE]);
                
                return throw_break_found_msg("Wrong Answer", "WA", test_cases);
            }
            continue;
        } else if is_compiled_error(&response_target.status) {
            return throw_compiler_error_msg("target", "<target-file>");
        }

        if time_target >= Duration::from_millis(timeout as u64) {
            // TLE Target file
            
            tle_count += 1;

            show_time_limit_exceeded(test_number, timeout);

            if save_bad || save_all {
                // Example: test_cases/testcase_tle_01.txt
                let file_name: &str = &format_filename_test_case(TEST_CASES_FOLDER, PREFIX_TLE_FILES, tle_count)[..];
                // save testcase
                save_test_case(file_name, QTEST_INPUT_FILE);
            }
            
            // check if the break_bad flag is high
            if break_bad {
                // remove input, output and error files
                remove_files(vec![QTEST_INPUT_FILE, QTEST_OUTPUT_FILE, QTEST_ERROR_FILE, QTEST_EXPECTED_FILE,
                    TARGET_BINARY_FILE, GEN_BINARY_FILE, CORRECT_BINARY_FILE]);
                return Ok(());
            }
        } else {
            // The time is in the allowed range
            let file_out = format!("{}/{}", root, QTEST_OUTPUT_FILE);
            let file_expected = format!("{}/{}", root, QTEST_EXPECTED_FILE);
            
            // Check WA Status
            if compare_file(&file_out, &file_expected, true) {
                ac_count += 1;
                // is OK
                show_accepted(test_number, mills_target as u32);

                if save_all {
                    // Example: test_cases/testcase_ac_01.txt
                    let file_name: &str = &format_filename_test_case(TEST_CASES_FOLDER, PREFIX_AC_FILES, ac_count)[..];
                    // save testcase
                    save_test_case(file_name, QTEST_INPUT_FILE);
                }
            } else {
                // WA found
                wa_count += 1;

                show_wrong_answer(test_number, mills_target as u32);

                if save_bad || save_all {
                    // Example: test_cases/testcase_wa_01.txt
                    let file_name: &str = &format_filename_test_case(TEST_CASES_FOLDER, PREFIX_WA_FILES, wa_count)[..];
                    // save testcase
                    save_test_case(file_name, QTEST_INPUT_FILE);
                }

                if break_bad {
                    // remove input, output and error files
                    remove_files(vec![QTEST_INPUT_FILE, QTEST_OUTPUT_FILE, QTEST_ERROR_FILE, QTEST_EXPECTED_FILE,
                        TARGET_BINARY_FILE, GEN_BINARY_FILE, CORRECT_BINARY_FILE]);
                    
                    return throw_break_found_msg("Wrong Answer", "WA", test_cases);
                }
            }
        }
    }
    
    show_stats(ac_count, wa_count, tle_count, rte_count);

    // remove input, output, error and binary files
    remove_files(vec![QTEST_INPUT_FILE, QTEST_OUTPUT_FILE, QTEST_ERROR_FILE, QTEST_EXPECTED_FILE,
        TARGET_BINARY_FILE, GEN_BINARY_FILE, CORRECT_BINARY_FILE]);

    Ok(())
}

pub fn compare_file(target_file: &String, correct_file: &String, ignore_space: bool) -> bool {
    let mut is_good = true;
    let target_content = match std::fs::read_to_string(target_file) {
        Ok(content) => Some(content),
        Err(_) => {
            is_good = false;
            None
        },
    };

    let correct_content = match std::fs::read_to_string(correct_file) {
        Ok(content) => Some(content),
        Err(_) => {
            is_good = false;
            None
        },
    };

    if !is_good { return false; }

    let mut target_content_vec: VecDeque<char> = VecDeque::new();
    let mut correct_content_vec: VecDeque<char> = VecDeque::new();
    
    is_good = match (target_content, correct_content) {
        (Some(_), None) => false,
        (None, Some(_)) => false,
        (None, None) => false,
        (Some(s1), Some(s2)) => {
            target_content_vec = s1.chars().collect::<VecDeque<char>>();
            correct_content_vec = s2.chars().collect::<VecDeque<char>>();
            true
        },
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

        // for correct_content_vec
        while !correct_content_vec.is_empty()
            && (*correct_content_vec.back().unwrap()==' '||*correct_content_vec.back().unwrap()=='\n') {
            correct_content_vec.pop_back();
        }
        while !correct_content_vec.is_empty()
            && (*correct_content_vec.front().unwrap()==' '||*correct_content_vec.front().unwrap()=='\n') {
            correct_content_vec.pop_front();
        }

        // replace "  " to " ", " \n" to "\n", "\n " to "\n" and "\n\n" to "\n"
        let mut target_tmp = target_content_vec.clone();
        let mut correct_tmp = correct_content_vec.clone();
        target_content_vec.clear();
        correct_content_vec.clear();

        if !target_tmp.is_empty() {
            target_content_vec.push_back(target_tmp.pop_front().unwrap());
        }
        while !target_tmp.is_empty() {
            match (target_content_vec.back(), target_tmp.pop_front()) {
                (Some(' '), Some('\n')) => {
                    target_content_vec.pop_back();
                    target_content_vec.push_back('\n');
                },
                (Some(' '), Some(' ')) => continue,
                (Some('\n'), Some(' ')) => continue,
                (Some('\n'), Some('\n')) => continue,
                (Some(_), Some(ch)) => target_content_vec.push_back(ch),
                _ => continue,
            }
        }
        if !correct_tmp.is_empty() {
            correct_content_vec.push_back(correct_tmp.pop_front().unwrap());
        }
        while !correct_tmp.is_empty() {
            match (correct_content_vec.back(), correct_tmp.pop_front()) {
                (Some(' '), Some('\n')) => {
                    correct_content_vec.pop_back();
                    correct_content_vec.push_back('\n');
                },
                (Some(' '), Some(' ')) => continue,
                (Some('\n'), Some(' ')) => continue,
                (Some('\n'), Some('\n')) => continue,
                (Some(_), Some(ch)) => correct_content_vec.push_back(ch),
                _ => continue,
            }
        }
    }

    target_content_vec == correct_content_vec
}