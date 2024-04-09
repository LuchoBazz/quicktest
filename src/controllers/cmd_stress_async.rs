use std::{collections::VecDeque, path::PathBuf};

use exitfailure::ExitFailure;

use crate::{
    cli::model::{stress_command::StressCommand, traits::AdapterCommand},
    constants::{CACHE_FOLDER, QTEST_INPUT_FILE},
    file_handler::file::{copy_file, create_folder_or_error, load_testcases_from_prefix},
    generator::generator::execute_generator,
    language::{
        get_language::{get_executor_generator, get_executor_target},
        language_handler::LanguageHandler,
    },
    runner::types::Language,
};

pub struct StressController {
    command: StressCommand,
    target_file_lang: Option<LanguageHandler>,
    generator_file_lang: Option<LanguageHandler>,
    cases: VecDeque<PathBuf>,
    test_number: u32,
    cases_len: usize,
    current_case: Option<PathBuf>,
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
        }
    }

    pub async fn run(&mut self) -> Result<(), ExitFailure> {
        self.create_initial_files()?;
        self.initialize_variables()?;
        self.load_testcases()?;

        while self.are_tests_pending() {
            self.increment_test_count();
            self.update_next_case();
            self.load_case_file()?;

            let mut can_continue = false;

            // run generator or load testcases using prefix
            execute_generator(
                &self.get_generator_lang_handler(),
                &self.command,
                &mut self.cases,
                self.test_number,
                &mut can_continue,
            )?;

            if !can_continue {
                break;
            }

            let _response_target = self.get_target_lang_handler().execute(
                self.command.get_timeout(),
                self.command.get_memory_limit(),
                self.test_number,
            );
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

        let generator_file_lang: LanguageHandler = *get_executor_generator(&self.command)?;
        self.generator_file_lang = Some(generator_file_lang);
        Ok(())
    }

    fn load_testcases(&mut self) -> Result<(), ExitFailure> {
        let prefix = &self.command.get_prefix()[..];
        load_testcases_from_prefix(&mut self.cases, prefix)?;
        self.cases_len = self.cases.len();
        Ok(())
    }

    fn are_tests_pending(&self) -> bool {
        (self.test_number as usize) < self.cases_len
    }

    fn increment_test_count(&mut self) {
        self.test_number += 1;
    }

    fn update_next_case(&mut self) {
        self.current_case = self.cases.pop_front();
    }

    fn get_current_case(&self) -> PathBuf {
        self.current_case.clone().unwrap()
    }

    fn load_case_file(&self) -> Result<(), ExitFailure> {
        // Load test case in stdin
        copy_file(self.get_current_case().to_str().unwrap(), QTEST_INPUT_FILE)?;
        Ok(())
    }

    fn get_target_lang_handler(&self) -> LanguageHandler {
        self.target_file_lang.clone().unwrap()
    }

    fn get_generator_lang_handler(&self) -> LanguageHandler {
        self.generator_file_lang.clone().unwrap()
    }
}
