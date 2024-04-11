use std::{collections::VecDeque, path::PathBuf, process, time::Duration};

use exitfailure::ExitFailure;

use crate::{
    cli::model::{stress_command::StressCommand, traits::AdapterCommand},
    constants::{
        CACHE_FOLDER, GEN_BINARY_FILE, PREFIX_AC_FILES, PREFIX_MLE_FILES, PREFIX_RTE_FILES,
        PREFIX_TLE_FILES, QTEST_ERROR_FILE, QTEST_INPUT_FILE, QTEST_OUTPUT_FILE,
        TARGET_BINARY_FILE, TEST_CASES_FOLDER,
    },
    error::handle_error::{throw_break_found_msg, throw_compiler_error_msg},
    file_handler::{
        async_file::remove_files_async,
        file::{
            copy_file, create_folder_or_error, format_filename_test_case,
            load_test_cases_from_status, load_testcases_from_prefix, remove_folder, save_test_case,
        },
    },
    generator::generator::execute_generator,
    language::{
        get_language::{get_executor_generator, get_executor_target},
        language_handler::LanguageHandler,
    },
    runner::{
        state_counter::StateCounter,
        types::{
            is_accepted, is_compiled_error, is_memory_limit_exceeded, is_runtime_error,
            is_time_limit_exceeded, Language, StatusResponse,
        },
    },
    views::style::{
        show_accepted, show_memory_limit_exceeded_error, show_runtime_error, show_stats,
        show_time_limit_exceeded,
    },
};

pub struct StressController {
    command: StressCommand,
    target_file_lang: Option<LanguageHandler>,
    generator_file_lang: Option<LanguageHandler>,
    cases: VecDeque<PathBuf>,
    test_number: u32,
    cases_len: usize,
    current_case: Option<PathBuf>,
    state_counter: StateCounter,
}

impl StressController {
    pub fn new(command: StressCommand) -> StressController {
        StressController {
            command,
            target_file_lang: None,
            generator_file_lang: None,
            cases: VecDeque::new(),
            test_number: 0,
            cases_len: 0,
            current_case: None,
            state_counter: StateCounter::default(),
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

            let response_target: StatusResponse = self.execute_target_handler()?;

            if is_runtime_error(&response_target.status) {
                self.runtime_error_handler(&response_target).await?;
            } else if is_compiled_error(&response_target.status) {
                return throw_compiler_error_msg("target", "<target-file>");
            } else if is_memory_limit_exceeded(&response_target.status) {
                self.memory_limit_exceeded_handler(&response_target).await?;
            } else if self.is_target_time_limit_exceeded(&response_target) {
                self.time_limit_exceeded_handler().await?;
            } else if is_accepted(&response_target.status) {
                self.accepted_handler(&response_target)?;
            }
        }

        self.show_summary();
        self.delete_temporary_files_cmd_stress().await?;
        self.check_errors_and_exit();

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

        let generator_file_lang: LanguageHandler = *get_executor_generator(&self.command)?;
        self.generator_file_lang = Some(generator_file_lang);
        Ok(())
    }

    fn delete_test_case_folder(&self) {
        if self.command.get_save_bad() || self.command.get_save_all() {
            // Remove all previous test cases
            remove_folder(TEST_CASES_FOLDER);
        }
    }

    fn load_testcases(&mut self) -> Result<(), ExitFailure> {
        load_test_cases_from_status(&self.command, &mut self.cases)?;

        let prefix = &self.command.get_prefix()[..];
        load_testcases_from_prefix(&mut self.cases, prefix)?;
        self.cases_len = self.cases.len();
        Ok(())
    }

    fn are_tests_pending(&self) -> bool {
        // self.command.has_test_cases(self.test_number)
        println!(
            "{} {} {}",
            self.test_number,
            self.command.get_test_cases(),
            self.command.can_run_cases()
        );
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

    fn execute_target_handler(&self) -> Result<StatusResponse, ExitFailure> {
        let response_target = self.get_target_lang_handler().execute(
            self.command.get_timeout(),
            self.command.get_memory_limit(),
            self.test_number,
        );
        Ok(response_target)
    }

    fn get_target_lang_handler(&self) -> LanguageHandler {
        self.target_file_lang.clone().unwrap()
    }

    fn get_generator_lang_handler(&self) -> LanguageHandler {
        self.generator_file_lang.clone().unwrap()
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
            self.delete_temporary_files_cmd_stress().await.ok();
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
            self.delete_temporary_files_cmd_stress().await.ok();
            return throw_break_found_msg(
                "Memory Limit Exceeded",
                "MLE",
                self.command.get_test_cases(),
            );
        }
        Ok(())
    }

    fn is_target_time_limit_exceeded(&self, response_target: &StatusResponse) -> bool {
        response_target.time >= Duration::from_millis(self.command.get_timeout() as u64)
            || is_time_limit_exceeded(&response_target.status)
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
            self.delete_temporary_files_cmd_stress().await.ok();
            return Err(failure::err_msg("").into()); // TODO: Errors Refactor
        }
        Ok(())
    }

    fn accepted_handler(&mut self, response_target: &StatusResponse) -> Result<(), ExitFailure> {
        self.state_counter.increase_ac();
        if self.command.get_save_all() {
            // Example: test_cases/testcase_ac_01.txt
            let file_name: &str = &format_filename_test_case(
                TEST_CASES_FOLDER,
                PREFIX_AC_FILES,
                self.state_counter.ac,
            )[..];
            save_test_case(file_name, QTEST_INPUT_FILE);
        }
        let mills_target = response_target.time.as_millis();
        show_accepted(self.test_number, mills_target as u32);
        Ok(())
    }

    fn check_errors_and_exit(&self) {
        // check if the target file has errors
        if self.state_counter.has_stress_command_error() {
            process::exit(exitcode::SOFTWARE);
        }
    }

    fn show_summary(&self) {
        show_stats(
            self.state_counter.ac,
            0,
            self.state_counter.tle,
            self.state_counter.rte,
            self.state_counter.mle,
        );
    }

    async fn delete_temporary_files_cmd_stress(&mut self) -> Result<(), tokio::io::Error> {
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
