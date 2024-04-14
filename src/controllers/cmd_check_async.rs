use std::{collections::VecDeque, path::PathBuf};

use exitfailure::ExitFailure;

use crate::{
    cli::model::{check_command::CheckCommand, traits::AdapterCommand},
    constants::{
        CACHE_FOLDER, CHECKER_BINARY_FILE, GEN_BINARY_FILE, PREFIX_RTE_FILES, QTEST_CHECKER_FILE,
        QTEST_ERROR_FILE, QTEST_INPUT_FILE, QTEST_OUTPUT_FILE, TARGET_BINARY_FILE,
        TEST_CASES_FOLDER,
    },
    file_handler::{
        async_file::remove_files_async,
        file::{
            copy_file, create_folder_or_error, format_filename_test_case,
            load_test_cases_from_status, load_testcases_from_prefix, remove_folder, save_test_case,
        },
    },
    generator::generator::execute_generator,
    language::{
        get_language::{get_executor_checker, get_executor_generator, get_executor_target},
        language_handler::LanguageHandler,
    },
    runner::{
        state_counter::StateCounter,
        types::{
            is_compiled_error, is_memory_limit_exceeded, is_runtime_error, Language, StatusResponse,
        },
    },
    views::style::show_runtime_error,
};

pub struct CheckController {
    command: CheckCommand,
    target_file_lang: Option<LanguageHandler>,
    generator_file_lang: Option<LanguageHandler>,
    checker_file_lang: Option<LanguageHandler>,
    cases: VecDeque<PathBuf>,
    test_number: u32,
    cases_len: usize,
    current_case: Option<PathBuf>,
    state_counter: StateCounter,
}

impl CheckController {
    pub fn new(command: CheckCommand) -> CheckController {
        CheckController {
            command,
            target_file_lang: None,
            generator_file_lang: None,
            checker_file_lang: None,
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
                // return throw_compiler_error_msg("target", "<target-file>");
            } else if is_memory_limit_exceeded(&response_target.status) {
                // self.memory_limit_exceeded_handler(&response_target).await?;
            }
            // else if self.is_target_time_limit_exceeded(&response_target) {
            // self.time_limit_exceeded_handler().await?;
            // }
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

        let checker_file_lang: LanguageHandler = *get_executor_checker(&self.command)?;
        self.checker_file_lang = Some(checker_file_lang);
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
            self.delete_temporary_files_cmd_checker().await.ok();
            return Err(failure::err_msg("").into()); // TODO: Errors Refactor
        }
        Ok(())
    }

    async fn delete_temporary_files_cmd_checker(&mut self) -> Result<(), tokio::io::Error> {
        remove_files_async(vec![
            QTEST_INPUT_FILE,
            QTEST_OUTPUT_FILE,
            QTEST_ERROR_FILE,
            QTEST_CHECKER_FILE,
            TARGET_BINARY_FILE,
            GEN_BINARY_FILE,
            CHECKER_BINARY_FILE,
        ])
        .await
    }
}
