/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

// std library
use std::collections::VecDeque;
use std::path::PathBuf;
use std::time::Duration;

// local library
use crate::file_handler::path::get_root_path;
use crate::generator::generator::execute_generator;
use crate::error::handle_error::{
    throw_runtime_error_msg, throw_time_limit_exceeded_msg,
    throw_break_found_msg, throw_compiler_error_msg
};
use crate::file_handler::file::{
    copy_file, create_folder_or_error, file_exists_or_error, format_filename_test_case,
    load_testcases, remove_files, remove_folder, save_test_case
};
use crate::painter::style::{
    show_accepted, show_runtime_error, show_stats, show_time_limit_exceeded,
    show_time_limit_exceeded_checker, show_wrong_answer
};
use crate::runner::types::{
    is_runtime_error, is_time_limit_exceeded,
    Language, is_compiled_error
};

// dependencies
use exitfailure::ExitFailure;

// Constants
use crate::constants::{
    CACHE_FOLDER, CHECKER_BINARY_FILE, GEN_BINARY_FILE, PREFIX_AC_FILES,
    PREFIX_RTE_FILES, PREFIX_TLE_FILES, PREFIX_WA_FILES, QTEST_CHECKER_FILE,
    QTEST_ERROR_FILE, QTEST_INPUT_FILE, QTEST_OUTPUT_FILE, TARGET_BINARY_FILE,
    TEST_CASES_FOLDER
};
use crate::util::lang::{
    get_language_by_ext_default,
    get_language_by_ext_set_output
};

pub fn run(target_file: PathBuf, checker_file: PathBuf,
        gen_file: PathBuf, timeout: u32, test_cases: u32, break_bad: bool,
        save_bad: bool, save_all: bool, run_all: bool, run_ac: bool,
        run_wa: bool, run_tle: bool, run_rte: bool) -> Result<(), ExitFailure>  {
    
    // create cache folder
    create_folder_or_error(CACHE_FOLDER)?;
    
    // verify that the target file exists
    file_exists_or_error(target_file.to_str().unwrap(), "<target-file>")?;

    // verify that the checker_file file exists
    file_exists_or_error(checker_file.to_str().unwrap(), "<checker-file>")?;

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

    let can_compile_gen = generator_file_lang.build();
    if !can_compile_gen {
        return throw_compiler_error_msg("generator", "<gen-file>");
    }
    
    let can_compile_target = target_file_lang.build();
    if !can_compile_target {
        return throw_compiler_error_msg("target", "<target-file>");
    }
    
    let can_compile_checker = checker_file_lang_lang.build();
    if !can_compile_checker {
        return throw_compiler_error_msg("checker", "<checker-file>");
    }

    if save_bad || save_all {
        // Remove all previous test cases
        remove_folder(TEST_CASES_FOLDER);
    }

    let mut cases: VecDeque<PathBuf> = VecDeque::new();
    load_testcases(&mut cases, run_all, run_ac, run_wa, run_tle, run_rte)?;
    
    let mut tle_count: u32 = 0;
    let mut wa_count:  u32 = 0;
    let mut rte_count: u32 = 0;
    let mut ac_count:  u32 = 0;

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
            execute_generator(generator_file_lang, timeout, test_number)?;
        }

        let response_target = target_file_lang.execute(timeout as u32, test_number);
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
            // check if the break_bad flag is high
            if break_bad {
                // remove input, output and error files
                remove_files(vec![QTEST_INPUT_FILE, QTEST_OUTPUT_FILE, QTEST_ERROR_FILE, QTEST_CHECKER_FILE,
                    TARGET_BINARY_FILE, GEN_BINARY_FILE, CHECKER_BINARY_FILE]);
                return Ok(());
            }
            continue;
        } else if is_compiled_error(&response_target.status) {
            return throw_compiler_error_msg("target", "<target-file>");
        }

        let response_checker = checker_file_lang_lang.execute(timeout as u32, test_number);
        let time_checker: Duration = response_checker.time;

        if is_runtime_error(&response_checker.status) {
            return throw_runtime_error_msg("checker", "<checker-file>");
        } else if is_compiled_error(&response_checker.status) {
            return throw_compiler_error_msg("checker", "<checker-file>");
        }

        if time_checker >= Duration::from_millis(timeout as u64) {
            // TLE checker file
            show_time_limit_exceeded_checker(test_number, timeout);
            return throw_time_limit_exceeded_msg("checker", "<checker-file>");
        }

        if time_target >= Duration::from_millis(timeout as u64) || is_time_limit_exceeded(&response_target.status) {
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
                remove_files(vec![QTEST_INPUT_FILE, QTEST_OUTPUT_FILE, QTEST_ERROR_FILE, QTEST_CHECKER_FILE,
                    TARGET_BINARY_FILE, GEN_BINARY_FILE, CHECKER_BINARY_FILE]);
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
                    remove_files(vec![QTEST_INPUT_FILE, QTEST_OUTPUT_FILE, QTEST_ERROR_FILE, QTEST_CHECKER_FILE,
                        TARGET_BINARY_FILE, GEN_BINARY_FILE, CHECKER_BINARY_FILE]);
                    
                    return throw_break_found_msg("Wrong Answer", "WA", test_cases);
                }
            }
        }
    }

    show_stats(ac_count, wa_count, tle_count, rte_count);
    
    // remove input, output, error and binary files
    remove_files(vec![QTEST_INPUT_FILE, QTEST_OUTPUT_FILE, QTEST_ERROR_FILE, QTEST_CHECKER_FILE,
        TARGET_BINARY_FILE, GEN_BINARY_FILE, CHECKER_BINARY_FILE]);

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