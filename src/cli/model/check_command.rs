/*
*  Quick Test: CLI for stress testing in competitive programming
*  Copyright (C) 2021 - Luis Miguel Báez
*  License: MIT (See the LICENSE file in the repository root directory)
*/

use std::path::PathBuf;

use super::traits::AdapterCommand;

pub struct CheckCommand {
    target_file: PathBuf,
    checker_file: PathBuf,
    gen_file: PathBuf,
    timeout: u32,
    memory_limit: u64,
    test_cases: u32,
    prefix: String,
    break_bad: bool,
    save_bad: bool,
    save_all: bool,
    run_all: bool,
    run_ac: bool,
    run_wa: bool,
    run_tle: bool,
    run_rte: bool,
    run_mle: bool,
}

#[allow(clippy::too_many_arguments)]
impl CheckCommand {
    pub fn new(
        target_file: PathBuf,
        checker_file: PathBuf,
        gen_file: PathBuf,
        timeout: u32,
        memory_limit: u64,
        test_cases: u32,
        prefix: String,
        break_bad: bool,
        save_bad: bool,
        save_all: bool,
        run_all: bool,
        run_ac: bool,
        run_wa: bool,
        run_tle: bool,
        run_rte: bool,
        run_mle: bool,
    ) -> CheckCommand {
        CheckCommand {
            target_file,
            checker_file,
            gen_file,
            timeout,
            memory_limit,
            test_cases,
            prefix,
            break_bad,
            save_bad,
            save_all,
            run_all,
            run_ac,
            run_wa,
            run_tle,
            run_rte,
            run_mle,
        }
    }
}

impl AdapterCommand for CheckCommand {
    fn get_target_file(&self) -> PathBuf {
        self.target_file.clone()
    }
    fn get_checker_file(&self) -> PathBuf {
        self.checker_file.clone()
    }
    fn get_generator_file(&self) -> PathBuf {
        self.gen_file.clone()
    }
    fn get_timeout(&self) -> u32 {
        self.timeout
    }
    fn get_memory_limit(&self) -> u64 {
        self.memory_limit
    }
    fn get_test_cases(&self) -> u32 {
        self.test_cases
    }
    fn get_break_bad(&self) -> bool {
        self.break_bad
    }
    fn get_save_bad(&self) -> bool {
        self.save_bad
    }
    fn get_save_all(&self) -> bool {
        self.save_all
    }
    fn get_run_all(&self) -> bool {
        self.run_all
    }
    fn get_run_ac(&self) -> bool {
        self.run_ac
    }
    fn get_run_wa(&self) -> bool {
        self.run_wa
    }
    fn get_run_tle(&self) -> bool {
        self.run_tle
    }
    fn get_run_rte(&self) -> bool {
        self.run_rte
    }
    fn get_run_mle(&self) -> bool {
        self.run_mle
    }
    fn get_prefix(&self) -> String {
        self.prefix.clone()
    }
    fn can_run_cases(&self) -> bool {
        self.get_run_all()
            || self.get_run_ac()
            || self.get_run_wa()
            || self.get_run_tle()
            || self.get_run_rte()
            || self.get_run_mle()
            || !self.get_prefix().is_empty()
    }
    fn has_test_cases(&self, test_number: u32) -> bool {
        test_number < self.get_test_cases() || self.can_run_cases()
    }
}
