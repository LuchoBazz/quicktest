use exitfailure::ExitFailure;

use crate::cli::model::cmp_command::CmpCommand;

pub struct CmpController {
    command: CmpCommand,
    // target_file_lang: Option<LanguageHandler>,
    // generator_file_lang: Option<LanguageHandler>,
    // checker_file_lang: Option<LanguageHandler>,
    // cases: VecDeque<PathBuf>,
    // test_number: u32,
    // cases_len: usize,
    // current_case: Option<PathBuf>,
    // state_counter: StateCounter,
}

impl CmpController {
    pub fn new(command: CmpCommand) -> CmpController {
        CmpController {
            command,
            // target_file_lang: None,
            // generator_file_lang: None,
            // checker_file_lang: None,
            // cases: VecDeque::new(),
            // test_number: 0,
            // cases_len: 0,
            // current_case: None,
            // state_counter: StateCounter::default(),
        }
    }

    pub async fn run(&mut self) -> Result<(), ExitFailure> {
        Ok(())
    }
}
