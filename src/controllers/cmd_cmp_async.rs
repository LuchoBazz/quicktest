use std::{collections::VecDeque, path::PathBuf};

use exitfailure::ExitFailure;

use crate::{
    cli::model::{cmp_command::CmpCommand, traits::AdapterCommand},
    constants::{CACHE_FOLDER, QTEST_INPUT_FILE, TEST_CASES_FOLDER},
    file_handler::{
        file::{
            copy_file, create_folder_or_error, load_test_cases_from_status,
            load_testcases_from_prefix, remove_folder,
        },
        path::get_root_path,
    },
    language::{
        get_language::{get_executor_correct, get_executor_generator, get_executor_target},
        language_handler::LanguageHandler,
    },
};

pub struct CmpController {
    command: CmpCommand,
    target_file_lang: Option<LanguageHandler>,
    generator_file_lang: Option<LanguageHandler>,
    correct_file_lang: Option<LanguageHandler>,
    cases: VecDeque<PathBuf>,
    test_number: u32,
    cases_len: usize,
    current_case: Option<PathBuf>,
    // state_counter: StateCounter,
}

impl CmpController {
    pub fn new(command: CmpCommand) -> CmpController {
        CmpController {
            command,
            target_file_lang: None,
            generator_file_lang: None,
            correct_file_lang: None,
            cases: VecDeque::new(),
            test_number: 0,
            cases_len: 0,
            current_case: None,
            // state_counter: StateCounter::default(),
        }
    }

    pub async fn run(&mut self) -> Result<(), ExitFailure> {
        self.delete_test_case_folder();
        self.create_initial_files()?;
        self.initialize_variables()?;
        self.load_testcases()?;

        let _root = &get_root_path()[..];

        while self.are_tests_pending() {
            self.increment_test_count();
            self.update_next_case();
            self.load_case_file()?;
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
}
