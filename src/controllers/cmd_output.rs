/*
*  Quick Test: CLI for stress testing in competitive programming
*  Copyright (C) 2021-present / Luis Miguel Báez
*  License: MIT (See the LICENSE file in the repository root directory)
*/

use std::{collections::VecDeque, path::PathBuf, time::Duration};

use exitfailure::ExitFailure;

use crate::{
    cli::model::{output_command::OutputCommand, traits::AdapterCommand},
    constants::{
        CACHE_FOLDER, GEN_BINARY_FILE, QTEST_ERROR_FILE, QTEST_INPUT_FILE, QTEST_OUTPUT_FILE,
        TARGET_BINARY_FILE,
    },
    error::handle_error::throw_compiler_error_msg,
    file_handler::{
        async_file::remove_files_async,
        file::{
            copy_file, create_folder_or_error, get_filename_output, load_testcases_from_prefix,
            save_test_case_output,
        },
    },
    language::{get_language::get_executor_target, language_handler::LanguageHandler},
    runner::types::{
        is_compiled_error, is_memory_limit_exceeded, is_runtime_error, is_time_limit_exceeded,
        Language,
    },
    views::style::{
        show_memory_limit_exceeded_error, show_ran_successfully, show_runtime_error,
        show_time_limit_exceeded,
    },
};

pub struct OutputController {
    #[allow(dead_code)]
    target_file: PathBuf,
    #[allow(dead_code)]
    prefix: String,
    #[allow(dead_code)]
    timeout: u32,
    #[allow(dead_code)]
    memory_limit: u64,
    #[allow(dead_code)]
    break_bad: bool,
    #[allow(dead_code)]
    save_out: bool,
    #[allow(dead_code)]
    target_file_lang: Option<LanguageHandler>,
    #[allow(dead_code)]
    cases: VecDeque<PathBuf>,
    #[allow(dead_code)]
    test_number: u32,
    #[allow(dead_code)]
    cases_len: usize,
}

impl OutputController {
    pub fn new(command: &OutputCommand) -> OutputController {
        let target_file_lang = match get_executor_target(command) {
            Ok(it) => Some(*it),
            Err(_) => None,
        };

        OutputController {
            target_file: command.get_target_file().clone(),
            prefix: command.get_prefix().clone(),
            timeout: command.get_timeout(),
            memory_limit: command.get_memory_limit(),
            break_bad: command.get_break_bad(),
            save_out: command.get_save_out(),
            target_file_lang,
            cases: VecDeque::new(),
            test_number: 0,
            cases_len: 0,
        }
    }

    #[allow(dead_code)]
    pub async fn run(&mut self) -> Result<(), ExitFailure> {
        self.create_initial_files()?;

        if self.target_file_lang.is_none() {
            return Ok(());
        }

        self.delete_temporary_files().await.ok();

        load_testcases_from_prefix(&mut self.cases, &self.prefix[..])?;

        self.cases_len = self.cases.len();

        while self.are_tests_pending() {
            self.increment_test_count();

            let case = self.cases.pop_front().unwrap();
            copy_file(case.to_str().unwrap(), QTEST_INPUT_FILE)?;

            let response_target = self.target_file_lang.as_mut().unwrap().execute(
                self.timeout,
                self.memory_limit,
                self.test_number,
            );
            let time_target: Duration = response_target.time;
            let mills_target = time_target.as_millis();

            if is_runtime_error(&response_target.status) {
                show_runtime_error(self.test_number, mills_target as u32);

                // check if the tle_breck flag is high
                if self.break_bad {
                    // remove input, output and error files
                    delete_temporary_files_cmp_output().await.ok();
                    return Ok(());
                }
                continue;
            } else if is_compiled_error(&response_target.status) {
                return throw_compiler_error_msg("target", "<target-file>");
            } else if is_memory_limit_exceeded(&response_target.status) {
                show_memory_limit_exceeded_error(self.test_number, mills_target as u32);
                // check if the tle_breck flag is high
                if self.break_bad {
                    // remove input, output and error files
                    delete_temporary_files_cmp_output().await.ok();
                    return Ok(());
                }
                continue;
            }

            if time_target >= Duration::from_millis(self.timeout as u64)
                || is_time_limit_exceeded(&response_target.status)
            {
                show_time_limit_exceeded(self.test_number, self.timeout);
                // check if the tle_breck flag is high
                if self.break_bad {
                    // remove input, output and error files
                    delete_temporary_files_cmp_output().await.ok();
                    return Ok(());
                }
            } else {
                if self.save_out {
                    let file_name = &get_filename_output(
                        &self.prefix[..],
                        case.file_name().unwrap().to_str().unwrap(),
                    )[..];

                    // save testcase
                    save_test_case_output(file_name, QTEST_OUTPUT_FILE);
                }
                show_ran_successfully(self.test_number, mills_target as u32);
            }
        }
        Ok(())
    }

    fn create_initial_files(&self) -> Result<(), ExitFailure> {
        // Check if the CACHE_FOLDER folder is already created
        create_folder_or_error(CACHE_FOLDER)?;
        Ok(())
    }

    fn are_tests_pending(&self) -> bool {
        (self.test_number as usize) < self.cases_len
    }

    fn increment_test_count(&mut self) {
        self.test_number += 1;
    }

    async fn delete_temporary_files(&self) -> Result<(), tokio::io::Error> {
        remove_files_async(vec![
            QTEST_INPUT_FILE,
            QTEST_OUTPUT_FILE,
            QTEST_ERROR_FILE,
            TARGET_BINARY_FILE,
            GEN_BINARY_FILE,
        ])
        .await
    }
}

/*
pub async fn run(command: &OutputCommand) -> Result<(), ExitFailure> {
    // Check if the CACHE_FOLDER folder is already created
    create_folder_or_error(CACHE_FOLDER)?;

    // Get the language depending on the extension of the target_file
    let target_file_lang: LanguageHandler = *get_executor_target(command)?;

    let mut cases: VecDeque<PathBuf> = VecDeque::new();
    load_testcases_from_prefix(&mut cases, &command.get_prefix()[..])?;

    let mut test_number: u32 = 0;

    let cases_len: usize = cases.len();

    while (test_number as usize) < cases_len {
        test_number += 1;

        // Load test case in stdin
        let case = cases.pop_front().unwrap();
        copy_file(case.to_str().unwrap(), QTEST_INPUT_FILE)?;

        let response_target = target_file_lang.execute(
            command.get_timeout(),
            command.get_memory_limit(),
            test_number,
        );
        let time_target: Duration = response_target.time;
        let mills_target = time_target.as_millis();

        if is_runtime_error(&response_target.status) {
            show_runtime_error(test_number, mills_target as u32);

            // check if the tle_breck flag is high
            if command.get_break_bad() {
                // remove input, output and error files
                delete_temporary_files_cmp_output().await.ok();
                return Ok(());
            }
            continue;
        } else if is_compiled_error(&response_target.status) {
            return throw_compiler_error_msg("target", "<target-file>");
        } else if is_memory_limit_exceeded(&response_target.status) {
            show_memory_limit_exceeded_error(test_number, mills_target as u32);
            // check if the tle_breck flag is high
            if command.get_break_bad() {
                // remove input, output and error files
                delete_temporary_files_cmp_output().await.ok();
                return Ok(());
            }
            continue;
        }

        if time_target >= Duration::from_millis(command.get_timeout() as u64)
            || is_time_limit_exceeded(&response_target.status)
        {
            show_time_limit_exceeded(test_number, command.get_timeout());
            // check if the tle_breck flag is high
            if command.get_break_bad() {
                // remove input, output and error files
                delete_temporary_files_cmp_output().await.ok();
                return Ok(());
            }
        } else {
            if command.get_save_out() {
                let file_name = &get_filename_output(
                    &command.get_prefix()[..],
                    case.file_name().unwrap().to_str().unwrap(),
                )[..];

                // save testcase
                save_test_case_output(file_name, QTEST_OUTPUT_FILE);
            }
            show_ran_successfully(test_number, mills_target as u32);
        }
    }

    Ok(())
}
*/

async fn delete_temporary_files_cmp_output() -> Result<(), tokio::io::Error> {
    remove_files_async(vec![
        QTEST_INPUT_FILE,
        QTEST_OUTPUT_FILE,
        QTEST_ERROR_FILE,
        TARGET_BINARY_FILE,
        GEN_BINARY_FILE,
    ])
    .await
}
