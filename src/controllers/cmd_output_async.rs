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
    command: OutputCommand,
    target_file_lang: Option<LanguageHandler>,
    cases: VecDeque<PathBuf>,
    test_number: u32,
    cases_len: usize,
}

impl OutputController {
    pub fn new(command: OutputCommand) -> OutputController {
        OutputController {
            command,
            target_file_lang: None,
            cases: VecDeque::new(),
            test_number: 0,
            cases_len: 0,
        }
    }

    pub async fn run(&mut self) -> Result<(), ExitFailure> {
        self.create_initial_files()?;
        self.initialize_variables()?;
        self.load_testcases()?;

        self.cases_len = self.cases.len();

        while self.are_tests_pending() {
            self.increment_test_count();

            // Load test case in stdin
            let case = self.cases.pop_front().unwrap();
            copy_file(case.to_str().unwrap(), QTEST_INPUT_FILE)?;

            let response_target = self.get_target_lang_handler().execute(
                self.command.get_timeout(),
                self.command.get_memory_limit(),
                self.test_number,
            );
            let time_target: Duration = response_target.time;
            let mills_target = time_target.as_millis();

            if is_runtime_error(&response_target.status) {
                show_runtime_error(self.test_number, mills_target as u32);

                // check if the tle_breck flag is high
                if self.command.get_break_bad() {
                    // remove input, output and error files
                    self.delete_temporary_files_cmp_output().await.ok();
                    return Ok(());
                }
                continue;
            } else if is_compiled_error(&response_target.status) {
                return throw_compiler_error_msg("target", "<target-file>");
            } else if is_memory_limit_exceeded(&response_target.status) {
                show_memory_limit_exceeded_error(self.test_number, mills_target as u32);
                // check if the tle_breck flag is high
                if self.command.get_break_bad() {
                    // remove input, output and error files
                    self.delete_temporary_files_cmp_output().await.ok();
                    return Ok(());
                }
                continue;
            }

            if time_target >= Duration::from_millis(self.command.get_timeout() as u64)
                || is_time_limit_exceeded(&response_target.status)
            {
                show_time_limit_exceeded(self.test_number, self.command.get_timeout());
                // check if the tle_breck flag is high
                if self.command.get_break_bad() {
                    // remove input, output and error files
                    self.delete_temporary_files_cmp_output().await.ok();
                    return Ok(());
                }
            } else {
                if self.command.get_save_out() {
                    let file_name = &get_filename_output(
                        &self.command.get_prefix()[..],
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

    fn initialize_variables(&mut self) -> Result<(), ExitFailure> {
        // Get the language depending on the extension of the target_file
        let target_file_lang: LanguageHandler = *get_executor_target(&self.command)?;
        self.target_file_lang = Some(target_file_lang);
        Ok(())
    }

    fn get_target_lang_handler(&self) -> LanguageHandler {
        self.target_file_lang.clone().unwrap()
    }

    fn load_testcases(&mut self) -> Result<(), ExitFailure> {
        load_testcases_from_prefix(&mut self.cases, &self.command.get_prefix()[..])?;
        Ok(())
    }

    fn are_tests_pending(&self) -> bool {
        (self.test_number as usize) < self.cases_len
    }

    fn increment_test_count(&mut self) {
        self.test_number += 1;
    }

    async fn delete_temporary_files_cmp_output(&mut self) -> Result<(), tokio::io::Error> {
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
