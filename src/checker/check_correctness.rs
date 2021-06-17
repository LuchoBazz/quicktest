/*
 *  Quick Test: CLI for stress testing in competitive programming
 *  Copyright (C) 2021 - Luis Miguel Báez
 *  License: MIT (See the LICENSE file in the repository root directory)
 */

use std::collections::VecDeque;
use std::path::PathBuf;
use std::fs;
use std::time::Duration;

// dependencies
use exitfailure::ExitFailure;

// local library
use crate::error::handle_error::{
    throw_break_found_msg, throw_compiler_error_msg,
    throw_runtime_error_msg, throw_time_limit_exceeded_msg
};
use crate::file_handler::file::{
    create_folder_or_error, file_exists_or_error,
    remove_files, remove_files_with_prefix,
    write_file
};
use crate::file_handler::path::get_root_path;
use crate::painter::style::{
    show_accepted, show_time_limit_exceeded,
    show_time_limit_exceeded_correct,
    show_time_limit_exceeded_generator,
    show_wrong_answer
};
use crate::runner::types::{
    Language,
    is_compiled_error,
    is_runtime_error
};

// Constants
use crate::constants::{
    CACHE_FOLDER, PREFIX_WA_FILES, TEST_CASES_FOLDER,
    TARGET_BINARY_FILE, CORRECT_BINARY_FILE, GEN_BINARY_FILE,
    QTEST_INPUT_FILE, QTEST_OUTPUT_FILE,
    QTEST_ERROR_FILE, QTEST_EXPECTED_FILE
};
use crate::util::lang::{
    get_language_by_ext_default,
    get_language_by_ext_set_output
};

pub fn run(target_file: PathBuf, correct_file: PathBuf,
        gen_file: PathBuf, timeout: u32, test_cases: u32, wa_break: bool,
        save_cases: bool) -> Result<(), ExitFailure>  {

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

    if save_cases {
        // remove test cases prefixed with test_cases/testcase_tle*.txt
        let prefix = &format!("{}/{}*", TEST_CASES_FOLDER, PREFIX_WA_FILES)[..];
        remove_files_with_prefix(prefix);
    }

    let mut tle_count: u32 = 0;
    let mut wa_count: u32 = 0;

    for test_number in 1..=test_cases {
        let response_gen = generator_file_lang.execute(timeout as u32);
        let time_gen: Duration = response_gen.time;

        if is_runtime_error(&response_gen.status) {
            return throw_runtime_error_msg("generator", "<gen-file>");
        } else if is_compiled_error(&response_gen.status) {
            return throw_compiler_error_msg("generator", "<gen-file>");
        }
        
        if time_gen >= Duration::from_millis(timeout as u64) {
            // TLE Generator
            show_time_limit_exceeded_generator(test_number, timeout);
            return throw_time_limit_exceeded_msg("generator", "<gen-file>");
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

        if is_runtime_error(&response_target.status) {
            return throw_runtime_error_msg("target", "<target-file>");
        } else if is_compiled_error(&response_target.status) {
            return throw_compiler_error_msg("target", "<target-file>");
        }

        let mills_target: u128 = time_target.as_millis();

        if time_target >= Duration::from_millis(timeout as u64) {
            // TLE Target file
            
            tle_count += 1;

            show_time_limit_exceeded(test_number, timeout);

            if save_cases {
                // create test_cases folder
                create_folder_or_error(TEST_CASES_FOLDER)?;
                // Example: test_cases/testcase_tle_1.txt
                let file_name: &str = &format!( "{}/{}_{}.txt", TEST_CASES_FOLDER, PREFIX_WA_FILES, tle_count)[..];
                write_file(file_name, fs::read_to_string(QTEST_INPUT_FILE).unwrap().as_bytes())?;
            }
            
            // check if the wa_break flag is high
            if wa_break {
                // remove input, output and error files
                remove_files(vec![
                    QTEST_INPUT_FILE,
                    QTEST_OUTPUT_FILE,
                    QTEST_ERROR_FILE,
                    QTEST_EXPECTED_FILE,
                    TARGET_BINARY_FILE,
                    GEN_BINARY_FILE,
                    CORRECT_BINARY_FILE
                ]);
                return Ok(());
            }
        } else {
            // The time is in the allowed range
            let file_out = format!("{}/{}", root, QTEST_OUTPUT_FILE);
            let file_expected = format!("{}/{}", root, QTEST_EXPECTED_FILE);
            
            // Check WA Status
            if compare_file(&file_out, &file_expected, true) {
                // is OK
                show_accepted(test_number, mills_target as u32);
            } else {
                // WA found
                wa_count += 1;

                show_wrong_answer(test_number, mills_target as u32);

                // Save the input of the test case that gave status wa
                if save_cases {
                    create_folder_or_error(TEST_CASES_FOLDER)?;
                    // Example: test_cases/testcase_wa_1.txt
                    let file_name: &str = &format!( "{}/{}_{}.txt", TEST_CASES_FOLDER, PREFIX_WA_FILES, wa_count)[..];
                    write_file(file_name, fs::read_to_string(QTEST_INPUT_FILE).unwrap().as_bytes())?;
                }

                if wa_break {
                    // remove input, output and error files
                    remove_files(vec![
                        QTEST_INPUT_FILE,
                        QTEST_OUTPUT_FILE,
                        QTEST_ERROR_FILE,
                        QTEST_EXPECTED_FILE,
                        TARGET_BINARY_FILE,
                        GEN_BINARY_FILE,
                        CORRECT_BINARY_FILE
                    ]);
                    return throw_break_found_msg("Wrong Answer", "WA", test_cases);
                }
            }
        }
    }

    // remove input, output, error and binary files
    remove_files(vec![
        QTEST_INPUT_FILE,
        QTEST_OUTPUT_FILE,
        QTEST_ERROR_FILE,
        QTEST_EXPECTED_FILE,
        TARGET_BINARY_FILE,
        GEN_BINARY_FILE,
        CORRECT_BINARY_FILE
    ]);

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