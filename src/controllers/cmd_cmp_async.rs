use std::{collections::VecDeque, path::PathBuf, time::Duration};

use exitfailure::ExitFailure;

use crate::{
    cli::model::{cmp_command::CmpCommand, traits::AdapterCommand},
    constants::{
        CACHE_FOLDER, CORRECT_BINARY_FILE, GEN_BINARY_FILE, PREFIX_AC_FILES, PREFIX_MLE_FILES,
        PREFIX_RTE_FILES, PREFIX_TLE_FILES, PREFIX_WA_FILES, QTEST_ERROR_FILE, QTEST_EXPECTED_FILE,
        QTEST_INPUT_FILE, QTEST_OUTPUT_FILE, TARGET_BINARY_FILE, TEST_CASES_FOLDER,
    },
    error::handle_error::{
        throw_break_found_msg, throw_compiler_error_msg, throw_memory_limit_exceeded_msg,
        throw_runtime_error_msg, throw_time_limit_exceeded_msg,
    },
    file_handler::{
        async_file::remove_files_async,
        file::{
            copy_file, create_folder_or_error, format_filename_test_case,
            load_test_cases_from_status, load_testcases_from_prefix, read_file, remove_folder,
            save_test_case,
        },
        path::get_root_path,
    },
    generator::generator::execute_generator,
    language::{
        get_language::{get_executor_correct, get_executor_generator, get_executor_target},
        language_handler::LanguageHandler,
    },
    runner::{
        state_counter::StateCounter,
        types::{
            is_compiled_error, is_memory_limit_exceeded, is_runtime_error, is_time_limit_exceeded,
            Language, StatusResponse,
        },
    },
    views::{
        diff_line_by_line::diff_line_by_line,
        style::{
            show_accepted, show_input_test_case, show_memory_limit_exceeded_error,
            show_runtime_error, show_time_limit_exceeded, show_time_limit_exceeded_correct,
            show_wrong_answer,
        },
    },
};

use super::cmd_cmp::compare_file;

pub struct CmpController {
    command: CmpCommand,
    target_file_lang: Option<LanguageHandler>,
    generator_file_lang: Option<LanguageHandler>,
    correct_file_lang: Option<LanguageHandler>,
    cases: VecDeque<PathBuf>,
    test_number: u32,
    cases_len: usize,
    current_case: Option<PathBuf>,
    state_counter: StateCounter,
    file_in: String,
    file_out: String,
    file_expected: String,
}

impl CmpController {
    pub fn new(command: CmpCommand) -> CmpController {
        let root = get_root_path();

        CmpController {
            command,
            target_file_lang: None,
            generator_file_lang: None,
            correct_file_lang: None,
            cases: VecDeque::new(),
            test_number: 0,
            cases_len: 0,
            current_case: None,
            state_counter: StateCounter::default(),
            file_in: format!("{}/{}", &root[..], QTEST_INPUT_FILE),
            file_out: format!("{}/{}", &root[..], QTEST_INPUT_FILE),
            file_expected: format!("{}/{}", &root[..], QTEST_INPUT_FILE),
        }
    }

    pub async fn run(&mut self) -> Result<(), ExitFailure> {
        self.delete_test_case_folder();
        self.create_initial_files()?;
        self.initialize_variables()?;
        self.load_testcases()?;

        while self.are_tests_pending() {
            self.increment_test_count();
            self.update_next_case();
            self.load_case_file()?;

            let generator_execution_success: bool = self.execute_generator_handler()?;

            if !generator_execution_success {
                break;
            }

            let response_correct: StatusResponse = self.execute_correct_handler()?;

            if is_runtime_error(&response_correct.status) {
                return throw_runtime_error_msg("correct", "<correct-file>");
            } else if is_compiled_error(&response_correct.status) {
                return throw_compiler_error_msg("correct", "<correct-file>");
            } else if is_memory_limit_exceeded(&response_correct.status) {
                return throw_memory_limit_exceeded_msg("correct", "<correct-file>");
            } else if self.is_status_time_limit_exceeded(&response_correct) {
                show_time_limit_exceeded_correct(self.test_number, self.command.get_timeout());
                return throw_time_limit_exceeded_msg("correct", "<correct-file>");
            }

            let response_target: StatusResponse = self.execute_target_handler()?;

            if is_runtime_error(&response_target.status) {
                self.runtime_error_handler(&response_target).await?;
            } else if is_compiled_error(&response_target.status) {
                return throw_compiler_error_msg("target", "<target-file>");
            } else if is_memory_limit_exceeded(&response_target.status) {
                self.memory_limit_exceeded_handler(&response_target).await?;
            } else if self.is_status_time_limit_exceeded(&response_target) {
                self.time_limit_exceeded_handler().await?;
            }

            if compare_file(&self.file_out, &self.file_expected, true) {
                // is OK
                self.accepted_handler(&response_target)?;
            } else {
                // WA found
                self.wrong_answer_handler(&response_target).await?;
            }
        }

        Ok(())
    }

    fn delete_test_case_folder(&self) {
        if self.command.get_save_bad() || self.command.get_save_all() {
            // Remove all previous test cases
            remove_folder(TEST_CASES_FOLDER);
        }
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

        let generator_file_lang: LanguageHandler = *get_executor_generator(&self.command)?;
        self.generator_file_lang = Some(generator_file_lang);

        let correct_file_lang: LanguageHandler = *get_executor_correct(&self.command)?;
        self.correct_file_lang = Some(correct_file_lang);
        Ok(())
    }

    fn load_testcases(&mut self) -> Result<(), ExitFailure> {
        load_test_cases_from_status(&self.command, &mut self.cases)?;

        let prefix = &self.command.get_prefix()[..];
        load_testcases_from_prefix(&mut self.cases, prefix)?;
        self.cases_len = self.cases.len();
        Ok(())
    }

    fn are_tests_pending(&self) -> bool {
        // TODO: check this condition
        self.test_number < self.command.get_test_cases() || self.command.can_run_cases()
    }

    fn increment_test_count(&mut self) {
        self.test_number += 1;
    }

    fn update_next_case(&mut self) {
        self.current_case = self.cases.pop_front();
    }

    fn load_case_file(&self) -> Result<(), ExitFailure> {
        // Load test case in stdin
        if let Some(case) = self.current_case.clone() {
            copy_file(case.to_str().unwrap(), QTEST_INPUT_FILE)?;
        }
        Ok(())
    }

    fn get_generator_lang_handler(&self) -> LanguageHandler {
        self.generator_file_lang.clone().unwrap()
    }

    fn execute_generator_handler(&mut self) -> Result<bool, ExitFailure> {
        let mut can_continue = false;

        // run generator or load testcases using prefix
        execute_generator(
            &self.get_generator_lang_handler(),
            &self.command,
            &mut self.cases,
            self.test_number,
            &mut can_continue,
        )?;

        Ok(can_continue)
    }

    fn get_correct_lang_handler(&self) -> LanguageHandler {
        self.correct_file_lang.clone().unwrap()
    }

    fn execute_correct_handler(&self) -> Result<StatusResponse, ExitFailure> {
        let response_correct = self.get_correct_lang_handler().execute(
            self.command.get_timeout(),
            self.command.get_memory_limit(),
            self.test_number,
        );
        Ok(response_correct)
    }

    fn is_status_time_limit_exceeded(&self, response_target: &StatusResponse) -> bool {
        response_target.time >= Duration::from_millis(self.command.get_timeout() as u64)
            || is_time_limit_exceeded(&response_target.status)
    }

    fn get_target_lang_handler(&self) -> LanguageHandler {
        self.target_file_lang.clone().unwrap()
    }

    fn execute_target_handler(&self) -> Result<StatusResponse, ExitFailure> {
        let response_target = self.get_target_lang_handler().execute(
            self.command.get_timeout(),
            self.command.get_memory_limit(),
            self.test_number,
        );
        Ok(response_target)
    }

    async fn runtime_error_handler(
        &mut self,
        response_target: &StatusResponse,
    ) -> Result<(), ExitFailure> {
        self.state_counter.increase_rte();
        let mills_target = response_target.time.as_millis();
        show_runtime_error(self.test_number, mills_target as u32);

        // Save the input of the test case that gave status tle
        if self.command.get_save_bad() || self.command.get_save_all() {
            // Example: test_cases/testcase_rte_01.txt
            let file_name: &str = &format_filename_test_case(
                TEST_CASES_FOLDER,
                PREFIX_RTE_FILES,
                self.state_counter.rte,
            )[..];
            // save testcase
            save_test_case(file_name, QTEST_INPUT_FILE);
        }

        // check if the tle_breck flag is high
        if self.command.get_break_bad() {
            // remove input, output and error files
            self.delete_temporary_files_cmd_cmp().await.ok();
            return Err(failure::err_msg("").into()); // TODO: Errors Refactor
        }
        Ok(())
    }

    async fn memory_limit_exceeded_handler(
        &mut self,
        response_target: &StatusResponse,
    ) -> Result<(), ExitFailure> {
        self.state_counter.increase_mle();
        let mills_target = response_target.time.as_millis();
        show_memory_limit_exceeded_error(self.test_number, mills_target as u32);

        if self.command.get_save_bad() || self.command.get_save_all() {
            // Example: test_cases/testcase_mle_01.txt
            let file_name: &str = &format_filename_test_case(
                TEST_CASES_FOLDER,
                PREFIX_MLE_FILES,
                self.state_counter.mle,
            )[..];
            // save testcase
            save_test_case(file_name, QTEST_INPUT_FILE);
        }

        // check if the tle_breck flag is high
        if self.command.get_break_bad() {
            // remove input, output and error files
            self.delete_temporary_files_cmd_cmp().await.ok();
            return throw_break_found_msg(
                "Memory Limit Exceeded",
                "MLE",
                self.command.get_test_cases(),
            );
        }
        Ok(())
    }

    async fn time_limit_exceeded_handler(&mut self) -> Result<(), ExitFailure> {
        self.state_counter.increase_tle();
        show_time_limit_exceeded(self.test_number, self.command.get_timeout());

        // Save the input of the test case that gave status tle
        if self.command.get_save_bad() || self.command.get_save_all() {
            // Example: test_cases/testcase_tle_01.txt
            let file_name: &str = &format_filename_test_case(
                TEST_CASES_FOLDER,
                PREFIX_TLE_FILES,
                self.state_counter.tle,
            )[..];
            // save testcase
            save_test_case(file_name, QTEST_INPUT_FILE);
        }

        // check if the tle_breck flag is high
        if self.command.get_break_bad() {
            // remove input, output and error files
            self.delete_temporary_files_cmd_cmp().await.ok();
            return Err(failure::err_msg("").into()); // TODO: Errors Refactor
        }
        Ok(())
    }

    fn accepted_handler(&mut self, response_target: &StatusResponse) -> Result<(), ExitFailure> {
        self.state_counter.increase_ac();
        let mills_target = response_target.time.as_millis();
        show_accepted(self.test_number, mills_target as u32);

        if self.command.get_save_all() {
            // Example: test_cases/testcase_ac_01.txt
            let file_name: &str = &format_filename_test_case(
                TEST_CASES_FOLDER,
                PREFIX_AC_FILES,
                self.state_counter.ac,
            )[..];
            // save testcase
            save_test_case(file_name, QTEST_INPUT_FILE);
        }
        Ok(())
    }

    async fn wrong_answer_handler(
        &mut self,
        response_target: &StatusResponse,
    ) -> Result<(), ExitFailure> {
        self.state_counter.increase_wa();
        let mills_target = response_target.time.as_millis();
        show_wrong_answer(self.test_number, mills_target as u32);

        if self.command.get_diff() {
            let mut tout = std::io::stdout();
            let input = read_file(&self.file_in[..]).unwrap();
            let expected = read_file(&self.file_expected[..]).unwrap();
            let output = read_file(&self.file_out[..]).unwrap();
            show_input_test_case(&mut tout, &input[..]);
            diff_line_by_line(&mut tout, &expected[..], &output[..]);
        }

        if self.command.get_save_bad() || self.command.get_save_all() {
            // Example: test_cases/testcase_wa_01.txt
            let file_name: &str = &format_filename_test_case(
                TEST_CASES_FOLDER,
                PREFIX_WA_FILES,
                self.state_counter.wa,
            )[..];
            // save testcase
            save_test_case(file_name, QTEST_INPUT_FILE);
        }

        if self.command.get_break_bad() {
            // remove input, output and error files
            self.delete_temporary_files_cmd_cmp().await.ok();
            return throw_break_found_msg("Wrong Answer", "WA", self.command.get_test_cases());
        }
        Ok(())
    }

    async fn delete_temporary_files_cmd_cmp(&mut self) -> Result<(), tokio::io::Error> {
        remove_files_async(vec![
            QTEST_INPUT_FILE,
            QTEST_OUTPUT_FILE,
            QTEST_ERROR_FILE,
            QTEST_EXPECTED_FILE,
            TARGET_BINARY_FILE,
            GEN_BINARY_FILE,
            CORRECT_BINARY_FILE,
        ])
        .await
    }
}
